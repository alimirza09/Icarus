use icarus::graphics::renderer;
use icarus::html::parser;
use parser::parse_html;
use raylib::prelude::*;

fn main() {
    println!("Icarus Browser - DOM Test\n");

    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let _html = rt.block_on(async { icarus::networking::http::connect_to_http_site("https:") });

    let html = std::fs::read_to_string("/home/ali/Projects/icarus/test.html")
        .expect("Error reading test.html");

    println!("Parsing HTML\n");
    let document = parse_html(&html);

    println!("DOM Tree:");
    println!("=========\n");
    document.print_tree();
    println!("=============\n");

    let text = document.root.get_text_content();
    println!("{}", text.trim());
    println!("===============\n");

    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("Icarus Browser")
        .resizable()
        .build();

    rl.set_target_fps(60);

    let font_data = std::fs::read("/home/ali/Projects/icarus/resources/NotoSerif-Black.ttf")
        .expect("Failed to read font");
    let bold_font_data = std::fs::read("/home/ali/Projects/icarus/resources/NotoSerif-Bold.ttf")
        .expect("failed to read bold font");

    let font = rl
        .load_font_from_memory(&thread, ".ttf", &font_data, 64, None)
        .expect("Failed to load font");

    let bold_font = rl
        .load_font_from_memory(&thread, ".ttf", &bold_font_data, 64, None)
        .expect("Failed to load bold font");

    let mut scroll_state = renderer::ScrollableDocument::new(rl.get_screen_height() as u32);

    let title = document.get_elements_by_tag_name("title");
    if !title.is_empty() {
        rl.set_window_title(&thread, &title[0].get_text_content());
    }

    while !rl.window_should_close() {
        let scroll_speed = 20;
        let page_scroll = rl.get_screen_height() - 50;

        if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
            scroll_state.scroll_up(scroll_speed);
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
            scroll_state.scroll_down(scroll_speed);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_PAGE_UP) {
            scroll_state.scroll_up(page_scroll);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_PAGE_DOWN) {
            scroll_state.scroll_down(page_scroll);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_HOME) {
            scroll_state.scroll_offset = 0;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_END) {
            scroll_state.scroll_offset =
                (scroll_state.content_height - scroll_state.viewport_height).max(0);
        }

        let wheel_move = rl.get_mouse_wheel_move();
        if wheel_move != 0.0 {
            if wheel_move > 0.0 {
                scroll_state.scroll_up((wheel_move * 30.0) as i32);
            } else {
                scroll_state.scroll_down((wheel_move.abs() * 30.0) as i32);
            }
        }

        scroll_state.viewport_height = rl.get_screen_height();

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();

        {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::BLACK);

            renderer::render_document_scrollable(
                &mut d,
                &document,
                &font,
                &bold_font,
                &mut scroll_state,
                screen_width,
                screen_height,
            );
        }
    }
}
