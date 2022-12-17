use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use minify_html::minify;
use once_cell::sync::OnceCell;

use crate::{elements::transform_html, markdown::markdown_to_html};

mod template;

#[derive(Debug)]
struct RawBlogPost<'a> {
    markdown: &'a str,
    slug: &'a str,
    title: &'a str,
    date: &'a str,
    author: &'a str,
    tags: &'a [&'a str],
    description: &'a str,
}

static RAW_BLOG_POSTS: &[RawBlogPost<'static>] = &[
    RawBlogPost {
        markdown: include_str!("../../blog-posts/multicraft-stretch.md"),
        slug: "multicraft-stretch",
        title: "Installing Multicraft on Debian 9 (\"Stretch\")",
        date: "2018-07-26",
        author: "Piper McCorkle",
        tags: &["linux", "minecraft", "debian"],
        description:
            "A up-to-date guide for installing and running Multicraft on Debian 9, a.k.a. Stretch",
    },
    RawBlogPost {
        markdown: include_str!("../../blog-posts/radical-left/index.md"),
        slug: "radical-left",
        title: "What Is This \"Radical Left\" I Keep Hearing About?",
        date: "2022-09-23",
        author: "Piper McCorkle",
        tags: &["essay"],
        description: "Many media organizations like to throw around the term \"radical left,\" apparently characterizing some political movement or stance. These organizations make it seem as if there is some \"radical leftist\" cabal, supported by the Democratic Party, plotting the downfall of American values and virtues. Is this the case, or do those on the left believe something much different?\n\nThis is an exploratory essay I wrote for my _English Composition I_ class ([ENGL 1301](http://catalog.blinn.edu/preview_course.php?catoid=15&coid=18725)) at Blinn College."
    }
];

#[derive(Debug)]
pub enum DateMaybeTime {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
}

impl DateMaybeTime {
    fn parse(s: &str) -> Result<Self> {
        if s.len() == "0000-00-00".len() {
            Ok(Self::Date(s.parse()?))
        } else {
            Ok(Self::DateTime(s.parse()?))
        }
    }
}

#[derive(Debug)]
pub struct ParsedBlogPost<'a> {
    raw: &'a RawBlogPost<'a>,
    date: DateMaybeTime,
    html: String,
}

impl<'a> ParsedBlogPost<'a> {
    fn parse(raw: &'a RawBlogPost<'a>) -> Result<Self> {
        let original_html = markdown_to_html(raw.markdown)?;
        let transformed_html = transform_html(&original_html)?;
        let minified_html = String::from_utf8(minify(
            transformed_html.as_bytes(),
            &minify_html::Cfg {
                do_not_minify_doctype: false,
                ensure_spec_compliant_unquoted_attribute_values: true,
                keep_closing_tags: true,
                keep_html_and_head_opening_tags: true,
                keep_spaces_between_attributes: false,
                keep_comments: true,
                minify_css: true,
                minify_js: true,
                remove_bangs: false,
                remove_processing_instructions: false,
            },
        ))?;
        let parsed_date = DateMaybeTime::parse(raw.date)?;
        Ok(Self {
            raw,
            date: parsed_date,
            html: minified_html,
        })
    }

    pub fn slug(&self) -> &'a str {
        self.raw.slug
    }

    pub fn title(&self) -> &'a str {
        self.raw.title
    }

    pub fn date(&self) -> &DateMaybeTime {
        &self.date
    }

    pub fn author(&self) -> &'a str {
        self.raw.author
    }

    pub fn tags(&self) -> &'a [&'a str] {
        self.raw.tags
    }

    pub fn description(&self) -> &'a str {
        self.raw.description
    }

    pub fn html(&self) -> &str {
        &self.html
    }

    pub fn render(&self) -> String {
        template::render_blogpost(self)
    }
}

static BLOG_POSTS: OnceCell<Vec<ParsedBlogPost<'static>>> = OnceCell::new();

pub fn blog_posts() -> &'static [ParsedBlogPost<'static>] {
    BLOG_POSTS.get_or_init(|| {
        RAW_BLOG_POSTS
            .iter()
            .map(|raw| ParsedBlogPost::parse(raw).unwrap())
            .collect()
    })
}
