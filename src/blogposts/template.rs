use super::ParsedBlogPost;

pub fn render_blogpost(blogpost: &ParsedBlogPost<'_>) -> String {
    format!(
        "<h1>{}</h1><h2>{}</h2>{}",
        blogpost.title(),
        blogpost.description(),
        blogpost.html()
    )
}
