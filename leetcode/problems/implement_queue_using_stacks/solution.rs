struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.in_stack.push(x)
    }
    
    fn pop(&mut self) -> i32 {
        if(self.out_stack.is_empty()) {
            while let Some(x) = self.in_stack.pop() {
                self.out_stack.push(x);
            }
        }
        self.out_stack.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if(self.out_stack.is_empty()) {
            while let Some(x) = self.in_stack.pop() {
                self.out_stack.push(x);
            }
        }
        *self.out_stack.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
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