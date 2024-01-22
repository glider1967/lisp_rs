use std::rc::Rc;

use anyhow::Result;

#[derive(Debug, Clone)]
pub enum MalTypes {
    Nil,
    Bool(bool),
    Num(i64),
    Sym(String),
    Func(fn(i64, i64) -> i64),
    List(Rc<Vec<MalTypes>>),
}

pub type MalRet = Result<MalTypes>;
