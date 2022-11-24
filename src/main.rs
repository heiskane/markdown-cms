use actix_files;
use actix_web::{
    get, http::header::ContentType, middleware::Logger, web, App, HttpResponse, HttpServer,
};
use askama::Template;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Template)]
#[template(path = "layout.html")]
struct Post<'a> {
    // TODO: Add implemetation for FromStr that parses the metadata and all that good good juice
    name: &'a str,
    content: &'a str,
}

#[get("/posts/{post}/")]
async fn post(post: web::Path<String>, posts: web::Data<HashMap<String, PathBuf>>) -> HttpResponse {
    let post_name = post.into_inner();
    if !posts.contains_key(&post_name) {
        return HttpResponse::NotFound().body("Post not found");
    }
    let response = Post {
        name: &post_name,
        content: &fs::read_to_string(posts.get(&post_name).unwrap()).unwrap(),
    };
    return HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(response.render().unwrap());
}

fn get_posts() -> HashMap<String, PathBuf> {
    let paths = fs::read_dir("./content").unwrap();

    let mut posts = HashMap::new();
    for dir in paths.into_iter() {
        let path = dir.unwrap().path();
        posts.insert(
            path.file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
            path,
        );
    }
    posts
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let posts = web::Data::new(get_posts());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(posts.clone())
            .service(actix_files::Files::new("/static", "./static"))
            .service(post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
