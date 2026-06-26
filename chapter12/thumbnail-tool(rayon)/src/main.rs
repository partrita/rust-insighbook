// clap 크레이트에서 명령줄 인수를 분석하기 위한 Parser 매크로를 가져옵니다.
use clap::Parser;
// 병렬 반복자(Parallel Iterator) 사용을 위해 rayon 크레이트의 IntoParallelIterator 트레이트를 가져옵니다.
use rayon::iter::IntoParallelIterator;
// rayon의 다양한 병렬 작업 메서드들을 사용할 수 있도록 모든 Prelude 모듈을 가져옵니다.
use rayon::prelude::*;
use std::{
    // 파일 시스템에서 디렉터리를 만들고 읽기 위한 함수들을 가져옵니다.
    fs::{create_dir_all, read_dir},
    // 파일 경로를 나타내며 가공할 수 있는 PathBuf 구조체를 가져옵니다.
    path::PathBuf,
};

// Parser 매크로를 이용해 인수를 정의합니다.
#[derive(Parser)]
struct Args {
    /// 썸네일 작성 대상 이미지 폴더
    input: PathBuf,
    /// 썸네일을 저장할 폴더
    output: PathBuf,
}

fn main() {
    // 프로그램 실행 시 들어온 명령줄 인수를 분석합니다.
    let args = Args::parse();

    // 썸네일을 저장할 출력 대상 폴더를 생성합니다. (존재하지 않으면 부모 폴더까지 전부 생성)
    create_dir_all(&args.output).unwrap();

    // 입력 디렉터리 내의 모든 항목을 읽어서 Vec 구조에 모읍니다(collect).
    // read_dir()이 성공하면 Result에서 ReadDir 반복자를 가져오며, collect()를 통해 Vec<Result<DirEntry, Error>> 타입으로 변환됩니다.
    let items: Vec<_> = read_dir(&args.input).unwrap().collect();

    // items 벡터를 rayon의 병렬 반복자(Parallel Iterator)인 ParallelIterator 형태로 변환합니다.
    // into_par_iter()는 내부적으로 Rayon의 글로벌 스레드 풀을 이용하여 반복 작업을 여러 스레드에 나누어 병렬로 처리합니다.
    let result = items.into_par_iter().map(|item| {
        // Result 타입인 item의 포장을 unwrap()으로 풀어 실제 DirEntry 데이터를 얻습니다.
        let item = item.unwrap();
        // 파일 또는 폴더의 절대/상대 경로를 가져옵니다.
        let path = item.path();

        // 만약 대상이 폴더인 경우에는 처리를 건너뛰고 0을 반환합니다. (이미지만 처리해야 하므로)
        if path.is_dir() {
            return 0;
        }

        // 출력할 디렉터리에 이미지 파일 이름만 붙여서 최종 저장 파일 경로를 작성합니다.
        let output_path = args.output.join(path.file_name().unwrap());

        // 이미지 파일을 엽니다. (디코딩 및 읽기 작업)
        let img = image::open(&path);

        // 이미지 오픈에 성공하면 썸네일을 만들어 저장하고, 성공의 의미로 1을 반환합니다.
        if let Ok(img) = img {
            // 이미지 비율을 유지하면서 64x64 크기로 축소합니다.
            let thumbnail = img.thumbnail(64, 64);
            // 썸네일을 출력 파일 경로에 저장합니다.
            thumbnail.save(output_path).unwrap();
            1 // 성공 카운트 리턴
        } else {
            0 // 이미지 파일 열기에 실패했거나 이미지가 아니면 0 리턴
        }
    });

    // 병렬 작업의 결과(각 스레드가 리턴한 1 또는 0)를 모두 더해(sum) 총 몇 개의 이미지가 처리되었는지 구합니다.
    // sum 메서드는 병렬 반복을 종결(Reduce)짓고 최종 합계를 u32 타입으로 계산합니다.
    println!("Processed {} images", result.sum::<u32>());
}
