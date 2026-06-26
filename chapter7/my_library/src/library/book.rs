// 디버깅 목적으로 println!("{:?}", book)과 같이 구조체 내부 정보를 출력할 수 있도록 Debug 트레이트를 유도(derive)합니다.
#[derive(Debug)]
// 외부 크레이트 및 모듈에서 Book 구조체를 사용하고 인스턴스를 생성할 수 있도록 pub 구조체로 선언합니다.
// 또한 구조체의 각 필드(title, author)도 외부에서 직접 읽고 쓸 수 있도록 pub 키워드를 붙여줍니다.
pub struct Book {
    pub title: String,  // 책 제목 (소유권이 있는 String 타입)
    pub author: String, // 저자 이름 (소유권이 있는 String 타입)
}

impl Book {
    // Book 인스턴스를 새롭게 생성하여 반환하는 공개 생성자 함수(new)입니다.
    // 매개변수로 문자열의 참조 즉, 문자열 슬라이스(&str) 타입을 전달받습니다.
    pub fn new(title: &str, author: &str) -> Self {
        // 구조체 리터럴을 통해 필드 값을 대입하고 인스턴스를 반환합니다.
        Self {
            // 복사 비용이 발생하지만, 소유권이 있는 String 타입으로 변환하기 위해 to_string()을 호출합니다.
            title: title.to_string(),
            author: author.to_string(),
        }
    }
}
