use std::rc::Rc;

use anyhow::Result;

#[derive(Debug, Clone)]
pub enum MalTypes {
    Nil,
    Bool(bool),
    Num(i64),
    Sym(String),
    RustFunc(fn(Vec<MalTypes>) -> MalRet),
    List(Rc<Vec<MalTypes>>),
}

pub type MalRet = Result<MalTypes>;
