use crate::css::types::{Display, FontStyle, FontWeight};
use crate::dom::{Node, NodeData};
use raylib::prelude::*;
use std::rc::Rc;

const BASE_FONT_SIZE: f32 = 26.0;

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
            default_color: Color::BLACK,
            max_width: screen_width - 20,
            font_size: BASE_FONT_SIZE,
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
        let spacing = 0.0;
        font.measure_text(text, size, spacing)
    }

    fn render_text_wrapped(
        &mut self,
        handle: &mut RaylibDrawHandle,
        text: &str,
        font: &Font,
        size: f32,
        color: Color,
        bg_color: Option<Color>,
    ) {
        let text = text.trim();
        if text.is_empty() {
            return;
        }

        let words: Vec<&str> = text.split_whitespace().collect();
        let mut line = String::new();
        let spacing = 0.0;
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
                let line_width = self.measure_text(&line, font, size).x as i32;

                if self.is_visible(self.current_y, line_height) {
                    if let Some(bg) = bg_color {
                        handle.draw_rectangle(
                            self.current_x,
                            screen_y,
                            line_width,
                            line_height,
                            bg,
                        );
                    }

                    handle.draw_text_ex(
                        font,
                        &line,
                        Vector2::new(self.current_x as f32, screen_y as f32),
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
            let line_width = self.measure_text(&line, font, size).x as i32;

            if self.is_visible(self.current_y, line_height) {
                if let Some(bg) = bg_color {
                    handle.draw_rectangle(self.current_x, screen_y, line_width, line_height, bg);
                }

                handle.draw_text_ex(
                    font,
                    &line,
                    Vector2::new(self.current_x as f32, screen_y as f32),
                    size,
                    spacing,
                    color,
                );
            }

            self.current_y += line_height + 4;
        }
    }

    fn should_skip_node(&self, node: &Node) -> bool {
        let style = node.computed_style.borrow();
        if let Some(Display::None) = style.display {
            return true;
        }

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

        let style = node.computed_style.borrow();

        if let Some(margin_top) = style.margin_top {
            self.add_spacing(margin_top as i32);
        }

        let old_x = self.current_x;
        if let Some(padding_left) = style.padding_left {
            self.current_x += padding_left as i32;
        }

        let is_bold = match &style.font_weight {
            Some(FontWeight::Bold) | Some(FontWeight::Bolder) => true,
            Some(FontWeight::Weight(w)) if *w >= 700 => true,
            _ => false,
        };

        let is_italic = matches!(
            &style.font_style,
            Some(FontStyle::Italic) | Some(FontStyle::Oblique)
        );

        let font = match (is_bold, is_italic) {
            (true, _) => self.bold_font,
            (false, true) => self.italic_font.unwrap_or(self.regular_font),
            (false, false) => self.regular_font,
        };

        let size = style.font_size.unwrap_or(self.font_size);

        let color = style.color.unwrap_or(self.default_color);
        let bg_color = style.background_color;

        match &node.data {
            NodeData::Element { name, .. } => match name.local.as_str() {
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        font,
                        size,
                        color,
                        bg_color,
                    );
                }
                "p" => {
                    for child in node.children.borrow().iter() {
                        self.render_node(handle, child);
                    }
                }
                "div" | "body" => {
                    for child in node.children.borrow().iter() {
                        self.render_node(handle, child);
                    }
                }
                "strong" | "b" | "em" | "i" | "span" => {
                    for child in node.children.borrow().iter() {
                        self.render_node(handle, child);
                    }
                }
                "ul" | "ol" => {
                    for child in node.children.borrow().iter() {
                        self.render_node(handle, child);
                    }
                }
                "li" => {
                    let text = format!("• {}", node.get_text_content().trim());
                    self.render_text_wrapped(handle, &text, font, size, color, bg_color);
                }
                "br" => {
                    let line_height = self.measure_text("A", font, size).y as i32;
                    self.add_spacing(line_height);
                }
                "hr" => {
                    let screen_y = self.current_y - self.scroll_offset;
                    if self.is_visible(self.current_y, 1) {
                        handle.draw_line(
                            self.margin_left,
                            screen_y,
                            self.screen_width - self.margin_right,
                            screen_y,
                            color,
                        );
                    }
                    self.add_spacing(10);
                }
                "blockquote" => {
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        font,
                        size.max(BASE_FONT_SIZE),
                        Color::new(150, 150, 150, 255),
                        bg_color,
                    );
                }
                "code" | "pre" => {
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        font,
                        size,
                        Color::new(200, 150, 100, 255),
                        bg_color,
                    );
                }
                "a" => {
                    self.render_text_wrapped(
                        handle,
                        &node.get_text_content(),
                        font,
                        size,
                        Color::new(100, 150, 255, 255),
                        bg_color,
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
                    self.render_text_wrapped(handle, contents, font, size, color, bg_color);
                }
            }
            _ => {
                for child in node.children.borrow().iter() {
                    self.render_node(handle, child);
                }
            }
        }

        self.current_x = old_x;

        if let Some(margin_bottom) = style.margin_bottom {
            self.add_spacing(margin_bottom as i32);
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
        .with_color(Color::BLACK)
        .with_scroll_offset(scroll_state.scroll_offset)
        .with_font_size(BASE_FONT_SIZE);

    render_ctx.render_node(handle, &document.root);
    scroll_state.content_height = render_ctx.current_y;
}
