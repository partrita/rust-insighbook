use std::{
    collections::{hash_map::Entry, HashMap},
    io::stdin,
};

fn main() {
    let mut memory = Memory::new();
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        // 1줄씩 읽어서 빈 줄이면 종료
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 공백문자로 구분
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // 메모리 쓰기
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with('+') {
            let result = memory.add(tokens[0], prev_result);
            print_output(result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            let result = memory.add(tokens[0], -prev_result);
            print_output(result);
            continue;
        }

        // 수식 계산
        let left: f64 = eval_token(tokens[0], &memory);
        let right: f64 = eval_token(tokens[2], &memory);
        let result = eval_expression(left, tokens[1], right);
        match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                // 입력이 올바르지 않으면 여기로
                unreachable!()
            }
        };

        // 결과 표시
        print_output(result);

        prev_result = result;
    }
}

fn print_output(value: f64) {
    println!("  => {}", value);
}

struct Memory {
    slots: HashMap<String, f64>,
}
impl Memory {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    fn add(&mut self, token: &str, prev_result: f64) -> f64 {
        let slot_name = token[3..token.len() - 1].to_string();
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                // 메모리를 찾았으므로 값을 변경하고 표시
                *entry.get_mut() += prev_result;
                *entry.get()
            }
            Entry::Vacant(entry) => {
                // 메모리를 찾지 못하면 요소를 추가
                entry.insert(prev_result);
                prev_result
            }
        }
    }

    fn get(&self, token: &str) -> f64 {
        let slot_name = &token[3..];
        match self.slots.get(slot_name) {
            // 메모리를 찾았으므로 값을 반환
            Some(value) => *value,
            // 메모리를 찾지 못하면 초깃값 반환
            None => 0.0,
        }
    }
}

fn eval_token(token: &str, memory: &Memory) -> f64 {
    if token.starts_with("mem") {
        memory.get(token)
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            // 입력이 올바르지 않으면 여기로
            unreachable!()
        }
    }
}
