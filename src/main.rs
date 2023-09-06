use std::{collections::HashMap, io::{Result, Write}, fs::{File, self}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u32, Task>,
    users: HashMap<u32, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn save_to_file(&self) -> Result<()> {
        let data = serde_json::to_string(self)?;
        let mut file = File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file(&self) -> Result<Self> {
        let content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&content)?;
        Ok(db)
    }
}

trait TaskTrait {
    fn add_task(&mut self, task: Task);
    fn get_task(&self, id: &u32) -> Option<&Task>;
    fn get_tasks(&self) -> Vec<&Task>;
    fn update_task(&mut self, task: Task);
    fn delete_task(&mut self, id: &u32);
}

impl TaskTrait for Database {
    fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get_task(&self, id: &u32) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn get_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn update_task(&mut self, task: Task) {
        self.add_task(task);
    }

    fn delete_task(&mut self, id: &u32) {
        self.tasks.remove(id);
    }
}

trait UserTrait {
    fn add_user(&mut self, user: User);
    fn get_user(&self, id: &u32) -> Option<&User>;
    fn get_users(&self) -> Vec<&User>;
    fn update_user(&mut self, user: User);
    fn delete_user(&mut self, id: &u32);
}

impl UserTrait for Database {
    fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user(&self, id: &u32) -> Option<&User> {
        self.users.get(id)
    }

    fn get_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    fn update_user(&mut self, user: User) {
        self.add_user(user);
    }

    fn delete_user(&mut self, id: &u32) {
        self.users.remove(id);
    }
}

fn main() {
    println!("Hello, world!");
}
