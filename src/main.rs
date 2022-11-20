use actix_web::{get, App, HttpResponse, HttpServer, Responder, http::header::{CONTENT_TYPE, ContentType}, middleware::Logger};
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
    content: &'a str,
}

#[get("/")]
async fn hello() -> HttpResponse {
    let res = HelloTemplate { name: "World", content: concat!(
        "# Hello\n",
        "## World\n",
        "### Potato\n",
        "Tomato is not a tomato",
    ) };
    HttpResponse::Ok().content_type(ContentType::html())
        .body(res.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
 
    HttpServer::new(|| {
        App::new().wrap(Logger::default())
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
