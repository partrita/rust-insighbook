use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    sync::mpsc::channel,
    thread,
};

#[derive(Parser)]
struct Args {
    /// 썸네일 작성 대상 이미지 폴더
    input: PathBuf,
    /// 썸네일을 저장할 폴더
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    // 출력 대상 폴더 작성
    create_dir_all(&args.output).unwrap();

    let mut handles = vec![];
    let mut channels = vec![];
    let (counter_tx, counter_rx) = channel::<usize>();

    // 수신 측 = 썸네일 작성 처리 쪽 시작
    for _ in 0..4 {
        let (tx, rx) = channel::<PathBuf>();
        channels.push(tx);
        let counter_tx = counter_tx.clone();
        let output = args.output.clone();
        handles.push(thread::spawn(move || {
            while let Ok(path) = rx.recv() {
                let output_path = output.join(path.file_name().unwrap());
                let img = image::open(&path);
                if let Ok(img) = img {
                    let thumbnail = img.thumbnail(64, 64);
                    thumbnail.save(output_path).unwrap();

                    counter_tx.send(1).unwrap();
                }
            }
        }));
    }

    // 송신 측은 이미지 파일 경로를 전송함
    for (index, item) in read_dir(&args.input).unwrap().enumerate() {
        let item = item.unwrap();
        let path = item.path();
        if path.is_dir() {
            continue;
        }
        channels[index % channels.len()].send(path).unwrap();
    }

    // 처리 완료 알림
    for channel in channels {
        drop(channel);
    }
    drop(counter_tx);

    println!("Processed {} images", counter_rx.iter().count());
}
