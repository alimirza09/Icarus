use icarus::{html::parser, init};
use parser::parse_html;
use sight::Color;

fn main() {
    println!("Icarus Browser - DOM Test\n");

    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Icarus Browser Test</title>
        </head>
        <body>
            <h1>Welcome to Icarus!</h1>
            <p>The browser engine parses the HTML and displays text content.</p>
            <div>
                This is some text in a div.
                It should wrap nicely when it reaches the edge of the screen.
                The quick brown fox, jumps over the lazy dog?!
                THE QUICK BROWN FOX; "JUMPS OVER THE LAZY DOG'".:
            </div>
        </body>
        </html>
    "#;

    println!("Parsing HTML...\n");
    let document = parse_html(html);

    println!("DOM Tree:");
    println!("=========\n");
    document.print_tree();

    println!("\n\nText Content:");
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

    while ctx.window.is_open() && !ctx.window.is_key_down(minifb::Key::Escape) {
        ctx.clear(Color::BLACK);

        let title = document.get_elements_by_tag_name("title");
        if title.len() == 1 {
            let text = title[0].get_text_content();
            ctx.window.set_title(&text);
        }

        let content = &document
            .root
            .get_text_content()
            .replace(&title[0].get_text_content(), "");

        let headings = document.get_elements_by_tag_name("h1");

        let headings_text = headings[0].get_text_content();

        ctx.draw_text_antialiased_ttf::<sight::ttf::TtfFont>(
            &bold_font,
            &headings_text,
            0,
            0,
            24.0,
            Color::GREEN,
        );

        let paragraphs = content.replace(&headings_text, "");

        ctx.draw_text_antialiased_ttf::<sight::ttf::TtfFont>(
            &font,
            &paragraphs.trim(),
            0,
            24,
            11.0,
            Color::GREEN,
        );

        ctx.present().expect("Failed");
    }
}
