// clap 크레이트에서 명령줄 인수를 편리하게 파싱하기 위한 Parser 매크로를 가져옵니다.
use clap::Parser;
use std::{
    // 파일 시스템에서 디렉터리를 생성하고 디렉터리 내 목록을 읽는 함수를 가져옵니다.
    fs::{create_dir_all, read_dir},
    // 파일 경로 조작을 위해 PathBuf 구조체를 사용합니다.
    path::PathBuf,
    // 여러 스레드가 동시에 안전하게 카운트 값을 공유하고 수정하도록 Arc와 Mutex를 가져옵니다.
    sync::{Arc, Mutex},
    // 스레드 생성을 위해 thread 모듈을 가져옵니다.
    thread,
};

// Parser 매크로를 사용하여 구조체Args를 명령줄 파서 구조체로 만듭니다.
#[derive(Parser)]
struct Args {
    /// 썸네일 작성 대상 이미지 폴더
    input: PathBuf,
    /// 썸네일을 저장할 폴더
    output: PathBuf,
}

fn main() {
    // 명령줄 인수를 분석하여 변수 args에 바인딩합니다.
    let args = Args::parse();

    // 썸네일을 저장할 폴더를 디스크 상에 생성합니다. (이미 존재하면 아무 일도 하지 않음)
    create_dir_all(&args.output).unwrap();

    // 입력 디렉터리 안의 모든 파일 경로를 모을 가변 벡터를 생성합니다.
    let mut all_paths = vec![];
    // 입력 디렉터리 내부 파일 목록을 순회합니다.
    for entry in read_dir(&args.input).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // 폴더인 경우에는 파일만 처리해야 하므로 추가하지 않고 넘어갑니다.
        if path.is_dir() {
            continue;
        }
        all_paths.push(path);
    }

    // 처리된 이미지 개수를 누적할 가변 카운터를 생성하고,
    // 여러 스레드가 동시에 접근해 수정할 수 있도록 Mutex 및 Arc로 안전하게 감쌉니다.
    let processed_count = Arc::new(Mutex::new(0));

    // 워커 스레드들의 JoinHandle을 저장할 벡터입니다.
    let mut handles = vec![];

    // 전체 경로 리스트(all_paths)를 4개의 분량(청크)으로 쪼갭니다.
    // (all_paths.len() + 3) / 4는 4개 스레드로 균등 분할하기 위해 올림(ceiling) 연산을 처리한 크기입니다.
    for chunk in all_paths.chunks((all_paths.len() + 3) / 4) {
        // chunk는 &[PathBuf] 슬라이스 형태이므로, 스레드 내부로 값을 넘기기 위해 새로운 벡터로 복사(to_vec)합니다.
        let chunk = chunk.to_vec();
        // 각 스레드가 전역 카운터의 락을 얻을 수 있도록 Arc 포인터를 복제합니다.
        let processed_count = processed_count.clone();
        // 저장 폴더의 경로를 각 스레드로 넘겨주기 위해 복제합니다.
        let output = args.output.clone();

        // 스레드를 새롭게 스폰하고 핸들을 벡터에 저장합니다.
        // move 키워드는 chunk, processed_count, output의 소유권을 스레드 안으로 이동시킵니다.
        handles.push(thread::spawn(move || {
            // 할당된 청크 안의 파일 경로들을 순회하며 작업을 수행합니다.
            for path in chunk {
                // 출력 폴더 밑에 원본 파일명을 붙인 저장 파일 경로를 작성합니다.
                let output_path = output.join(path.file_name().unwrap());
                // 이미지 파일을 읽습니다.
                let img = image::open(&path);

                // 이미지 로드에 성공했다면 썸네일을 생성하고 파일에 저장합니다.
                if let Ok(img) = img {
                    let thumbnail = img.thumbnail(64, 64); // 64x64 크기로 썸네일 렌더링
                    thumbnail.save(output_path).unwrap(); // 로컬 디스크에 파일 저장

                    // 전역 카운터(processed_count)의 잠금(lock)을 획득합니다.
                    // 이 락은 lock() 호출 시점부터 MutexGuard인 writer가 활성 상태인 동안 유지됩니다.
                    let mut writer = processed_count.lock().unwrap();
                    // 내부의 정수 값을 역참조(*)하여 1 증가시킵니다.
                    *writer += 1;
                    // 이 반복문의 중괄호가 끝날 때 writer의 라이프타임이 끝나며 자동으로 락이 해제되어
                    // 다른 스레드가 대기 상태에서 풀려나 값을 수정할 수 있습니다.
                }
            }
        }));
    }

    // 메인 스레드가 각 워커 스레드의 실행이 모두 완료될 때까지 블로킹 상태로 기다립니다.
    for handle in handles {
        handle.join().unwrap();
    }

    // 최종적으로 안전하게 누적된 완료 카운트 값을 콘솔에 출력합니다.
    // as_ref()로 Arc가 가리키는 Mutex 참조를 얻은 뒤, lock().unwrap()으로 락을 얻어 최종 카운트 값을 읽습니다.
    println!(
        "Processed {} images",
        processed_count.as_ref().lock().unwrap()
    );
}
