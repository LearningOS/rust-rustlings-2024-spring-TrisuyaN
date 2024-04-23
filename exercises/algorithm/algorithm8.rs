/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM NOT DON

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

    pub fn peek(&self) -> Result<&T, &str> {
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

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>,
    size:u32
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new(),
            size:0
        }
    }
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
        self.size+=1;
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
		//Err("Stack is empty")

        let res: Result<T, &str>;

        if !self.is_empty(){
            for _ in 1..=self.size-1{
                let tmp = self.q1.dequeue().unwrap();
                self.q2.enqueue(tmp);
            }
            if let Ok(x) = self.q1.dequeue() {
                res = Ok(x);
                std::mem::swap(&mut self.q1, &mut self.q2);
                self.size-=1;
                return res;
            }
        }
        return Err("Stack is empty");
    }
    pub fn is_empty(&self) -> bool {
		self.size == 0
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
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