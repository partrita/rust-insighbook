use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    sync::{Arc, Mutex},
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

    // 처리 대상 파일 배열
    let mut all_paths = vec![];
    for entry in read_dir(&args.input).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // 폴더는 대상에서 제외
            continue;
        }
        all_paths.push(path);
    }

    let processed_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for chunk in all_paths.chunks((all_paths.len() + 3) / 4) {
        let chunk = chunk.to_vec();
        let processed_count = processed_count.clone();
        let output = args.output.clone();
        handles.push(thread::spawn(move || {
            // let mut local_count = 0; // 삭제
            for path in chunk {
                let output_path = output.join(path.file_name().unwrap());
                let img = image::open(&path); // 이미지 파일 읽기
                if let Ok(img) = img {
                    let thumbnail = img.thumbnail(64, 64); // 썸네일 만들기
                    thumbnail.save(output_path).unwrap(); // 파일 저장
                    // local_count += 1; // 다음처럼 수정
                    let mut writer = processed_count.lock().unwrap();
                    *writer += 1;
                }
            }
            // 삭제
            // let mut writer = processed_count.lock().unwrap();
            // *writer += local_count;
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Processed {} images",
        processed_count.as_ref().lock().unwrap()
    );
}
