use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Context};
use fnv::FnvHashMap;

use crate::types::{MalRet, MalVal};

#[derive(Debug, Clone)]
pub struct EnvInternal {
    data: RefCell<FnvHashMap<String, MalVal>>,
    outer: Option<Env>,
}
pub type Env = Rc<EnvInternal>;
pub fn new_env(outer: Option<Env>) -> Env {
    Rc::new(EnvInternal {
        data: RefCell::new(FnvHashMap::default()),
        outer,
    })
}

pub fn set_env(env: &Env, key: MalVal, val: MalVal) -> MalRet {
    match key {
        MalVal::Sym(sym) => {
            env.data.borrow_mut().insert(sym, val.clone());
            Ok(val)
        }
        _ => Err(anyhow!("invalid key type")),
    }
}

fn find_env(env: &Env, key: &String) -> Option<Env> {
    if env.data.borrow().contains_key(key) {
        Some(env.clone())
    } else {
        match env.outer.clone() {
            Some(e) => find_env(&e, key),
            None => None,
        }
    }
}

pub fn get_env(env: &Env, key: &MalVal) -> MalRet {
    match key {
        MalVal::Sym(s) => {
            let found_env = find_env(&env, s);
            if found_env.is_some() {
                Ok(found_env
                    .unwrap()
                    .data
                    .borrow()
                    .get(s)
                    .context(format!("`{}` not found", s))?
                    .clone())
            } else {
                Err(anyhow!(format!("`{}` not found", s)))
            }
        }
        _ => Err(anyhow!("invalid key type")),
    }
}
