use std::collections::HashMap;

struct TimeMap {
    data: HashMap<String, Vec<(i32, String)>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {data: HashMap::new()}
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data.entry(key).or_insert(Vec::new()).push((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(vec) = self.data.get(&key) {
            let loc = match vec.binary_search_by_key(&timestamp, |&(ts, _)| ts) {
                Ok(pos) => pos,
                Err(pos) => {
                    if pos == 0 {
                        return String::new();
                    }
                    pos - 1
                }
            };
            vec[loc].1.clone()
        } else {
            String::new()
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
