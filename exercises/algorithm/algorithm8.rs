/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peak(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T>
{
    q1:Queue<T>,
    q2:Queue<T>
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1:Queue::<T>::new(),
            q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        while self.q1.size() > 1 {
            // 这里使用 unwrap 是安全的，因为检查了 size > 1
            self.q2.enqueue(self.q1.dequeue().unwrap());
        }

        // 修复2：使用 unwrap() 来解除对 self.q1 的借用
        // 此时 q1 只有一个元素，所以 unwrap 总是安全的
        let last_elem = self.q1.dequeue().unwrap();

        // 现在可以安全地交换了，因为没有活跃的借用
        std::mem::swap(&mut self.q1, &mut self.q2);

        Ok(last_elem)
    }
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_queue(){
        // 修复1：更新测试中的类型名称
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}