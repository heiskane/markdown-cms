use actix_web::{
    get, http::header::ContentType, middleware::Logger, web, App, HttpResponse, HttpServer,
};
use actix_files;
use askama::Template;
use chrono::{DateTime, Local};
use serde::Deserialize;
use regex::Regex;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Template)]
#[template(path = "layout.html")]
struct Post<'a> {
    name: &'a str,
    content: &'a str,
    metadata: PostMetadata<'a>,
}

#[derive(Deserialize)]
struct PostMetadata<'a> {
    title: Option<&'a str>,
    description: &'a str,
    date: DateTime<Local>,
}

#[get("/posts/{post}/")]
async fn post(post: web::Path<String>, posts: web::Data<HashMap<String, PathBuf>>) -> HttpResponse {
    let post_name = post.into_inner();
    
    if !posts.contains_key(&post_name) {
        return HttpResponse::NotFound().body("Post not found");
    }

    let content = fs::read_to_string(posts.get(&post_name).unwrap()).unwrap();
    let re = Regex::new(r"^---([\s\S]*?)(\n---)").unwrap();
    let metadata = re.captures(&content).expect("Getting metadata from markdown").get(1).unwrap().as_str();
    let stripped_content = re.replace(&content, "").into_owned();

    let meta_obj: PostMetadata = serde_yaml::from_str(metadata).unwrap();

    let response = Post {
        name: &post_name,
        content: &stripped_content,
        metadata: meta_obj,
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(response.render().unwrap())
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
