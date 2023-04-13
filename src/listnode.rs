use std::ptr;

#[allow(dead_code)]
pub struct UnsafeListNode {
    pub val: i32,
    pub next: *mut UnsafeListNode,
}

#[allow(dead_code)]
impl UnsafeListNode {
    pub fn new(val: i32) -> UnsafeListNode {
        UnsafeListNode {
            val,
            next: ptr::null_mut(),
        }
    }

    pub fn from(vals: Vec<i32>) -> UnsafeListNode {
        if vals.len() == 0 {
            return UnsafeListNode::new(0);
        }

        let mut head = UnsafeListNode::new(vals[0]);

        for i in 1..vals.len() {
            head.insert(vals[i]);
        }

        head
    }

    pub fn new_as_ptr(val: i32) -> *mut UnsafeListNode {
        &mut UnsafeListNode {
            val,
            next: ptr::null_mut(),
        }
    }

    pub fn as_ptr(&mut self) -> *mut UnsafeListNode {
        self
    }

    pub fn insert(&mut self, val: i32) {
        let mut cur = self;
        unsafe {
            while !(*cur).next.is_null() {
                cur = &mut *(*cur).next;
            }
            (*cur).next = Box::into_raw(Box::new(UnsafeListNode::new(val)));
        }
    }

    pub fn get_next(&self) -> &mut UnsafeListNode {
        unsafe { &mut (*(self.next)) }
    }

    pub fn set_next(&mut self, next: &mut UnsafeListNode) {
        (*self).next = next;
    }

    pub fn show(&self) {
        let mut cur = self;
        let mut max = 0;
        const MAX: usize = 10;
        unsafe {
            while !(*cur).next.is_null() {
                print!("{} -> ", (*cur).val);
                cur = &mut *(*cur).next;

                max += 1;
                if max > MAX {
                    break;
                }
            }
            println!("{}", (*cur).val);
        }
    }
}

#[test]
fn test_unsafe_list_node() {
    let mut n = UnsafeListNode::from(vec![1, 2, 3, 4, 5, 6, 7]);
    n.show();

    unsafe {
        let head = n.as_ptr();
        let mut p = n.get_next();

        while p.val != 5 {
            p = (*p).next.as_mut().unwrap();
        }

        p.set_next(&mut (*head));

        println!("showing ring:");
        (*p).show();
    }
}
