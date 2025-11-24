use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::users::model::{User, UpsertUser};

pub async fn create_user(request: UpsertUser, shared_hashmap: &Arc<Mutex<HashMap<String, User>>>) -> Option<User> {
    let mut acquired_map = shared_hashmap.lock().unwrap_or_else(|_| {
        println!("Error unwrapping Option!");
        panic!("Mutex lock failed");
    });

    if let Some(_user) = acquired_map.get(&request.email) {
        None
    } else {
        let new_user = User {
            id: (acquired_map.len() as i32) + 1,
            email: request.email.clone(),
            password: request.password,
            fullname: request.fullname,
            role: request.role,
        };
        acquired_map.insert(request.email, new_user.clone());
        Some(new_user)
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
                id: user.id,
                email: user.email.clone(),
                password: request.password,
                fullname: request.fullname,
                role: request.role,
            };
            acquired_map.insert(email.clone(), updated_user.clone());
            Some(updated_user)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_hashmap() -> Arc<Mutex<HashMap<String, User>>> {
        Arc::new(Mutex::new(HashMap::new()))
    }

    fn create_test_upsert_user(email: &str) -> UpsertUser {
        UpsertUser {
            email: email.to_string(),
            password: "these_pretzels_are_making_me_thirsty".to_string(),
            fullname: "Kramer".to_string(),
            role: "entrepreneur".to_string(),
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let hashmap = create_test_hashmap();
        let request = create_test_upsert_user("jerry@seinfeld.com");

        let result = create_user(request, &hashmap).await;

        assert!(result.is_some());
        let user = result.unwrap();
        assert_eq!(user.email, "jerry@seinfeld.com");
        assert_eq!(user.password, "these_pretzels_are_making_me_thirsty");
        assert_eq!(user.fullname, "Kramer");
        assert_eq!(user.role, "entrepreneur");
        assert_eq!(user.id, 1);
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let hashmap = create_test_hashmap();
        let request1 = create_test_upsert_user("george@yankees.com");
        let request2 = create_test_upsert_user("george@yankees.com");

        let result1 = create_user(request1, &hashmap).await;
        assert!(result1.is_some());

        let result2 = create_user(request2, &hashmap).await;
        assert!(result2.is_none());
    }

    #[tokio::test]
    async fn test_create_multiple_users() {
        let hashmap = create_test_hashmap();

        let user1 = create_user(create_test_upsert_user("jerry@apartments5a.com"), &hashmap).await;
        let user2 = create_user(create_test_upsert_user("kramer@apartments5b.com"), &hashmap).await;
        let user3 = create_user(create_test_upsert_user("newman@apartments5e.com"), &hashmap).await;

        assert!(user1.is_some());
        assert!(user2.is_some());
        assert!(user3.is_some());

        assert_eq!(user1.unwrap().id, 1);
        assert_eq!(user2.unwrap().id, 2);
        assert_eq!(user3.unwrap().id, 3);
    }

    #[tokio::test]
    async fn test_get_user_by_email_success() {
        let hashmap = create_test_hashmap();
        let request = create_test_upsert_user("elaine@pendant_publishing.com");

        create_user(request, &hashmap).await;

        let result = get_user_by_email(&"elaine@pendant_publishing.com".to_string(), &hashmap).await;

        assert!(result.is_some());
        let user = result.unwrap();
        assert_eq!(user.email, "elaine@pendant_publishing.com");
    }

    #[tokio::test]
    async fn test_get_user_by_email_not_found() {
        let hashmap = create_test_hashmap();

        let result = get_user_by_email(&"larry_david@curb.com".to_string(), &hashmap).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_update_user_by_email_success() {
        let hashmap = create_test_hashmap();
        let request = create_test_upsert_user("puddy@devils.com");

        create_user(request, &hashmap).await;

        let update_request = UpsertUser {
            email: "puddy@devils.com".to_string(),
            password: "yeah_thats_right".to_string(),
            fullname: "David Puddy".to_string(),
            role: "car_salesman".to_string(),
        };

        let result = update_user_by_email(&"puddy@devils.com".to_string(), update_request, &hashmap).await;

        assert!(result.is_some());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.password, "yeah_thats_right");
        assert_eq!(updated_user.fullname, "David Puddy");
        assert_eq!(updated_user.role, "car_salesman");
        assert_eq!(updated_user.email, "puddy@devils.com");
    }

    #[tokio::test]
    async fn test_update_user_by_email_not_found() {
        let hashmap = create_test_hashmap();

        let update_request = UpsertUser {
            email: "babu@dreamcafe.com".to_string(),
            password: "very_bad_man".to_string(),
            fullname: "Babu Bhatt".to_string(),
            role: "restaurant_owner".to_string(),
        };

        let result = update_user_by_email(&"babu@dreamcafe.com".to_string(), update_request, &hashmap).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_delete_user_by_email_success() {
        let hashmap = create_test_hashmap();
        let request = create_test_upsert_user("crazy_joe_davola@opera.com");

        create_user(request, &hashmap).await;

        let result = delete_user_by_email(&"crazy_joe_davola@opera.com".to_string(), &hashmap).await;

        assert!(result.is_some());
        let deleted_user = result.unwrap();
        assert_eq!(deleted_user.email, "crazy_joe_davola@opera.com");

        // Verify user is actually deleted
        let get_result = get_user_by_email(&"crazy_joe_davola@opera.com".to_string(), &hashmap).await;
        assert!(get_result.is_none());
    }

    #[tokio::test]
    async fn test_delete_user_by_email_not_found() {
        let hashmap = create_test_hashmap();

        let result = delete_user_by_email(&"bob_sacamano@urban_legend.com".to_string(), &hashmap).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let hashmap = create_test_hashmap();

        // Create multiple users concurrently
        let hashmap1 = Arc::clone(&hashmap);
        let hashmap2 = Arc::clone(&hashmap);
        let hashmap3 = Arc::clone(&hashmap);

        let handle1 = tokio::spawn(async move {
            create_user(create_test_upsert_user("helen@seinfeld.com"), &hashmap1).await
        });

        let handle2 = tokio::spawn(async move {
            create_user(create_test_upsert_user("estelle@costanza.com"), &hashmap2).await
        });

        let handle3 = tokio::spawn(async move {
            create_user(create_test_upsert_user("susan@ross.com"), &hashmap3).await
        });

        let results = tokio::join!(handle1, handle2, handle3);

        assert!(results.0.unwrap().is_some());
        assert!(results.1.unwrap().is_some());
        assert!(results.2.unwrap().is_some());

        // Verify all users were created
        let map = hashmap.lock().unwrap();
        assert_eq!(map.len(), 3);
    }
}

