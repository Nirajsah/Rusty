#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Node {
    data: u32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: u32, nxt: Option<Box<Node>>) -> Self {
        let x = Node { data, next: nxt };
        x
    }
}

fn main() {
    let x = Node::new(1, None);
    let y = Node::new(2, Some(Box::new(x.clone())));
    println!("x: {:?}, y: {:?}", x.data, y.next);
}
