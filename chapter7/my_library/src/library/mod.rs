// 'library' 디렉터리 내부의 하위 파일/디렉터리 모듈들을 정의하고 외부로 공개하기 위해 pub mod로 선언합니다.

// 'book' 모듈(book.rs 파일)을 공개하여 외부 크레이트 등에서도 Book 구조체를 다룰 수 있게 합니다.
pub mod book;

// 'bookshelf' 모듈(bookshelf.rs 파일)을 공개하여 외부에서 Bookshelf 구조체를 다룰 수 있게 합니다.
pub mod bookshelf;
