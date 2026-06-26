// my_library 크레이트 내의 library 모듈에서 Book 구조체와 Bookshelf 구조체를 가져옵니다.
use my_library::library::{book::Book, bookshelf::Bookshelf};

fn main() {
    // 새로운 책장(Bookshelf) 인스턴스를 가변(mutable) 변수로 생성합니다. (책을 추가해야 하므로 mut 필요)
    let mut shelf = Bookshelf::new();

    // Book::new 함수를 사용해서 책 제목과 저자명을 지정해 두 권의 책 인스턴스를 생성합니다.
    let book1 = Book::new("ChatGPT! AI로 배우는 Rust!", "홍길동");
    let book2 = Book::new("Python 프로그래밍 입문", "최영희");

    // 책장(shelf)에 책 두 권을 추가(add_book)합니다.
    shelf.add_book(book1);
    shelf.add_book(book2);

    // 책장에서 제목에 "chatgpt"(대소문자 무관)가 포함된 책을 검색합니다.
    let found_books = shelf.search_books("chatgpt");

    // 검색된 책들의 목록을 디버그 포맷({:?})을 사용하여 화면에 출력합니다.
    println!("{:?}", found_books);
}
