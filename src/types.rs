use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum MalTypes {
    Nil,
    Bool(bool),
    Num(i64),
    Sym(String),
    List(Rc<Vec<MalTypes>>),
}
