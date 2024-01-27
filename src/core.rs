use anyhow::{anyhow, bail, Ok};

use crate::{
    printer::print,
    types::{
        MalRet,
        MalVal::{self, Bool, Nil, Num},
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
    ]
}
