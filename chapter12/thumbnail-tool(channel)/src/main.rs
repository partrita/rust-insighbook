// clap 크레이트에서 명령줄 인수 분석을 위해 Parser 매크로를 가져옵니다.
use clap::Parser;
use std::{
    // 파일 시스템에서 디렉터리를 만들고 읽기 위한 함수들을 가져옵니다.
    fs::{create_dir_all, read_dir},
    // 파일 경로 조작을 위해 PathBuf 구조체를 사용합니다.
    path::PathBuf,
    // 스레드 간 통신을 위해 다중 생산자, 단일 소비자(Multi-producer, single-consumer) 채널을 가져옵니다.
    sync::mpsc::channel,
    // 병렬 처리를 위해 OS 스레드 모듈을 가져옵니다.
    thread,
};

// 명령줄 인수를 저장할 Args 구조체를 선언하고 clap 파서 동작을 유도합니다.
#[derive(Parser)]
struct Args {
    /// 썸네일 작성 대상 이미지 폴더
    input: PathBuf,
    /// 썸네일을 저장할 폴더
    output: PathBuf,
}

fn main() {
    // 명령줄 인수를 분석하여 args에 담습니다.
    let args = Args::parse();

    // 썸네일 이미지를 저장할 폴더를 일괄 생성합니다. 실패 시 프로그램이 종료됩니다.
    create_dir_all(&args.output).unwrap();

    // 생성된 워커(Worker) 스레드들의 조인 핸들을 담을 벡터입니다.
    let mut handles = vec![];
    // 각 스레드로 작업(이미지 경로)을 보낼 송신단(Sender)들을 담을 벡터입니다.
    let mut channels = vec![];
    // 처리된 이미지 개수를 누적해서 메인 스레드에 전달할 채널을 생성합니다.
    // counter_tx는 각 스레드가 작업을 끝낼 때마다 1을 송신하고, counter_rx는 이를 수신합니다.
    let (counter_tx, counter_rx) = channel::<usize>();

    // 4개의 워커 스레드를 생성하여 병렬로 이미지를 처리하도록 세팅합니다.
    for _ in 0..4 {
        // 개별 워커 스레드와 통신할 이미지 경로 전송 채널(tx, rx)을 생성합니다.
        let (tx, rx) = channel::<PathBuf>();
        // 메인 스레드가 각 워커에 작업을 분배할 수 있도록 송신기(tx)를 외부 벡터에 보관합니다.
        channels.push(tx);

        // 각 스레드가 완료 신호를 보낼 수 있도록 카운터 송신기(counter_tx)를 복제합니다.
        let counter_tx = counter_tx.clone();
        // 각 스레드에서 저장할 출력 디렉터리 경로를 복제합니다.
        let output = args.output.clone();

        // 스레드를 스폰하여 실행합니다. move 키워드로 클론된 변수들의 소유권을 스레드 내부로 보냅니다.
        handles.push(thread::spawn(move || {
            // rx.recv()는 채널이 닫히기 전까지 메시지가 올 때까지 대기(블로킹)합니다.
            // 송신단(tx)이 모두 drop되어 채널이 닫히면 Ok(path) 대신 에러가 반환되어 while 문이 종료됩니다.
            while let Ok(path) = rx.recv() {
                // 원본 파일명을 출력 폴더 경로에 연결하여 최종 저장 경로를 만듭니다.
                let output_path = output.join(path.file_name().unwrap());
                // 이미지 파일을 읽습니다.
                let img = image::open(&path);

                // 이미지 열기에 성공했을 경우에만 썸네일 처리 로직을 실행합니다.
                if let Ok(img) = img {
                    // 가로세로 64 크기로 썸네일을 생성합니다.
                    let thumbnail = img.thumbnail(64, 64);
                    // 생성된 썸네일 이미지를 파일로 저장합니다.
                    thumbnail.save(output_path).unwrap();

                    // 성공적으로 이미지를 하나 처리했으므로 카운터 송신기를 통해 '1'을 전송합니다.
                    counter_tx.send(1).unwrap();
                }
            }
        }));
    }

    // 메인 스레드가 입력 디렉터리 안의 이미지들을 하나씩 읽으면서 채널을 통해 워커 스레드들에게 분배합니다.
    // enumerate()를 사용해 이미지의 순서(index)를 가져와서 라운드 로빈(Round Robin) 방식으로 고르게 나눕니다.
    for (index, item) in read_dir(&args.input).unwrap().enumerate() {
        let item = item.unwrap();
        let path = item.path();
        // 폴더인 경우는 썸네일 제작 대상에서 스킵합니다.
        if path.is_dir() {
            continue;
        }
        // 인덱스를 채널 개수로 나눈 나머지를 사용해 워커 스레드들에 번갈아가며 이미지 경로를 송신합니다.
        channels[index % channels.len()].send(path).unwrap();
    }

    // 메인 스레드가 모든 파일 전송을 마쳤으므로, 작업 채널들의 송신기를 명시적으로 소멸(drop)시킵니다.
    // 송신기가 다 닫히면 워커 스레드들의 rx.recv()가 에러를 반환하며 while 루프가 끝나고, 각 스레드는 종료됩니다.
    for channel in channels {
        drop(channel);
    }

    // 메인 스레드가 가지고 있던 원본 counter_tx 역시 소멸(drop)시켜야 합니다.
    // 이를 삭제해야 counter_rx가 더 이상 수신할 메시지가 없음을 인지하고 대기를 끝낼 수 있습니다.
    drop(counter_tx);

    // counter_rx.iter()는 채널로 전송되는 데이터에 대한 반복자(Iterator)를 제공합니다.
    // 이 반복자는 모든 counter_tx 송신단이 소멸할 때까지 데이터를 계속 꺼내며 대기하다가 채널이 닫히면 종료됩니다.
    // count() 메서드는 반복자가 처리한 총 아이템 개수를 리턴하므로 최종 완료된 이미지 개수가 됩니다.
    println!("Processed {} images", counter_rx.iter().count());
}
