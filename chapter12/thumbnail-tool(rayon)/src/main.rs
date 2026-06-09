use clap::Parser;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
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

    // 처리 대상 파일 배열
    let items: Vec<_> = read_dir(&args.input).unwrap().collect();
    let result = items.into_par_iter().map(|item| {
        let item = item.unwrap();
        let path = item.path();
        let output_path = args.output.join(path.file_name().unwrap());
        let img = image::open(&path);
        if let Ok(img) = img {
            let thumbnail = img.thumbnail(64, 64);
            thumbnail.save(output_path).unwrap();
            1
        } else {
            0
        }
    });

    println!("Processed {} images", result.sum::<u32>());
}
