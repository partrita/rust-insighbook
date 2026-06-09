#![allow(clippy::all, unused)]

fn main() {
    println!("Hello, world!");
}

fn solve(s: Vec<char>) -> bool {
    let n = s.len();

    // 아직 대응하는 )를 발견하지 못한 (의 개수를 left로 정의
    // 마지막에는 left=0이 되어야 함
    let mut left = 0;

    // 문자열을 왼쪽에서부터 한 글자씩 찾기
    for i in 0..n {
        if s[i] == '(' {
            // 확인한 문자가 (라면 left에 1 더하기
            left += 1;
        } else {
            // 확인한 문자가 )일 때 left가 0이라면 올바른 괄호 나열이 아님
            if left == 0 {
                return false;
            } else {
                // left가 0보다 크면 left에 1 빼기
                left -= 1;
            }
        }
    }

    // 모든 문자를 확인했을 때 left가 0이라면 문자열은 올바른 괄호 나열
    if left == 0 {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ans = solve(vec!['(', ')', '(', ')', '(', ')', '(', ')', '(', ')']);
        assert_eq!(ans, true);
    }
}
