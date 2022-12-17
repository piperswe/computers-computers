use anyhow::Result;
use pulldown_cmark::{html::write_html, Options, Parser};

pub fn markdown_to_html(markdown: &str) -> Result<String> {
    let options = Options::empty();
    let parser = Parser::new_ext(markdown, options);
    let mut html = Vec::<u8>::new();
    write_html(&mut html, parser)?;
    Ok(String::from_utf8(html)?)
}
