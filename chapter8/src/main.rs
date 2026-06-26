// 표준 라이브러리에서 파일 입출력 및 버퍼링 처리를 위한 모듈을 가져옵니다.
use std::{
    fs::File, // 파일 열기 및 생성을 위한 구조체
    io::{BufReader, BufWriter}, // 입출력 속도 향상을 위해 버퍼를 사용하는 리더와 라이터
};

// 날짜와 시간을 다루기 위한 외부 라이브러리(chrono)의 NaiveDateTime 구조체를 가져옵니다.
use chrono::NaiveDateTime;
// 명령줄 인수(CLI) 분석을 쉽게 해주는 외부 라이브러리(clap)의 매크로 및 구조체를 가져옵니다.
use clap::{Parser, Subcommand};
// 데이터를 JSON 등의 형식으로 변환(직렬화/역직렬화)해주는 외부 라이브러리(serde)의 매크로를 가져옵니다.
use serde::{Deserialize, Serialize};

// 일정 정보를 담는 구조체입니다.
// Debug: println!("{:?}", s);와 같이 디버그 출력을 가능하게 함
// Clone: 구조체 복사를 가능하게 함
// PartialEq, Eq: 구조체 간의 값 비교(==)를 가능하게 함
// Serialize, Deserialize: 이 구조체를 JSON 등 파일 데이터로 쓰거나 읽을 수 있게 함
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Schedule {
    // 일정의 고유 ID
    id: u64,
    // 스터디 모임의 제목/이름
    subject: String,
    // 일정의 시작 시각
    start: NaiveDateTime,
    // 일정의 종료 시각
    end: NaiveDateTime,
}

impl Schedule {
    // 두 일정이 시간상으로 서로 겹치는지 판단하는 메서드입니다.
    // 겹치면 true, 겹치지 않으면 false를 반환합니다.
    fn intersects(&self, other: &Schedule) -> bool {
        // [self.start, self.end]와 [other.start, other.end]가 겹치기 위한 조건:
        // 내 시작 시간이 상대방 종료 시간보다 이전이고, 상대방 시작 시간이 내 종료 시간보다 이전이어야 함
        self.start < other.end && other.start < self.end
    }
}

// 여러 일정을 관리하는 캘린더 구조체입니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Calendar {
    // Schedule 구조체들을 담는 벡터(동적 배열)
    schedules: Vec<Schedule>,
}

// 일정을 저장할 JSON 파일 경로를 상수로 정의합니다.
const SCHEDULE_FILE: &str = "schedule.json";

// CLI(Command Line Interface) 파서를 정의하기 위한 구조체입니다.
#[derive(Parser)]
struct Cli {
    // 서브커맨드(명령 하위에 들어가는 하위 명령어들)를 파싱합니다.
    #[command(subcommand)]
    command: Commands,
}

// 사용자가 실행할 수 있는 하위 명령어(서브커맨드)들을 열거형으로 정의합니다.
#[derive(Subcommand)]
enum Commands {
    /// 일정 목록 표시
    List,
    /// 일정 추가 (필요한 매개변수를 함께 지정)
    Add {
        /// 제목
        subject: String,
        /// 시작 시각 (형식: YYYY-MM-DDThh:mm:ss)
        start: NaiveDateTime,
        /// 종료 시각 (형식: YYYY-MM-DDThh:mm:ss)
        end: NaiveDateTime,
    },
    /// 일정 삭제 (ID 기준)
    Delete {
        /// 일정 ID
        id: u64,
    },
}

// 프로그램의 진입점(메인 함수)입니다.
fn main() {
    // 명령줄 인수를 분석하여 Cli 구조체 형태로 가져옵니다.
    let options = Cli::parse();

    // 입력된 하위 명령어에 따라 분기 처리합니다.
    match options.command {
        // 'list' 명령어가 실행된 경우
        Commands::List => {
            // 저장된 JSON 파일에서 일정 정보를 읽어옵니다.
            let calendar = read_calendar();
            // 화면에 보기 좋게 목록을 표시합니다.
            show_list(&calendar);
        }
        // 'add' 명령어가 실행된 경우 (제목, 시작일시, 종료일시를 파라미터로 받음)
        Commands::Add {
            subject,
            start,
            end,
        } => {
            // 기존 일정 정보를 읽어옵니다. (가변성 mut 설정)
            let mut calendar = read_calendar();
            // 일정을 추가해보고 성공 여부를 판단합니다. (겹치는 일정이 없어야 성공)
            if add_schedule(&mut calendar, subject, start, end) {
                // 성공적으로 추가되었다면 파일에 변경 사항을 저장합니다.
                save_calendar(&calendar);
                println!("일정을 추가했습니다.");
            } else {
                // 기존 일정과 시간이 겹치면 에러 메시지를 출력합니다.
                println!("오류: 일정 중복입니다.");
            }
        }
        // 'delete' 명령어가 실행된 경우 (삭제할 일정 ID를 파라미터로 받음)
        Commands::Delete { id } => {
            // 기존 일정 정보를 읽어옵니다.
            let mut calendar = read_calendar();
            // 해당 ID의 일정을 찾아서 삭제해 봅니다.
            if delete_schedule(&mut calendar, id) {
                // 성공적으로 삭제되었다면 파일에 변경 사항을 저장합니다.
                save_calendar(&calendar);
                println!("일정을 삭제했습니다.");
            } else {
                // 존재하지 않는 ID면 에러 메시지를 출력합니다.
                println!("오류: 잘못된 ID입니다.");
            }
        }
    }
}

// JSON 파일에서 캘린더 데이터를 읽어오는 헬퍼 함수입니다.
fn read_calendar() -> Calendar {
    // 지정된 경로의 파일을 엽니다. 파일이 없으면 빈 Calendar를 반환합니다.
    let file = match File::open(SCHEDULE_FILE) {
        Ok(file) => file,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Calendar { schedules: Vec::new() };
        }
        Err(e) => panic!("파일을 여는 중 오류가 발생했습니다: {:?}", e),
    };
    // 성능 향상을 위해 파일 읽기 스트림에 버퍼를 추가합니다.
    let reader = BufReader::new(file);
    // JSON 리더를 통해 Calendar 구조체로 역직렬화하여 반환합니다.
    serde_json::from_reader(reader).unwrap_or_else(|_| Calendar { schedules: Vec::new() })
}

// 캘린더 데이터를 JSON 파일로 저장하는 헬퍼 함수입니다.
fn save_calendar(calendar: &Calendar) {
    // 지정된 경로에 쓰기 모드로 파일을 새로 생성하거나 덮어씁니다.
    let file = File::create(SCHEDULE_FILE).unwrap();
    // 성능 향상을 위해 파일 쓰기 스트림에 버퍼를 추가합니다.
    let writer = BufWriter::new(file);
    // Calendar 구조체 데이터를 JSON 포맷으로 직렬화하여 파일에 씁니다.
    serde_json::to_writer(writer, calendar).unwrap();
}

// 화면에 전체 일정을 표 형태로 출력하는 함수입니다.
fn show_list(calendar: &Calendar) {
    // 헤더(열 이름)를 출력합니다. 탭 문자(\t)로 정렬합니다.
    println!("ID\tSTART\tEND\tSUBJECT");
    // 모든 일정을 순회하며 정보를 형식화하여 출력합니다.
    for schedule in &calendar.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject
        );
    }
}

// 신규 일정을 캘린더에 추가하는 함수입니다.
fn add_schedule(
    calendar: &mut Calendar, // 일정을 수정해야 하므로 가변 참조(&mut)를 받습니다.
    subject: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> bool {
    // 신규 일정에 할당할 고유 ID를 결정합니다.
    // 기존 일정이 없으면 0번, 있으면 가장 최근 일정(마지막 원소) ID에 1을 더한 값으로 결정합니다.
    let id = if calendar.schedules.is_empty() {
        0
    } else {
        calendar.schedules.last().unwrap().id + 1
    };
    // 신규 Schedule 객체를 생성합니다.
    let new_schedule = Schedule {
        id,
        subject,
        start,
        end,
    };

    // 기존의 모든 일정과 신규 일정이 시간상으로 겹치는지 검사합니다.
    for schedule in &calendar.schedules {
        if schedule.intersects(&new_schedule) {
            // 하나라도 겹치는 일정이 있다면 false를 반환하고 추가를 중단합니다.
            return false;
        }
    }

    // 시간 겹침이 없다면 캘린더의 일정 리스트(Vec)에 신규 일정을 밀어 넣습니다(push).
    calendar.schedules.push(new_schedule);

    // 성공의 의미로 true를 반환합니다.
    true
}

// 지정된 ID에 해당하는 일정을 찾아 삭제하는 함수입니다.
fn delete_schedule(calendar: &mut Calendar, id: u64) -> bool {
    // 인덱스를 기준으로 모든 일정을 검사합니다.
    for i in 0..calendar.schedules.len() {
        // 일치하는 ID를 찾은 경우
        if calendar.schedules[i].id == id {
            // 해당 인덱스의 일정을 벡터에서 제거합니다.
            calendar.schedules.remove(i);
            // 삭제 성공했으므로 true를 즉시 반환합니다.
            return true;
        }
    }
    // 루프를 다 돌았음에도 ID를 찾지 못했으면 false를 반환합니다.
    false
}

// 단위 테스트 모듈 정의입니다.
// #[cfg(test)] 어노테이션은 cargo test 명령어를 실행할 때만 빌드 및 테스트하도록 지정합니다.
#[cfg(test)]
mod tests {
    use super::*; // 부모 모듈(상위 영역)의 정의들을 가져와서 사용할 수 있게 합니다.

    // 년, 월, 일, 시, 분, 초 정보를 받아 NaiveDateTime 객체를 쉽게 생성해주는 테스트용 헬퍼 함수입니다.
    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        // 옵셔널(Option) 처리를 위해 _opt 함수를 사용하고 유효한 날짜 시간임을 가정하여 unwrap()합니다.
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    // 다양한 테스트 케이스를 간결하게 표현할 수 있는 rstest 라이브러리를 사용합니다.
    use rstest::rstest;

    // 일정이 겹치는지 판단하는 intersects 함수가 올바르게 작동하는지 확인하는 테스트입니다.
    // #[case(...)] 매크로를 사용하여 여러 조건의 테스트 데이터를 한 번에 검증합니다.
    #[rstest]
    #[case(18, 15, 18, 45, false)] // 기존 일정 18:15 ~ 18:45 이므로 신규 일정(19:00 ~ 20:00)과 겹치지 않음 -> false
    #[case(18, 15, 19, 45, true)]  // 기존 일정 18:15 ~ 19:45 이므로 신규 일정과 일부 겹침 -> true
    #[case(18, 15, 20, 45, true)]  // 기존 일정 18:15 ~ 20:45 이므로 신규 일정을 통째로 포함하여 겹침 -> true
    #[case(19, 15, 19, 45, true)]  // 기존 일정 19:15 ~ 19:45 이므로 신규 일정 내에 포함되어 겹침 -> true
    #[case(19, 15, 20, 45, true)]  // 기존 일정 19:15 ~ 20:45 이므로 신규 일정의 뒷부분과 겹침 -> true
    #[case(20, 15, 20, 45, false)] // 기존 일정 20:15 ~ 20:45 이므로 신규 일정의 종료 시각보다 늦어 겹치지 않음 -> false
    fn test_schedule_intersects(
        #[case] h0: u32, // 테스트 케이스로부터 주입받을 기존 일정의 시작 시(hour)
        #[case] m0: u32, // 기존 일정의 시작 분(minute)
        #[case] h1: u32, // 기존 일정의 종료 시(hour)
        #[case] m1: u32, // 기존 일정의 종료 분(minute)
        #[case] should_intersect: bool, // 겹침 판정 기대값 (true 또는 false)
    ) {
        // 기존 일정을 생성합니다.
        let schedule = Schedule {
            id: 0,
            subject: "기존 일정".to_string(),
            start: naive_date_time(2025, 1, 1, h0, m0, 0),
            end: naive_date_time(2025, 1, 1, h1, m1, 0),
        };
        // 신규 일정(기준 일정: 19:00 ~ 20:00)을 생성합니다.
        let new_schedule = Schedule {
            id: 999,
            subject: "신규 일정".to_string(),
            start: naive_date_time(2025, 1, 1, 19, 0, 0),
            end: naive_date_time(2025, 1, 1, 20, 0, 0),
        };
        // 기대하는 겹침 여부 결과와 실제 함수 실행 결과가 일치하는지 단언(assert)합니다.
        assert_eq!(should_intersect, schedule.intersects(&new_schedule));
    }

    // 일정 추가 기능(add_schedule)에 대한 테스트입니다.
    #[test]
    fn test_add_schedule() {
        // 테스트용 가변 캘린더 데이터를 초기화합니다.
        let mut calendar = Calendar {
            schedules: vec![Schedule {
                id: 0,
                subject: "테스트 일정".to_string(),
                start: naive_date_time(2024, 11, 19, 11, 22, 33),
                end: naive_date_time(2024, 11, 19, 22, 33, 44),
            }],
        };
        // 캘린더에 신규 일정을 추가해 봅니다.
        add_schedule(
            &mut calendar,
            "테스트 일정2".to_string(),
            naive_date_time(2024, 12, 8, 9, 0, 0),
            naive_date_time(2024, 12, 8, 10, 30, 0),
        );
        // 추가 후에 기대되는 최종 캘린더 상태(기대값)를 정의합니다.
        let expected = Calendar {
            schedules: vec![
                Schedule {
                    id: 0,
                    subject: "테스트 일정".to_string(),
                    start: naive_date_time(2024, 11, 19, 11, 22, 33),
                    end: naive_date_time(2024, 11, 19, 22, 33, 44),
                },
                Schedule {
                    id: 1, // ID는 자동으로 마지막 일정 ID(0)에 1을 더한 1이 할당되어야 합니다.
                    subject: "테스트 일정2".to_string(),
                    start: naive_date_time(2024, 12, 8, 9, 0, 0),
                    end: naive_date_time(2024, 12, 8, 10, 30, 0),
                },
            ],
        };
        // 실제 결과값과 기대값이 일치하는지 비교합니다.
        assert_eq!(expected, calendar);
    }

    // 일정 삭제 기능(delete_schedule)에 대한 테스트입니다.
    #[test]
    fn test_delete_schedule() {
        // 테스트용 가변 캘린더를 3개의 일정(ID 0, 1, 2)으로 세팅합니다.
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
        // 시험삼아 ID = 0번 일정을 삭제합니다.
        delete_schedule(&mut calendar, 0);
        // 삭제하면 ID 1번과 2번 일정만 남아야 합니다.
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
        // 이어서 ID = 1번 일정을 삭제합니다.
        delete_schedule(&mut calendar, 1);
        // 삭제 후에는 ID 2번 일정만 남아 있어야 합니다.
        let expected = Calendar {
            schedules: vec![Schedule {
                id: 2,
                subject: "추가 가능한 일정".to_string(),
                start: naive_date_time(2024, 12, 15, 10, 0, 0),
                end: naive_date_time(2024, 12, 15, 11, 00, 0),
            }],
        };
        assert_eq!(expected, calendar);
        // 마지막으로 ID = 2번 일정을 삭제하고, 삭제 성공(true)이 잘 리턴되는지 확인합니다.
        assert!(delete_schedule(&mut calendar, 2));
        // 삭제하면 캘린더가 텅 비어야 합니다.
        let expected = Calendar { schedules: vec![] };
        assert_eq!(expected, calendar);
    }
}

