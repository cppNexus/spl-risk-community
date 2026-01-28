use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Cache entry with expiration
#[derive(Clone)]
struct CacheEntry<T> {
    value: T,
    inserted_at: Instant,
}

impl<T> CacheEntry<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            inserted_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.inserted_at.elapsed() > ttl
    }
}

/// Thread-safe LRU cache with TTL
pub struct Cache<T: Clone> {
    data: Arc<RwLock<HashMap<Pubkey, CacheEntry<T>>>>,
    ttl: Duration,
    max_size: usize,
}

impl<T: Clone> Cache<T> {
    pub fn new(ttl: Duration, max_size: usize) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            ttl,
            max_size,
        }
    }

    /// Get value from cache if not expired
    pub fn get(&self, key: &Pubkey) -> Option<T> {
        let data = self.data.read().ok()?;

        if let Some(entry) = data.get(key) {
            if !entry.is_expired(self.ttl) {
                return Some(entry.value.clone());
            }
        }

        None
    }

    /// Insert value into cache
    pub fn insert(&self, key: Pubkey, value: T) {
        let mut data = match self.data.write() {
            Ok(guard) => guard,
            Err(_) => return, // Poisoned lock, skip caching
        };

        // Remove expired entries first
        data.retain(|_, entry| !entry.is_expired(self.ttl));

        // If at capacity, remove oldest entry
        if data.len() >= self.max_size {
            if let Some(oldest_key) = data
                .iter()
                .min_by_key(|(_, entry)| entry.inserted_at)
                .map(|(k, _)| *k)
            {
                data.remove(&oldest_key);
            }
        }

        data.insert(key, CacheEntry::new(value));
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        if let Ok(mut data) = self.data.write() {
            data.clear();
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let data = self.data.read().ok();
        let size = data.as_ref().map(|d| d.len()).unwrap_or(0);

        let expired_count = data
            .as_ref()
            .map(|d| {
                d.values()
                    .filter(|entry| entry.is_expired(self.ttl))
                    .count()
            })
            .unwrap_or(0);

        CacheStats {
            size,
            capacity: self.max_size,
            expired_entries: expired_count,
        }
    }
}

impl<T: Clone> Clone for Cache<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            ttl: self.ttl,
            max_size: self.max_size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub capacity: usize,
    pub expired_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_insert_and_get() {
        let cache: Cache<String> = Cache::new(Duration::from_secs(60), 100);
        let key = Pubkey::new_unique();
        let value = "test_value".to_string();

        cache.insert(key, value.clone());

        assert_eq!(cache.get(&key), Some(value));
    }

    #[test]
    fn test_cache_expiration() {
        let cache: Cache<String> = Cache::new(Duration::from_millis(100), 100);
        let key = Pubkey::new_unique();
        let value = "test_value".to_string();

        cache.insert(key, value.clone());
        assert_eq!(cache.get(&key), Some(value));

        // Wait for expiration
        thread::sleep(Duration::from_millis(150));

        assert_eq!(cache.get(&key), None);
    }

    #[test]
    fn test_cache_max_size() {
        let cache: Cache<u64> = Cache::new(Duration::from_secs(60), 3);

        for i in 0..5 {
            let key = Pubkey::new_unique();
            cache.insert(key, i);
        }

        let stats = cache.stats();
        assert!(stats.size <= 3);
    }

    #[test]
    fn test_cache_clear() {
        let cache: Cache<String> = Cache::new(Duration::from_secs(60), 100);

        for _ in 0..10 {
            cache.insert(Pubkey::new_unique(), "value".to_string());
        }

        assert!(cache.stats().size > 0);

        cache.clear();
        assert_eq!(cache.stats().size, 0);
    }

    #[test]
    fn test_cache_clone() {
        let cache1: Cache<String> = Cache::new(Duration::from_secs(60), 100);
        let key = Pubkey::new_unique();

        cache1.insert(key, "value".to_string());

        let cache2 = cache1.clone();
        assert_eq!(cache2.get(&key), Some("value".to_string()));
    }
}
