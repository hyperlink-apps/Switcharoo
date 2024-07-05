use super::password::Password;

pub struct User {}

impl User {
    pub async fn create(
        email: &str,
        password: &str,
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<(), String> {
        let hash = Password::hash(password)?;

        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2);
            "#,
            email,
            hash
        )
        .execute(pool)
        .await
        .map_err(|e| {
            println!("Error inserting new user: {}", e.to_string());
            String::from("Error inserting new user.")
        })?;

        return Ok(());
    }

    pub async fn check_email_password(
        email: &str,
        password: &str,
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<(), String> {
        let query_result = sqlx::query!(
            r#"
            SELECT
                password_hash
            FROM
                users
            WHERE
                email = $1;
            "#,
            email
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Error getting password hash: {}", e.to_string());
            "Internal server error."
        })?;

        match query_result {
            Some(row) => {
                if let Some(hash) = row.password_hash {
                    let result = Password::validate(password, &hash).map_err(|e| {
                        eprintln!("Error validating password hash: {}", e.to_string());
                        "Internal server error."
                    })?;

                    if result == false {
                        return Err(String::from("Incorrect email or password."));
                    }
                } else {
                    return Err(String::from("Incorrect email or password."));
                }
            }
            None => return Err(String::from("Incorrect email or password.")),
        }

        Ok(())
    }

    pub async fn exists_by_email(
        email: &str,
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS count
            FROM
                users
            WHERE
                email = $1;
            "#,
            email
        )
        .fetch_one(pool)
        .await?;

        return Ok(result.count.unwrap_or(0) != 0);
    }
}
