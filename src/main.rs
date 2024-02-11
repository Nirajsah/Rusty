#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next: None }
    }
}

fn main() {
    let one = Node::new(32);
    let two = Node {
        data: 12,
        next: Some(Box::new(one)),
    };
    println!("{:?}", two.next.unwrap().data);
}
