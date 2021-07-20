use crate::{
    error,
    errors::{Error, Result},
};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigValue {
    Ident(String),
    Array(Vec<ConfigValue>),
    Pair(String, Box<ConfigValue>),
    None,
}

struct ConfigParser {
    current: usize,
    line: usize,
    input: String,
    output: Vec<ConfigValue>,
}
impl ConfigParser {
    pub fn new(input: impl ToString) -> Self {
        Self {
            current: 0,
            line: 1,
            input: input.to_string(),
            output: vec![],
        }
    }
    fn advance(&mut self) -> char {
        let c = self.peek().unwrap();
        self.current += 1;
        c
    }
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.current)
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    fn parse_ident(&mut self) -> Result<String> {
        let terminating = &['\n', '\r', ' ', '\t', ')'];
        let mut out = String::new();
        while !self.is_at_end() && !terminating.contains(&self.peek().unwrap()) {
            out.push(self.advance());
        }
        Ok(out)
    }
    fn parse_one(&mut self) -> Result<ConfigValue> {
        let current = self.advance();
        match current {
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            '(' => {
                let key = self.parse_ident()?;
                let mut body = vec![];
                while !self.is_at_end() && self.peek() != Some(')') {
                    let val = self.parse_one()?;
                    if val != ConfigValue::None {
                        body.push(val)
                    }
                }
                return if self.peek() != Some(')') {
                    error!("line {}: Expected `)`, found EOF.", self.line)
                } else {
                    self.advance();
                    Ok(ConfigValue::Pair(key, Box::new(ConfigValue::Array(body))))
                };
            }
            x => {
                let mut s = x.to_string();
                s.push_str(&self.parse_ident()?);
                return Ok(ConfigValue::Ident(s));
            }
        }
        Ok(ConfigValue::None)
    }
    pub fn parse(&mut self) -> Result<Vec<ConfigValue>> {
        while !self.is_at_end() {
            let val = self.parse_one()?;
            if val != ConfigValue::None {
                self.output.push(val);
            }
        }
        Ok(self.output.clone())
    }
}

fn parse_string(s: impl ToString) -> Result<Vec<ConfigValue>> {
    ConfigParser::new(s).parse()
}
pub fn parse_file(name: impl ToString) -> Result<Vec<ConfigValue>> {
    ConfigParser::new(
        fs::read_to_string(&name.to_string())
            .map_err(|e| Error(format!("Failed to read file: {}: {}.", name.to_string(), e)))?,
    )
    .parse()
}
pub fn find_val(values: &[ConfigValue], key: impl ToString) -> Option<ConfigValue> {
    let key = key.to_string();
    for val in values {
        if let ConfigValue::Pair(k, v) = val {
            if k.as_str() == key.as_str() {
                return Some(*v.clone());
            }
        }
    }
    None
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn no_paren() {
        parse_string("(jsp a b").unwrap();
    }

    #[test]
    fn parser() -> Result<()> {
        assert_eq!(
            parse_string("(jsp a b c)\n(non plus)")?,
            vec![
                ConfigValue::Pair(
                    "jsp".to_string(),
                    Box::new(ConfigValue::Array(vec![
                        ConfigValue::Ident("a".to_string()),
                        ConfigValue::Ident("b".to_string()),
                        ConfigValue::Ident("c".to_string())
                    ]))
                ),
                ConfigValue::Pair(
                    "non".to_string(),
                    Box::new(ConfigValue::Array(vec![ConfigValue::Ident(
                        "plus".to_string()
                    )]))
                )
            ]
        );
        Ok(())
    }
}
