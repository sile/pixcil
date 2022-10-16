use pagurus::failure::OrFail;
use pagurus::spatial::Size;
use pagurus::{Game, Result};
use pagurus_sdl_system::SdlSystemBuilder;

fn main() -> Result<()> {
    let mut game = pixcil::game::PixcilGame::default();
    let mut system = SdlSystemBuilder::new()
        .title("Pixcil")
        .window_size(Some(Size::from_wh(1200, 800)))
        .build()
        .or_fail()?;

    game.initialize(&mut system).or_fail()?;
    loop {
        let event = system.wait_event();
        let do_continue = game.handle_event(&mut system, event).or_fail()?;
        if !do_continue {
            break;
        }
    }

    Ok(())
}
