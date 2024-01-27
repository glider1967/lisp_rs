use std::rc::Rc;

use anyhow::{anyhow, bail, Result};
use env::{get_env, new_env, set_env};
use rustyline::{error::ReadlineError, DefaultEditor};
use types::{
    MalRet,
    MalVal::{self, Bool, List, MalFunc, Nil, RustFunc, Sym},
};

use crate::env::Env;
use crate::printer::print;
use crate::reader::read_str;
mod core;
mod env;
mod printer;
mod reader;
mod types;

fn eval_ast(ast: &MalVal, env: &Env) -> MalRet {
    match ast {
        List(list) => {
            let mut res = vec![];
            for val in list.iter() {
                res.push(eval(val.clone(), env.clone())?);
            }
            Ok(List(Rc::new(res)))
        }
        Sym(_) => Ok(get_env(&env, ast)?),
        _ => Ok(ast.clone()),
    }
}

// evaluate `ast`
// TCOのためにmutで受け取る
fn eval(mut ast: MalVal, mut env: Env) -> MalRet {
    let ret: MalRet;

    'tco: loop {
        ret = match ast.clone() {
            List(list) => {
                if list.is_empty() {
                    return Ok(ast);
                }

                let arg0 = &list[0];
                match arg0 {
                    Sym(sym) if sym == "def!" => {
                        set_env(&env, list[1].clone(), eval(list[2].clone(), env.clone())?)
                    }
                    Sym(sym) if sym == "let*" => {
                        env = new_env(Some(env.clone()));
                        let arglist = list[1].clone();
                        let body = list[2].clone();

                        match arglist {
                            List(binds) => {
                                let bind = &binds[0];
                                let expr = &binds[1];
                                let _ = set_env(
                                    &env,
                                    bind.clone(),
                                    eval(expr.clone(), env.clone())?,
                                );
                            }
                            _ => bail!("invalid arglist in let*"),
                        }

                        ast = body;
                        continue 'tco;
                    }
                    Sym(sym) if sym == "quote" => {
                        Ok(list[1].clone())
                    }
                    Sym(sym) if sym == "do" => {
                        let evals = eval_ast(
                            &List(Rc::new(list[1..list.len() - 1].to_vec())),
                            &env.clone(),
                        )?;
                        match evals {
                            List(_) => {
                                ast = list.last().unwrap_or(&Nil).clone();
                                continue 'tco;
                            }
                            _ => Err(anyhow!("invalid do form")),
                        }
                    }
                    Sym(sym) if sym == "if" => {
                        let cond = eval(list[1].clone(), env.clone())?;
                        match cond {
                            Bool(false) | Nil => {
                                if list.len() >= 4 {
                                    ast = list[3].clone();
                                    continue 'tco;
                                } else {
                                    Ok(Nil)
                                }
                            }
                            _ => {
                                if list.len() >= 3 {
                                    ast = list[2].clone();
                                    continue 'tco;
                                } else {
                                    Ok(Nil)
                                }
                            }
                        }
                    }
                    Sym(sym) if sym == "fn*" => {
                        let params = list[1].clone();
                        let body = list[2].clone();
                        Ok(MalFunc {
                            body: Rc::new(body),
                            params: Rc::new(params),
                            env: env.clone(),
                        })
                    }
                    _ => match eval_ast(&ast, &env)? {
                        List(list2) => {
                            let func = &list2[0];
                            let args = list2[1..].to_vec();
                            
                            match func {
                                RustFunc(f) => f(args),
                                MalFunc {body, params, env: ienv} => {
                                    env = {
                                        let new_env = new_env(Some(ienv.clone()));
                                        // &**params: &Rc<MalVal> -> &MalVal
                                        match &**params {
                                            List(binds) => {
                                                for (i, bind) in binds.iter().enumerate() {
                                                    set_env(&new_env, bind.clone(), args[i].clone())?;
                                                }
                                                Ok(new_env)
                                            }
                                            _ => Err(anyhow!("failed to bind")),
                                        }
                                    }?;
                                    ast = (&**body).clone();
                                    continue 'tco;
                                },
                                _ => Err(anyhow!("apttempt to call non-function")),
                            }
                        }
                        _ => Err(anyhow!("expected a list")),
                    },
                }
            }
            _ => eval_ast(&ast, &env),
        };

        break;
    }

    ret
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    let global_env = {
        let global_env = new_env(None);
        let core_funcs = core::ns();
        for (sym, func) in core_funcs {
            set_env(&global_env, Sym(sym.to_owned()), RustFunc(func))?;
        }
        global_env
    };

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                match read_str(&line) {
                    Ok(ast) => {
                        match eval(ast, global_env.clone()) {
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
