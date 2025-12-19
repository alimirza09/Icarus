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

    println!("\n\nFound Elements:");
    println!("===============\n");

    let paragraphs = document.get_elements_by_tag_name("p");
    println!("Found {} <p> tags:", paragraphs.len());
    for (i, p) in paragraphs.iter().enumerate() {
        let text = p.get_text_content();
        println!("  {}. {}", i + 1, text.trim());
    }

    let headings = document.get_elements_by_tag_name("h1");
    println!("\nFound {} <h1> tags:", headings.len());
    for h1 in headings.iter() {
        let text = h1.get_text_content();
        println!("  - {}", text.trim());
    }

    println!("===============\n");

    let mut ctx = init().expect("Failed");
    let font_data =
        std::fs::read("/home/ali/Projects/icarus/resources/FONT.BDF").expect("Failed to read font");

    let font = sight::bdf::parse_bdf_font(&font_data).expect("Failed to parse BDF");

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

        font.draw_text(&content, 0, 0, sight::Color::GREEN, |x, y, color| {
            ctx.put_pixel(x, y, color);
        });
        ctx.present().expect("Failed");
    }
}
