use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
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
        if tokens[0] == "mem+" {
            add_and_print_memory(&mut memory, prev_result);
            continue;
        } else if tokens[0] == "mem-" {
            add_and_print_memory(&mut memory, -prev_result);
            continue;
        }

        // 수식 계산
        let left: f64 = eval_token(tokens[0], memory);
        let right: f64 = eval_token(tokens[2], memory);
        let result = eval_expression(left, tokens[1], right);

        // 결과 표시
        print_output(result);

        prev_result = result;
    }
}

fn print_output(value: f64) {
    println!("  => {}", value);
}

fn add_and_print_memory(memory: &mut f64, prev_result: f64) {
    *memory += prev_result;
    print_output(*memory);
}

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
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
