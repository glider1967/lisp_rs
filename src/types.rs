use std::rc::Rc;

use anyhow::Result;

use crate::env::Env;

#[derive(Debug, Clone)]
pub enum MalVal {
    Nil,
    Bool(bool),
    Num(i64),
    Sym(String),
    RustFunc(fn(Vec<MalVal>) -> MalRet),
    MalFunc {
        body: Rc<MalVal>,
        params: Rc<MalVal>,
        env: Env,
    },
    List(Rc<Vec<MalVal>>),
}

pub type MalRet = Result<MalVal>;
