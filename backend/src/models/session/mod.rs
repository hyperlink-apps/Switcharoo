use moka::future::Cache;
use rand::Rng;

pub struct Session {}

impl Session {
    pub async fn get_expiration(cache: &Cache<String, i64>, key: &str) -> Option<i64> {
        cache.get(key).await
    }

    pub async fn get_and_increase_expiration(cache: &Cache<String, i64>, key: &str) -> Option<i64> {
        let expiration = Self::get_expiration(cache, key).await;

        if let Some(_) = expiration {
            cache
                .insert(key.to_string(), (30 * 60) + chrono::Utc::now().timestamp())
                .await;
        }

        expiration
    }

    pub async fn create(cache: &Cache<String, i64>) -> (String, i64) {
        let token: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let expiration = (30 * 60) + chrono::Utc::now().timestamp();

        cache.insert(token.clone(), expiration).await;

        (token, expiration)
    }

    pub async fn delete(cache: &Cache<String, i64>, key: &str) {
        cache.remove(key).await;
    }
}
