use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
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

    let mut processed_count = 0;
    for item in read_dir(&args.input).unwrap() {
        let item = item.unwrap();
        let input_path = item.path();
        if input_path.is_dir() {
            // 폴더는 대상에서 제외
            continue;
        }

        let img = image::open(&input_path); // 이미지 파일 읽기
        if let Ok(img) = img {
            let thumbnail = img.thumbnail(64, 64); // 썸네일 만들기
            let output_path = args.output.join(input_path.file_name().unwrap());
            thumbnail.save(output_path).unwrap(); // 파일 저장
            processed_count += 1;
        }
    }

    println!("Processed {} images", processed_count);
}
