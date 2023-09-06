use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    name: String,
    completed: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u32, Task>,
    users: HashMap<u32, User>
}

fn main() {
    println!("Hello, world!");
}
