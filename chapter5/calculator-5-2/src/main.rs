// 표준 입력을 처리하기 위한 stdin 함수를 std::io 모듈에서 가져옵니다.
use std::io::stdin;

fn main() {
    // stdin().lines()는 표준 입력으로부터 들어오는 입력을 한 줄씩 읽는 반복자(Iterator)를 반환합니다.
    // 각 줄(line)이 입력될 때마다 loop가 수행됩니다.
    for line in stdin().lines() {
        // 1줄씩 읽어서 빈 줄이면 종료
        // Result 에러를 처리하기 위해 unwrap()을 사용합니다.
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 공백문자를 기준으로 문자열을 쪼개어(split) 문자열 조각들의 벡터(Vec<&str>)를 만듭니다.
        // 예: "3 + 5"는 ["3", "+", "5"]가 됩니다.
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // 수식 계산
        // 첫 번째 토큰(왼쪽 피연산자)을 실수형(f64)으로 파싱합니다.
        let left: f64 = tokens[0].parse().unwrap();
        // 세 번째 토큰(오른쪽 피연산자)을 실수형(f64)으로 파싱합니다.
        let right: f64 = tokens[2].parse().unwrap();
        // 두 번째 토큰(연산자)에 따라 match를 수행하여 적절한 연산을 결정합니다.
        let result = match tokens[1] {
            "+" => left + right, // 덧셈
            "-" => left - right, // 뺄셈
            "*" => left * right, // 곱셈
            "/" => left / right, // 나눗셈
            _ => {
                // 정의되지 않은 연산자가 입력되면 unreachable!()을 통해 프로그램을 중단(패닉)시킵니다.
                unreachable!()
            }
        };

        // 계산된 결과를 출력 함수로 넘겨서 표시합니다.
        print_output(result);
    }
}

// 계산 결과를 지정된 포맷으로 출력하는 헬퍼 함수입니다.
fn print_output(value: f64) {
    println!("  => {}", value);
}

