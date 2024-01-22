use std::rc::Rc;

use anyhow::{anyhow, bail, Result};
use rustyline::{error::ReadlineError, DefaultEditor};
use types::MalTypes;

use crate::printer::print;
use crate::reader::read_str;
mod printer;
mod reader;
mod types;

fn eval_ast(ast: &MalTypes) -> Result<MalTypes> {
    match ast {
        MalTypes::List(list) => {
            let mut res = vec![];
            for val in list.iter() {
                res.push(eval(val)?);
            }
            Ok(MalTypes::List(Rc::new(res)))
        }
        MalTypes::Sym(s) => match &s[..] {
            "+" => Ok(MalTypes::Func(|x, y| x + y)),
            "-" => Ok(MalTypes::Func(|x, y| x - y)),
            "*" => Ok(MalTypes::Func(|x, y| x * y)),
            _ => Ok(ast.clone()),
        },
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: &MalTypes) -> Result<MalTypes> {
    match ast {
        MalTypes::List(list) => {
            if list.is_empty() {
                Ok(ast.clone())
            } else {
                match eval_ast(ast)? {
                    MalTypes::List(list2) => {
                        let func = &list2[0];
                        let args = list2[1..].to_vec();
                        apply(&func, &args)
                    }
                    _ => Err(anyhow!("expected a list")),
                }
            }
        }
        _ => eval_ast(ast),
    }
}

fn apply(func: &MalTypes, args: &Vec<MalTypes>) -> Result<MalTypes> {
    if let MalTypes::Func(f) = func {
        if args.len() != 2 {
            bail!("invalid length of arguments");
        }
        match (&args[0], &args[1]) {
            (MalTypes::Num(x), MalTypes::Num(y)) => Ok(MalTypes::Num(f(*x, *y))),
            _ => Err(anyhow!("invalid number binary args")),
        }
    } else {
        Err(anyhow!("invalid function"))
    }
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                println!("Line: {}", &line);
                let ast = read_str(&line)?;
                dbg!(print(&ast));
                dbg!(print(&eval(&ast)?));
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
