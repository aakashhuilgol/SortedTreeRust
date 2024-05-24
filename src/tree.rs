use std::fmt;

#[derive(Debug, Clone)]
pub struct Data {
    pub name: String,
    pub age: i32,
}

#[derive(Debug, Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: Box<Data>,
}

pub struct SortedContainer {
    root: Option<Box<Node>>,
}

impl Node {
    fn new(data: Data) -> Node {
        return Node {
            left: None,
            right: None,
            data: Box::new(data),
        };
    }

    fn contains(&self, data: &Data) -> bool {
        if self.data.age == data.age && self.data.name == data.name {
            return true;
        }

        if data.age <= self.data.age {
            match &self.left {
                Some(n) => return n.contains(data),
                None => return false,
            }
        } else {
            match &self.right {
                Some(n) => return n.contains(data),
                None => return false,
            }
        }
    }

    fn insert(&mut self, data: &Data) {
        if data.age <= self.data.age {
            match &mut self.left {
                Some(n) => n.insert(data),
                None => {
                    self.left = Some(Box::new(Node::new(data.clone())));
                }
            }
        } else {
            match &mut self.right {
                Some(n) => n.insert(data),
                None => {
                    self.right = Some(Box::new(Node::new(data.clone())));
                }
            }
        }
    }

    fn find_by_data<'a>(
        node_o: &'a mut Option<Box<Node>>,
        data: &Data,
    ) -> &'a mut Option<Box<Node>> {
        if let Some(node) = node_o.as_ref() {
            if node.data.age == data.age && node.data.name == data.name {
                return node_o;
            }

            if data.age <= node.data.age {
                if let Some(n) = node_o {
                    return Node::find_by_data(&mut n.left, data);
                }
            } else {
                if let Some(n) = node_o {
                    return Node::find_by_data(&mut n.right, data);
                }
            }
        }

        return node_o;
    }

    fn get_min_child(&mut self) -> Option<Box<Node>> {
        match &mut self.left {
            Some(n) => return n.get_min_child(),
            None => return None,
        }
    }

    fn delete(node_o: &mut Option<Box<Node>>) -> Option<Box<Data>> {
        if let Some(node) = node_o {
            let left = node.left.take();
            let right = node.right.take();
            let data = node.data.clone();

            let old_data = Some(data);

            match (left, right) {
                (Some(l), Some(mut r)) => {
                    let mut min = r.get_min_child();
                    let min_data = Node::delete(&mut min);

                    match min_data {
                        Some(data) => {
                            node.data = data;
                        }
                        None => {
                            r.left = Some(l);
                            *node_o = Some(r);
                        }
                    }
                }
                (Some(l), None) => {
                    *node_o = Some(l);
                }
                (None, Some(r)) => {
                    *node_o = Some(r);
                }
                (None, None) => {
                    *node_o = None;
                }
            }

            return old_data;
        }

        return None;
    }
}

impl SortedContainer {
    pub fn new() -> SortedContainer {
        return SortedContainer { root: None };
    }

    pub fn insert_node(&mut self, data: Data) {
        match &mut self.root {
            Some(root) => root.insert(&data),
            None => self.root = Some(Box::new(Node::new(data))),
        }
    }

    pub fn contains(&self, data: &Data) -> bool {
        match &self.root {
            Some(root) => return root.contains(&data),
            None => return false,
        }
    }

    pub fn reset(&mut self) {
        self.root = None;
    }

    pub fn delete(&mut self, data: &Data) {
        let to_delete = Node::find_by_data(&mut self.root, data);

        Node::delete(to_delete);
    }
}

impl fmt::Display for SortedContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.root {
            Some(root) => {
                root.fmt(f).ok();
                return write!(f, "");
            }
            None => write!(f, "null"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{{\"{}\":\"{}\"}},", self.data.age, self.data.name).ok();

        match &self.left {
            Some(node) => {
                node.fmt(f).ok();
            }
            None => {
                write!(f, "null").ok();
            }
        }
        write!(f, ",").ok();
        match &self.right {
            Some(node) => {
                node.fmt(f).ok();
            }
            None => {
                write!(f, "null").ok();
            }
        }
        return write!(f, "]");
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::Data;
    use crate::tree::SortedContainer;

    #[test]
    fn adds_nodes() {
        let mut tree = SortedContainer::new();

        tree.insert_node(Data {
            age: 22,
            name: "Biba".to_string(),
        });
        tree.insert_node(Data {
            age: 29,
            name: "Boba".to_string(),
        });

        let root = tree.root.unwrap();

        assert_eq!(root.data.name, "Biba");
        assert_eq!(root.data.age, 22);

        let node = root.right.unwrap();

        assert_eq!(node.data.name, "Boba");
        assert_eq!(node.data.age, 29);
    }

    #[test]
    fn contains_nodes() {
        let mut tree = SortedContainer::new();

        tree.insert_node(Data {
            age: 22,
            name: "Biba".to_string(),
        });
        tree.insert_node(Data {
            age: 29,
            name: "Boba".to_string(),
        });

        assert_eq!(
            tree.contains(&Data {
                age: 22,
                name: "Biba".to_string()
            }),
            true
        );
        assert_eq!(
            tree.contains(&Data {
                age: 29,
                name: "Boba".to_string()
            }),
            true
        );
        assert_eq!(
            tree.contains(&Data {
                age: 22,
                name: "Boba".to_string()
            }),
            false
        );
        assert_eq!(
            tree.contains(&Data {
                age: 29,
                name: "Biba".to_string()
            }),
            false
        );
    }

    #[test]
    fn tree_reset() {
        let mut tree = SortedContainer::new();

        tree.insert_node(Data {
            age: 22,
            name: "Biba".to_string(),
        });
        tree.insert_node(Data {
            age: 29,
            name: "Boba".to_string(),
        });

        tree.reset();

        match &tree.root {
            Some(_node) => {
                assert!(false)
            }
            None => {
                assert!(true)
            }
        }
    }

    #[test]
    fn delete_nodes() {
        let mut tree = SortedContainer::new();

        let biba = Data {
            age: 22,
            name: "Biba".to_string(),
        };
        let boba = Data {
            age: 29,
            name: "Boba".to_string(),
        };

        tree.insert_node(biba.clone());
        tree.insert_node(boba.clone());

        assert_eq!(tree.contains(&biba), true);
        assert_eq!(tree.contains(&boba), true);

        tree.delete(&biba);

        assert_eq!(tree.contains(&biba), false);
        assert_eq!(tree.contains(&boba), true);

        tree.delete(&biba);

        tree.delete(&boba);

        assert_eq!(tree.contains(&boba), false);
    }

    #[test]
    fn delete_nodes2() {
        let mut tree = SortedContainer::new();

        let a10 = Data {
            age: 10,
            name: "a".to_string(),
        };

        let a5 = Data {
            age: 5,
            name: "a".to_string(),
        };

        let a100 = Data {
            age: 100,
            name: "a".to_string(),
        };
        
        let a150 = Data {
            age: 150,
            name: "a".to_string(),
        };

        let a125 = Data {
            age: 125,
            name: "a".to_string(),
        };

        let a130 = Data {
            age: 130,
            name: "a".to_string(),
        };

        let a175 = Data {
            age: 175,
            name: "a".to_string(),
        };

        let a200 = Data {
            age: 200,
            name: "a".to_string(),
        };

        tree.insert_node(a10.clone());
        tree.insert_node(a5.clone());
        tree.insert_node(a100.clone());
        tree.insert_node(a150.clone());
        tree.insert_node(a125.clone());
        tree.insert_node(a130.clone());
        tree.insert_node(a175.clone());
        tree.insert_node(a200.clone());

        assert_eq!(tree.contains(&a10), true);
        assert_eq!(tree.contains(&a5), true);
        assert_eq!(tree.contains(&a100), true);
        assert_eq!(tree.contains(&a150), true);
        assert_eq!(tree.contains(&a125), true);
        assert_eq!(tree.contains(&a130), true);
        assert_eq!(tree.contains(&a175), true);
        assert_eq!(tree.contains(&a200), true);

        tree.delete(&a150);

        assert_eq!(tree.contains(&a10), true);
        assert_eq!(tree.contains(&a5), true);
        assert_eq!(tree.contains(&a100), true);
        assert_eq!(tree.contains(&a150), false);
        assert_eq!(tree.contains(&a125), true);
        assert_eq!(tree.contains(&a130), true);
        assert_eq!(tree.contains(&a175), true);
        assert_eq!(tree.contains(&a200), true);

        tree.delete(&a10);

        assert_eq!(tree.contains(&a10), false);
        assert_eq!(tree.contains(&a5), true);
        assert_eq!(tree.contains(&a100), true);
        assert_eq!(tree.contains(&a150), false);
        assert_eq!(tree.contains(&a125), true);
        assert_eq!(tree.contains(&a130), true);
        assert_eq!(tree.contains(&a175), true);
        assert_eq!(tree.contains(&a200), true);

    }
}
