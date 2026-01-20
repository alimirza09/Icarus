use crate::css::parser::parse_inline_style;
use crate::css::types::{ComputedStyle, FontStyle, FontWeight};
use crate::dom::Node;
use std::rc::Rc;

pub fn compute_styles(document: &crate::dom::Document) {
    compute_node_styles(&document.root, &ComputedStyle::new());
}

fn compute_node_styles(node: &Rc<Node>, parent_style: &ComputedStyle) {
    let mut style = parent_style.clone();

    if let Some(element_name) = node.element_name() {
        apply_default_element_styles(&mut style, element_name);
    }

    if let Some(style_attr) = node.get_attribute("style") {
        let inline_style = parse_inline_style(&style_attr);
        style.merge(&inline_style);
    }

    *node.computed_style.borrow_mut() = style.clone();

    for child in node.children.borrow().iter() {
        compute_node_styles(child, &style);
    }
}

fn apply_default_element_styles(style: &mut ComputedStyle, element_name: &str) {
    match element_name {
        "h1" => {
            style.font_size = Some(56.0);
            style.font_weight = Some(FontWeight::Bold);
            style.margin_top = Some(20.0);
            style.margin_bottom = Some(10.0);
        }
        "h2" => {
            style.font_size = Some(44.0);
            style.font_weight = Some(FontWeight::Bold);
            style.margin_top = Some(16.0);
            style.margin_bottom = Some(8.0);
        }
        "h3" => {
            style.font_size = Some(36.0);
            style.font_weight = Some(FontWeight::Bold);
            style.margin_top = Some(12.0);
            style.margin_bottom = Some(6.0);
        }
        "h4" => {
            style.font_size = Some(30.0);
            style.font_weight = Some(FontWeight::Bold);
            style.margin_top = Some(8.0);
            style.margin_bottom = Some(4.0);
        }
        "p" | "div" => {
            style.margin_bottom = Some(16.0);
        }
        "strong" | "b" => {
            style.font_weight = Some(FontWeight::Bold);
        }
        "em" | "i" => {
            style.font_style = Some(FontStyle::Italic);
        }
        "blockquote" => {
            style.margin_top = Some(12.0);
            style.margin_bottom = Some(12.0);
            style.padding_left = Some(60.0);
        }
        "code" | "pre" => {
            style.font_size = Some(26.0);
            style.margin_top = Some(8.0);
            style.margin_bottom = Some(8.0);
        }
        "ul" | "ol" => {
            style.margin_top = Some(8.0);
            style.margin_bottom = Some(8.0);
            style.padding_left = Some(40.0);
        }
        _ => {}
    }
}
