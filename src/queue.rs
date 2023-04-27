use std::fmt::Debug;

struct CircleQueue<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    count: usize,
}

#[allow(dead_code)]
impl<T> CircleQueue<T>
where
    T: Clone + Copy + Debug + PartialEq,
{
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![None; size],
            head: 0,
            tail: 0,
            size,
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count == self.size
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("The queue is full");
        }

        self.data[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.size;
        self.count += 1;
        Ok(())
    }

    pub fn dequeue(&mut self) -> Result<Option<T>, &str> {
        if self.is_empty() {
            return Err("The queue is empty");
        }

        let val = self.data[self.head].take();
        self.head = (self.head + 1) % self.size;
        self.count -= 1;
        Ok(val)
    }

    pub fn front(&self) -> Option<&T> {
        match self.data[self.head] {
            Some(ref val) => Some(val),
            None => None,
        }
    }

    pub fn show(&self) {
        println!(
            "head: {}, tail: {}, count: {}",
            self.head, self.tail, self.count
        );
        for i in 0..self.size {
            match self.data[i] {
                Some(ref val) => print!("{:?} ", val),
                None => print!("None "),
            }
        }
        println!();
    }
}

#[test]
fn test_queue() {
    let mut q = CircleQueue::new(3);

    q.enqueue(1).unwrap();
    q.enqueue(2).unwrap();

    q.dequeue().unwrap();

    q.enqueue(3).unwrap();
    q.enqueue(4).unwrap();

    q.show();
}
