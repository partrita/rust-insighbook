// my_library 크레이트 내부의 library 모듈로부터 Book 구조체와 Bookshelf 구조체를 가져옵니다.
use my_library::library::{book::Book, bookshelf::Bookshelf};

fn main() {
    // 새로운 책장(Bookshelf) 인스턴스를 가변(mutable) 형태로 생성합니다. (책 추가 및 수정이 필요하므로 mut로 설정)
    let mut shelf = Bookshelf::new();

    // Book::new 연관 함수를 통해 제목과 저자명을 지정하고 두 권의 책 인스턴스를 생성합니다.
    let book1 = Book::new("ChatGPT! AI로 배우는 Rust!", "홍길동");
    let book2 = Book::new("Python 프로그래밍 입문", "최영희");

    // 책장(shelf)에 생성한 책(book1, book2)을 추가(add_book)합니다.
    shelf.add_book(book1);
    shelf.add_book(book2);

    // 책장 인스턴스의 search_books 메서드를 활용해 제목에 "chatgpt"가 포함되는 책들을 검색합니다.
    let found_books = shelf.search_books("chatgpt");

    // 검색된 결과 목록을 화면에 디버그 모드로 출력합니다.
    println!("{:?}", found_books);
}
