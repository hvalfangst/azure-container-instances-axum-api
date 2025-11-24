use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::users::model::{User, UpsertUser};

pub async fn create_user(request: UpsertUser, shared_hashmap: &Arc<Mutex<HashMap<String, User>>>) -> Option<User> {
    let mut acquired_map = shared_hashmap.lock().unwrap_or_else(|_| {
        println!("Error unwrapping Option!");
        panic!("Mutex lock failed");
    });

    let email_clone = request.email.clone();
    if let Some(_user) = acquired_map.get(&email_clone) {
        None
    } else {
        let new_user = User {
            id: (acquired_map.len() as i32) + 1,
            email: email_clone.clone(),
            password: request.password,
            fullname: request.fullname,
            role: request.role,
        };
        acquired_map.insert(email_clone.clone(), new_user.clone());
        Some(new_user.clone())
    }
}

pub async fn get_user_by_email(email: &String, shared_hashmap: &Arc<Mutex<HashMap<String, User>>>) -> Option<User> {
    let acquired_map = shared_hashmap.lock().unwrap_or_else(|_| {
        println!("Error unwrapping Option!");
        panic!("Mutex lock failed");
    });
    acquired_map.get(email).cloned()
}

pub async fn update_user_by_email(email: &String, request: UpsertUser, shared_hashmap: &Arc<Mutex<HashMap<String, User>>>) -> Option<User> {
    let mut acquired_map = shared_hashmap.lock().unwrap_or_else(|_| {
        println!("Error unwrapping Option!");
        panic!("Mutex lock failed");
    });

    match acquired_map.get(email) {
        Some(user) => {
            let updated_user = User {
                id: user.id.clone(),
                email: user.email.clone(),
                password: request.password,
                fullname: request.fullname,
                role: request.role,
            };
            acquired_map.insert(email.clone(), updated_user)
        }
        None => None,
    }
}

pub async fn delete_user_by_email(email: &String, shared_hashmap: &Arc<Mutex<HashMap<String, User>>>) -> Option<User> {
    let mut acquired_map = shared_hashmap.lock().unwrap_or_else(|_| {
        println!("Error unwrapping Option!");
        panic!("Mutex lock failed");
    });

    match acquired_map.get(email) {
        Some(_user) => {
            acquired_map.remove(email)
        }
        None => None
    }
}

