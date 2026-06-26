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

    // 표준 입력으로부터 한 줄씩 읽는 루프입니다.
    for line in stdin().lines() {
        // 1줄씩 읽어서 빈 줄이면 종료
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 입력받은 문자열을 파싱하여 토큰(Token) 벡터로 분할합니다.
        let tokens = Token::split(&line);

        // 첫 번째 토큰의 유형에 따라 수식을 평가하거나 메모리 작업을 수행합니다.
        match &tokens[0] {
            // 메모리 더하기 토큰인 경우 (예: memA+)
            Token::MemoryPlus(memory_name) => {
                // 메모리 이름 문자열을 소유권을 가지는 String으로 변환합니다.
                let memory_name = memory_name.to_string();
                // 지정된 메모리에 이전 결과를 더하고 그 결과를 받아옵니다.
                let result = memory.add(memory_name, prev_result);
                // 결과값을 화면에 출력합니다.
                print_output(result);
            }
            // 메모리 빼기 토큰인 경우 (예: memA-)
            Token::MemoryMinus(memory_name) => {
                // 메모리 이름 문자열을 소유권을 가지는 String으로 변환합니다.
                let memory_name = memory_name.to_string();
                // 지정된 메모리에서 이전 결과를 뺍니다 (-prev_result를 전달하여 더함).
                let result = memory.add(memory_name, -prev_result);
                // 결과값을 화면에 출력합니다.
                print_output(result);
            }
            // 일반 수식인 경우
            _ => {
                // 전체 토큰들과 메모리 정보를 인자로 전달하여 수식을 계산(평가)합니다.
                let result = eval_expression(&tokens, &memory);

                // 계산된 최종 결과를 표시합니다.
                print_output(result);
                // 다음 계산의 mem+, mem- 연산을 위해 이번 결과값을 prev_result에 저장합니다.
                prev_result = result;
            }
        };
    }
}

// 계산 결과값을 출력하는 함수입니다.
fn print_output(value: f64) {
    println!("  => {}", value);
}

// 여러 메모리 슬롯을 관리하는 구조체 정의
struct Memory {
    slots: HashMap<String, f64>,
}
impl Memory {
    // Memory 초기 생성자 함수
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    // 특정 메모리 슬롯에 값을 더하는 메서드
    fn add(&mut self, slot_name: String, prev_result: f64) -> f64 {
        match self.slots.entry(slot_name) {
            // 이미 존재하는 슬롯이면 기존 값에 더해줍니다.
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += prev_result;
                *entry.get()
            }
            // 존재하지 않는 슬롯이면 새로 생성하고 더할 값을 저장합니다.
            Entry::Vacant(entry) => {
                entry.insert(prev_result);
                prev_result
            }
        }
    }

    // 특정 메모리 슬롯의 값을 가져오는 메서드
    fn get(&self, slot_name: &str) -> f64 {
        // 값을 복사해서 가져오고, 없으면 0.0을 반환합니다.
        self.slots.get(slot_name).copied().unwrap_or(0.0)
    }
}

// 수식을 구성하는 토큰들을 표현하는 열거형
#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),         // 숫자 (예: 1.23)
    MemoryRef(String),   // 메모리 조회 (예: memA)
    MemoryPlus(String),  // 메모리 가산 (예: memA+)
    MemoryMinus(String), // 메모리 감산 (예: memA-)
    Plus,                // 더하기 연산자 (+)
    Minus,               // 빼기 연산자 (-)
    Asterisk,            // 곱하기 연산자 (*)
    Slash,               // 나누기 연산자 (/)
    LParen,              // 여는 괄호 (()
    RParen,              // 닫는 괄호 ())
}
impl Token {
    // 문자열 조각 하나를 해석하여 적절한 Token 열거형 값으로 변환하는 함수
    fn parse(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash,
            "(" => Self::LParen,
            ")" => Self::RParen,
            // "mem"으로 시작하는 경우 메모리 명령어 또는 변수 참조로 간주합니다.
            _ if value.starts_with("mem") => {
                // "mem" 다음 글자부터 끝까지를 메모리 슬롯 이름으로 가져옵니다.
                let mut memory_name = value[3..].to_string();
                if value.ends_with('+') {
                    // 맨 끝에 있는 '+' 문자를 제거합니다.
                    memory_name.pop();
                    Self::MemoryPlus(memory_name)
                } else if value.ends_with('-') {
                    // 맨 끝에 있는 '-' 문자를 제거합니다.
                    memory_name.pop();
                    Self::MemoryMinus(memory_name)
                } else {
                    // 단순 메모리 참조
                    Self::MemoryRef(memory_name)
                }
            }
            // 연산자도 메모리 키워드도 아니면 실수형 숫자로 파싱하여 Number 토큰으로 생성합니다.
            _ => Self::Number(value.parse().unwrap()),
        }
    }

    // 입력 한 줄을 공백 단위로 쪼갠 뒤 각각 parse하여 Token 벡터를 생성합니다.
    fn split(text: &str) -> Vec<Self> {
        text.split(char::is_whitespace).map(Self::parse).collect()
    }
}

// 전체 수식 토큰 배열을 평가하는 최상위 함수
fn eval_expression(tokens: &[Token], memory: &Memory) -> f64 {
    // 덧셈/뺄셈 레벨의 파싱 함수를 호출하여 수식의 전체 값을 계산합니다. (시작 인덱스는 0)
    let (result, index) = eval_additive_expression(tokens, 0, memory);
    // 올바르게 수식 전체가 파싱되었다면, 최종 파싱 진행 위치(index)는 토큰 배열의 총 길이와 같아야 합니다.
    assert_eq!(tokens.len(), index);
    result
}

// 덧셈(+)과 뺄셈(-) 연산을 파싱하여 평가하는 함수 (우선순위가 곱셈/나눗셈보다 낮으므로 먼저 호출되어 그 결과를 누적)
fn eval_additive_expression(tokens: &[Token], index: usize, memory: &Memory) -> (f64, usize) {
    let mut index = index;
    let mut result;
    // 먼저 곱셈/나눗셈 우선순위의 하위 수식을 평가합니다.
    (result, index) = eval_multiplicative_expression(tokens, index, memory);
    // 다음 토큰이 덧셈(+) 또는 뺄셈(-)인 동안 반복하여 계속해서 연산합니다.
    while index < tokens.len() {
        match &tokens[index] {
            Token::Plus => {
                // 연산자 다음(index + 1)의 하위 수식을 곱셈/나눗셈 우선순위로 평가합니다.
                let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory);
                result += value;
                index = next; // 파싱 인덱스를 업데이트합니다.
            }
            Token::Minus => {
                let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory);
                result -= value;
                index = next;
            }
            // 덧셈, 뺄셈 연산자가 아니면 루프를 종료합니다.
            _ => break,
        }
    }
    (result, index)
}

// 곱셈(*)과 나눗셈(/) 연산을 파싱하여 평가하는 함수
fn eval_multiplicative_expression(tokens: &[Token], index: usize, memory: &Memory) -> (f64, usize) {
    let mut index = index;
    let mut result;
    // 가장 높은 우선순위를 가지는 1차 수식(괄호, 숫자, 메모리 등)을 먼저 평가합니다.
    (result, index) = eval_primary_expression(tokens, index, memory);
    // 다음 토큰이 곱셈(*) 또는 나눗셈(/)인 동안 반복하여 계산합니다.
    while index < tokens.len() {
        match &tokens[index] {
            Token::Asterisk => {
                // 연산자 다음(index + 1)의 1차 수식을 평가합니다.
                let (value, next) = eval_primary_expression(tokens, index + 1, memory);
                result *= value;
                index = next;
            }
            Token::Slash => {
                let (value, next) = eval_primary_expression(tokens, index + 1, memory);
                result /= value;
                index = next;
            }
            _ => break,
        }
    }
    (result, index)
}

// 가장 기본이 되는 1차 수식(괄호 수식, 단일 숫자, 메모리 참조)을 파싱하여 평가하는 함수
fn eval_primary_expression(tokens: &[Token], index: usize, memory: &Memory) -> (f64, usize) {
    let first_token = &tokens[index];
    match first_token {
        // 여는 괄호 '(' 토큰을 만나면 새로운 수식이 시작됩니다.
        Token::LParen => {
            // 괄호 안의 전체 수식(덧셈 레벨부터 시작)을 계산합니다.
            let (result, next) = eval_additive_expression(tokens, index + 1, memory);
            // 수식 계산이 끝나고 나면 바로 다음 토큰이 반드시 닫는 괄호 ')'이어야 합니다.
            assert_eq!(Token::RParen, tokens[next]);
            // 닫는 괄호 다음 위치(next + 1)로 인덱스를 전진시키고 계산된 값을 반환합니다.
            (result, next + 1)
        }
        // 토큰이 숫자인 경우
        Token::Number(value) => {
            // 숫자 자체의 값과 다음 토큰 위치(index + 1)를 반환합니다.
            (*value, index + 1)
        }
        // 토큰이 메모리 참조(예: memA)인 경우
        Token::MemoryRef(memory_name) => {
            // 메모리 슬롯에서 값을 읽어오고 다음 위치(index + 1)를 반환합니다.
            (memory.get(memory_name), index + 1)
        }
        _ => {
            // 잘못된 토큰 위치에서 primary expression 평가를 시도하면 패닉을 발생시킵니다.
            unreachable!()
        }
    }
}
