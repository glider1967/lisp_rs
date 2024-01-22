use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum MalTypes {
    Nil,
    Bool(bool),
    Num(i64),
    Sym(String),
    Func(fn(i64, i64) -> i64),
    List(Rc<Vec<MalTypes>>),
}
