use actix_web::{
    get, http::header::ContentType, middleware::Logger, web, App, HttpResponse, HttpServer,
};
use actix_files;
use askama::Template;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use regex::Regex;
use std::{collections::HashMap, fs};


#[derive(Template)]
#[template(path = "hello.html")]
struct Post<'a> {
    name: &'a str,
    content: &'a str,
    metadata: &'a PostMetadata,
}

struct InternalPost {
    name: String,
    content: String,
    metadata: PostMetadata,
}

#[derive(Template)]
#[template(path = "listing.html")]
struct PostListing<'a> {
    posts: Vec<&'a InternalPost>,
}

#[derive(Deserialize)]
struct PostMetadata {
    title: Option<String>,
    description: String,
    date: DateTime<Utc>,  // Maybe use last modified date on the file?
}

#[get("/")]
async fn post_listing(posts: web::Data<HashMap<String, InternalPost>>) -> HttpResponse {
    let posts = PostListing {
        posts: posts.values().collect(),
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(posts.render().unwrap())
}

#[get("/posts/{post}/")]
async fn post<'a>(post: web::Path<String>, posts: web::Data<HashMap<String, InternalPost>>) -> HttpResponse {
    let post_name = post.into_inner();

    if !posts.contains_key(&post_name) {
        return HttpResponse::NotFound().body("Post not found");
    }

    let internal_post = posts.get(&post_name).unwrap();
    let post_template = Post {
        name: &internal_post.name,
        content: &internal_post.content,
        metadata: &internal_post.metadata,
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(post_template.render().unwrap())
}


// TODO: Add better error handling by returning result?
fn get_posts(content_path: &str) -> HashMap<String, InternalPost> {
    let paths = fs::read_dir(content_path).unwrap();

    paths.into_iter().fold(HashMap::new(), |mut a, d| {
        let path = d.unwrap().path();
        let post_name = path.file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let content = fs::read_to_string(path).unwrap();
        let re = Regex::new(r"^---([\s\S]*?)(\n---)").unwrap();
        let metadata = re.captures(&content).expect("Getting metadata from markdown").get(1).unwrap().as_str();
        let stripped_content = re.replace(&content, "").into_owned();

        let meta_obj = serde_yaml::from_str(metadata).unwrap();

        a.insert(post_name.to_string(), InternalPost {
            name: post_name.to_string(),
            content: stripped_content,
            metadata: meta_obj,
        });
        a
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let posts = web::Data::new(get_posts("./content"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(posts.clone())
            .service(actix_files::Files::new("/static", "./static"))
            .service(post_listing)
            .service(post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::get_posts;

    #[test]
    fn test_post_gen() {
        let posts = get_posts("./test_content");

        assert_eq!(posts.len(), 1);

        let post = posts.get("potato").unwrap();

        assert_eq!(post.name, "potato");
        assert_eq!(post.metadata.title, Some("imma title".to_string()));
        assert_eq!(post.metadata.description, "imma description");
    }
}
