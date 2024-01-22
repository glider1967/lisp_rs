use std::{cell::RefCell, rc::Rc};

use anyhow::{Context, anyhow};
use fnv::FnvHashMap;

use crate::types::{MalTypes, MalRet};

#[derive(Debug, Clone)]
pub struct Env {
    data: RefCell<FnvHashMap<String, MalTypes>>,
    outer: Option<Rc<Env>>,
}

impl Env {
    pub fn new(outer: Option<Rc<Env>>) -> Self {
        Self {
            data: RefCell::new(FnvHashMap::default()),
            outer,
        }
    }

    pub fn set(&mut self, key: String, val: MalTypes) {
        self.data.borrow_mut().insert(key, val);
    }

    fn find(&self, key: &String) -> Option<Self> {
        if self.data.borrow().contains_key(key) {
            Some(self.clone())
        } else {
            match self.outer.clone() {
                Some(env) => env.find(key),
                None => None,
            }
        }
    }

    pub fn get(&self, key: &MalTypes) -> MalRet {
        match key {
            MalTypes::Sym(s) => {
                let found_env = self.find(s);
                if found_env.is_some() {
                    Ok(found_env.unwrap().data.borrow().get(s)
                    .context(format!("`{}` not found", s))?.clone())
                } else {
                    Err(anyhow!(format!("`{}` not found", s)))
                }
            },
            _ => Err(anyhow!("invalid key type"))
        }
    }
}