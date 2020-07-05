use std::env;
use std::fs;
use std::path::Path;

use super::utils;

use serde::{Deserialize, Serialize};

pub const TODOS_FILE: &str = "todos.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub label: Option<String>,
    pub parent: Option<String>,
}

impl Todo {
    fn read_todos() {
        let path = Path::new(utils::APP_DATA_DIRECTORY_ROOT).join(TODOS_FILE);
    }
}
