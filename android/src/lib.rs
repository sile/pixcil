use pagurus::failure::OrFail;
use pagurus::Game;
use pagurus_android_system::AndroidSystemBuilder;

#[ndk_glue::main(backtrace = "on")]
pub fn main() {
    let mut game = pixcil::game::PixcilGame::default();
    let mut system = AndroidSystemBuilder::new()
        .build()
        .or_fail()
        .unwrap_or_else(|e| panic!("{e}"));

    game.initialize(&mut system)
        .or_fail()
        .unwrap_or_else(|e| panic!("{e}"));
    loop {
        let event = system
            .wait_event()
            .or_fail()
            .unwrap_or_else(|e| panic!("{e}"));
        let do_continue = game
            .handle_event(&mut system, event)
            .or_fail()
            .unwrap_or_else(|e| panic!("{e}"));
        if !do_continue {
            break;
        }
    }

    #[allow(deprecated)]
    ndk_glue::native_activity().finish()
}
