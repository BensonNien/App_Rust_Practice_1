use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>
}
struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u64
}

// Note: 
// 1. Option<T> is an enumerated type
// One unobvious detail is that Rust can eliminate the tag field of Option<T> when the type T is a reference, Box, or other smart pointer type.
// This effectively eliminates null pointer dereferences.
//
// 2. Rc<T> is the Reference Counted Smart Pointer
// In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
// However, there are cases when a single value might have multiple owners. 
// For example, it's graph data structures.
//
// 3. RefCell<T> is useful when you're sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
// Because some Static analysis is impossible, if the Rust compiler can't be sure the code complies with the ownership rules, it might reject a correct pro- gram; 
// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>: 
// • Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners. 
// • Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime. 
// • Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
