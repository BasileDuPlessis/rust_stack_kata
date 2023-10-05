fn main() {
    println!("Hello, world!");
}

struct Stack{
    size: usize,
    capacity: usize
}

impl Stack {
    fn new(capacity: usize) -> Stack {
        Stack {
            size: 0,
            capacity
        }
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn get_size(&self) -> usize {
        self.size
    }
    fn push(&mut self, value:usize) {
        if self.size == self.capacity {
            panic!("Capacity overflow");
        }            
        self.size += 1;
    }
    fn pop(&mut self) {
        if self.size == 0 {
            panic!("Capacity overflow");
        }
        self.size -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        stack: Stack
    }

    impl TestContext {
        fn setup() -> TestContext {
            TestContext {
                stack : Stack::new(2)
            }
        }
    }
    #[test]
    fn newly_create_stack_should_be_sempty() {
        let test_context = TestContext::setup();
        assert!(test_context.stack.is_empty());
        assert_eq!(test_context.stack.get_size(), 0);
    }

    #[test]
    fn after_one_push_stack_size_should_be_one() {
        let mut test_context = TestContext::setup();
        test_context.stack.push(1);
        assert_eq!(test_context.stack.get_size(), 1);
    }

    #[test]
    fn after_one_push_and_one_pop_stack_size_should_be_empty() {
        let mut test_context = TestContext::setup();
        test_context.stack.push(1);
        test_context.stack.pop();
        assert!(test_context.stack.is_empty());
    }

    #[test]
    #[should_panic]
    fn when_pushed_past_limit_stack_overflows() {
        let mut test_context = TestContext::setup();
        test_context.stack.push(1);
        test_context.stack.push(1);
        test_context.stack.push(1);
    }

    #[test]
    #[should_panic]
    fn when_empty_stack_is_popped_stack_overflows() {
        let mut test_context = TestContext::setup();
        test_context.stack.pop();
        test_context.stack.pop();
        test_context.stack.pop();
    }

}

