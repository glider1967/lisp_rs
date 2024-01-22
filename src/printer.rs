use crate::types::MalTypes;

pub fn print(mal: &MalTypes) -> String {
    match mal {
        MalTypes::Atom(s) => s.clone(),
        MalTypes::List(l) => {
            format!(
                "({})",
                l.iter()
                    .map(|x| print(x))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
    }
}
