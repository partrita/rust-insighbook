// 표준 입력을 처리하기 위한 stdin 함수를 std::io 모듈에서 가져옵니다.
use std::io::stdin;

fn main() {
    // 계산기의 메모리(M) 값을 저장할 64비트 실수형 가변 변수입니다. 0.0으로 초기화합니다.
    let mut memory: f64 = 0.0;
    // 이전 계산 결과를 임시 저장할 가변 변수입니다. mem+ / mem- 명령을 수행할 때 사용됩니다.
    let mut prev_result: f64 = 0.0;

    // 표준 입력으로부터 한 줄씩 읽는 루프입니다.
    for line in stdin().lines() {
        // 1줄씩 읽어서 빈 줄이면 종료
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 공백문자를 기준으로 문자열을 잘라 토큰 벡터를 만듭니다.
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // 메모리 쓰기 명령 처리
        // 첫 번째 토큰이 "mem+"인 경우: 현재 메모리에 이전 계산 결과를 더합니다.
        if tokens[0] == "mem+" {
            add_and_print_memory(&mut memory, prev_result);
            continue; // 아래 수식 계산 단계를 거치지 않고 다음 루프로 넘어갑니다.
        // 첫 번째 토큰이 "mem-"인 경우: 현재 메모리에서 이전 계산 결과를 뺍니다.
        } else if tokens[0] == "mem-" {
            // -prev_result를 더하는 방식으로 차감을 구현합니다.
            add_and_print_memory(&mut memory, -prev_result);
            continue; // 다음 루프로 넘어갑니다.
        }

        // 수식 계산
        // 왼쪽 피연산자 토큰을 해석합니다. ("mem" 문자열이면 메모리 값을, 아니면 파싱한 숫자 값을 가져옵니다.)
        let left: f64 = eval_token(tokens[0], memory);
        // 오른쪽 피연산자 토큰을 해석합니다.
        let right: f64 = eval_token(tokens[2], memory);
        // 계산을 실행하고 결과를 result에 담습니다.
        let result = eval_expression(left, tokens[1], right);

        // 계산된 결과 표시
        print_output(result);

        // 이번 계산 결과를 다음 루프를 위해 prev_result에 업데이트합니다.
        prev_result = result;
    }
}

// 계산 결과를 지정된 형식으로 출력하는 함수입니다.
fn print_output(value: f64) {
    println!("  => {}", value);
}

// 메모리에 특정 값을 누적(가산/감산)하고, 누적된 결과를 출력하는 헬퍼 함수입니다.
// memory는 수정이 필요하므로 가변 참조(&mut f64)로 전달받습니다.
fn add_and_print_memory(memory: &mut f64, prev_result: f64) {
    // 참조 역참조 연산자(*)를 사용하여 실제 메모리 값을 업데이트합니다.
    *memory += prev_result;
    // 변경된 메모리 값을 화면에 출력합니다.
    print_output(*memory);
}

// 토큰의 문자열을 실수로 변환하는 함수입니다.
// 만약 토큰이 "mem"이라면 저장된 메모리(memory) 변수값을 그대로 반환하고,
// 숫자를 나타내는 문자열이라면 실수형 f64로 파싱합니다.
fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
    } else {
        token.parse().unwrap()
    }
}

// 왼쪽 피연산자, 연산자 기호, 오른쪽 피연산자를 인자로 받아 연산 결과를 반환하는 함수입니다.
fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right, // 덧셈
        "-" => left - right, // 뺄셈
        "*" => left * right, // 곱셈
        "/" => left / right, // 나눗셈
        _ => {
            // 다른 잘못된 연산자가 올 경우 프로그램을 패닉 상태로 중단시킵니다.
            unreachable!()
        }
    }
}
