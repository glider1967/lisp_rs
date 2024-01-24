use crate::types::MalVal;

pub fn print(mal: &MalVal) -> String {
    match mal {
        MalVal::Nil => "NIL".to_owned(),
        MalVal::Bool(b) => {
            if *b {
                "TRUE".to_owned()
            } else {
                "FALSE".to_owned()
            }
        }
        MalVal::Num(n) => n.to_string(),
        MalVal::Sym(s) => s.clone(),
        MalVal::List(l) => {
            format!(
                "({})",
                l.iter()
                    .map(|x| print(x))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        MalVal::RustFunc(_) => "<builtin func>".to_owned(),
    }
}
