use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Error, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type NodePt<'a> = Rc<RefCell<Node<'a>>>;

macro_rules! new_node_ptr {
    ($data:expr) => {
        Rc::new(RefCell::new(Node::new($data)))
    }
}

macro_rules! new_node {
    ($data:expr) => {
        Node::new(Some($data))
    }
}

#[derive(Debug)]
struct Node<'a> {
    data: Option<&'a str>,
    next: Option<NodePt<'a>>,
}

impl<'a> Hash for Node<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<'a> Node<'a> {
    fn new(data: Option<&'a str>) -> Self {
        Self {
            data,
            next: None,
        }
    }
}

#[derive(Debug)]
struct HashChainNode<'a> {
    radio_expand: f64,
    ratio_shrink: f64,
    size: usize,
    count: usize,
    table: HashMap<usize, NodePt<'a>>,
}


impl<'a> Default for HashChainNode<'a> {
    fn default() -> HashChainNode<'a> {
        Self {
            size: 1,
            count: 0,
            radio_expand: 1.25,
            ratio_shrink: 0.5,
            table: HashMap::new(),
        }
    }
}


impl<'a> HashChainNode<'a> {
    pub fn insert(&mut self, node: Node<'a>) {
        let index = self.get_table_index(&node);
        HashChainNode::insert_table(&mut self.table, index, Rc::new(RefCell::new(node)));
        self.table_doubling();
        self.count += 1;
    }

     fn rehash(&mut self, head: &NodePt<'a>, table: &mut HashMap<usize, NodePt<'a>>) {
        let mut pt = head.clone();
        loop {
            let index = self.get_table_index(&RefCell::borrow(&pt));
            let data = RefCell::borrow(&pt).data.clone();
            match data {
                Some(_d) => {
                    HashChainNode::insert_table(table, index, new_node_ptr!(RefCell::borrow(&pt).data.clone()));
                },
                None => {

                }
            }
            match &RefCell::borrow(&pt.clone()).next {
                Some(i) => {
                    pt = i.clone();
                }
                None => {
                    break;
                }
            };
            self.count += 1;
        }
    }

    fn insert_table(table: &mut HashMap<usize, NodePt<'a>>, index: usize, node: NodePt<'a>) {
        if !table.contains_key(&index) {
            table.insert(index, new_node_ptr!(None));
        }
        let entry = table.get(&index).unwrap();
        let mut head = entry.clone();
        while let Some(i) = &RefCell::borrow(&head.clone()).next {
            head = i.clone();
        }
        head.borrow_mut().next = Some(node);
    }

    pub fn delete(&mut self, node: &Node) {
        let index = self.get_table_index(node);
        let mut entry = match self.table.get(&index) {
            Some(e) => {
                Some((*e).clone())
            }
            None => {
                None
            }
        };
        loop {
            match entry {
                Some(e) => {
                    let next_node = RefCell::borrow(&e).next.clone();
                    match next_node {
                        Some(n) => {
                            if RefCell::borrow(&n).data.unwrap() == node.data.unwrap() {
                                RefCell::borrow_mut(&e.clone()).next = match &RefCell::borrow(&n).next {
                                    Some(n) => {
                                        Some((*n).clone())
                                    }
                                    None => {
                                        None
                                    }
                                };
                                break;
                            }
                            entry = Some(n.clone());
                        }
                        None => {
                            break;
                        }
                    }
                }
                None => { break; }
            }
        }
        self.count -= 1;
        self.table_shrinking();
    }

    fn table_doubling(&mut self) {
        let load_factor = self.get_load_factor();
        if load_factor <= self.radio_expand {
            return;
        }
        self.size *= 2;
        let mut table = HashMap::<usize, NodePt<'a>>::new();
        for v in self.table.clone().values() {
            self.rehash(v, &mut table);
        }
        self.table = table;
    }

    fn table_shrinking(&mut self) {
        let load_factor = self.get_load_factor();
        if load_factor >= self.ratio_shrink {
            return;
        }
        self.size /= 2;
        let mut table = HashMap::<usize, NodePt<'a>>::new();
        for v in self.table.clone().values() {
            self.rehash(v, &mut table);
        }
        self.table = table;
    }

    fn get_load_factor(&self) -> f64 {
        self.count as f64 / self.size as f64
    }

    fn get_node_hash_value(&self, node: &Node) -> u64 {
        let mut s = DefaultHasher::new();
        node.hash(&mut s);
        s.finish()
    }

    fn get_table_index(&self, node: &Node) -> usize {
        let hash = self.get_node_hash_value(node);
        hash as usize % self.size
    }

    pub fn search(&self, node: &Node) -> Option<NodePt<'a>> {
        let index = self.get_table_index(node);
        let entry = self.table.get(&index).unwrap();
        let mut head = entry.clone();
        while let Some(i) = &RefCell::borrow(&head.clone()).next {
            if RefCell::borrow(i).data == node.data {
                return Some((*i).clone());
            }
            head = i.clone();
        }
        None
    }
}

impl<'a> fmt::Display for HashChainNode<'a> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        for (k, v) in self.table.iter() {
            let mut head = v.clone();
            print!("{}:", k);
            loop {
                let next_head = {
                    let node = RefCell::borrow(&head);
                    {
                        match node.data {
                            Some(n) => {
                                print!("{} ", n);
                            }
                            None => {}
                        };
                        match &node.next {
                            Some(next) => {
                                Some(next.clone())
                            }
                            None => { None }
                        }
                    }
                };
                match next_head {
                    Some(n) => {
                        head = n.clone();
                    }
                    None => { break; }
                }
            }
            println!();
        }
        Ok(())
    }
}

fn main() {
    let n1 = new_node!("1");
    let n2 = new_node!("2");
    let n3 = new_node!("3");
    let n4 = new_node!("4");
    let n5 = new_node!("5");

    let n3d = new_node!("3");
    let mut hash_chain: HashChainNode = HashChainNode::default();
    hash_chain.insert(n1);
    hash_chain.insert(n2);
    hash_chain.insert(n3);
    hash_chain.insert(n4);
    hash_chain.insert(n5);
    println!("{}", hash_chain);
    hash_chain.delete(&n3d);
    println!("{}", hash_chain);
}