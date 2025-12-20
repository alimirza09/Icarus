use crate::dom::{Node, NodeData};
use sight::ttf::TtfFont;
use sight::{Color, Sight};
use std::rc::Rc;

pub struct RenderContext<'a> {
    pub sight: &'a mut Sight,
    pub current_y: i32,
    pub current_x: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub scroll_offset: i32,
    pub content_height: i32,
    pub regular_font: &'a TtfFont<'a>,
    pub bold_font: &'a TtfFont<'a>,
    pub italic_font: Option<&'a TtfFont<'a>>,
    pub default_color: Color,
    pub max_width: i32,
    pub font_size: f32,
    pub heading_sizes: [f32; 4],
}

impl<'a> RenderContext<'a> {
    pub fn new(sight: &'a mut Sight, regular_font: &'a TtfFont, bold_font: &'a TtfFont) -> Self {
        let width = sight.width() as i32;
        RenderContext {
            sight,
            current_y: 0,
            current_x: 10,
            margin_left: 10,
            margin_right: 10,
            scroll_offset: 0,
            content_height: 0,
            regular_font,
            bold_font,
            italic_font: None,
            default_color: Color::GREEN,
            max_width: width - 20,
            font_size: 13.0,
            heading_sizes: [28.0, 22.0, 18.0, 15.0],
        }
    }

    pub fn with_italic_font(mut self, italic_font: &'a TtfFont) -> Self {
        self.italic_font = Some(italic_font);
        self
    }

    pub fn with_margins(mut self, left: i32, right: i32, top: i32) -> Self {
        self.margin_left = left;
        self.margin_right = right;
        self.current_y = top;
        self.current_x = left;
        self.max_width = self.sight.width() as i32 - left - right;
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
        let screen_height = self.sight.height() as i32;
        screen_y + height >= 0 && screen_y < screen_height
    }

    fn render_text_wrapped(&mut self, text: &str, font: &TtfFont, size: f32, color: Color) {
        let text = text.trim();
        if text.is_empty() {
            return;
        }

        let words: Vec<&str> = text.split_whitespace().collect();
        let mut line = String::new();

        for word in words {
            let test_line = if line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", line, word)
            };

            let test_width = font.text_width(&test_line, size) as i32;

            if test_width > self.max_width && !line.is_empty() {
                let screen_y = self.current_y - self.scroll_offset;
                if self.is_visible(self.current_y, font.text_height(size) as i32) {
                    self.sight.draw_text_antialiased_ttf::<TtfFont>(
                        font,
                        &line,
                        self.current_x,
                        screen_y,
                        size,
                        color,
                    );
                }
                self.current_y += font.text_height(size) as i32 + 2;
                line = word.to_string();
            } else {
                line = test_line;
            }
        }

        if !line.is_empty() {
            let screen_y = self.current_y - self.scroll_offset;
            if self.is_visible(self.current_y, font.text_height(size) as i32) {
                self.sight.draw_text_antialiased_ttf::<TtfFont>(
                    font,
                    &line,
                    self.current_x,
                    screen_y,
                    size,
                    color,
                );
            }
            self.current_y += font.text_height(size) as i32 + 4;
        }
    }

    fn should_skip_node(&self, node: &Node) -> bool {
        if let Some(name) = node.element_name() {
            match name {
                "title" | "script" | "style" | "head" => return true,
                _ => {}
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
                        self.add_spacing(10);
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.bold_font,
                            self.heading_sizes[0],
                            Color::rgb(100, 200, 100),
                        );
                        self.add_spacing(5);
                    }
                    "h2" => {
                        self.add_spacing(8);
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.bold_font,
                            self.heading_sizes[1],
                            Color::rgb(120, 200, 120),
                        );
                        self.add_spacing(4);
                    }
                    "h3" => {
                        self.add_spacing(6);
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.bold_font,
                            self.heading_sizes[2],
                            Color::rgb(140, 200, 140),
                        );
                        self.add_spacing(3);
                    }
                    "h4" => {
                        self.add_spacing(4);
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.bold_font,
                            self.heading_sizes[3],
                            self.default_color,
                        );
                        self.add_spacing(2);
                    }
                    "p" => {
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.regular_font,
                            self.font_size,
                            self.default_color,
                        );
                        self.add_spacing(8);
                    }
                    "div" => {
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.regular_font,
                            self.font_size,
                            self.default_color,
                        );
                        self.add_spacing(4);
                    }
                    "strong" | "b" => {
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.bold_font,
                            self.font_size,
                            self.default_color,
                        );
                    }
                    "em" | "i" => {
                        let text = node.get_text_content();
                        let font = self.italic_font.unwrap_or(self.regular_font);
                        self.render_text_wrapped(&text, font, self.font_size, self.default_color);
                    }
                    "ul" | "ol" => {
                        self.add_spacing(4);
                        let old_x = self.current_x;
                        self.current_x += 20;

                        for child in node.children.borrow().iter() {
                            self.render_node(child);
                        }

                        self.current_x = old_x;
                        self.add_spacing(4);
                    }
                    "li" => {
                        let text = node.get_text_content();
                        let bullet = "• ";
                        let full_text = format!("{}{}", bullet, text.trim());
                        self.render_text_wrapped(
                            &full_text,
                            self.regular_font,
                            self.font_size,
                            self.default_color,
                        );
                    }
                    "br" => {
                        self.add_spacing(self.regular_font.text_height(self.font_size) as i32);
                    }
                    "hr" => {
                        self.add_spacing(8);
                        let screen_y = self.current_y - self.scroll_offset;
                        if self.is_visible(self.current_y, 1) {
                            for x in
                                self.margin_left..(self.sight.width() as i32 - self.margin_right)
                            {
                                self.sight.put_pixel(x, screen_y, self.default_color);
                            }
                        }
                        self.add_spacing(8);
                    }
                    "blockquote" => {
                        self.add_spacing(6);
                        let old_x = self.current_x;
                        self.current_x += 30;

                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.regular_font,
                            self.font_size.max(13.0),
                            Color::rgb(150, 150, 150),
                        );

                        self.current_x = old_x;
                        self.add_spacing(6);
                    }
                    "code" | "pre" => {
                        self.add_spacing(4);
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.regular_font,
                            self.font_size.max(13.0),
                            Color::rgb(200, 150, 100),
                        );
                        self.add_spacing(4);
                    }
                    "a" => {
                        let text = node.get_text_content();
                        self.render_text_wrapped(
                            &text,
                            self.regular_font,
                            self.font_size,
                            Color::rgb(100, 150, 255),
                        );
                    }
                    _ => {
                        for child in node.children.borrow().iter() {
                            self.render_node(child);
                        }
                    }
                }
            }
            NodeData::Text { contents } => {
                if !contents.trim().is_empty() {
                    self.render_text_wrapped(
                        contents,
                        self.regular_font,
                        self.font_size,
                        self.default_color,
                    );
                }
            }
            NodeData::Document | NodeData::Doctype { .. } | NodeData::Comment { .. } => {
                for child in node.children.borrow().iter() {
                    self.render_node(child);
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

pub fn render_document_scrollable(
    sight: &mut Sight,
    document: &crate::dom::Document,
    regular_font: &TtfFont,
    bold_font: &TtfFont,
    scroll_state: &mut ScrollableDocument,
) {
    let mut render_ctx = RenderContext::new(sight, regular_font, bold_font)
        .with_margins(15, 15, 15)
        .with_color(Color::rgb(200, 255, 200))
        .with_scroll_offset(scroll_state.scroll_offset)
        .with_font_size(13.0);

    render_ctx.render_node(&document.root);

    scroll_state.content_height = render_ctx.current_y;
}

// Usage example in main.rs:
//
// use icarus::graphics::renderer::{render_document_scrollable, ScrollableDocument};
//
// let mut scroll_state = ScrollableDocument::new(ctx.height());
//
// while ctx.window.is_open() && !ctx.window.is_key_down(minifb::Key::Escape) {
//     // Handle scrolling
//     if ctx.window.is_key_down(minifb::Key::Up) {
//         scroll_state.scroll_up(20);
//     }
//     if ctx.window.is_key_down(minifb::Key::Down) {
//         scroll_state.scroll_down(20);
//     }
//     if ctx.window.is_key_down(minifb::Key::PageUp) {
//         scroll_state.scroll_up(ctx.height() as i32 - 50);
//     }
//     if ctx.window.is_key_down(minifb::Key::PageDown) {
//         scroll_state.scroll_down(ctx.height() as i32 - 50);
//     }
//     if ctx.window.is_key_down(minifb::Key::Home) {
//         scroll_state.scroll_offset = 0;
//     }
//     if ctx.window.is_key_down(minifb::Key::End) {
//         scroll_state.scroll_offset = (scroll_state.content_height - scroll_state.viewport_height).max(0);
//     }
//
//     ctx.clear(Color::BLACK);
//
//     // Set window title from <title> tag
//     let title = document.get_elements_by_tag_name("title");
//     if !title.is_empty() {
//         ctx.window.set_title(&title[0].get_text_content());
//     }
//
//     // Render the document with scrolling
//     render_document_scrollable(&mut ctx, &document, &font, &bold_font, &mut scroll_state);
//
//     ctx.present().expect("Failed");
// }
