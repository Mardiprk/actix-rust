use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Clone)]
pub struct UserStore {
    pub users: Arc<Mutex<HashMap<u32, User>>>,
}

pub async fn get_users(store: web::Data<UserStore>) -> impl Responder {
    let users = store.users.lock().unwrap();
    let all_users: Vec<_> = users.values().cloned().collect();
    HttpResponse::Ok().json(all_users)
}

pub async fn get_user(path: web::Path<u32>, store: web::Data<UserStore>) -> impl Responder {
    let id = path.into_inner();
    let users = store.users.lock().unwrap();
    if let Some(user) = users.get(&id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

pub async fn create_user(
    user: web::Json<User>,
    store: web::Data<UserStore>,
) -> impl Responder {
    let mut users = store.users.lock().unwrap();
    users.insert(user.id, user.into_inner());
    HttpResponse::Created().body("User created")
}

pub async fn update_user(
    path: web::Path<u32>,
    user: web::Json<User>,
    store: web::Data<UserStore>,
) -> impl Responder {
    let id = path.into_inner();
    let mut users = store.users.lock().unwrap();
    if users.contains_key(&id) {
        users.insert(id, user.into_inner());
        HttpResponse::Ok().body("User updated")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

pub async fn delete_user(path: web::Path<u32>, store: web::Data<UserStore>) -> impl Responder {
    let id = path.into_inner();
    let mut users = store.users.lock().unwrap();
    if users.remove(&id).is_some() {
        HttpResponse::Ok().body("User deleted")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}
