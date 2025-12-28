use crate::dom::{Node, NodeData};
use raylib::prelude::*;
use std::rc::Rc;

const BASE_FONT_SIZE: f32 = 26.0;
const H1_SIZE: f32 = 56.0;
const H2_SIZE: f32 = 44.0;
const H3_SIZE: f32 = 36.0;
const H4_SIZE: f32 = 30.0;
const CODE_FONT_SIZE: f32 = 26.0;

pub struct RenderContext<'a> {
    pub current_y: i32,
    pub current_x: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub scroll_offset: i32,
    pub content_height: i32,
    pub regular_font: &'a Font,
    pub bold_font: &'a Font,
    pub italic_font: Option<&'a Font>,
    pub default_color: Color,
    pub max_width: i32,
    pub font_size: f32,
    pub heading_sizes: [f32; 4],
    pub screen_width: i32,
    pub screen_height: i32,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        regular_font: &'a Font,
        bold_font: &'a Font,
        screen_width: i32,
        screen_height: i32,
    ) -> Self {
        RenderContext {
            current_y: 0,
            current_x: 10,
            margin_left: 10,
            margin_right: 10,
            scroll_offset: 0,
            content_height: 0,
            regular_font,
            bold_font,
            italic_font: None,
            default_color: Color::new(0, 255, 0, 255),
            max_width: screen_width - 20,
            font_size: BASE_FONT_SIZE,
            heading_sizes: [H1_SIZE, H2_SIZE, H3_SIZE, H4_SIZE],
            screen_width,
            screen_height,
        }
    }

    pub fn with_italic_font(mut self, italic_font: &'a Font) -> Self {
        self.italic_font = Some(italic_font);
        self
    }

    pub fn with_margins(mut self, left: i32, right: i32, top: i32) -> Self {
        self.margin_left = left;
        self.margin_right = right;
        self.current_y = top;
        self.current_x = left;
        self.max_width = self.screen_width - left - right;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.default_color = color;
        self
    }

    pub fn with_scroll_offset(mut self, offset: i32) -> Self {
        self.scroll_offset = offset;
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    fn add_spacing(&mut self, pixels: i32) {
        self.current_y += pixels;
    }

    fn is_visible(&self, y: i32, height: i32) -> bool {
        let screen_y = y - self.scroll_offset;
        screen_y + height >= 0 && screen_y < self.screen_height
    }

    fn measure_text(&self, text: &str, font: &Font, size: f32) -> Vector2 {
        let spacing = size / 10.0;
        font.measure_text(text, size, spacing)
    }

    fn render_text_wrapped(
        &mut self,
        handle: &mut RaylibDrawHandle,
        text: &str,
        font: &Font,
        size: f32,
        color: Color,
    ) {
        let text = text.trim();
        if text.is_empty() {
            return;
        }

        let words: Vec<&str> = text.split_whitespace().collect();
        let mut line = String::new();
        let spacing = size / 10.0;
        let wrap_guard = size as i32;

        for word in words {
            let test_line = if line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", line, word)
            };

            let test_width = self.measure_text(&test_line, font, size).x as i32;

            if test_width + wrap_guard > self.max_width && !line.is_empty() {
                let screen_y = self.current_y - self.scroll_offset;
                let line_height = self.measure_text(&line, font, size).y as i32;

                if self.is_visible(self.current_y, line_height) {
                    handle.draw_text_ex(
                        font,
                        &line,
                        Vector2::new(self.margin_left as f32, screen_y as f32),
                        size,
                        spacing,
                        color,
                    );
                }

                self.current_y += line_height + 2;
                line = word.to_string();
            } else {
                line = test_line;
            }
        }

        if !line.is_empty() {
            let screen_y = self.current_y - self.scroll_offset;
            let line_height = self.measure_text(&line, font, size).y as i32;

            if self.is_visible(self.current_y, line_height) {
                handle.draw_text_ex(
                    font,
                    &line,
                    Vector2::new(self.margin_left as f32, screen_y as f32),
                    size,
                    spacing,
                    color,
                );
            }

            self.current_y += line_height + 4;
        }
    }

    fn should_skip_node(&self, node: &Node) -> bool {
        if let Some(name) = node.element_name() {
            matches!(name, "title" | "script" | "style" | "head")
        } else {
            false
        }
    }

    fn render_node(&mut self, handle: &mut RaylibDrawHandle, node: &Rc<Node>) {
        if self.should_skip_node(node) {
            return;
        }

        match &node.data {
            NodeData::Element { name, .. } => match name.local.as_str() {
                "h1" => {
                    self.add_spacing(20);
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.bold_font,
                        self.heading_sizes[0],
                        Color::new(100, 200, 100, 255),
                    );
                    self.add_spacing(10);
                }
                "h2" => {
                    self.add_spacing(16);
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.bold_font,
                        self.heading_sizes[1],
                        Color::new(120, 200, 120, 255),
                    );
                    self.add_spacing(8);
                }
                "h3" => {
                    self.add_spacing(12);
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.bold_font,
                        self.heading_sizes[2],
                        Color::new(140, 200, 140, 255),
                    );
                    self.add_spacing(6);
                }
                "h4" => {
                    self.add_spacing(8);
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.bold_font,
                        self.heading_sizes[3],
                        self.default_color,
                    );
                    self.add_spacing(4);
                }
                "p" | "div" => {
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.regular_font,
                        self.font_size,
                        self.default_color,
                    );
                    self.add_spacing(16);
                }
                "strong" | "b" => {
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.bold_font,
                        self.font_size,
                        self.default_color,
                    );
                }
                "em" | "i" => {
                    let font = self.italic_font.unwrap_or(self.regular_font);
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        font,
                        self.font_size,
                        self.default_color,
                    );
                }
                "ul" | "ol" => {
                    self.add_spacing(8);
                    let old_x = self.current_x;
                    self.current_x += 40;
                    for child in node.children.borrow().iter() {
                        self.render_node(handle, child);
                    }
                    self.current_x = old_x;
                    self.add_spacing(8);
                }
                "li" => {
                    let text = format!("• {}", node.get_text_content().trim());
                    self.render_text_wrapped(
                        handle,
                        &text,
                        self.regular_font,
                        self.font_size,
                        self.default_color,
                    );
                }
                "br" => {
                    let line_height =
                        self.measure_text("A", self.regular_font, self.font_size).y as i32;
                    self.add_spacing(line_height);
                }
                "hr" => {
                    self.add_spacing(16);
                    let screen_y = self.current_y - self.scroll_offset;
                    if self.is_visible(self.current_y, 1) {
                        handle.draw_line(
                            self.margin_left,
                            screen_y,
                            self.screen_width - self.margin_right,
                            screen_y,
                            self.default_color,
                        );
                    }
                    self.add_spacing(16);
                }
                "blockquote" => {
                    self.add_spacing(12);
                    let old_x = self.current_x;
                    self.current_x += 60;
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.regular_font,
                        self.font_size.max(BASE_FONT_SIZE),
                        Color::new(150, 150, 150, 255),
                    );
                    self.current_x = old_x;
                    self.add_spacing(12);
                }
                "code" | "pre" => {
                    self.add_spacing(8);
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.regular_font,
                        CODE_FONT_SIZE,
                        Color::new(200, 150, 100, 255),
                    );
                    self.add_spacing(8);
                }
                "a" => {
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        self.regular_font,
                        self.font_size,
                        Color::new(100, 150, 255, 255),
                    );
                }
                _ => {
                    for child in node.children.borrow().iter() {
                        self.render_node(handle, child);
                    }
                }
            },
            NodeData::Text { contents } => {
                if !contents.trim().is_empty() {
                    self.render_text_wrapped(
                        handle,
                        contents,
                        self.regular_font,
                        self.font_size,
                        self.default_color,
                    );
                }
            }
            _ => {
                for child in node.children.borrow().iter() {
                    self.render_node(handle, child);
                }
            }
        }
    }
}

pub struct ScrollableDocument {
    pub scroll_offset: i32,
    pub content_height: i32,
    pub viewport_height: i32,
}

impl ScrollableDocument {
    pub fn new(viewport_height: u32) -> Self {
        Self {
            scroll_offset: 0,
            content_height: 0,
            viewport_height: viewport_height as i32,
        }
    }

    pub fn scroll_up(&mut self, amount: i32) {
        self.scroll_offset = (self.scroll_offset - amount).max(0);
    }

    pub fn scroll_down(&mut self, amount: i32) {
        let max_scroll = (self.content_height - self.viewport_height).max(0);
        self.scroll_offset = (self.scroll_offset + amount).min(max_scroll);
    }

    pub fn can_scroll_up(&self) -> bool {
        self.scroll_offset > 0
    }

    pub fn can_scroll_down(&self) -> bool {
        self.scroll_offset < (self.content_height - self.viewport_height).max(0)
    }
}

pub fn render_document_scrollable<'a>(
    handle: &mut RaylibDrawHandle,
    document: &crate::dom::Document,
    regular_font: &'a Font,
    bold_font: &'a Font,
    scroll_state: &mut ScrollableDocument,
    screen_width: i32,
    screen_height: i32,
) {
    let mut render_ctx = RenderContext::new(regular_font, bold_font, screen_width, screen_height)
        .with_margins(15, 15, 15)
        .with_color(Color::new(200, 255, 200, 255))
        .with_scroll_offset(scroll_state.scroll_offset)
        .with_font_size(BASE_FONT_SIZE);

    render_ctx.render_node(handle, &document.root);
    scroll_state.content_height = render_ctx.current_y;
}
