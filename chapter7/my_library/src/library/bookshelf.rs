use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use super::book::Book;

pub struct Bookshelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}

impl Default for Bookshelf {
    fn default() -> Self {
        Self::new()
    }
}

impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self {
            books: Vec::new(),
            matcher,
        }
    }

    // 책을 추가하는 메서드
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // 제목으로 책을 검색하는 메서드
    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        let mut found_books = vec![];
        for book in &self.books {
            let match_result = self.matcher.fuzzy_match(&book.title, title_query);
            if let Some(score) = match_result {
                if score > 0 {
                    found_books.push(book);
                }
            }
        }
        found_books
    }

    // 제목이 완전 일치하는 책을 검색하는 메서드
    pub fn search_books_exact(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title == title_query)
            .collect()
    }
    // 제목 일부로 책을 검색하는 메서드
    pub fn search_books_partial(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title.contains(title_query))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Book, Bookshelf};
    #[test]
    fn test_bookshelf() {
        let mut shelf = Bookshelf::new();
        let book1 = Book::new("ChatGPT! AI로 배우는 Rust!", "홍길동");
        let book2 = Book::new("Python 프로그래밍 입문", "최영희");
        shelf.add_book(book1);
        shelf.add_book(book2);
        let found_books = shelf.search_books("chatgpt");
        println!("{:?}", found_books);
    }
}
