use icarus::html::parser;
use parser::parse_html;

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
            <p>This is a simple HTML page rendered with no CSS.</p>
            <p>The browser engine parses the HTML and displays text content.</p>
            <div>
                This is some text in a div.
                It should wrap nicely when it reaches the edge of the screen.
            </div>
            <p>More paragraphs can be added here.</p>
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
}
