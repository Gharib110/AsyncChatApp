use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json::error::Category;
use crate::chats::Chats;

pub struct ChatTracker(Mutex<HashMap<Arc<String>, Arc<Chats>>>);

impl ChatTracker {
    pub fn new() -> ChatTracker {
        ChatTracker(Mutex::new(HashMap::new()))
    }

    pub fn find(&self, name: &String) -> Option<Arc<Chats>> {
        self.0.lock().unwrap().get(name).cloned()
    }

    pub fn find_or_new(&self, name: Arc<String>) -> Arc<Chats> {
        self.0.lock().unwrap().entry(name.clone()).or_insert_with(|| Arc::new(Chats::new(name))).clone()
    }

}