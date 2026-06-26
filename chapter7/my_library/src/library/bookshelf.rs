// 외부 크레이트인 fuzzy_matcher에서 퍼지 매칭 인터페이스(FuzzyMatcher)와 구현체(SkimMatcherV2)를 가져옵니다.
// 퍼지 매칭이란 문자열의 일부만 일치하거나 오타가 있어도 유사도를 측정하여 검색해 주는 기능입니다.
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

// 부모 모듈(super 즉, library 모듈) 하위의 book 모듈로부터 Book 구조체를 가져옵니다.
use super::book::Book;

// 책들을 저장할 책장 구조체입니다. 외부(다른 크레이트)에서 인스턴스화하여 사용할 수 있도록 pub로 선언합니다.
// 구조체 자체는 공개(pub)되지만, 내부 필드(books, matcher)는 pub 키워드가 없으므로 
// 구조체가 속한 모듈 외부에서는 직접 필드에 접근하여 수정하거나 읽을 수 없습니다(캡슐화).
pub struct Bookshelf {
    books: Vec<Book>,      // Book 구조체 객체들을 저장하는 동적 배열(Vector) 컬렉션
    matcher: SkimMatcherV2, // 퍼지 문자열 검색을 수행할 매처 객체
}

// Default 트레이트를 구현하여 Bookshelf::default()를 호출할 때 기본 인스턴스를 생성할 수 있게 합니다.
// 보통 struct가 기본값을 가질 수 있도록 할 때 이 트레이트를 구현합니다.
impl Default for Bookshelf {
    fn default() -> Self {
        // 이미 구현된 생성자인 new() 메서드를 호출하여 기본 객체를 반환합니다.
        Self::new()
    }
}

impl Bookshelf {
    // 새로운 Bookshelf 인스턴스를 생성하여 반환하는 생성자 함수입니다.
    // pub 키워드가 있으므로 외부에서 Bookshelf::new()로 호출할 수 있습니다.
    pub fn new() -> Self {
        // SkimMatcherV2의 기본값을 생성합니다.
        let matcher = SkimMatcherV2::default();
        // Bookshelf 인스턴스를 구조체 리터럴 표현으로 생성하여 반환합니다.
        Self {
            books: Vec::new(), // 빈 벡터를 생성하여 대입합니다.
            matcher,           // 변수명과 필드명이 같으므로 생략형태(field init shorthand)로 전달합니다.
        }
    }

    // 책을 책장에 추가하는 메서드입니다. 
    // self의 내부 상태(books 벡터)를 수정해야 하므로 가변 참조(&mut self)를 매개변수로 받습니다.
    pub fn add_book(&mut self, book: Book) {
        // books 벡터의 끝에 전달받은 book 객체를 추가(push)합니다.
        self.books.push(book);
    }

    // 퍼지 검색(Fuzzy Search)을 통해 입력된 쿼리와 유사한 제목의 책들을 검색합니다.
    // &self를 받아 bookshelf 객체의 소유권을 가져가지 않고 읽기 전용으로 참조합니다.
    // 검색 쿼리 문자열(&str)을 받아, Bookshelf 내부 책들의 불변 참조(&Book) 목록을 담은 벡터를 반환합니다.
    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        // 검색 결과를 저장할 빈 벡터를 생성합니다.
        let mut found_books = vec![];
        // books 벡터의 각 요소에 대해 반복문(for)을 돕니다. 불변 참조(&self.books)이므로 요소도 참조(&book)가 됩니다.
        for book in &self.books {
            // 퍼지 매칭 점수를 계산합니다. 일치하는 경우 Some(score)를 반환하고, 일치하지 않으면 None을 반환합니다.
            let match_result = self.matcher.fuzzy_match(&book.title, title_query);
            // match_result가 Some인 경우 score 값을 바인딩하여 조건문을 실행합니다(if let 패턴 매칭).
            if let Some(score) = match_result {
                // 매칭 점수가 0점보다 높은 경우(즉, 연관성이 있는 경우) 결과 벡터에 해당 책의 참조를 추가합니다.
                if score > 0 {
                    found_books.push(book);
                }
            }
        }
        // 찾은 책들의 참조 리스트인 found_books 벡터를 반환합니다 (Rust는 마지막 줄에 세미콜론이 없으면 식의 값이 반환됩니다).
        found_books
    }

    // 제목이 완전히 일치하는 책들을 검색하는 메서드입니다.
    // 반복자(iter)와 filter를 활용한 함수형 스타일의 가독성 높은 컬렉션 처리 방식입니다.
    pub fn search_books_exact(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter() // 벡터의 각 요소를 순회하는 반복자(Iterator)를 생성합니다.
            // filter 메서드는 클로저(|book| ...) 내부 조건이 true인 요소만 남깁니다.
            .filter(|book| book.title == title_query)
            // 수집기(collect)를 사용하여 걸러진 반복자의 결과들을 Vec<&Book> 타입의 벡터로 변환합니다.
            .collect()
    }

    // 제목의 일부가 포함되어 있는지 일치 여부를 검사하여 책을 검색하는 메서드입니다.
    pub fn search_books_partial(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter() // 반복자 생성
            // 책 제목에 title_query가 부분 문자열로 포함되어 있는지(contains) 확인합니다.
            .filter(|book| book.title.contains(title_query))
            .collect() // 필터링된 참조를 수집하여 벡터로 빌드 후 반환
    }
}

// 'cargo test' 명령어를 통해 테스트 코드를 빌드하고 실행할 때에만 컴파일에 포함되는 테스트 모듈 정의입니다.
#[cfg(test)]
mod tests {
    // 부모 모듈(super)에 있는 Book 구조체와 Bookshelf 구조체를 가져옵니다.
    use super::{Book, Bookshelf};
    
    // 이 함수가 단위 테스트(Unit Test) 메서드임을 컴파일러에 알리는 어트리뷰트입니다.
    #[test]
    fn test_bookshelf() {
        // 가변 책장을 만듭니다.
        let mut shelf = Bookshelf::new();
        // 테스트용 Book 구조체 인스턴스를 두 개 생성합니다.
        let book1 = Book::new("ChatGPT! AI로 배우는 Rust!", "홍길동");
        let book2 = Book::new("Python 프로그래밍 입문", "최영희");
        // 생성한 책들을 책장에 추가합니다.
        shelf.add_book(book1);
        shelf.add_book(book2);
        // "chatgpt"로 퍼지 검색을 수행합니다 (대소문자 등을 유연하게 매칭).
        let found_books = shelf.search_books("chatgpt");
        // 검색 결과를 터미널 디버그 출력으로 확인합니다.
        println!("{:?}", found_books);
    }
}

