use crate::dom::{Node, NodeData};
use sight::ttf::TtfFont;
use sight::{Color, Sight};
use std::rc::Rc;

pub struct RenderContext<'a> {
    pub ctx: &'a mut Sight,
    pub current_y: i32,
    pub current_x: i32,
    pub line_height: i32,
    pub regular_font: &'a TtfFont<'a>,
    pub bold_font: &'a TtfFont<'a>,
}

impl<'a> RenderContext<'a> {
    pub fn new(ctx: &'a mut Sight, regular_font: &'a TtfFont, bold_font: &'a TtfFont) -> Self {
        RenderContext {
            ctx,
            current_y: 0,
            current_x: 0,
            line_height: 14,
            regular_font,
            bold_font,
        }
    }

    fn render_text(&mut self, text: &str, font_size: f32, font: &TtfFont, color: Color) {
        if !text.trim().is_empty() {
            self.ctx.draw_text_antialiased_ttf::<TtfFont>(
                font,
                text.trim(),
                self.current_x,
                self.current_y,
                font_size,
                color,
            );
            self.current_y += (font_size as i32) + 4;
        }
    }

    fn should_skip_node(&self, node: &Node) -> bool {
        if let Some(name) = node.element_name() {
            if name.eq_ignore_ascii_case("title") {
                return true;
            }
        }
        false
    }

    fn render_node(&mut self, node: &Rc<Node>) {
        if self.should_skip_node(node) {
            return;
        }

        match &node.data {
            NodeData::Element { name, .. } => {
                let tag = name.local.as_str();

                match tag {
                    "h1" => {
                        let text = node.get_text_content();
                        self.render_text(&text, 24.0, self.bold_font, Color::GREEN);
                    }
                    "h2" => {
                        let text = node.get_text_content();
                        self.render_text(&text, 20.0, self.bold_font, Color::GREEN);
                    }
                    "h3" => {
                        let text = node.get_text_content();
                        self.render_text(&text, 16.0, self.bold_font, Color::GREEN);
                    }
                    "p" | "div" => {
                        let text = node.get_text_content();
                        self.render_text(&text, 14.0, self.regular_font, Color::GREEN);
                    }
                    _ => {
                        for child in node.children.borrow().iter() {
                            self.render_node(child);
                        }
                    }
                }
            }
            NodeData::Text { contents } => {
                self.render_text(contents, 11.0, self.regular_font, Color::GREEN);
            }
            NodeData::Document | NodeData::Doctype { .. } | NodeData::Comment { .. } => {
                for child in node.children.borrow().iter() {
                    self.render_node(child);
                }
            }
        }
    }
}

pub fn render_document(
    ctx: &mut Sight,
    document: &crate::dom::Document,
    regular_font: &TtfFont,
    bold_font: &TtfFont,
) {
    let mut render_ctx = RenderContext::new(ctx, regular_font, bold_font);
    render_ctx.render_node(&document.root);
}
