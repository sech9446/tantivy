use tantivy::tokenizer::{Token, TokenFilter, TokenStream, BoxTokenStream};
use std::mem;
use regex::Regex;

impl TokenFilter for RegexFilter {
    fn transform<'a>(&self, token_stream: BoxTokenStream<'a>) -> BoxTokenStream<'a> {
        BoxTokenStream::from(RegexFilterTokenStream {
            tail: token_stream,
            re: self.re.clone(),
            replacer: self.replacer.clone(),
        })
    }
}

#[derive(Clone)]
pub struct RegexFilter {
    re: Regex,
    replacer: String,
}
impl RegexFilter {
    pub fn new(re: Regex, replacer: String) -> Self {
        RegexFilter{ re, replacer }
    }
}

pub struct RegexFilterTokenStream<'a> {
    tail: BoxTokenStream<'a>,
    re: Regex,
    replacer: String,
}

impl<'a> TokenStream for RegexFilterTokenStream<'a> {
    fn advance(&mut self) -> bool {
        if !self.tail.advance() {
            return false;
        }
        let processed = self.re.replace_all(&self.tail.token().text, self.replacer.as_str()).into_owned();
        self.tail.token_mut().text.clear();
        self.tail.token_mut().text.push_str(&processed);
        true
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tantivy::tokenizer::{SimpleTokenizer, TextAnalyzer};

    #[test]
    fn test_regex_filter() {
        assert_eq!(
            helper("2020-01-01 2021-02-02", r"(?P<y>\d4)-(?P<m>\d2)-(?P<d>\d2)", "$m/$d/$y"),
            vec!["01/01/2020".to_string(), "02/02/2021".to_string()]
        );
    }

    fn helper(text: &str, regex: &str, replacer: &str) -> Vec<String> {
        let mut tokens = vec![];
        let mut token_stream = TextAnalyzer::from(SimpleTokenizer)
            .filter(RegexFilter::new(Regex::new(regex).unwrap(), replacer.to_string()))
            .token_stream(text);
        while token_stream.advance() {
            let token_text = token_stream.token().text.clone();
            tokens.push(token_text);
        }
        tokens
    }
}
