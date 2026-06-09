#![allow(clippy::all, unused)]

fn main() {
    println!("Hello, world!");
}

fn solve1(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // a와 b 요소 개수 취득
    let n = a.len();
    let m = b.len();

    // 빈 Vec의 ans 생성
    let mut ans = vec![];

    // a 요소를 모두 ans에 넣기
    for i in 0..n {
        ans.push(a[i]);
    }

    // b 요소를 모두 ans에 넣기
    for i in 0..m {
        ans.push(b[i]);
    }

    // ans를 오름차순으로 정렬
    ans.sort();
    ans
}

fn solve2(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // a와 b 요소 개수 취득
    let n = a.len();
    let m = b.len();

    // 빈 Vec의 ans 생성
    let mut ans = vec![];

    // a의 첫 번째 요소를 가리키는 값을 i로 정의
    // i에 1을 더하면 a의 첫 번째 요소를 삭제한 것과 같은 효과
    // i=n이면 a가 비었다고 판정 가능
    let mut i = 0;

    // 마찬가지 방식으로 b의 첫 번째 요소를 가리키는 값을 j로 정의
    let mut j = 0;

    // a와 b가 모두 비워질 때까지 다음 처리를 반복
    while i < n || j < m {
        if i == n {
            // a가 비었을 때 b의 첫 번째 요소를 ans에 추가
            ans.push(b[j]);

            // 추가된 b의 첫 번째 요소 제거
            j += 1;
        } else if j == m {
            // b가 비었을 때 a의 첫 번째 요소를 ans에 추가
            ans.push(a[i]);

            // 추가된 a의 첫 번째 요소 제거
            i += 1;
        } else if a[i] < b[j] {
            // a의 첫 번째 요소가 b의 첫 번째 요소보다 작으면
            // a의 첫 번째 요소를 ans에 추가
            ans.push(a[i]);

            // 추가된 a의 첫 번째 요소 제거
            i += 1;
        } else {
            // b의 첫 번째 요소가 a의 첫 번째 요소보다 작거나 같으면
            // b의 첫 번째 요소를 ans에 추가
            ans.push(b[j]);

            // 추가된 b의 첫 번째 요소 제거
            j += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let ans = solve1(vec![1, 2, 4, 5, 8], vec![2, 3, 4, 5, 6, 7]);
        assert_eq!(ans, vec![1, 2, 2, 3, 4, 4, 5, 5, 6, 7, 8]);
    }

    #[test]
    fn test_solve2() {
        let ans = solve2(vec![1, 2, 4, 5, 8], vec![2, 3, 4, 5, 6, 7]);
        assert_eq!(ans, vec![1, 2, 2, 3, 4, 4, 5, 5, 6, 7, 8]);
    }
}
