use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use anyhow::Result;
use lol_html::{
    element,
    html_content::{ContentType, Element},
    text,
};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

use super::ElementContentHandler;

#[derive(Default)]
pub struct CodeElement {
    active: bool,
    code: String,
}

fn format_code(language: Option<String>, code: &str) -> Result<String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = language
        .and_then(|language| {
            ps.find_syntax_by_token(&language).or_else(|| {
                log::warn!("invalid syntax highlighting language: {}", language);
                ps.find_syntax_by_first_line(code.lines().next()?)
            })
        })
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let theme = &ts.themes["base16-ocean.dark"];
    let highlighted = highlighted_html_for_string(code, &ps, syntax, theme)?;
    Ok(highlighted)
}

fn el_language(el: &Element) -> Option<String> {
    let class = el.get_attribute("class")?;
    let mut classes = class.split_ascii_whitespace();
    let language_class = classes.find(|class| class.starts_with("language-"))?;
    let language = language_class.strip_prefix("language-")?;
    Some(language.to_string())
}

pub fn element_content_handlers<'h, 's>(
    state: Rc<RefCell<CodeElement>>,
) -> Vec<ElementContentHandler<'h, 's>> {
    let element_state = state.clone();
    let text_state = state.clone();
    vec![
        element!("pre code", move |el| {
            let language = el_language(el);
            {
                let mut state = element_state.borrow_mut();
                state.active = true;
                state.code.clear();
            }
            let end_state = element_state.clone();
            el.on_end_tag(move |end| {
                let state_refcell: &RefCell<CodeElement> = end_state.borrow();
                let state = state_refcell.borrow();
                let formatted = format_code(language, &state.code)?;
                end.before(&formatted, ContentType::Html);
                Ok(())
            })?;
            Ok(())
        }),
        text!("pre code", move |t| {
            let state_refcell: &RefCell<CodeElement> = text_state.borrow();
            if state_refcell.borrow().active {
                let mut state = state.borrow_mut();
                state.code += t.as_str();
                t.remove();
            }
            Ok(())
        }),
    ]
}
