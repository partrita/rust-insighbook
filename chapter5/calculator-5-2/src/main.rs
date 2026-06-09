use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        // 1줄씩 읽어서 빈 줄이면 종료
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 공백문자로 구분
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // 수식 계산
        let left: f64 = tokens[0].parse().unwrap();
        let right: f64 = tokens[2].parse().unwrap();
        let result = match tokens[1] {
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
    }
}

fn print_output(value: f64) {
    println!("  => {}", value);
}
