use std::rc::Rc;

use anyhow::{anyhow, bail, Ok};

use crate::{
    printer::print,
    types::{
        MalRet,
        MalVal::{self, Bool, List, Nil, Num},
    },
};

macro_rules! binary {
    ($type: ident, $ret:ident, $fn:expr) => {
        |a: Vec<MalVal>| {
            if a.len() != 2 {
                bail!("expecting two args");
            }
            match (a[0].clone(), a[1].clone()) {
                ($type(a0), $type(a1)) => Ok($ret($fn(a0, a1))),
                _ => Err(anyhow!("invalid type of args")),
            }
        }
    };
}

fn prn(args: Vec<MalVal>) -> MalRet {
    for arg in args {
        println!("{}", print(&arg));
    }
    Ok(Nil)
}

fn cons(args: Vec<MalVal>) -> MalRet {
    let car = args[0].clone();
    let cdr = args[1].clone();
    match cdr {
        List(v) => {
            let mut new_v = vec![car];
            new_v.extend_from_slice(&v);
            Ok(List(Rc::new(new_v.to_vec())))
        }
        _ => Err(anyhow!("non-seq passed to concat")),
    }
}

fn concat(args: Vec<MalVal>) -> MalRet {
    let mut new_v = vec![];
    for seq in args.iter() {
        match seq {
            List(v) => new_v.extend_from_slice(v),
            _ => bail!("non-seq passed to concat"),
        }
    }
    Ok(List(Rc::new(new_v.to_vec())))
}

fn count(args: Vec<MalVal>) -> MalRet {
    let li = &args[0];
    match li {
        List(l) => {
            Ok(Num(l.len() as i64))
        }
        _ => Err(anyhow!("non-seq passed to count"))
    }
}

//==================================================================

//==================================================================

pub fn ns() -> Vec<(&'static str, fn(Vec<MalVal>) -> MalRet)> {
    vec![
        ("+", binary!(Num, Num, |x, y| x + y)),
        ("-", binary!(Num, Num, |x, y| x - y)),
        ("*", binary!(Num, Num, |x, y| x * y)),
        ("/", binary!(Num, Num, |x, y| x / y)),
        ("=", binary!(Num, Bool, |x, y| x == y)),
        ("<", binary!(Num, Bool, |x, y| x < y)),
        ("<=", binary!(Num, Bool, |x, y| x <= y)),
        (">", binary!(Num, Bool, |x, y| x > y)),
        (">=", binary!(Num, Bool, |x, y| x >= y)),
        ("and", binary!(Bool, Bool, |x, y| x && y)),
        ("or", binary!(Bool, Bool, |x, y| x || y)),
        ("prn", prn),
        ("cons", cons),
        ("concat", concat),
        ("count", count),
    ]
}
