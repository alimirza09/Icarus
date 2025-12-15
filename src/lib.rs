use anyhow::Result;
use minifb::Menu;

pub mod dom;
pub mod graphics;
pub mod html;

pub fn init() -> Result<sight::Sight> {
    let mut ctx = sight::Sight::new(800, 500, "Icarus").expect("Failed to create main window");
    let main_menu = Menu::new("main")?;
    ctx.window.add_menu(&main_menu);
    ctx.window.set_target_fps(60);
    Ok(ctx)
}
