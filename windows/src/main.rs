use pagurus::{failure::OrFail, spatial::Size, Game, Result};
use pagurus_windows_system::WindowsSystemBuilder;
use pixcil::game::PixcilGame;

fn main() -> Result<()> {
    let mut game = PixcilGame::default();
    let mut system = WindowsSystemBuilder::new("Pixcil")
        .window_size(Some(Size::from_wh(1200, 600)))
        .enable_audio(false)
        .build()
        .or_fail()?;
    game.initialize(&mut system).or_fail()?;
    loop {
        let event = system.next_event();
        let do_continue = game.handle_event(&mut system, event).or_fail()?;
        if !do_continue {
            break;
        }
    }
    Ok(())
}
