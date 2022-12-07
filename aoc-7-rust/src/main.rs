#![allow(dead_code)]
use std::io::{self, BufRead};

// Recursive struct that contains name, ref to parent and integer and a vector of children
#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    parent: Option<&'a mut Node<'a>>,
    value: i32,
    children: Vec<&'a Node<'a>>,
}
// constructor for Node
impl<'a> Node<'a> {
    fn new(name: &'a str, parent: Option<&'a mut Node<'a>>, value: i32) -> Node<'a> {
        Node {
            name: name,
            parent: parent,
            value: value,
            children: Vec::new(),
        }
    }
    // add a child to the node
    fn add_child(&mut self, child: &'a Node<'a>) {
        self.children.push(child);
    }
    // add a value to the node
    fn add_value(&mut self, value: i32) {
        self.value += value;
    }
    // sum all the children values
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for child in &self.children {
            sum += child.value;
        }
        sum+self.value
    }
}
#[test]
// test sum node
fn test_sum() {
    let mut node = Node::new("test", None, 1);
    node.add_value(2);
    assert_eq!(node.sum(), 3);
}
// test add child
#[test]
fn test_add_child() {
    let mut node = Node::new("test", None, 1);
    let child = Node::new("child", Some(& mut node), 1);
    node.add_child(&child);
    assert_eq!(node.children.len(), 1);
}



fn main() {
    // Read stdin line by line and detect if it starts with $.
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.starts_with("$") {
            println!("Found a command: {}", line);

        }
    }

}
