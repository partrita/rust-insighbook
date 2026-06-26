// Actix-web 웹 프레임워크에서 웹 애플리케이션 구축 및 HTTP 처리를 위해 필요한 도구들을 가져옵니다.
use actix_web::{
    App, HttpResponse, HttpResponseBuilder, HttpServer, get, http::StatusCode, post, web,
};
// Askama 템플릿 엔진에서 템플릿 렌더링 및 에러 처리를 위한 도구들을 가져옵니다.
use askama::{Error, Template};
// sqlx 라이브러리에서 SQLite 비동기 데이터베이스 연결 풀 및 결과를 가져오기 위한 트레이트/구조체를 가져옵니다.
use sqlx::{Row, SqlitePool};

// Askama 템플릿 객체를 인자로 받아, 렌더링에 성공하면 HttpResponse 객체로 변환하여 반환합니다.
// 렌더링 중 오류가 발생할 경우 unwrap_or_else 블록에서 500 내부에러(Internal Server Error) 응답을 반환합니다.
// <T: askama::Template>은 제네릭 타입 매개변수로, Askama의 Template 트레이트를 구현한 모든 구조체를 의미합니다.
fn into_response<T: askama::Template>(tmpl: &T) -> HttpResponse {
    try_into_response(tmpl).unwrap_or_else(|err| HttpResponse::from_error(err.into_io_error()))
}

// 템플릿 객체를 HTML 텍스트 문자열로 실제로 렌더링하는 헬퍼 함수입니다.
// 에러가 발생할 가능성이 있으므로 Result 타입을 반환합니다.
fn try_into_response<T: askama::Template>(tmpl: &T) -> Result<HttpResponse, Error> {
    // 템플릿을 HTML 문자열로 렌더링합니다. 실패하면 ? 연산자가 에러를 즉시 호출자에게 반환합니다.
    let value = tmpl.render()?;
    // HTTP 응답 코드를 200 OK로 설정하고 HTML 문서 응답 헤더 및 본문(value)을 세팅하여 응답을 빌드합니다.
    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .content_type("text/html; charset=UTF-8")
        .body(value))
}

// `hello.html` 템플릿을 렌더링하기 위한 구조체 정의입니다.
// Template 파생 매크로를 사용하여 templates/hello.html 파일과 매핑합니다.
#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    // 템플릿 내에서 {{ name }} 형태로 출력될 문자열 변수입니다.
    name: String,
}

// "/hello/{name}" GET 요청을 가로채서 처리하는 비동기 라우트 핸들러 함수입니다.
// web::Path<String>을 사용하면 URL 경로 매개변수 중 {name}에 해당하는 값을 안전하게 추출할 수 있습니다.
#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> HttpResponse {
    // URL 경로에서 얻은 name 값을 소유권 채로 가져와 템플릿 구조체를 생성합니다.
    let hello = HelloTemplate {
        name: name.into_inner(),
    };
    // 템플릿을 렌더링하여 HttpResponse 형태로 반환합니다.
    into_response(&hello)
}

// `todo.html` 템플릿을 렌더링하기 위한 구조체 정의입니다.
// templates/todo.html 파일과 매핑됩니다.
#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {
    // 화면의 할 일 목록 리스트에 바인딩할 문자열 벡터입니다.
    tasks: Vec<String>,
}

// 루트 경로("/") GET 요청을 처리하는 비동기 핸들러 함수입니다.
// web::Data<SqlitePool>을 사용하여 Actix-web 앱 내에 공유 등록된 SQLite 데이터베이스 풀 인스턴스를 주입받습니다.
#[get("/")]
async fn todo(pool: web::Data<SqlitePool>) -> HttpResponse {
    // SQL 쿼리를 비동기적으로 실행하여 tasks 테이블의 모든 task를 조회합니다.
    // pool.as_ref()를 통해 풀에 대한 참조를 sqlx 쿼리에 전달하며, 비동기이므로 .await로 결과를 기다린 후 unwrap()합니다.
    let rows = sqlx::query("SELECT task FROM tasks;")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    // 데이터베이스 조회 행(rows)을 순회하며 task 컬럼의 텍스트(String) 값을 추출하고 벡터로 수집합니다.
    let tasks: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("task"))
        .collect();
    // 할 일 목록 템플릿을 생성합니다.
    let todo = TodoTemplate { tasks };
    // 템플릿을 HTML 응답으로 변환하여 반환합니다.
    into_response(&todo)
}

// HTML 폼(Form) 전송 데이터를 구조체로 바인딩(역직렬화)하기 위한 구조체입니다.
// Deserialize 매크로를 통해 HTTP Form의 필드명을 구조체 필드명과 자동으로 일치시켜 줍니다.
#[derive(serde::Deserialize)]
struct Task {
    // 삭제할 대상의 텍스트 ID
    id: Option<String>,
    // 신규 추가할 할 일 텍스트
    task: Option<String>,
}

// "/update" POST 요청을 처리하여 할 일 목록을 추가하거나 삭제하는 비동기 핸들러입니다.
// web::Form<Task>를 사용하여 브라우저가 전송한 Form 데이터를 자동으로 Task 구조체 형태로 파싱하여 주입받습니다.
#[post("/update")]
async fn update(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    // web::Form 래퍼 클래스로부터 내부 구조체 Task의 소유권을 획득합니다.
    let task = form.into_inner();

    // 만약 Form 데이터에 삭제할 항목의 id(일정 내용)가 들어있다면, 데이터베이스에서 삭제 쿼리를 실행합니다.
    if let Some(id) = task.id {
        sqlx::query("DELETE FROM tasks WHERE task = ?")
            .bind(id) // 물음표(?) 플레이스홀더 자리에 id 변수를 바인딩합니다.
            .execute(pool.as_ref())
            .await
            .unwrap();
    }
    // Form 데이터에 신규 추가할 task 문자열이 들어있고 비어있지 않다면, 데이터베이스에 추가 쿼리를 실행합니다.
    match task.task {
        Some(task) if !task.is_empty() => {
            sqlx::query("INSERT INTO tasks (task) VALUES (?)")
                .bind(task) // 플레이스홀더(?) 자리에 task 변수 값을 바인딩하여 SQL 인젝션을 방지합니다.
                .execute(pool.as_ref())
                .await
                .unwrap();
        }
        _ => {} // 그 외의 경우는 아무것도 하지 않습니다.
    }

    // 데이터베이스 갱신 작업 완료 후, 메인 페이지("/")로 사용자 화면을 리다이렉트(302 Found)시킵니다.
    HttpResponse::Found()
        .append_header(("Location", "/")) // 리다이렉션 위치 헤더 추가
        .finish()
}

// Actix-web 웹 애플리케이션의 비동기 메인 진입점입니다.
// 이 어노테이션은 actix-web의 비동기 런타임(tokio 기반) 상에서 main 함수가 비동기(async)로 구동되도록 포장해 줍니다.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 메모리에만 상주하는 가상의 SQLite 데이터베이스 풀("sqlite::memory:")을 비동기 연결하여 생성합니다.
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // 비동기 sqlx 실행기를 통해 테이블 생성 쿼리를 보냅니다.
    sqlx::query("CREATE TABLE tasks (task TEXT);")
        .execute(&pool)
        .await
        .unwrap();

    // 웹 서비스 시작 전에 테스트용 초기 데이터를 3건 삽입합니다.
    sqlx::query("INSERT INTO tasks (task) VALUES ('작업1');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO tasks (task) VALUES ('작업2');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO tasks (task) VALUES ('작업3');")
        .execute(&pool)
        .await
        .unwrap();

    // HTTP 웹 서버 인스턴스를 새롭게 기동시킵니다.
    HttpServer::new(move || {
        // move 키워드를 통해 외부 환경의 SQLite pool 변수를 클로저 내부로 캡처(소유권 이동)해옵니다.
        App::new()
            .service(hello) // "/hello/{name}" GET 라우트 서비스 등록
            .service(todo)  // "/" GET 라우트 서비스 등록
            .service(update) // "/update" POST 라우트 서비스 등록
            // 모든 핸들러에서 안전하게 데이터베이스 풀을 얻어 쓸 수 있도록 애플리케이션 데이터(app_data)에 등록합니다.
            .app_data(web::Data::new(pool.clone()))
    })
    // 로컬 루프백 주소(127.0.0.1)와 포트 번호 8080을 결합하여 대기합니다.
    .bind(("127.0.0.1", 8080))?
    // 서버 프로세스를 시작(run)하고 비동기적으로 대기(await)합니다.
    .run()
    .await
}

