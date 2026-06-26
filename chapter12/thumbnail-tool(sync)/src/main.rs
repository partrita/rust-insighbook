// clap 크레이트에서 명령줄 인수 파싱을 도와주는 Parser 매크로를 가져옵니다.
use clap::Parser;
use std::{
    // 파일 시스템(fs) 모듈에서 폴더를 일괄 생성하는 create_dir_all과 디렉터리 내 목록을 읽는 read_dir 함수를 가져옵니다.
    fs::{create_dir_all, read_dir},
    // 파일이나 디렉터리 경로를 나타내며 수정/생성이 가능한 PathBuf 구조체를 가져옵니다.
    path::PathBuf,
};

// Parser 매크로를 유도(Derive)하여 명령줄 인수 구조체를 정의합니다.
// 이렇게 하면 자동으로 명령줄 인수 분석 코드가 빌드되어 편리하게 사용 가능합니다.
#[derive(Parser)]
struct Args {
    /// 썸네일을 생성할 대상 이미지들이 있는 입력 폴더 경로
    input: PathBuf,
    /// 생성된 썸네일 이미지를 저장할 출력 폴더 경로
    output: PathBuf,
}

fn main() {
    // 프로그램 실행 시 전달받은 명령줄 인수를 분석하여 Args 구조체 인스턴스로 파싱합니다.
    let args = Args::parse();

    // 썸네일을 저장할 출력 대상 폴더를 생성합니다.
    // 중간 경로에 없는 폴더들도 한 번에 생성(mkdir -p)하며, 실패 시 unwrap()으로 패닉을 발생시킵니다.
    create_dir_all(&args.output).unwrap();

    // 처리된 이미지 개수를 세기 위한 카운터 변수를 초기화합니다.
    let mut processed_count = 0;

    // 입력 디렉터리 내부를 읽고, 각 항목(파일 또는 폴더)을 하나씩 순회합니다.
    // read_dir은 Result를 반환하므로 unwrap()으로 성공 시의 ReadDir 반복자를 가져옵니다.
    for item in read_dir(&args.input).unwrap() {
        // 반복자가 반환하는 각 항목도 Result 타입이므로 unwrap()을 통해 DirEntry를 가져옵니다.
        let item = item.unwrap();
        // DirEntry로부터 파일이나 폴더의 전체 경로를 PathBuf 형태로 추출합니다.
        let input_path = item.path();

        // 경로가 디렉터리(폴더)인지 확인합니다.
        if input_path.is_dir() {
            // 폴더인 경우에는 썸네일 작성 대상에서 제외하고 다음 루프로 넘어갑니다.
            continue;
        }

        // image 크레이트를 사용하여 지정된 경로의 이미지 파일을 읽어옵니다.
        // 이 작업은 동기(Synchronous) 방식으로 작동하며, 파일 로딩 및 디코딩을 수행합니다.
        let img = image::open(&input_path);

        // 이미지 읽기에 성공하여 Result가 Ok(DynamicImage) 형태인 경우에만 블록을 실행합니다.
        if let Ok(img) = img {
            // 가로 64, 세로 64 크기의 썸네일 이미지를 생성합니다.
            // 비율을 유지하면서 썸네일을 제작하는 동기 처리 방식입니다.
            let thumbnail = img.thumbnail(64, 64);

            // 출력 폴더 경로 뒤에 원본 파일명을 붙여 저장할 전체 경로를 생성합니다.
            // input_path.file_name()은 Option<&OsStr>을 반환하므로 unwrap()으로 파일명을 꺼냅니다.
            let output_path = args.output.join(input_path.file_name().unwrap());

            // 생성된 썸네일을 지정된 출력 경로에 파일로 저장합니다.
            // 저장 실패 시 unwrap()으로 프로그램을 정지시킵니다.
            thumbnail.save(output_path).unwrap();

            // 성공적으로 저장한 경우 처리된 파일 카운트를 1 증가시킵니다.
            processed_count += 1;
        }
    }

    // 모든 작업이 끝난 후 최종적으로 처리 완료된 이미지 개수를 콘솔에 출력합니다.
    println!("Processed {} images", processed_count);
}
