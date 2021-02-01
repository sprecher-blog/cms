// Sprecher - minimal blogging web app
mod article;

use actix_web::{web, App, HttpResponse, HttpServer, Result, get};
use askama::Template;
use crate::article::Post;
use std::fs;

// Load in static assets into binary
const UIKIT: &str = include_str!("../css/site.css");
const GRADIENTS: &str = include_str!("../css/gradients.css");
const JS: &str = include_str!("../js/uikit.min.js");
const ICONS: &str = include_str!("../js/uikit-icons.min.js");

// Create an index template
#[derive(Template)]
#[template(path = "card.html")]
pub struct Posts {
    posts: Vec<Post>,
}

// Read the articles and serve the previews of them
#[get("/")]
async fn index() -> Result<HttpResponse> {
    let dirs = fs::read_dir("./articles")?;
    let mut posts = vec![];
    for file in dirs {
        let file = file?.path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.ends_with("md") {
            let mut post = Post::new();
            post.load(fs::read_to_string(&file)?);
            post.url = format!("/articles/{}", name.split('.').next().unwrap());
            posts.push(post);
        }
    }
    let home = Posts {
        posts: posts,
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(home))
}

#[get("/articles/{file}")]
async fn articles(web::Path(file): web::Path<String>) -> Result<HttpResponse> {
    let contents = fs::read_to_string(format!("articles/{}.md", file))?;
    let mut article = Post::new();
    if article.load(contents).is_none() {
        Ok(HttpResponse::BadRequest().content_type("text/plain").body("Invalid sprecher-markdown file"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html").body(article.render().unwrap()))
    }
}

// Main function to handle requests
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // Pages
            .service(index)
            .service(articles)
            // Various resources
            .route("/css/site.css", web::get().to(|| HttpResponse::Ok().body(UIKIT)))
            .route("/css/gradients.css", web::get().to(|| HttpResponse::Ok().body(GRADIENTS)))
            .route("/js/uikit.min.js", web::get().to(|| HttpResponse::Ok().body(JS)))
            .route("/js/uikit-icons.min.js", web::get().to(|| HttpResponse::Ok().body(ICONS)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
