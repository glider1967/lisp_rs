use std::rc::Rc;

use anyhow::{bail, Context, Ok, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::types::MalTypes;

#[derive(Debug, Clone)]
struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn next(&mut self) -> Result<String> {
        self.pos += 1;
        Ok(self
            .tokens
            .get(self.pos - 1)
            .context("underflow")?
            .to_string())
    }

    fn peek(&self) -> Result<String> {
        Ok(self.tokens.get(self.pos).context("underflow")?.to_string())
    }
}

fn tokenize(string: &str) -> Vec<String> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###)
            .unwrap()
    });

    let mut res = vec![];
    for cap in RE.captures_iter(string) {
        if cap[1].starts_with(";") {
            continue;
        }
        res.push(String::from(&cap[1]));
    }
    res
}

pub fn read_str(string: &str) -> Result<MalTypes> {
    let tokens = tokenize(string);
    if tokens.len() == 0 {
        bail!("no input");
    }

    read_form(&mut Reader { tokens, pos: 0 })
}

fn read_form(reader: &mut Reader) -> Result<MalTypes> {
    let token = reader.peek()?;
    match &token[..] {
        "(" => read_list(reader),
        _ => read_atom(reader),
    }
}

fn read_list(reader: &mut Reader) -> Result<MalTypes> {
    let mut list = Vec::<MalTypes>::new();
    reader.next()?;
    loop {
        let token = reader.peek().context("expected `)`, got EOF")?;
        if token == ")" {
            break;
        }
        list.push(read_form(reader)?);
    }
    let _ = reader.next();
    Ok(MalTypes::List(Rc::new(list)))
}

fn read_atom(reader: &mut Reader) -> Result<MalTypes> {
    static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^-?[0-9]+$").unwrap());
    let token = reader.next()?;
    match &token[..] {
        "nil" => Ok(MalTypes::Nil),
        "true" => Ok(MalTypes::Bool(true)),
        "false" => Ok(MalTypes::Bool(false)),
        _ => {
            if NUM_RE.is_match(&token) {
                Ok(MalTypes::Num(token.parse().unwrap()))
            } else {
                Ok(MalTypes::Sym(token))
            }
        },
    }
}
