use std::{cell::RefCell, ptr, rc::Rc};

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

    pub fn set_next(&mut self, next: *mut UnsafeListNode) {
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

    pub fn get_first_with_val(&mut self, val: i32) -> Option<&mut UnsafeListNode> {
        let mut cur = self;
        unsafe {
            while !(*cur).next.is_null() {
                if (*cur).val == val {
                    return Some(&mut *cur);
                }
                cur = &mut *(*cur).next;
            }
            if (*cur).val == val {
                return Some(&mut *cur);
            }
        }
        None
    }

    pub fn to_next(&mut self) {
        let next = (*self).next;
        if !next.is_null() {
            let next = unsafe { Box::from_raw(next) };
            *self = *next;
        }
    }

    pub fn len(&self) -> usize {
        let mut cur = self;
        let mut len = 0;
        unsafe {
            while !(*cur).next.is_null() {
                cur = &mut *(*cur).next;
                len += 1;
            }
        }
        len
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct ListNode<T> {
    pub node: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Clone for ListNode<T> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
        }
    }
}

impl<T> ListNode<T>
where
    T: std::cmp::PartialEq + std::fmt::Debug + Copy,
{
    pub fn new(val: T) -> ListNode<T> {
        ListNode {
            node: Some(Rc::new(RefCell::new(Node { val, next: None }))),
        }
    }

    pub fn from(vals: Vec<T>) -> Option<ListNode<T>> {
        if vals.len() == 0 {
            return None;
        }

        let mut head = ListNode::new(vals[0]);

        for i in 1..vals.len() {
            head.insert(vals[i]);
        }

        Some(head)
    }

    pub fn insert(&mut self, val: T) {
        let mut cur = self.clone();
        while cur.next().is_some() {
            cur.go_next();
        }

        cur.set_next(ListNode::new(val));
    }

    /// get the next node
    pub fn next(&mut self) -> Option<ListNode<T>> {
        let node = self.node.clone();
        match node {
            Some(n) => {
                let next = n.borrow().next.clone();
                match next {
                    Some(n) => Some(ListNode { node: Some(n) }),
                    None => None,
                }
            }
            None => None,
        }
    }

    /// equal to `p = p.next`
    pub fn go_next(&mut self) {
        let node = self.node.clone();
        match node {
            Some(n) => {
                let next = n.borrow().next.clone();
                self.node = next;
            }
            None => {}
        }
    }

    // get the next node
    pub fn get_next(&mut self) -> Option<ListNode<T>> {
        let node = self.node.as_ref().unwrap().borrow_mut();
        if node.next.is_none() {
            return None;
        }

        let next = node.next.as_ref().unwrap().clone();
        Some(ListNode { node: Some(next) })
    }

    pub fn get_first_node_with_value(&mut self, val: T) -> Option<ListNode<T>> {
        let mut cur = self.node.as_ref().unwrap().clone();
        while cur.borrow().next.is_some() {
            if cur.borrow().val == val {
                return Some(ListNode { node: Some(cur) });
            }
            let tmp = cur.borrow().next.as_ref().unwrap().clone();
            cur = tmp;
        }
        if cur.borrow().val == val {
            return Some(ListNode { node: Some(cur) });
        }
        None
    }

    // set the next node, can be None
    pub fn set_next(&mut self, n: ListNode<T>) {
        let mut head = self.node.as_ref().unwrap().borrow_mut();
        match n.node {
            Some(node) => head.next = Some(node.clone()),
            None => head.next = None,
        }
    }

    pub fn val(&self) -> T {
        let node = self.node.clone().unwrap();
        let node = node.borrow();
        node.val
    }

    pub fn set_val(&mut self, val: T) {
        let mut head = self.node.as_ref().unwrap().borrow_mut();
        head.val = val;
    }

    /// print the listnode elements.
    pub fn show(&self) {
        println!("{}", self.stringify());
    }

    /// Sum the listnode elements in a string.  
    /// To advoid the infinite loop, show max 10 elements.
    pub fn stringify(&self) -> String {
        let mut cur = self.clone();
        let mut n = 0;
        const MAX: usize = 10;
        let mut s = String::new();

        while cur.next().is_some() {
            s.push_str(&format!("{:?} -> ", cur.val()));

            cur.go_next();
            n += 1;
            if n > MAX {
                break;
            }
        }
        s.push_str(&format!("{:?}", cur.val()));
        s
    }

    pub fn len(&self) -> usize {
        let mut n = 0;
        let mut cur = self.clone();

        while cur.node.is_some() {
            cur.go_next();
            n += 1;
        }

        n
    }
}

#[test]
fn test_list_node() {
    let mut n = ListNode::from(vec![1, 2, 3, 4, 5, 6, 7]).unwrap();
    println!("{}", n.stringify());

    let mut n2 = n.get_next().unwrap();
    // 2 -> 3 -> 4 -> 5 -> 6 -> 7
    println!("{}", n2.stringify());

    n2.go_next();
    // 3 -> 4 -> 5 -> 6 -> 7
    println!("{}", n2.stringify());

    let mut n3 = n.get_first_node_with_value(5).unwrap();
    n3.set_next(n.node.clone());

    n3.show();
}
