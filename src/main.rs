use actix_files;
use actix_web::{
    get, http::header::ContentType, middleware::Logger, web, App, HttpResponse, HttpServer,
};
use anyhow::Result;
use askama::Template;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize)]
struct Config {
    content_path: String,
}

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

/// Metadata for a post
///
/// Struct containing the metadata of a post. This struct is
/// constructed from yaml that is read from the posts markdown file.
///
///
/// # Examples
/// Example markdown file containing metadata:
/// ```md
/// ---
/// title: this is a title
/// description: this is the description
/// date: this should be the posting date
/// ---
///
/// # Title
/// just normal markdown here
/// ```
///
/// Get metadata from the markdown like this:
/// ```rust
/// // read the markdown file
/// let content = fs::read_to_string(path)?;
///
/// // regex to get metadata block from markdown
/// let re = Regex::new(r"^---([\s\S]*?)(\n---)")?;
///
/// // save metadata yaml to a variable
/// let metadata = re
///     .captures(&content)
///     .expect("Getting metadata from markdown")
///     .get(1)
///     .unwrap()
///     .as_str();
///
/// // strip metadata from markdown
/// let stripped_content = re.replace(&content, "").into_owned();
///
/// // create the metadata object from yaml
/// let meta_obj = serde_yaml::from_str(metadata)?;
/// ```
#[derive(Deserialize)]
struct PostMetadata {
    title: Option<String>,
    description: String,
    date: DateTime<Utc>, // Maybe use last modified date on the file?
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
async fn post<'a>(
    post: web::Path<String>,
    posts: web::Data<HashMap<String, InternalPost>>,
) -> HttpResponse {
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
fn get_posts(content_path: &str) -> Result<HashMap<String, InternalPost>> {
    let paths = fs::read_dir(content_path)?;

    paths.into_iter().try_fold(HashMap::new(), |mut a, d| {
        let path = d?.path();
        let post_name = path
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let content = fs::read_to_string(path)?;
        let re = Regex::new(r"^---([\s\S]*?)(\n---)")?;
        let metadata = re
            .captures(&content)
            .expect("Getting metadata from markdown")
            .get(1)
            .unwrap()
            .as_str();
        let stripped_content = re.replace(&content, "").into_owned();

        let meta_obj = serde_yaml::from_str(metadata)?;

        a.insert(
            post_name.to_string(),
            InternalPost {
                name: post_name.to_string(),
                content: stripped_content,
                metadata: meta_obj,
            },
        );
        Ok(a)
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = envy::from_env::<Config>().unwrap();

    let post_map = get_posts(&config.content_path).expect("Getting posts");
    let posts = web::Data::new(post_map);

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

    use actix_web::test;

    use super::*;

    #[test]
    async fn test_post_gen() {
        let posts = get_posts("./test_content").unwrap();

        assert_eq!(posts.len(), 1);

        let post_obj = posts.get("potato").unwrap();

        assert_eq!(post_obj.name, "potato");
        assert_eq!(post_obj.metadata.title, Some("imma title".to_string()));

        assert_eq!(post_obj.metadata.description, "imma description");
    }
}
