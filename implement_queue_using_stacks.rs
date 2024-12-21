struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            input: Vec::new(),
            output: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        self.input.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.maybe_turnover();
        // safe to unwrap since the problem guarantees validity of pop call
        self.output.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        self.maybe_turnover();
        // safe to unwrap since the problem guarantees validity of peek call
        *self.output.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }

    fn maybe_turnover(&mut self) {
        if self.output.is_empty() {
            while let Some(v) = self.input.pop() {
                self.output.push(v);
            }
        }
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
