use std::ptr;

#[allow(dead_code)]
pub struct ListNode {
    pub val: i32,
    pub next: *mut ListNode,
}

#[allow(dead_code)]
impl ListNode {
    pub fn new(val: i32) -> ListNode {
        ListNode {
            val,
            next: ptr::null_mut(),
        }
    }

    pub fn from(vals: Vec<i32>) -> ListNode {
        if vals.len() == 0 {
            return ListNode::new(0);
        }

        let mut head = ListNode::new(vals[0]);

        for i in 1..vals.len() {
            head.insert(vals[i]);
        }

        head
    }

    pub fn new_as_ptr(val: i32) -> *mut ListNode {
        &mut ListNode {
            val,
            next: ptr::null_mut(),
        }
    }

    pub fn as_ptr(&mut self) -> *mut ListNode {
        self
    }

    pub fn insert(&mut self, val: i32) {
        let mut cur = self;
        unsafe {
            while !(*cur).next.is_null() {
                cur = &mut *(*cur).next;
            }
            (*cur).next = Box::into_raw(Box::new(ListNode::new(val)));
        }
    }

    pub fn get_next(&mut self) -> *mut ListNode {
        self.next
    }

    pub fn show(&self) {
        let mut cur = self;
        unsafe {
            while !(*cur).next.is_null() {
                print!("{} -> ", (*cur).val);
                cur = &mut *(*cur).next;
            }
            println!("{}", (*cur).val);
        }
    }
}
