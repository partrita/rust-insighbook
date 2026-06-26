// my_package 크레이트의 라이브러리(lib.rs)에서 공개한 hello 함수를 가져옵니다.
use my_package::hello;

fn main() {
    // hello 함수를 호출하면서 매개변수로 "bin_1" 문자열 슬라이스를 전달합니다.
    hello("bin_1");
}
