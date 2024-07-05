use crate::models::{password::Password, user::User};
use email_address::EmailAddress;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl CreateUser {
    pub async fn validate(
        &self,
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Option<String>, String> {
        let mut result: Option<String> = None;

        if User::exists_by_email(&self.email, pool)
            .await
            .map_err(|e| e.to_string())?
        {
            result = Some(String::from("Email address is already in use."));
        } else if !EmailAddress::is_valid(&self.email) {
            result = Some(String::from("Email address is not valid."));
        } else if &self.password != &self.confirm_password {
            result = Some(String::from("Passwords don't match."))
        } else if let Some(message) = Password::check_strength(&self.password) {
            result = Some(message);
        }

        return Ok(result);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
