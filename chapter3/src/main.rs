// rand 크레이트에서 난수 생성을 위한 Rng 트레이트를 가져옵니다.
use rand::Rng;

fn main() {
    // 맞춘 정답 개수를 기록하는 가변 변수(mut)를 선언하고 0으로 초기화합니다.
    let mut num_of_correct = 0;
    // 맞춘 정답의 개수가 3개 미만인 동안 반복합니다. (총 3문제를 맞추면 루프 종료)
    while num_of_correct < 3 {
        // rand::rng().random_range(1..=2)를 통해 1 또는 2 중 하나의 난수를 생성하여 퀴즈 모드를 결정합니다.
        let quiz_mode = rand::rng().random_range(1..=2);
        // 생성된 퀴즈 모드(1 또는 2)에 따라 매칭을 수행합니다.
        match quiz_mode {
            // 퀴즈 모드가 1인 경우 (덧셈 문제)
            1 => loop {
                // 0 이상 100 미만의 정수 난수를 생성하여 첫 번째 피연산자로 지정합니다.
                let op1 = rand::rng().random_range(0..100);
                // 0 이상 100 미만의 정수 난수를 생성하여 두 번째 피연산자로 지정합니다.
                let op2 = rand::rng().random_range(0..100);
                // 화면에 문제를 출력합니다. (예: 30 + 45 = ??)
                println!("{} + {} = ??", op1, op2);
                // 사용자에게 정답을 입력하라는 안내 메시지를 출력합니다.
                println!("??의 값을 입력하세요:");
                // 입력을 저장할 가변(mut) String 인스턴스를 새롭게 생성합니다.
                let mut ans_input = String::new();
                // 표준 입력(stdin)으로부터 한 줄을 읽어 ans_input 변수에 저장합니다. unwrap()은 에러 시 패닉을 발생시킵니다.
                std::io::stdin().read_line(&mut ans_input).unwrap();
                // 입력값 앞뒤의 공백/줄바꿈 문자를 제거(.trim())하고, 32비트 정수형(i32)으로 변환(.parse::<i32>())합니다.
                let ans_input = ans_input.trim().parse::<i32>().unwrap();
                // 입력한 값이 두 피연산자의 합과 같다면 정답 처리합니다.
                if ans_input == op1 + op2 {
                    println!("정답!");
                    // 맞춘 개수를 1 증가시킵니다.
                    num_of_correct += 1;
                    // 현재 루프(loop)를 탈출하여 다음 문제로 넘어갑니다.
                    break;
                } else {
                    // 입력값이 틀린 경우 오답 메시지를 출력하고, 이 문제(loop)를 다시 풉니다.
                    println!("오답!");
                }
            },
            // 퀴즈 모드가 2인 경우 (뺄셈 문제)
            2 => loop {
                // 0 이상 100 미만의 정수 난수를 생성하여 첫 번째 피연산자로 지정합니다.
                let op1 = rand::rng().random_range(0..100);
                // 0 이상 100 미만의 정수 난수를 생성하여 두 번째 피연산자로 지정합니다.
                let op2 = rand::rng().random_range(0..100);
                // 화면에 문제를 출력합니다. (예: 50 - 20 = ??)
                println!("{} - {} = ??", op1, op2);
                // 사용자에게 정답을 입력하라는 안내 메시지를 출력합니다.
                println!("??의 값을 입력하세요:");
                // 입력을 저장할 가변(mut) String 인스턴스를 새롭게 생성합니다.
                let mut ans_input = String::new();
                // 표준 입력(stdin)으로부터 한 줄을 읽어 ans_input 변수에 저장합니다.
                std::io::stdin().read_line(&mut ans_input).unwrap();
                // 입력값 앞뒤의 공백/줄바꿈 문자를 제거하고 정수로 파싱합니다.
                let ans_input = ans_input.trim().parse::<i32>().unwrap();
                // 입력한 값이 두 피연산자의 차와 같다면 정답 처리합니다.
                if ans_input == op1 - op2 {
                    println!("정답!");
                    // 맞춘 개수를 1 증가시킵니다.
                    num_of_correct += 1;
                    // 현재 루프를 탈출하여 다음 문제로 넘어갑니다.
                    break;
                } else {
                    // 틀린 경우 오답 메시지를 출력하고, 이 문제(loop)를 다시 풉니다.
                    println!("오답!");
                }
            },
            // 1과 2 이외의 값은 발생할 수 없으므로 도달 불가능(unreachable!) 처리를 합니다.
            _ => unreachable!(),
        }
    }
    // 3문제를 모두 맞추어 while 루프를 탈출하면 최종 클리어 메시지를 출력합니다.
    println!("클리어!")
}
