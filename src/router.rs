use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest};

pub async fn favicon(req: HttpRequest) -> HttpResponse {
    let file_path = std::path::PathBuf::from("res/favicon.ico");
    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();
    file.into_response(&req)
}

/* Async function for explicit HTTP GET endpoint. */
pub async fn hi() -> impl Responder {
    HttpResponse::Ok().body("Hi!")
}

/* Annotated HTTP GET endpoint. */
#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

/* Annotated HTTP GET endpoint with path parameter. */
#[get("/hello/{name}")]
pub async fn hello_name(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

/* Annotated HTTP POST endpoint. */
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
