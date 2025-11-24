use regex::Regex;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub fullname: String,
    pub role: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertUser {
    pub email: String,
    pub password: String,
    pub fullname: String,
    pub role: String,
}

impl UpsertUser {
    pub fn is_valid_email(&self) -> bool {
        let email_pattern = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9]([A-Za-z0-9-]*[A-Za-z0-9])?(\.[A-Za-z0-9]([A-Za-z0-9-]*[A-Za-z0-9])?)*\.[A-Za-z]{2,}$").unwrap();
        email_pattern.is_match(&self.email)
    }
}

pub fn validate_email(body: &UpsertUser) -> bool {
    body.is_valid_email()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email_formats() {
        let valid_emails = vec![
            "jerry@seinfeld.com",
            "cosmo.kramer@kramerica.com",
            "elaine+jpeterman@catalog.co.uk",
            "art_vandelay@latex.io",
            "5150@bobsacamano.com",
            "george@yankees.example.com",
        ];

        for email in valid_emails {
            let user = UpsertUser {
                email: email.to_string(),
                password: "yada_yada".to_string(),
                fullname: "Jerry Seinfeld".to_string(),
                role: "comedian".to_string(),
            };
            assert!(user.is_valid_email(), "Expected {} to be valid", email);
            assert!(validate_email(&user), "Expected {} to be valid via validate_email", email);
        }
    }

    #[test]
    fn test_invalid_email_formats() {
        let invalid_emails = vec![
            "hellonewman",
            "@manssiere.com",
            "soup_nazi@",
            "bizzaro_jerry@.com",
            "bubble boy @spaceship.com",
            "babu@dreamcafe",
            "",
            "puddy@saab..dealership",
        ];

        for email in invalid_emails {
            let user = UpsertUser {
                email: email.to_string(),
                password: "no_soup_for_you".to_string(),
                fullname: "Soup Nazi".to_string(),
                role: "chef".to_string(),
            };
            assert!(!user.is_valid_email(), "Expected {} to be invalid", email);
            assert!(!validate_email(&user), "Expected {} to be invalid via validate_email", email);
        }
    }

    #[test]
    fn test_upsert_user_creation() {
        let user = UpsertUser {
            email: "jackie@chiles.com".to_string(),
            password: "outrageous_egregious_preposterous".to_string(),
            fullname: "Jackie Chiles".to_string(),
            role: "lawyer".to_string(),
        };

        assert_eq!(user.email, "jackie@chiles.com");
        assert_eq!(user.password, "outrageous_egregious_preposterous");
        assert_eq!(user.fullname, "Jackie Chiles");
        assert_eq!(user.role, "lawyer");
    }

    #[test]
    fn test_user_creation() {
        let user = User {
            id: 1,
            email: "morty@seinfeld.com".to_string(),
            password: "my_wallet".to_string(),
            fullname: "Morty Seinfeld".to_string(),
            role: "retired_raincoat_salesman".to_string(),
        };

        assert_eq!(user.id, 1);
        assert_eq!(user.email, "morty@seinfeld.com");
        assert_eq!(user.password, "my_wallet");
        assert_eq!(user.fullname, "Morty Seinfeld");
        assert_eq!(user.role, "retired_raincoat_salesman");
    }

    #[test]
    fn test_user_clone() {
        let user = User {
            id: 1,
            email: "steinbrenner@yankees.com".to_string(),
            password: "big_stein".to_string(),
            fullname: "George Steinbrenner".to_string(),
            role: "yankees_owner".to_string(),
        };

        let cloned = user.clone();
        assert_eq!(user.id, cloned.id);
        assert_eq!(user.email, cloned.email);
    }
}