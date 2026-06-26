use std::{collections::HashMap, fs::OpenOptions};

use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};

// App 구조체에 Parser 매크로를 유도(derive)하여 커맨드라인 애플리케이션의 시작점으로 설정합니다.
// 버전 정보를 "1.0"으로 지정합니다.
#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    // 세부 명령 처리를 위한 서브커맨드 구조체(Command)를 포함합니다.
    #[clap(subcommand)]
    command: Command,
}

// 하위 명령어(서브커맨드)들을 정의하는 열거형(Enum)입니다.
#[derive(Subcommand)]
enum Command {
    /// 신규 계좌 작성 (NewArgs 인자를 받음)
    New(NewArgs),
    /// 계좌 입금 (DepositArgs 인자를 받음)
    Deposit(DepositArgs),
    /// 계좌 출금 (WithdrawArgs 인자를 받음)
    Withdraw(WithdrawArgs),
    /// CSV에서 가져오기 (ImportArgs 인자를 받음)
    Import(ImportArgs),
    /// 리포트 작성 (ReportArgs 인자를 받음)
    Report(ReportArgs),
}

// 신규 계좌 생성 명령어의 인자를 정의하는 구조체입니다.
#[derive(Args)]
struct NewArgs {
    // 계좌 이름 (예: 사용자 이름 또는 파일명으로 활용됨)
    account_name: String,
}

impl NewArgs {
    // 신규 계좌 생성 명령을 실행하는 함수입니다.
    fn run(&self) {
        // 계좌 이름에 ".csv" 확장자를 붙여 파일명을 생성합니다.
        let file_name = format!("{}.csv", self.account_name);
        // 주어진 파일명으로 새로운 CSV 파일을 쓰기 모드로 엽니다. (에러 발생 시 unwrap으로 패닉)
        let mut writer = Writer::from_path(file_name).unwrap();
        // 헤더 레코드인 "날짜", "용도", "금액" 열을 파일에 기록합니다.
        writer.write_record(["날짜", "용도", "금액"]).unwrap();
        // 버퍼에 남아있는 모든 데이터를 파일에 완전히 저장(플러시)합니다.
        writer.flush().unwrap();
    }
}

// 입금 명령어의 인자를 정의하는 구조체입니다.
#[derive(Args)]
struct DepositArgs {
    account_name: String, // 계좌 이름
    date: NaiveDate,      // 입금 날짜 (YYYY-MM-DD)
    usage: String,        // 입금 용도
    amount: u32,          // 입금 금액 (양수)
}

impl DepositArgs {
    // 입금 명령을 실행하는 함수입니다.
    fn run(&self) {
        // 기존 파일 끝에 데이터를 추가하기 위해 OpenOptions를 생성하고
        // 쓰기 권한(write)과 덧붙이기(append) 모드를 켭니다.
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // 추가 모드 활성화 (기존 데이터 보존)
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        // 파일 쓰기 옵션을 사용하여 CSV Writer 인스턴스를 생성합니다.
        let mut writer = Writer::from_writer(open_option);
        // 날짜, 용도, 금액을 순서대로 포맷하여 CSV 레코드로 기록합니다.
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(), // 날짜를 YYYY-MM-DD 문자열로 포맷팅
                self.usage.to_string(),
                self.amount.to_string(),
            ])
            .unwrap();
        // 버퍼를 비워 변경 내용을 디스크에 반영합니다.
        writer.flush().unwrap();
    }
}

// 출금 명령어의 인자를 정의하는 구조체입니다.
#[derive(Args)]
struct WithdrawArgs {
    account_name: String, // 계좌 이름
    date: NaiveDate,      // 출금 날짜 (YYYY-MM-DD)
    usage: String,        // 출금 용도
    amount: u32,          // 출금 금액
}

impl WithdrawArgs {
    // 출금 명령을 실행하는 함수입니다. (입금과 유사하지만 금액 앞에 마이너스가 붙습니다)
    fn run(&self) {
        // 기존 파일에 이어서 기록할 수 있도록 쓰기 및 추가 모드로 파일을 엽니다.
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // 추가 모드
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        // CSV Writer를 생성합니다.
        let mut writer = Writer::from_writer(open_option);
        // 출금 금액이므로 금액 앞에 "-"를 붙여서 저장합니다.
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                format!("-{}", self.amount), // 음수 기호 추가
            ])
            .unwrap();
    }
}

// 외부 CSV 파일 데이터를 현재 계좌 파일로 가져오는 명령어의 인자입니다.
#[derive(Args)]
struct ImportArgs {
    src_file_name: String,    // 가져올 소스 CSV 파일 경로
    dst_account_name: String, // 저장할 타겟 계좌 이름
}

impl ImportArgs {
    // 파일 가져오기 명령을 실행하는 함수입니다.
    fn run(&self) {
        // 대상 계좌 파일의 끝에 추가하기 위해 쓰기 및 추가 모드로 파일을 엽니다.
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // 추가 모드
            .open(format!("{}.csv", self.dst_account_name))
            .unwrap();
        // 데이터를 단순히 덧붙이기 위해, 헤더 정보(날짜, 용도, 금액 등)는 다시 쓰지 않도록
        // has_headers(false) 옵션으로 설정된 Writer를 생성합니다.
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(open_option);
        // 소스 CSV 파일을 읽어들이기 위해 Reader를 엽니다.
        let mut reader = Reader::from_path(&self.src_file_name).unwrap();
        // 소스 파일의 행을 돌며 Record 구조체 타입으로 역직렬화(deserialize)합니다.
        for result in reader.deserialize() {
            let record: Record = result.unwrap();
            // 읽어온 레코드를 타겟 파일에 차례대로 직렬화하여 기록합니다.
            writer.serialize(record).unwrap();
        }
    }
}

// CSV 파일의 레코드 레이아웃에 매핑되는 구조체입니다.
// serde를 이용해 날짜, 용도, 금액 필드를 자동으로 변환합니다.
#[derive(Serialize, Deserialize)]
struct Record {
    날짜: NaiveDate,
    용도: String,
    금액: i32, // 출금은 음수가 될 수 있으므로 부호가 있는 i32를 사용합니다.
}

// 계좌 정보의 월별 리포트를 요약하기 위한 인자 구조체입니다.
#[derive(Args)]
struct ReportArgs {
    // 리포트를 생성할 여러 CSV 파일들의 목록
    files: Vec<String>,
}

impl ReportArgs {
    // 리포트 생성 명령을 실행하는 함수입니다.
    fn run(&self) {
        // 월별 금액의 합계를 저장할 해시맵을 생성합니다. (키: "YYYY-MM", 값: 금액 합계)
        let mut map = HashMap::new();
        // 지정된 파일들을 하나씩 처리합니다.
        for file in &self.files {
            // 파일을 읽을 Reader를 생성합니다.
            let mut reader = Reader::from_path(file).unwrap();
            // 각 행을 돌며 Record로 변환하여 월별 금액 누적을 진행합니다.
            for result in reader.deserialize() {
                let record: Record = result.unwrap();
                // 날짜를 "YYYY-MM" 형식으로 포맷팅하여 해시맵의 키로 사용하며,
                // 키가 없으면 기본값인 0을 삽입한 뒤, 참조를 얻어 해당 키에 해당하는 금액을 더해줍니다.
                let sum = map
                    .entry(record.날짜.format("%Y-%m").to_string())
                    .or_insert(0);
                *sum += record.금액;
            }
        }
        // 최종 요약된 월별 가계부 금액 상태를 디버그 형식으로 콘솔에 출력합니다.
        println!("{:?}", map);
    }
}

// 애플리케이션의 엔트리 포인트(진입점)인 main 함수입니다.
fn main() {
    // 커맨드라인 인자들을 파싱하여 App 구조체 인스턴스를 얻습니다.
    let args = App::parse();
    // 매치(match) 문을 통해 입력된 서브커맨드에 해당하는 실행 로직(run)을 처리합니다.
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit(args) => args.run(),
        Command::Withdraw(args) => args.run(),
        Command::Import(args) => args.run(),
        Command::Report(args) => args.run(),
    }
}
