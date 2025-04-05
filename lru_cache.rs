use std::collections::HashMap;

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    storage: Vec<(i32, i32, usize, usize)>,
    next_free: usize,
    sentinel: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;

        let mut storage = vec![(0, 0, 0, 0); capacity + 1];
        storage[0].2 = 0;
        storage[0].3 = 0;

        Self {
            capacity,
            map: HashMap::new(),
            storage,
            next_free: 1,
            sentinel: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&index) = self.map.get(&key) {
            let prev = self.storage[index].2;
            let next = self.storage[index].3;

            self.storage[prev].3 = next;
            self.storage[next].2 = prev;

            let tmp = self.storage[self.sentinel].3;
            self.storage[index].3 = tmp;
            self.storage[tmp].2 = index;

            self.storage[index].2 = self.sentinel;
            self.storage[self.sentinel].3 = index;

            self.storage[index].1
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&index) = self.map.get(&key) {
            let prev = self.storage[index].2;
            let next = self.storage[index].3;

            self.storage[prev].3 = next;
            self.storage[next].2 = prev;

            let tmp = self.storage[self.sentinel].3;
            self.storage[index].3 = tmp;
            self.storage[tmp].2 = index;

            self.storage[index].2 = self.sentinel;
            self.storage[self.sentinel].3 = index;

            self.storage[index].1 = value;
        } else if self.map.len() < self.capacity {
            let index = self.next_free;
            self.next_free += 1;

            self.map.insert(key, index);

            let tmp = self.storage[self.sentinel].3;
            self.storage[index].3 = tmp;
            self.storage[tmp].2 = index;

            self.storage[index].2 = self.sentinel;
            self.storage[self.sentinel].3 = index;

            self.storage[index].0 = key;
            self.storage[index].1 = value;
        } else {
            let index = self.storage[self.sentinel].2;
            self.map.remove(&self.storage[index].0);

            self.map.insert(key, index);

            let prev = self.storage[index].2;
            let next = self.storage[index].3;

            self.storage[prev].3 = next;
            self.storage[next].2 = prev;

            let tmp = self.storage[self.sentinel].3;

            self.storage[index].2 = self.sentinel;
            self.storage[self.sentinel].3 = index;

            self.storage[index].3 = tmp;
            self.storage[tmp].2 = index;

            self.storage[index].0 = key;
            self.storage[index].1 = value;
        };
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */