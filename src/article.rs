// Article.rs - Tools for reading sprecher-markdown format
use comrak::{ComrakOptions, markdown_to_html};
use askama::Template;

const READING_CPM: usize = 1600;

// Create a post template
#[derive(Template, Debug, Clone)]
#[template(path = "article.html", escape = "none")]
pub struct Post {
    pub title: String,
    pub author: String,
    pub description: String,
    pub read_time: usize,
    pub body: String,
    pub url: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            title: "Untitled article".to_string(),
            author: "Sprecher".to_string(),
            description: "This article has no description".to_string(),
            body: "This article has no body".to_string(),
            read_time: 1,
            url: "articles/".to_string(),
        }
    }
    pub fn load(&mut self, s: String) -> Option<()> {
        // Parses an article
        let mut lines = s.split('\n').into_iter();
        // Parse header and extract information
        while let Some(i) = lines.next() {
            let mut parts = i.split(':').map(|x| x.trim());
            match (parts.next(), parts.last()) {
                (Some("title"), Some(title)) => 
                    self.title = title.to_string(),
                (Some("author"), Some(author)) => 
                    self.author = author.to_string(),
                (Some("description"), Some(desc)) => 
                    self.description = desc.to_string(),
                (Some("___"), None) | (Some("---"), None) => break,
                (Some(""), None) => (),
                _ => return None,
            }
        }
        // Convert body
        let mut options = ComrakOptions::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.superscript = true;
        options.extension.footnotes = true;
        options.extension.description_lists = true;
        self.body = markdown_to_html(
            &lines.collect::<Vec<_>>().join("\n"), 
            &options
        );
        // Calculate read time
        self.read_time = (
            self.body
                .chars()
                .filter(|x| x.is_ascii())
                .count() as f64 / 
                READING_CPM as f64
        ).round() as usize;
        if self.read_time == 0 { self.read_time = 1 }
        Some(())
    }
}
