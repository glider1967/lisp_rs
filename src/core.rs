use std::rc::Rc;

use anyhow::{anyhow, bail, Ok};

use crate::{
    printer::print,
    types::{
        MalRet,
        MalVal::{self, Bool, Nil, Num, List},
    },
};

macro_rules! num_binary {
    ($ret:ident, $fn:expr) => {
        |a: Vec<MalVal>| {
            if a.len() != 2 {
                bail!("expecting (num, num) args");
            }
            match (a[0].clone(), a[1].clone()) {
                (Num(a0), Num(a1)) => Ok($ret($fn(a0, a1))),
                _ => Err(anyhow!("expecting (num, num) args")),
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
        List(v)=> {
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
            List(v)=> new_v.extend_from_slice(v),
            _ => bail!("non-seq passed to concat"),
        }
    }
    Ok(List(Rc::new(new_v.to_vec())))
}

//==================================================================

//==================================================================

pub fn ns() -> Vec<(&'static str, fn(Vec<MalVal>) -> MalRet)> {
    vec![
        ("+", num_binary!(Num, |x, y| x + y)),
        ("-", num_binary!(Num, |x, y| x - y)),
        ("*", num_binary!(Num, |x, y| x * y)),
        ("/", num_binary!(Num, |x, y| x / y)),
        ("=", num_binary!(Bool, |x, y| x == y)),
        ("<", num_binary!(Bool, |x, y| x < y)),
        ("<=", num_binary!(Bool, |x, y| x <= y)),
        (">", num_binary!(Bool, |x, y| x > y)),
        (">=", num_binary!(Bool, |x, y| x >= y)),
        ("prn", prn),
        ("cons", cons),
        ("concat", concat),
    ]
}
