use std::collections::{BinaryHeap, HashMap};
use std::time::{Instant, Duration};
use std::cmp::Reverse;

type ExpiryHeap = BinaryHeap<Reverse<(Instant, usize, String)>>;

#[derive(Debug)]
pub struct Entry {
    pub value: Vec<u8>,
    expires_at: Option<Instant>,
    version: usize,
}

#[derive(Debug)]
pub struct Database {
    store: HashMap<String, Entry>,
    expiry_queue: ExpiryHeap,
}

impl Database {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            expiry_queue: BinaryHeap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Entry> {
        let expired = {
            let Some(e) = self.store.get(key) else {
                return None;
            };

            if let Some(expires_at) = e.expires_at {
                expires_at <= Instant::now()
            } else {
                false
            }
        };

        if expired {
            None
        } else {
            self.store.get(key)
        }
    }

    pub fn set(&mut self, key: &str, value: &[u8], ttl: Option<Duration>) {
        let entry = self.store.entry(key.into()).or_insert(Entry {
            value: Vec::new(),
            version: 0,
            expires_at: None,
        });

        let expires_at = match ttl {
            Some(dur) => Instant::now().checked_add(dur),
            None => None,
        };

        entry.version = entry.version.wrapping_add(1);
        entry.value = value.to_vec();
        entry.expires_at = expires_at;

        if let Some(expires_at) = entry.expires_at {
            self.expiry_queue.push(Reverse((expires_at, entry.version, key.into())));
        }
    }

    pub fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }

    pub fn time_until_next_expiration(&self) -> Option<Duration> {
        let now = Instant::now();

        self.expiry_queue.peek().map(| Reverse((when, _, _)) | {
            when.saturating_duration_since(now)
        })
    }

    pub fn delete_expired_keys(&mut self, budget: usize) {
        let now = Instant::now();

        for _ in 0 .. budget {
            match self.expiry_queue.peek().cloned() {
                Some(Reverse((when, version, key))) if when <= now => {
                    self.expiry_queue.pop();

                    let is_expiry_stale = match self.store.get(&key) {
                        Some(e) => e.version != version || e.expires_at != Some(when),
                        None => true,
                    };

                    if !is_expiry_stale {
                        self.delete(&key);
                    }
                }
                _ => break
            }
        }
    }
}
