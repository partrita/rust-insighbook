use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Schedule {
    // 일정 ID
    id: u64,
    // 스터디 모임 이름
    subject: String,
    // 시작 시각
    start: NaiveDateTime,
    // 종료 시각
    end: NaiveDateTime,
}
impl Schedule {
    fn intersects(&self, other: &Schedule) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Calendar {
    schedules: Vec<Schedule>,
}

const SCHEDULE_FILE: &str = "schedule.json";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 일정 목록 표시
    List,
    /// 일정 추가
    Add {
        /// 제목
        subject: String,
        /// 시작 시각
        start: NaiveDateTime,
        /// 종료 시각
        end: NaiveDateTime,
    },
    /// 일정 삭제
    Delete {
        /// 일정 ID
        id: u64,
    },
}

fn main() {
    let options = Cli::parse();

    match options.command {
        Commands::List => {
            let calendar = read_calendar();
            show_list(&calendar);
        }
        Commands::Add {
            subject,
            start,
            end,
        } => {
            let mut calendar = read_calendar();
            if add_schedule(&mut calendar, subject, start, end) {
                save_calendar(&calendar);
                println!("일정을 추가했습니다.");
            } else {
                println!("오류: 일정 중복입니다.");
            }
        }
        Commands::Delete { id } => {
            let mut calendar = read_calendar();
            if delete_schedule(&mut calendar, id) {
                save_calendar(&calendar);
                println!("일정을 삭제했습니다.");
            } else {
                println!("오류: 잘못된 ID입니다.");
            }
        }
    }
}

fn read_calendar() -> Calendar {
    let file = File::open(SCHEDULE_FILE).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn save_calendar(calendar: &Calendar) {
    let file = File::create(SCHEDULE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, calendar).unwrap();
}

fn show_list(calendar: &Calendar) {
    // 일정 표시
    println!("ID\tSTART\tEND\tSUBJECT");
    for schedule in &calendar.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject
        );
    }
}

fn add_schedule(
    calendar: &mut Calendar,
    subject: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> bool {
    // 일정 작성
    let id = if calendar.schedules.is_empty() {
        0
    } else {
        calendar.schedules.last().unwrap().id + 1
    };
    let new_schedule = Schedule {
        id,
        subject,
        start,
        end,
    };

    // 일정 중복 판정
    for schedule in &calendar.schedules {
        if schedule.intersects(&new_schedule) {
            return false;
        }
    }

    // 일정 추가
    calendar.schedules.push(new_schedule);

    true
}

fn delete_schedule(calendar: &mut Calendar, id: u64) -> bool {
    // 일정 삭제
    for i in 0..calendar.schedules.len() {
        if calendar.schedules[i].id == id {
            calendar.schedules.remove(i);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    use rstest::rstest;

    #[rstest]
    #[case(18, 15, 18, 45, false)]
    #[case(18, 15, 19, 45, true)]
    #[case(18, 15, 20, 45, true)]
    #[case(19, 15, 19, 45, true)]
    #[case(19, 15, 20, 45, true)]
    #[case(20, 15, 20, 45, false)]
    fn test_schedule_intersects(
        #[case] h0: u32,
        #[case] m0: u32,
        #[case] h1: u32,
        #[case] m1: u32,
        #[case] should_intersect: bool,
    ) {
        let schedule = Schedule {
            id: 0,
            subject: "기존 일정".to_string(),
            start: naive_date_time(2025, 1, 1, h0, m0, 0),
            end: naive_date_time(2025, 1, 1, h1, m1, 0),
        };
        let new_schedule = Schedule {
            id: 999,
            subject: "신규 일정".to_string(),
            start: naive_date_time(2025, 1, 1, 19, 0, 0),
            end: naive_date_time(2025, 1, 1, 20, 0, 0),
        };
        assert_eq!(should_intersect, schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_add_schedule() {
        let mut calendar = Calendar {
            schedules: vec![Schedule {
                id: 0,
                subject: "테스트 일정".to_string(),
                start: naive_date_time(2024, 11, 19, 11, 22, 33),
                end: naive_date_time(2024, 11, 19, 22, 33, 44),
            }],
        };
        add_schedule(
            &mut calendar,
            "테스트 일정2".to_string(),
            naive_date_time(2024, 12, 8, 9, 0, 0),
            naive_date_time(2024, 12, 8, 10, 30, 0),
        );
        let expected = Calendar {
            schedules: vec![
                Schedule {
                    id: 0,
                    subject: "테스트 일정".to_string(),
                    start: naive_date_time(2024, 11, 19, 11, 22, 33),
                    end: naive_date_time(2024, 11, 19, 22, 33, 44),
                },
                Schedule {
                    id: 1,
                    subject: "테스트 일정2".to_string(),
                    start: naive_date_time(2024, 12, 8, 9, 0, 0),
                    end: naive_date_time(2024, 12, 8, 10, 30, 0),
                },
            ],
        };
        assert_eq!(expected, calendar);
    }

    #[test]
    fn test_delete_schedule() {
        let mut calendar = Calendar {
            schedules: vec![
                Schedule {
                    id: 0,
                    subject: "테스트 일정".to_string(),
                    start: naive_date_time(2024, 11, 19, 11, 22, 33),
                    end: naive_date_time(2024, 11, 19, 22, 33, 44),
                },
                Schedule {
                    id: 1,
                    subject: "테스트 일정2".to_string(),
                    start: naive_date_time(2024, 12, 8, 9, 0, 0),
                    end: naive_date_time(2024, 12, 8, 10, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "추가 가능한 일정".to_string(),
                    start: naive_date_time(2024, 12, 15, 10, 0, 0),
                    end: naive_date_time(2024, 12, 15, 11, 00, 0),
                },
            ],
        };
        // 시험삼아 0번 일정을 삭제
        delete_schedule(&mut calendar, 0);
        // 삭제하면 이렇게 되어야 함
        let expected = Calendar {
            schedules: vec![
                Schedule {
                    id: 1,
                    subject: "테스트 일정2".to_string(),
                    start: naive_date_time(2024, 12, 8, 9, 0, 0),
                    end: naive_date_time(2024, 12, 8, 10, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "추가 가능한 일정".to_string(),
                    start: naive_date_time(2024, 12, 15, 10, 0, 0),
                    end: naive_date_time(2024, 12, 15, 11, 00, 0),
                },
            ],
        };
        assert_eq!(expected, calendar);
        // ID = 1 일정 삭제
        delete_schedule(&mut calendar, 1);
        // 삭제 후에는 이렇게 되어야 함
        let expected = Calendar {
            schedules: vec![Schedule {
                id: 2,
                subject: "추가 가능한 일정".to_string(),
                start: naive_date_time(2024, 12, 15, 10, 0, 0),
                end: naive_date_time(2024, 12, 15, 11, 00, 0),
            }],
        };
        assert_eq!(expected, calendar);
        // 마지막으로 ID = 2 일정 삭제
        assert!(delete_schedule(&mut calendar, 2));
        // 삭제하면 이렇게 되어야 함
        let expected = Calendar { schedules: vec![] };
        assert_eq!(expected, calendar);
    }
}
