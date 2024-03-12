use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::domain::question::Question;

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<String, Question>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            questions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
