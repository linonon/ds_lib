use std::{cell::RefCell, cmp::PartialEq, fmt::Debug, ptr, rc::Rc};

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

impl<T> Default for ListNode<T> {
    fn default() -> Self {
        Self { node: None }
    }
}

impl<T> PartialEq for ListNode<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.node.is_none() && other.node.is_none() {
            return true;
        }

        let a = self.node.as_ref().unwrap();
        let b = other.node.as_ref().unwrap();

        Rc::ptr_eq(a, b)
    }
}

impl<T> ListNode<T>
where
    T: PartialEq + Debug + Copy,
{
    pub fn new(val: T) -> ListNode<T> {
        ListNode {
            node: Some(Rc::new(RefCell::new(Node { val, next: None }))),
        }
    }

    pub fn is_none(&self) -> bool {
        self.node.is_none()
    }

    pub fn is_some(&self) -> bool {
        self.node.is_some()
    }

    // return ListNode{node: None}
    pub fn none() -> ListNode<T> {
        ListNode::default()
    }

    pub fn from(vals: Vec<T>) -> ListNode<T> {
        if vals.len() == 0 {
            return ListNode::none();
        }

        let mut head = ListNode::new(vals[0]);

        for i in 1..vals.len() {
            head.insert(vals[i]);
        }

        head
    }

    pub fn insert(&mut self, val: T) {
        let mut cur = self.clone();
        while cur.next().is_some() {
            cur.go_next();
        }

        cur.set_next(ListNode::new(val));
    }

    /// get the next node
    pub fn next(&mut self) -> ListNode<T> {
        let node = self.node.clone();
        let n = match node {
            Some(n) => n,
            None => return ListNode::default(),
        };

        let next = n.borrow().next.clone();
        match next {
            Some(n) => ListNode { node: Some(n) },
            None => ListNode::default(),
        }
    }

    // set the next node, can be None
    pub fn set_next(&mut self, n: ListNode<T>) {
        let node = self.node.clone();
        match node {
            Some(node) => {
                node.clone().borrow_mut().next = n.node;
            }
            None => {}
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

    /// Both `find` and `get` can be used to describe the purpose of a function in this case,
    /// but there are subtle semantic differences between them. Here are the differences:
    ///
    /// `Get` is often used to retrieve a known element from a data structure.
    /// In many cases, when using "get", developers expect to always be able to find the requested element.
    /// If the element cannot be found, an exception or error is usually thrown.
    ///
    /// `Find` emphasizes searching for a specific element in a data structure, which may or may not exist.
    /// When using "find", developers are often more willing to accept the fact that the function may not be able to find the requested element.
    /// Typically, the "find" function returns an Option type (such as in Rust) so that it can represent both the case where the element is found and where it is not found.
    pub fn find_first_node_with_value(&mut self, val: T) -> Option<ListNode<T>> {
        let mut cur = self.clone();
        while cur.next().is_some() {
            if cur.val() == val {
                return Some(cur);
            }
            cur.go_next();
        }
        if cur.val() == val {
            return Some(cur);
        }

        None
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

    /// print the `ListNode` elements.
    pub fn show(&self) {
        println!("{}", self.stringify());
    }

    /// Returns a string representation of a `ListNode` with a maximum of 10 elements.
    ///
    /// This function creates a string that contains the values of the `ListNode` elements, separated by a "->" arrow.
    /// To prevent infinite loops in case of circular references, the function limits the representation to a maximum
    /// of 10 elements. If the list has more than 10 elements, the output will be truncated.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::listnode::ListNode;
    /// let mut list = ListNode::from(vec![1,2,3]));
    /// assert_eq!(list.stringify(), "3 -> 2 -> 1");
    /// ```
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

    /// Returns the length of the `ListNode`.
    ///
    /// This function calculates the number of elements in the `ListNode` by traversing the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_structures::listnode::ListNode;
    /// let mut list = ListNode::from(vec![1,2,3]);
    /// assert_eq!(list.len(), 3);   
    /// ```
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
    let mut n = ListNode::from(vec![1, 2, 3, 4, 5, 6, 7]);
    println!("{}", n.stringify());

    let mut n2 = n.next();
    // 2 -> 3 -> 4 -> 5 -> 6 -> 7
    println!("{}", n2.stringify());

    n2.go_next();
    // 3 -> 4 -> 5 -> 6 -> 7
    println!("{}", n2.stringify());

    let mut n3 = n.find_first_node_with_value(5).unwrap();
    n3.set_next(n.clone());

    n3.show();
}
