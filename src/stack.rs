#[allow(unused)]
pub struct Stack<T> {
    size: usize,
    top: i64,
    data: Vec<T>,
}

#[allow(unused)]
impl<T> Stack<T>
where
    T: Copy + std::fmt::Debug,
{
    pub fn new(size: usize) -> Stack<T> {
        Stack {
            size,
            top: -1,
            data: Vec::with_capacity(size),
        }
    }

    pub fn empty(&self) -> bool {
        self.top == -1
    }

    pub fn top(&self) -> &T {
        if self.empty() {
            panic!("Stack underflow");
        }
        &self.data[self.top as usize]
    }

    pub fn full(&self) -> bool {
        self.top == self.size as i64 - 1
    }

    pub fn push(&mut self, value: T) {
        if self.full() {
            panic!("Stack overflow");
        }
        self.top += 1;
        self.data.push(value);
    }

    pub fn pop(&mut self) -> T {
        if self.empty() {
            panic!("Stack underflow");
        }
        self.top -= 1;
        self.data.pop().unwrap()
    }

    fn show(&self) {
        println!("{:?}", self.data);
    }
}
