use anyhow::{anyhow, bail};

use crate::types::{
    MalRet,
    MalVal::{self, Bool, Num},
};

// binaries
fn plus(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Num(*x + *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}
fn minus(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Num(*x - *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}
fn mult(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Num(*x * *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}
fn div(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Num(*x / *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}

fn eq(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Bool(*x == *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}
fn lt(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Bool(*x < *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}
fn le(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Bool(*x <= *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}
fn gt(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Bool(*x > *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}
fn ge(args: Vec<MalVal>) -> MalRet {
    if args.len() != 2 {
        bail!("invalid length of arguments");
    }
    match (&args[0], &args[1]) {
        (Num(x), Num(y)) => Ok(Bool(*x >= *y)),
        _ => Err(anyhow!("invalid number binary args")),
    }
}

//==================================================================

//==================================================================

pub fn ns() -> Vec<(&'static str, fn(Vec<MalVal>) -> MalRet)> {
    vec![
        ("+", plus),
        ("-", minus),
        ("*", mult),
        ("/", div),
        ("=", eq),
        ("<", lt),
        ("<=", le),
        (">", gt),
        (">=", ge),
    ]
}
