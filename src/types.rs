use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum MalTypes {
    Atom(String),
    List(Rc<Vec<MalTypes>>),
}
