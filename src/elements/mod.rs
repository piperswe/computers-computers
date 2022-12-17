use std::{borrow::Cow, cell::RefCell, rc::Rc};

use anyhow::Result;
use lol_html::{ElementContentHandlers, HtmlRewriter, Selector, Settings};

use self::code::CodeElement;

mod code;

pub type ElementContentHandler<'h, 's> = (Cow<'s, Selector>, ElementContentHandlers<'h>);

#[derive(Clone, Default)]
struct State {
    code_element: Rc<RefCell<CodeElement>>,
}

fn rewriter_settings<'h, 's>(state: State) -> Settings<'h, 's> {
    Settings {
        element_content_handlers: code::element_content_handlers(state.code_element),
        ..Default::default()
    }
}

pub fn transform_html(html: &str) -> Result<String> {
    let state = State::default();
    let settings = rewriter_settings(state);
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(settings, |c: &[u8]| output.extend_from_slice(c));
    rewriter.write(html.as_bytes())?;
    rewriter.end()?;
    Ok(String::from_utf8(output)?)
}
