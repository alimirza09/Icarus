use raylib::prelude::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum CSSValue {
    Color(Color),
    Length(f32, LengthUnit),
    Keyword(String),
    Number(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LengthUnit {
    Px,
    Pt,
    Em,
    Rem,
    Percent,
}

#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub font_size: Option<f32>,
    pub font_weight: Option<FontWeight>,
    pub font_style: Option<FontStyle>,
    pub margin_top: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub padding_top: Option<f32>,
    pub padding_bottom: Option<f32>,
    pub padding_left: Option<f32>,
    pub padding_right: Option<f32>,
    pub text_align: Option<TextAlign>,
    pub display: Option<Display>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    Weight(u16),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    None,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, other: &ComputedStyle) {
        if other.color.is_some() {
            self.color = other.color;
        }
        if other.background_color.is_some() {
            self.background_color = other.background_color;
        }
        if other.font_size.is_some() {
            self.font_size = other.font_size;
        }
        if other.font_weight.is_some() {
            self.font_weight = other.font_weight.clone();
        }
        if other.font_style.is_some() {
            self.font_style = other.font_style.clone();
        }
        if other.margin_top.is_some() {
            self.margin_top = other.margin_top;
        }
        if other.margin_bottom.is_some() {
            self.margin_bottom = other.margin_bottom;
        }
        if other.margin_left.is_some() {
            self.margin_left = other.margin_left;
        }
        if other.margin_right.is_some() {
            self.margin_right = other.margin_right;
        }
        if other.padding_top.is_some() {
            self.padding_top = other.padding_top;
        }
        if other.padding_bottom.is_some() {
            self.padding_bottom = other.padding_bottom;
        }
        if other.padding_left.is_some() {
            self.padding_left = other.padding_left;
        }
        if other.padding_right.is_some() {
            self.padding_right = other.padding_right;
        }
        if other.text_align.is_some() {
            self.text_align = other.text_align.clone();
        }
        if other.display.is_some() {
            self.display = other.display.clone();
        }
    }
}
