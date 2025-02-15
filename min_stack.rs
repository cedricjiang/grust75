struct MinStack {
    val: Vec<i32>,
    min_val: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self{val: Vec::new(), min_val: Vec::new()}
    }
    
    fn push(&mut self, val: i32) {
        self.val.push(val);
        self.min_val.push(val.min(*self.min_val.last().unwrap_or(&i32::MAX)));
    }
    
    fn pop(&mut self) {
        self.val.pop();
        self.min_val.pop();
    }
    
    fn top(&self) -> i32 {
        *self.val.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_val.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
