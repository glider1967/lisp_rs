use std::rc::Rc;

use anyhow::{anyhow, bail, Result};
use env::{get_env, new_env, set_env};
use rustyline::{error::ReadlineError, DefaultEditor};
use types::{
    MalRet,
    MalTypes::{self, List, Num, RustFunc, Sym},
};

use crate::env::Env;
use crate::printer::print;
use crate::reader::read_str;
mod env;
mod printer;
mod reader;
mod types;

fn eval_ast(ast: &MalTypes, env: &Env) -> MalRet {
    match ast {
        List(list) => {
            let mut res = vec![];
            for val in list.iter() {
                res.push(eval(val, env)?);
            }
            Ok(List(Rc::new(res)))
        }
        Sym(_) => Ok(get_env(&env, ast)?),
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: &MalTypes, env: &Env) -> MalRet {
    match ast {
        List(list) => {
            if list.is_empty() {
                return Ok(ast.clone());
            }

            let arg0 = &list[0];
            match arg0 {
                Sym(sym) if sym == "def!" => set_env(&env, list[1].clone(), eval(&list[2], env)?),
                Sym(sym) if sym == "let*" => {
                    let new_env = new_env(Some(env.clone()));
                    let arglist = list[1].clone();
                    let body = list[2].clone();

                    match arglist {
                        List(binds) => {
                            let bind = &binds[0];
                            let expr = &binds[1];
                            let _ = set_env(&new_env, bind.clone(), eval(expr, env)?);
                        }
                        _ => bail!("invalid arglist in let*"),
                    }

                    eval(&body, &new_env)
                }
                _ => match eval_ast(ast, env)? {
                    List(list2) => {
                        let func = &list2[0];
                        let args = list2[1..].to_vec();
                        apply(&func, args)
                    }
                    _ => Err(anyhow!("expected a list")),
                },
            }
        }
        _ => eval_ast(ast, env),
    }
}

fn apply(func: &MalTypes, args: Vec<MalTypes>) -> MalRet {
    if let RustFunc(f) = func {
        f(args)
    } else {
        Err(anyhow!("invalid function"))
    }
}

fn plus(args: Vec<MalTypes>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Num(*x + *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    let global_env = new_env(None);
    set_env(&global_env, Sym("+".to_owned()), RustFunc(plus))?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                println!("Line: {}", &line);
                match read_str(&line) {
                    Ok(ast) => {
                        dbg!(print(&ast));
                        match eval(&ast, &global_env) {
                            Ok(evaluated) => println!("{}", print(&evaluated)),
                            Err(err) => println!("Error: {:?}", err),
                        }
                    }
                    Err(err) => println!("Parse Error: {:?}", err),
                }
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
