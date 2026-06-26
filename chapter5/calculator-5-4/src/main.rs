// std 라이브러리에서 HashMap, Entry 진입점, 그리고 stdin 함수를 가져옵니다.
use std::{
    collections::{HashMap, hash_map::Entry},
    io::stdin,
};

fn main() {
    // 여러 개의 메모리 슬롯을 가질 수 있는 Memory 구조체의 인스턴스를 가변 변수로 생성합니다.
    let mut memory = Memory::new();
    // 이전 계산 결과를 보관할 변수입니다. (초기값은 0.0)
    let mut prev_result: f64 = 0.0;

    // 표준 입력으로부터 한 줄씩 받아오는 반복문입니다.
    for line in stdin().lines() {
        // 1줄씩 읽어서 빈 줄이면 종료
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 공백문자를 기준으로 입력 한 줄을 여러 토큰으로 쪼갭니다.
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // 메모리 관련 명령인지 확인 (예: "memA+", "memB-" 등)
        let is_memory = tokens[0].starts_with("mem");
        // 메모리 연산이고, 맨 끝 문자가 '+'인 경우 (예: memA+는 이전 결과를 memA 메모리에 더함)
        if is_memory && tokens[0].ends_with('+') {
            let result = memory.add(tokens[0], prev_result);
            print_output(result);
            continue; // 아래 일반 수식 계산을 건너뛰고 다음 입력으로 넘어갑니다.
        // 메모리 연산이고, 맨 끝 문자가 '-'인 경우 (예: memA-는 이전 결과를 memA 메모리에서 뺌)
        } else if is_memory && tokens[0].ends_with('-') {
            // 빼야 하므로 음수값(-prev_result)을 넘겨서 더해줍니다.
            let result = memory.add(tokens[0], -prev_result);
            print_output(result);
            continue; // 다음 입력으로 넘어갑니다.
        }

        // 수식 계산
        // 왼쪽 토큰을 평가합니다. "memA"와 같이 mem으로 시작하면 메모리에서 불러오고, 아니면 숫자로 변환합니다.
        let left: f64 = eval_token(tokens[0], &memory);
        // 오른쪽 토큰을 평가합니다.
        let right: f64 = eval_token(tokens[2], &memory);
        // 왼쪽 피연산자, 연산자, 오른쪽 피연산자를 이용하여 계산 결과를 얻습니다.
        let result = eval_expression(left, tokens[1], right);
        // (기존 코드 유지) 연산자에 대해 한 번 더 매칭을 수행합니다.
        match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                // 입력이 올바르지 않으면 프로그램을 패닉시킵니다.
                unreachable!()
            }
        };

        // 최종 계산 결과 출력
        print_output(result);

        // 다음 연산을 위해 이번 계산 결과를 prev_result에 보관합니다.
        prev_result = result;
    }
}

// 계산 결과값을 출력하는 함수입니다.
fn print_output(value: f64) {
    println!("  => {}", value);
}

// 여러 메모리 슬롯을 Map 형태로 보관하는 구조체 정의
struct Memory {
    // 키는 메모리 이름(예: "A", "B"), 값은 실수형(f64)
    slots: HashMap<String, f64>,
}
impl Memory {
    // Memory 인스턴스를 초기화하여 생성하는 연관 함수(생성자)
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    // 메모리 슬롯에 값을 가산/감산하는 메서드
    // 가변 참조(&mut self)를 통해 내부 HashMap을 수정합니다.
    fn add(&mut self, token: &str, prev_result: f64) -> f64 {
        // "memA+"에서 메모리 이름 부분인 "A"만 추출합니다. (인덱스 3부터 맨 끝 이전까지)
        let slot_name = token[3..token.len() - 1].to_string();
        // HashMap의 entry API를 이용하여 해당 키의 상태에 따라 동작을 분기합니다.
        match self.slots.entry(slot_name) {
            // 이미 키가 존재하는 경우 (Occupied)
            Entry::Occupied(mut entry) => {
                // 메모리를 찾았으므로 기존 값에 prev_result를 더해 업데이트합니다.
                *entry.get_mut() += prev_result;
                // 업데이트된 최종 값을 반환합니다.
                *entry.get()
            }
            // 키가 존재하지 않는 경우 (Vacant)
            Entry::Vacant(entry) => {
                // 메모리를 새로 만들고 전달받은 prev_result를 저장합니다.
                entry.insert(prev_result);
                // 저장한 결과값을 그대로 반환합니다.
                prev_result
            }
        }
    }

    // 특정 메모리 슬롯의 값을 조회하는 메서드 (읽기 전용이므로 &self)
    fn get(&self, token: &str) -> f64 {
        // "memA"에서 "mem" 이후의 문자열인 "A"만 추출하여 슬롯명으로 사용합니다.
        let slot_name = &token[3..];
        // HashMap에서 해당 슬롯명의 값을 찾습니다.
        match self.slots.get(slot_name) {
            // 저장된 메모리 값이 있으면 역참조하여 복사된 값을 반환합니다.
            Some(value) => *value,
            // 저장된 메모리 값이 없으면 기본값인 0.0을 반환합니다.
            None => 0.0,
        }
    }
}

// 토큰을 실제 연산 가능한 실수형 f64 값으로 변환(평가)하는 함수
fn eval_token(token: &str, memory: &Memory) -> f64 {
    // 토큰이 "mem"으로 시작하면 메모리 객체에서 값을 꺼내옵니다.
    if token.starts_with("mem") {
        memory.get(token)
    // "mem"이 아니면 숫자로 파싱하여 반환합니다.
    } else {
        token.parse().unwrap()
    }
}

// 사칙연산을 직접 계산해주는 함수
fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right, // 덧셈
        "-" => left - right, // 뺄셈
        "*" => left * right, // 곱셈
        "/" => left / right, // 나눗셈
        _ => {
            // 정의되지 않은 연산자가 입력되면 패닉 발생
            unreachable!()
        }
    }
}
