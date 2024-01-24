use std::rc::Rc;

use anyhow::Result;

#[derive(Debug, Clone)]
pub enum MalVal {
    Nil,
    Bool(bool),
    Num(i64),
    Sym(String),
    RustFunc(fn(Vec<MalVal>) -> MalRet),
    List(Rc<Vec<MalVal>>),
}

pub type MalRet = Result<MalVal>;
