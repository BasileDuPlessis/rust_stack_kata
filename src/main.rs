fn main() {

}

struct BoundedStack {
    size: usize,
    capacity: usize,
    elements: Vec<usize>,
}

struct ZeroCapacityStack {}

trait Stack {
    fn is_empty(&self) -> bool;
    fn push(&mut self, element: usize);
    fn pop(&mut self) -> usize;
    fn get_size(&self) -> usize;
    fn top(&self) -> usize;
    fn find(&self, element: usize) -> Option<usize>;
}

impl Stack for BoundedStack {
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn push(&mut self, element: usize) {
        if self.size == self.capacity {
            panic!("Capacity overflow");
        }
        self.elements.push(element);
        self.size += 1;
    }
    fn pop(&mut self) -> usize {
        if self.is_empty() {
            panic!("Capacity underflow");
        }
        self.size -= 1;
        self.elements[self.size]
    }
    fn top(&self) -> usize {
        if self.is_empty() {
            panic!("Empty");
        }
        self.elements[self.size - 1]
    }
    fn get_size(&self) -> usize {
        self.size
    }
    fn find(&self, element: usize) -> Option<usize> {
        for i in (0..=(self.size - 1)).rev() {
            if self.elements[i] == element {
                return Some(self.size - 1 - i);
            }
        }
        None
    }
}

impl Stack for ZeroCapacityStack {
    fn get_size(&self) -> usize {
        0
    }
    fn is_empty(&self) -> bool {
        true
    }
    fn pop(&mut self) -> usize {
        panic!("Capacity underflow");
    }
    fn top(&self) -> usize {
        panic!("Empty");
    }
    fn push(&mut self, _: usize) {
        panic!("Capacity overflow");
    }
    fn find(&self, _: usize) -> Option<usize> {
        None
    }
}

impl BoundedStack {
    fn new(capacity: usize) -> Box<dyn Stack> {
        if capacity == 0 {
            return Box::new(ZeroCapacityStack {});
        }
        Box::new(BoundedStack {
            size: 0,
            capacity,
            elements: Vec::with_capacity(capacity),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct TestContext {
        stack: Box<dyn Stack>,
    }
    impl TestContext {
        fn setup() -> TestContext {
            TestContext {
                stack: BoundedStack::new(2),
            }
        }
    }
    #[test]
    fn newly_create_stack_should_be_sempty() {
        let stack = TestContext::setup().stack;
        assert!(stack.is_empty());
        assert_eq!(stack.get_size(), 0);
    }

    #[test]
    fn after_one_push_stack_size_should_be_one() {
        let mut stack = TestContext::setup().stack;
        stack.push(1);
        assert_eq!(stack.get_size(), 1);
    }

    #[test]
    fn after_one_push_and_one_pop_stack_size_should_be_empty() {
        let mut stack = TestContext::setup().stack;
        stack.push(1);
        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    #[should_panic]
    fn when_pushed_past_limit_stack_overflows() {
        let mut stack = TestContext::setup().stack;
        stack.push(1);
        stack.push(1);
        stack.push(1);
    }

    #[test]
    #[should_panic]
    fn when_empty_stack_is_popped_stack_overflows() {
        let mut stack = TestContext::setup().stack;
        stack.pop();
        stack.pop();
        stack.pop();
    }

    #[test]
    fn when_one_is_pushed_one_is_popped() {
        let mut stack = TestContext::setup().stack;
        stack.push(1);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    fn when_one_two_is_pushed_two_one_is_popped() {
        let mut stack = TestContext::setup().stack;
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    #[should_panic]
    fn when_creating_stack_with_zero_capacity_every_push_should_overflow() {
        let mut stack = BoundedStack::new(0);
        stack.push(1);
    }

    #[test]
    fn when_one_is_pushed_one_is_on_top() {
        let mut stack = TestContext::setup().stack;
        stack.push(1);
        assert_eq!(stack.top(), 1);
    }
    #[test]
    #[should_panic]
    fn when_stack_is_empty_top_throws_empty() {
        let stack = TestContext::setup().stack;
        stack.top();
    }
    #[test]
    #[should_panic]
    fn with_zero_capacity_stack_any_top_throws_empty() {
        let stack = BoundedStack::new(0);
        stack.top();
    }
    #[test]
    fn given_stack_with_one_two_pushed_find_one() {
        let mut stack = TestContext::setup().stack;
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.find(1).unwrap(), 1);
        assert_eq!(stack.find(2).unwrap(), 0);
    }
    #[test]
    fn with_zero_capacity_stack_find_throws_none() {
        let stack = BoundedStack::new(0);
        assert_eq!(stack.find(2), None);
    }
}
