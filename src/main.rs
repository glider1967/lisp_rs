use std::rc::Rc;

use anyhow::{anyhow, bail, Result};
use env::{get_env, new_env, set_env};
use rustyline::{error::ReadlineError, DefaultEditor};
use types::{MalRet, MalTypes};

use crate::env::Env;
use crate::printer::print;
use crate::reader::read_str;
mod env;
mod printer;
mod reader;
mod types;

fn eval_ast(ast: &MalTypes, env: &Env) -> MalRet {
    match ast {
        MalTypes::List(list) => {
            let mut res = vec![];
            for val in list.iter() {
                res.push(eval(val, env)?);
            }
            Ok(MalTypes::List(Rc::new(res)))
        }
        MalTypes::Sym(_) => Ok(get_env(&env, ast)?),
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: &MalTypes, env: &Env) -> MalRet {
    match ast {
        MalTypes::List(list) => {
            if list.is_empty() {
                return Ok(ast.clone());
            }

            let arg0 = &list[0];
            match arg0 {
                MalTypes::Sym(sym) if sym == "def!" => {
                    set_env(&env, list[1].clone(), eval(&list[2], env)?)
                }
                MalTypes::Sym(sym) if sym == "let*" => {
                    let new_env = new_env(Some(env.clone()));
                    let arglist = list[1].clone();
                    let body = list[2].clone();

                    match arglist {
                        MalTypes::List(binds) => {
                            let bind = &binds[0];
                            let expr = &binds[1];
                            let _ = set_env(&new_env, bind.clone(), eval(expr, env)?);
                        }
                        _ => bail!("invalid arglist in let*"),
                    }

                    eval(&body, &new_env)
                }
                _ => match eval_ast(ast, env)? {
                    MalTypes::List(list2) => {
                        let func = &list2[0];
                        let args = list2[1..].to_vec();
                        apply(&func, &args)
                    }
                    _ => Err(anyhow!("expected a list")),
                },
            }
        }
        _ => eval_ast(ast, env),
    }
}

fn apply(func: &MalTypes, args: &Vec<MalTypes>) -> MalRet {
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

    let global_env = new_env(None);
    set_env(
        &global_env,
        MalTypes::Sym("+".to_owned()),
        MalTypes::Func(|x, y| x + y),
    )?;
    set_env(
        &global_env,
        MalTypes::Sym("-".to_owned()),
        MalTypes::Func(|x, y| x - y),
    )?;
    set_env(
        &global_env,
        MalTypes::Sym("*".to_owned()),
        MalTypes::Func(|x, y| x * y),
    )?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                println!("Line: {}", &line);
                let ast = read_str(&line)?;
                dbg!(print(&ast));
                dbg!(print(&eval(&ast, &global_env)?));
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
