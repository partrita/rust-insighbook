use std::{collections::HashMap, fs::OpenOptions};

use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 신규 계좌 작성
    New(NewArgs),
    /// 계좌 입금
    Deposit(DepositArgs),
    /// 계좌 출금
    Withdraw(WithdrawArgs),
    /// CSV에서 가져오기
    Import(ImportArgs),
    /// 리포트 작성
    Report(ReportArgs),
}

#[derive(Args)]
struct NewArgs {
    account_name: String,
}

impl NewArgs {
    fn run(&self) {
        let file_name = format!("{}.csv", self.account_name);
        let mut writer = Writer::from_path(file_name).unwrap();
        writer.write_record(["날짜", "용도", "금액"]).unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct DepositArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}

impl DepositArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // 추가 모드
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                self.amount.to_string(),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct WithdrawArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}

impl WithdrawArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // 추가 모드
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                format!("-{}", self.amount),
            ])
            .unwrap();
    }
}

#[derive(Args)]
struct ImportArgs {
    src_file_name: String,
    dst_account_name: String,
}

impl ImportArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // 추가 모드
            .open(format!("{}.csv", self.dst_account_name))
            .unwrap();
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(open_option);
        let mut reader = Reader::from_path(&self.src_file_name).unwrap();
        for result in reader.deserialize() {
            let record: Record = result.unwrap();
            writer.serialize(record).unwrap();
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Record {
    날짜: NaiveDate,
    용도: String,
    금액: i32,
}

#[derive(Args)]
struct ReportArgs {
    files: Vec<String>,
}

impl ReportArgs {
    fn run(&self) {
        let mut map = HashMap::new();
        for file in &self.files {
            let mut reader = Reader::from_path(file).unwrap();
            for result in reader.deserialize() {
                let record: Record = result.unwrap();
                let sum = map
                    .entry(record.날짜.format("%Y-%m").to_string())
                    .or_insert(0);
                *sum += record.금액;
            }
        }
        println!("{:?}", map);
    }
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit(args) => args.run(),
        Command::Withdraw(args) => args.run(),
        Command::Import(args) => args.run(),
        Command::Report(args) => args.run(),
    }
}
