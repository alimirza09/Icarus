use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute as Html5Attribute, ExpandedName, QualName as Html5QualName};
use html5ever::{ParseOpts, parse_document};
use std::cell::RefCell;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;

use crate::dom::{Attribute, Document, Node, NodeData, QualName};

pub struct DomSink {
    document: RefCell<Document>,
    quirks_mode: RefCell<QuirksMode>,
}

impl DomSink {
    pub fn new() -> Self {
        DomSink {
            document: RefCell::new(Document::new()),
            quirks_mode: RefCell::new(QuirksMode::NoQuirks),
        }
    }

    fn convert_qualname(name: &Html5QualName) -> QualName {
        QualName {
            prefix: name.prefix.as_ref().map(|p| p.to_string()),
            ns: name.ns.to_string(),
            local: name.local.to_string(),
            ns_atom: name.ns.clone(),
            local_atom: name.local.clone(),
        }
    }

    fn convert_attrs(attrs: &[Html5Attribute]) -> Vec<Attribute> {
        attrs
            .iter()
            .map(|attr| Attribute {
                name: Self::convert_qualname(&attr.name),
                value: attr.value.to_string(),
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct Handle(Rc<Node>);

impl TreeSink for DomSink {
    type Handle = Handle;
    type Output = Document;
    type ElemName<'a> = ExpandedName<'a>;

    fn finish(self) -> Self::Output {
        self.document.into_inner()
    }

    fn parse_error(&self, _msg: std::borrow::Cow<'static, str>) {}

    fn get_document(&self) -> Self::Handle {
        let doc = self.document.borrow();
        Handle(Rc::clone(&doc.root))
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> Self::ElemName<'a> {
        match &target.0.data {
            NodeData::Element { name, .. } => ExpandedName {
                ns: &name.ns_atom,
                local: &name.local_atom,
            },
            _ => panic!("elem_name called on non-element"),
        }
    }

    fn create_element(
        &self,
        name: Html5QualName,
        attrs: Vec<Html5Attribute>,
        _flags: ElementFlags,
    ) -> Self::Handle {
        Handle(Node::new(NodeData::Element {
            name: Self::convert_qualname(&name),
            attrs: Self::convert_attrs(&attrs),
        }))
    }

    fn create_comment(&self, text: html5ever::tendril::StrTendril) -> Self::Handle {
        Handle(Node::new(NodeData::Comment {
            contents: text.to_string(),
        }))
    }

    fn create_pi(
        &self,
        _target: html5ever::tendril::StrTendril,
        _data: html5ever::tendril::StrTendril,
    ) -> Self::Handle {
        Handle(Node::new(NodeData::Comment {
            contents: String::new(),
        }))
    }

    fn append(&self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            NodeOrText::AppendNode(node) => {
                Node::append_child(&parent.0, Rc::clone(&node.0));
            }
            NodeOrText::AppendText(text) => {
                let mut children = parent.0.children.borrow_mut();
                if let Some(last) = children.last() {
                    if let NodeData::Text { contents } = &last.data {
                        let mut new_contents = contents.clone();
                        new_contents.push_str(&text);

                        children.pop();
                        drop(children);

                        let new_text_node = Node::new(NodeData::Text {
                            contents: new_contents,
                        });
                        Node::append_child(&parent.0, new_text_node);
                        return;
                    }
                }
                drop(children);

                let text_node = Node::new(NodeData::Text {
                    contents: text.to_string(),
                });
                Node::append_child(&parent.0, text_node);
            }
        }
    }

    fn append_based_on_parent_node(
        &self,
        element: &Self::Handle,
        _prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        self.append(element, child);
    }

    fn append_doctype_to_document(
        &self,
        name: html5ever::tendril::StrTendril,
        public_id: html5ever::tendril::StrTendril,
        system_id: html5ever::tendril::StrTendril,
    ) {
        let doc = self.document.borrow();
        let doctype = Node::new(NodeData::Doctype {
            name: name.to_string(),
            public_id: public_id.to_string(),
            system_id: system_id.to_string(),
        });
        Node::append_child(&doc.root, doctype);
    }

    fn get_template_contents(&self, target: &Self::Handle) -> Self::Handle {
        target.clone()
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        Rc::ptr_eq(&x.0, &y.0)
    }

    fn set_quirks_mode(&self, mode: QuirksMode) {
        *self.quirks_mode.borrow_mut() = mode;
    }

    fn append_before_sibling(&self, sibling: &Self::Handle, new_node: NodeOrText<Self::Handle>) {
        let parent = sibling
            .0
            .parent
            .borrow()
            .upgrade()
            .expect("append_before_sibling: no parent");

        match new_node {
            NodeOrText::AppendNode(node) => {
                Node::insert_before(&parent, Rc::clone(&node.0), &sibling.0);
            }
            NodeOrText::AppendText(text) => {
                let text_node = Node::new(NodeData::Text {
                    contents: text.to_string(),
                });
                Node::insert_before(&parent, text_node, &sibling.0);
            }
        }
    }

    fn add_attrs_if_missing(&self, _target: &Self::Handle, _attrs: Vec<Html5Attribute>) {}

    fn remove_from_parent(&self, target: &Self::Handle) {
        if let Some(parent) = target.0.parent.borrow().upgrade() {
            Node::remove_child(&parent, &target.0);
        }
    }

    fn reparent_children(&self, node: &Self::Handle, new_parent: &Self::Handle) {
        let children: Vec<_> = node
            .0
            .children
            .borrow()
            .iter()
            .map(|c| Rc::clone(c))
            .collect();

        node.0.children.borrow_mut().clear();

        for child in children {
            Node::append_child(&new_parent.0, child);
        }
    }
}

pub fn parse_html(html: &str) -> Document {
    let sink = DomSink::new();
    parse_document(sink, ParseOpts::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap()
}
