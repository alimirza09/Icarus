use html5ever::{LocalName, Namespace};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualName {
    pub prefix: Option<String>,
    pub ns: String,
    pub local: String,
    pub ns_atom: Namespace,
    pub local_atom: LocalName,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: QualName,
    pub value: String,
}

#[derive(Debug)]
pub enum NodeData {
    Document,
    Element {
        name: QualName,
        attrs: Vec<Attribute>,
    },
    Text {
        contents: String,
    },
    Comment {
        contents: String,
    },
    Doctype {
        name: String,
        public_id: String,
        system_id: String,
    },
}

pub struct Node {
    pub data: NodeData,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(data: NodeData) -> Rc<Self> {
        Rc::new(Node {
            data,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    pub fn append_child(parent: &Rc<Node>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }

    pub fn insert_before(parent: &Rc<Node>, child: Rc<Node>, reference: &Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        let mut children = parent.children.borrow_mut();

        if let Some(pos) = children.iter().position(|n| Rc::ptr_eq(n, reference)) {
            children.insert(pos, child);
        } else {
            children.push(child);
        }
    }

    pub fn remove_child(parent: &Rc<Node>, child: &Rc<Node>) {
        let mut children = parent.children.borrow_mut();
        children.retain(|n| !Rc::ptr_eq(n, child));
    }

    pub fn element_name(&self) -> Option<&str> {
        match &self.data {
            NodeData::Element { name, .. } => Some(&name.local),
            _ => None,
        }
    }

    pub fn text_content(&self) -> Option<&str> {
        match &self.data {
            NodeData::Text { contents } => Some(contents),
            _ => None,
        }
    }

    pub fn get_text_content(&self) -> String {
        let mut text = String::new();
        self.collect_text(&mut text);
        text
    }

    fn collect_text(&self, buffer: &mut String) {
        match &self.data {
            NodeData::Text { contents } => buffer.push_str(contents),
            _ => {
                for child in self.children.borrow().iter() {
                    child.collect_text(buffer);
                }
            }
        }
    }

    pub fn walk<F>(&self, visitor: &mut F)
    where
        F: FnMut(&Node),
    {
        visitor(self);
        for child in self.children.borrow().iter() {
            child.walk(visitor);
        }
    }
}

pub struct Document {
    pub root: Rc<Node>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            root: Node::new(NodeData::Document),
        }
    }

    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<Rc<Node>> {
        let mut results = Vec::new();
        self.collect_elements_by_tag_name(&self.root, tag_name, &mut results);
        results
    }

    fn collect_elements_by_tag_name(
        &self,
        node: &Rc<Node>,
        tag_name: &str,
        results: &mut Vec<Rc<Node>>,
    ) {
        if let Some(name) = node.element_name() {
            if name.eq_ignore_ascii_case(tag_name) {
                results.push(Rc::clone(node));
            }
        }

        for child in node.children.borrow().iter() {
            self.collect_elements_by_tag_name(child, tag_name, results);
        }
    }

    pub fn print_tree(&self) {
        self.print_node(&self.root, 0);
    }

    fn print_node(&self, node: &Rc<Node>, depth: usize) {
        let indent = "  ".repeat(depth);
        match &node.data {
            NodeData::Document => println!("{}Document", indent),
            NodeData::Element { name, .. } => println!("{}Element: {}", indent, name.local),
            NodeData::Text { contents } => {
                let trimmed = contents.trim();
                if !trimmed.is_empty() {
                    println!("{}Text: {:?}", indent, trimmed);
                }
            }
            NodeData::Comment { .. } => println!("{}Comment", indent),
            NodeData::Doctype { name, .. } => println!("{}Doctype: {}", indent, name),
        }

        for child in node.children.borrow().iter() {
            self.print_node(child, depth + 1);
        }
    }
}
