use super::types::*;
use raylib::prelude::Color;

pub fn parse_inline_style(style_str: &str) -> ComputedStyle {
    let mut style = ComputedStyle::new();

    for declaration in style_str.split(';') {
        let declaration = declaration.trim();
        if declaration.is_empty() {
            continue;
        }

        if let Some((prop, value)) = declaration.split_once(':') {
            let prop = prop.trim().to_lowercase();
            let value = value.trim();

            match prop.as_str() {
                "color" => {
                    if let Some(color) = parse_color(value) {
                        style.color = Some(color);
                    }
                }
                "background-color" | "background" => {
                    if let Some(color) = parse_color(value) {
                        style.background_color = Some(color);
                    }
                }
                "font-size" => {
                    if let Some(size) = parse_length(value) {
                        style.font_size = Some(size);
                    }
                }
                "font-weight" => {
                    style.font_weight = Some(parse_font_weight(value));
                }
                "font-style" => {
                    style.font_style = Some(parse_font_style(value));
                }
                "margin" => {
                    if let Some(margin) = parse_length(value) {
                        style.margin_top = Some(margin);
                        style.margin_bottom = Some(margin);
                        style.margin_left = Some(margin);
                        style.margin_right = Some(margin);
                    }
                }
                "margin-top" => {
                    if let Some(margin) = parse_length(value) {
                        style.margin_top = Some(margin);
                    }
                }
                "margin-bottom" => {
                    if let Some(margin) = parse_length(value) {
                        style.margin_bottom = Some(margin);
                    }
                }
                "margin-left" => {
                    if let Some(margin) = parse_length(value) {
                        style.margin_left = Some(margin);
                    }
                }
                "margin-right" => {
                    if let Some(margin) = parse_length(value) {
                        style.margin_right = Some(margin);
                    }
                }
                "padding" => {
                    if let Some(padding) = parse_length(value) {
                        style.padding_top = Some(padding);
                        style.padding_bottom = Some(padding);
                        style.padding_left = Some(padding);
                        style.padding_right = Some(padding);
                    }
                }
                "padding-top" => {
                    if let Some(padding) = parse_length(value) {
                        style.padding_top = Some(padding);
                    }
                }
                "padding-bottom" => {
                    if let Some(padding) = parse_length(value) {
                        style.padding_bottom = Some(padding);
                    }
                }
                "padding-left" => {
                    if let Some(padding) = parse_length(value) {
                        style.padding_left = Some(padding);
                    }
                }
                "padding-right" => {
                    if let Some(padding) = parse_length(value) {
                        style.padding_right = Some(padding);
                    }
                }
                "text-align" => {
                    style.text_align = Some(parse_text_align(value));
                }
                "display" => {
                    style.display = Some(parse_display(value));
                }
                _ => {}
            }
        }
    }

    style
}

fn parse_color(value: &str) -> Option<Color> {
    let value = value.trim().to_lowercase();

    match value.as_str() {
        "black" => return Some(Color::BLACK),
        "white" => return Some(Color::WHITE),
        "red" => return Some(Color::RED),
        "green" => return Some(Color::GREEN),
        "blue" => return Some(Color::BLUE),
        "yellow" => return Some(Color::YELLOW),
        "orange" => return Some(Color::ORANGE),
        "purple" => return Some(Color::PURPLE),
        "gray" | "grey" => return Some(Color::GRAY),
        "darkgray" | "darkgrey" => return Some(Color::DARKGRAY),
        "lightgray" | "lightgrey" => return Some(Color::LIGHTGRAY),
        _ => {}
    }

    if value.starts_with('#') {
        let hex = &value[1..];
        if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            return Some(Color::new(r, g, b, 255));
        } else if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(Color::new(r, g, b, 255));
        }
    }

    if value.starts_with("rgb(") && value.ends_with(')') {
        let params = &value[4..value.len() - 1];
        let parts: Vec<&str> = params.split(',').collect();
        if parts.len() == 3 {
            let r = parts[0].trim().parse::<u8>().ok()?;
            let g = parts[1].trim().parse::<u8>().ok()?;
            let b = parts[2].trim().parse::<u8>().ok()?;
            return Some(Color::new(r, g, b, 255));
        }
    }

    if value.starts_with("rgba(") && value.ends_with(')') {
        let params = &value[5..value.len() - 1];
        let parts: Vec<&str> = params.split(',').collect();
        if parts.len() == 4 {
            let r = parts[0].trim().parse::<u8>().ok()?;
            let g = parts[1].trim().parse::<u8>().ok()?;
            let b = parts[2].trim().parse::<u8>().ok()?;
            let a = (parts[3].trim().parse::<f32>().ok()? * 255.0) as u8;
            return Some(Color::new(r, g, b, a));
        }
    }

    None
}

fn parse_length(value: &str) -> Option<f32> {
    let value = value.trim().to_lowercase();

    if value.ends_with("px") {
        let num = value.trim_end_matches("px").trim();
        return num.parse::<f32>().ok();
    }

    if value.ends_with("pt") {
        let num = value.trim_end_matches("pt").trim();

        return num.parse::<f32>().ok().map(|v| v * 1.333);
    }

    if value.ends_with("em") {
        let num = value.trim_end_matches("em").trim();

        return num.parse::<f32>().ok().map(|v| v * 16.0);
    }

    value.parse::<f32>().ok()
}

fn parse_font_weight(value: &str) -> FontWeight {
    match value.trim().to_lowercase().as_str() {
        "normal" => FontWeight::Normal,
        "bold" => FontWeight::Bold,
        "bolder" => FontWeight::Bolder,
        "lighter" => FontWeight::Lighter,
        num => {
            if let Ok(weight) = num.parse::<u16>() {
                FontWeight::Weight(weight)
            } else {
                FontWeight::Normal
            }
        }
    }
}

fn parse_font_style(value: &str) -> FontStyle {
    match value.trim().to_lowercase().as_str() {
        "italic" => FontStyle::Italic,
        "oblique" => FontStyle::Oblique,
        _ => FontStyle::Normal,
    }
}

fn parse_text_align(value: &str) -> TextAlign {
    match value.trim().to_lowercase().as_str() {
        "left" => TextAlign::Left,
        "right" => TextAlign::Right,
        "center" => TextAlign::Center,
        "justify" => TextAlign::Justify,
        _ => TextAlign::Left,
    }
}

fn parse_display(value: &str) -> Display {
    match value.trim().to_lowercase().as_str() {
        "block" => Display::Block,
        "inline" => Display::Inline,
        "inline-block" => Display::InlineBlock,
        "none" => Display::None,
        _ => Display::Block,
    }
}
