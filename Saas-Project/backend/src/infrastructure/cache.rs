use redis::{AsyncCommands, Client, RedisError};
use serde::{de::DeserializeOwned, Serialize};
// use std::time::Duration;
use tracing::{debug, error, info, instrument};

#[derive(Clone)]
pub struct CacheService {
    client: Client,
}

impl CacheService {
    #[instrument(skip(redis_url))]
    pub fn new(redis_url: &str) -> Result<Self, RedisError> {
        info!("🔄 Initializing Redis cache service");
        let client = Client::open(redis_url)?;
        info!("✅ Redis cache service initialized");

        Ok(Self { client })
    }

    #[instrument(skip(self, key, value), fields(key = %key))]
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        expiry_secs: Option<u64>,
    ) -> Result<(), RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let serialized = serde_json::to_string(value).map_err(|e| {
            tracing::error!("Serialization error: {}", e);
            RedisError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to serialize value",
            ))
        })?;

        tracing::debug!("Setting cache key: {}", key);

        match expiry_secs {
            Some(secs) => {
                conn.set_ex::<_, _, ()>(key, serialized, secs).await?;
                tracing::debug!("Cache key set with expiry: {}s", secs);
            }
            None => {
                conn.set::<_, _, ()>(key, serialized).await?;
                debug!("Cache key set with no expiry");
            }
        }

        Ok(())
    }

    #[instrument(skip(self), fields(key = %key))]
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisError> {
        let mut conn = self.client.get_async_connection().await?;

        debug!("Getting cache key: {}", key);

        let result: Option<String> = conn.get(key).await?;

        match result {
            Some(data) => {
                let deserialized = serde_json::from_str(&data).map_err(|e| {
                    error!("Deserialization error for key {}: {}", key, e);
                    RedisError::from(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Failed to deserialize value",
                    ))
                })?;
                debug!("Cache hit for key: {}", key);
                Ok(Some(deserialized))
            }
            None => {
                debug!("Cache miss for key: {}", key);
                Ok(None)
            }
        }
    }

    #[instrument(skip(self), fields(key = %key))]
    pub async fn delete(&self, key: &str) -> Result<(), RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        tracing::debug!("Deleting cache key: {}", key);
        conn.del::<_, ()>(key).await?;
        tracing::debug!("Cache key deleted: {}", key);
        Ok(())
    }

    #[instrument(skip(self), fields(pattern = %pattern))]
    pub async fn delete_by_pattern(&self, pattern: &str) -> Result<(), RedisError> {
        let mut conn = self.client.get_async_connection().await?;

        tracing::debug!("Deleting cache keys by pattern: {}", pattern);

        // Get keys matching pattern
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(pattern)
            .query_async::<_, Vec<String>>(&mut conn)
            .await?;

        if !keys.is_empty() {
            // Delete all matching keys
            redis::cmd("DEL")
                .arg(keys.clone())
                .query_async::<_, ()>(&mut conn)
                .await?;

            tracing::debug!(
                "Deleted {} cache keys matching pattern: {}",
                keys.len(),
                pattern
            );
        } else {
            tracing::debug!("No keys found matching pattern: {}", pattern);
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn health_check(&self) -> Result<(), RedisError> {
        let mut conn = self.client.get_async_connection().await?;

        let pong: String = redis::cmd("PING").query_async(&mut conn).await?;

        if pong != "PONG" {
            return Err(RedisError::from(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                "Redis health check failed",
            )));
        }

        Ok(())
    }
}
