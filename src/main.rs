use icarus::graphics::renderer;
use icarus::{html::parser, init};
use parser::parse_html;
use sight::Color;

fn main() {
    println!("Icarus Browser - DOM Test\n");

    let html = std::fs::read_to_string("/home/ali/Projects/icarus/test.html").unwrap();

    println!("Parsing HTML\n");
    let document = parse_html(&html);

    println!("DOM Tree:");
    println!("=========\n");
    document.print_tree();

    println!("=============\n");
    let text = document.root.get_text_content();
    println!("{}", text.trim());
    println!("===============\n");

    let mut ctx = init().expect("Failed");
    let font_data = std::fs::read("/home/ali/Projects/icarus/resources/NotoSerif-Black.ttf")
        .expect("Failed to read font");
    let bold_font_data = std::fs::read("/home/ali/Projects/icarus/resources/NotoSerif-Bold.ttf")
        .expect("failed to read bold font");

    let font = sight::ttf::TtfFont::from_bytes(&font_data).expect("Failed to parse ttf");
    let bold_font =
        sight::ttf::TtfFont::from_bytes(&bold_font_data).expect("failed to parse bold font");

    let mut scroll_state = icarus::graphics::renderer::ScrollableDocument::new(ctx.height());

    while ctx.window.is_open() && !ctx.window.is_key_down(minifb::Key::Escape) {
        if ctx.window.is_key_down(minifb::Key::Up) || ctx.window.is_key_down(minifb::Key::W) {
            scroll_state.scroll_up(20);
        }
        if ctx.window.is_key_down(minifb::Key::Down) || ctx.window.is_key_down(minifb::Key::S) {
            scroll_state.scroll_down(20);
        }
        if ctx.window.is_key_down(minifb::Key::PageUp) {
            scroll_state.scroll_up(ctx.height() as i32 - 50);
        }
        if ctx.window.is_key_down(minifb::Key::PageDown) {
            scroll_state.scroll_down(ctx.height() as i32 - 50);
        }
        if ctx.window.is_key_down(minifb::Key::Home) {
            scroll_state.scroll_offset = 0;
        }
        if ctx.window.is_key_down(minifb::Key::End) {
            scroll_state.scroll_offset =
                (scroll_state.content_height - scroll_state.viewport_height).max(0);
        }

        ctx.clear(Color::BLACK);

        let title = document.get_elements_by_tag_name("title");
        if !title.is_empty() {
            ctx.window.set_title(&title[0].get_text_content());
        }

        renderer::render_document_scrollable(
            &mut ctx,
            &document,
            &font,
            &bold_font,
            &mut scroll_state,
        );

        ctx.present().expect("Failed");
    }
}
