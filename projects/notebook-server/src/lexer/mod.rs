// https://juejin.cn/post/6844903874529083400
use std::fmt::{self, Display, Formatter};

struct Token {
    kind: Option<String>,
    content: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            Some(s) => {
                write!(f, r#"<span class="{}">{}</span>"#, s, self.content)
            }
            None => {
                write!(f, "{}", self.content)
            }
        }
    }
}

struct Oxide;

impl Oxide {
    pub fn highlight(&self, text: &str, lang: &str) {
        self.escape().tokenize().to_string()
    }

    pub fn tokenize(&self, text: &str, lang: &str) {
        let s = vec![text];
        self.match_grammar()
    }
    pub fn match_grammar(&self, text: String, vec: String, grammar: String) {}
}

pub fn escape_xml(s: &str) {
    let mut out = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&amp"),
            _ => out.push(c),
        }
    }
}
