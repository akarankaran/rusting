use std::fmt;

#[derive(Debug)]
enum Pattern {
    Literal(char),
    Wildcard,
}

struct Matcher {
    pattern: Vec<Pattern>,
}

impl Matcher {
    fn new(pattern_str: &str) -> Self {
        let pattern = pattern_str.chars()
            .map(|c| if c == '_' { Pattern::Wildcard } else { Pattern::Literal(c) })
            .collect();
        Matcher { pattern }
    }

    fn matches(&self, text: &str) -> bool {
        let mut text_chars = text.chars().peekable();
        for p in &self.pattern {
            match p {
                Pattern::Literal(c) => {
                    if text_chars.next() != Some(*c) {
                        return false;
                    }
                }
                Pattern::Wildcard => {
                    if let Some(&next) = text_chars.peek() {
                        text_chars.next();
                        if next == '_' {
                            continue;
                        }
                    } else {
                        return true;
                    }
                }
            }
        }
        text_chars.count() == 0
    }
}

fn main() {
    let patterns = vec!["_a", "b_", "_c_", "d_ef", "h__llo"];
    let texts = vec!["a", "b", "abc", "dgef", "hello", "hallo"];

    for pattern_str in patterns {
        let matcher = Matcher::new(pattern_str);
        println!("Pattern: {}", pattern_str);
        for text in &texts {
            println!("Text: {}, Matches: {}", text, matcher.matches(text));
        }
    }
}