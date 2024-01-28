use std::rc::Rc;

use anyhow::{anyhow, bail, Result};
use env::{bind_env, find_env, get_env, new_env, set_env};
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
#[macro_use]
mod types;

fn qq_iter(elts: &Vec<MalVal>) -> MalVal {
    let mut acc = list![];
    for elt in elts.iter().rev() {
        if let List(v) = elt {
            if v.len() == 2 {
                if let Sym(ref s) = v[0] {
                    if s == "splice-unquote" {
                        acc = list![Sym("concat".to_string()), v[1].clone(), acc];
                        continue;
                    }
                }
            }
        }
        acc = list![Sym("cons".to_string()), quasiquote(&elt), acc];
    }
    return acc;
}

fn quasiquote(ast: &MalVal) -> MalVal {
    match ast {
        List(v) => {
            if v.len() == 2 {
                if let Sym(ref s) = v[0] {
                    if s == "unquote" {
                        return v[1].clone();
                    }
                }
            }
            qq_iter(&v)
        }
        _ => ast.clone(),
    }
}

fn is_macro_call(ast: &MalVal, env: &Env) -> Option<(MalVal, Vec<MalVal>)> {
    match ast {
        List(v) => match v[0] {
            Sym(ref s) => match find_env(env, s) {
                Some(e) => match get_env(&e, &v[0]) {
                    Ok(f @ MalFunc { is_macro: true, .. }) => Some((f, v[1..].to_vec())),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn macroexpand(mut ast: MalVal, env: &Env) -> Result<(bool, MalVal)> {
    let mut was_expanded = false;
    while let Some((mf, args)) = is_macro_call(&ast, env) {
        ast = {
            match &mf {
                MalFunc {
                    body,
                    params,
                    env: ienv,
                    ..
                } => {
                    let a = &**body;
                    let p = &**params;
                    let fn_env = bind_env(&ienv, p, &args)?;
                    Ok(eval(a.clone(), fn_env)?)
                }
                _ => Err(anyhow!("unreachable: macroexpand")),
            }
        }?;
        was_expanded = true;
    }
    Ok((was_expanded, ast))
}

fn eval_ast(ast: &MalVal, env: &Env) -> MalRet {
    match ast {
        List(list) => {
            let mut res = vec![];
            for val in list.iter() {
                res.push(eval(val.clone(), env.clone())?);
            }
            Ok(list!(res))
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

                match macroexpand(ast.clone(), &env) {
                    Ok((true, new_ast)) => {
                        ast = new_ast;
                        continue 'tco;
                    }
                    _ => (),
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

                        if let List(arglist) = arglist {
                            for binds_ast in arglist.iter() {
                                match binds_ast {
                                    List(binds) => {
                                        let bind = &binds[0];
                                        let expr = &binds[1];
                                        match bind {
                                            Sym(_) => {
                                                let _ = set_env(
                                                    &env,
                                                    bind.clone(),
                                                    eval(expr.clone(), env.clone())?,
                                                );
                                            }
                                            _ => bail!("non-sym arg in let*"),
                                        }
                                    }
                                    _ => bail!("invalid arglist in let*"),
                                }
                            }
                        } else {
                            bail!("invalid arglist in let*");
                        }

                        ast = body;
                        continue 'tco;
                    }
                    Sym(sym) if sym == "quote" => Ok(list[1].clone()),
                    Sym(sym) if sym == "quasiquoteexpand" => Ok(quasiquote(&list[1])),
                    Sym(sym) if sym == "quasiquote" => {
                        ast = quasiquote(&list[1]);
                        continue 'tco;
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
                            is_macro: false,
                            env: env.clone(),
                        })
                    }
                    Sym(sym) if sym == "defmacro!" => {
                        let a1 = list[1].clone();
                        let a2 = list[2].clone();
                        let r = eval(a2, env.clone())?;
                        match r {
                            MalFunc {
                                body,
                                params,
                                env: ienv,
                                ..
                            } => Ok(set_env(
                                &ienv,
                                a1.clone(),
                                MalFunc {
                                    body: body.clone(),
                                    params: params.clone(),
                                    is_macro: true,
                                    env: ienv.clone(),
                                },
                            )?),
                            _ => Err(anyhow!("set macro on non-func")),
                        }
                    }
                    Sym(sym) if sym == "macroexpand" => match macroexpand(list[1].clone(), &env) {
                        Ok((_, new_ast)) => Ok(new_ast),
                        Err(e) => Err(e),
                    },
                    _ => match eval_ast(&ast, &env)? {
                        List(list2) => {
                            let func = &list2[0];
                            let args = list2[1..].to_vec();

                            match func {
                                RustFunc(f) => f(args),
                                MalFunc {
                                    body,
                                    params,
                                    env: ienv,
                                    ..
                                } => {
                                    env = bind_env(&ienv, &**params, &args)?;
                                    ast = (&**body).clone();
                                    continue 'tco;
                                }
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
    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

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
                rl.add_history_entry(&line)?;
                rl.save_history(".mal-history").unwrap();
                match read_str(&line) {
                    Ok(ast) => match eval(ast, global_env.clone()) {
                        Ok(evaluated) => println!("{}", print(&evaluated)),
                        Err(err) => println!("Error: {:?}", err),
                    },
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
