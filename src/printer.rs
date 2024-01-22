use crate::types::MalTypes;

pub fn print(mal: &MalTypes) -> String {
    match mal {
        MalTypes::Nil => "NIL".to_owned(),
        MalTypes::Bool(b) => {
            if *b {
                "TRUE".to_owned()
            } else {
                "FALSE".to_owned()
            }
        }
        MalTypes::Num(n) => n.to_string(),
        MalTypes::Sym(s) => s.clone(),
        MalTypes::List(l) => {
            format!(
                "({})",
                l.iter()
                    .map(|x| print(x))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        MalTypes::RustFunc(_) => "<builtin func>".to_owned(),
    }
}
