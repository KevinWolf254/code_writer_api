use std::{collections::HashMap, io::{Result, Write}, fs::{File, self}, sync::Mutex};

use actix_cors::Cors;
use actix_web::{web, Responder, HttpResponse, get, HttpServer, App, http::header::{self, ContentType}, delete, post, put};
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

    fn load_from_file() -> Result<Self> {
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

struct AppState {
    db: Mutex<Database>
}

async fn create_task(state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut database = state.db.lock().unwrap();
    database.add_task(task.into_inner());
    database.save_to_file().unwrap();
    HttpResponse::Ok()
}

#[get("/tasks/")]
async fn get_tasks(state: web::Data<AppState>) -> impl Responder {
    let database = state.db.lock().unwrap();
    let tasks = database.get_tasks();
    let body = serde_json::to_string(&tasks).unwrap();
    HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
}

#[get("/tasks/{id}")]
async fn get_task(state: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
    let database = state.db.lock().unwrap();
    let task = database.get_task(&id).unwrap();
    let body = serde_json::to_string(task).unwrap();
    HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
}

async fn update_task(state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut database = state.db.lock().unwrap();
    database.update_task(task.into_inner());
    database.save_to_file().unwrap();
    HttpResponse::Ok()
}

#[delete("/tasks/{id}")]
async fn delete_task(state: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
    let mut database = state.db.lock().unwrap();
    database.delete_task(&id);
    HttpResponse::Ok()
}


#[post("/users")]
async fn create_user(state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut database = state.db.lock().unwrap();
    database.add_user(user.into_inner());
    database.save_to_file().unwrap();
    HttpResponse::Ok()
}

#[get("/users/")]
async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let database = state.db.lock().unwrap();
    let users = database.get_users();
    let body = serde_json::to_string(&users).unwrap();
    HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
}

#[get("/users/{id}")]
async fn get_user(state: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
    let database = state.db.lock().unwrap();
    let user = database.get_user(&id).unwrap();
    let body = serde_json::to_string(user).unwrap();
    HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
}

#[put("/users/{id}")]
async fn update_user(state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut database = state.db.lock().unwrap();
    database.update_user(user.into_inner());
    database.save_to_file().unwrap();
    HttpResponse::Ok()
}

#[delete("/users/{id}")]
async fn delete_user(state: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
    let mut database = state.db.lock().unwrap();
    database.delete_user(&id);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let load_from_file = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new()
    };

    let data = web::Data::new(AppState {
        db: Mutex::new(load_from_file)
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_header| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .service(get_tasks)
            .service(get_task)
            .route("/tasks/", web::post().to(create_task))
            .route("/tasks/", web::put().to(update_task))
            .service(delete_task)
            .service(get_users)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
