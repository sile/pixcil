use jni::objects::{JObject, JValue};
use pagurus::failure::OrFail;
use pagurus::Game;
use pagurus_android_system::AndroidSystemBuilder;

#[ndk_glue::main(backtrace = "on")]
pub fn main() {
    initialize();

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

    ndk_glue::native_activity().finish();
}

fn initialize() {
    let activity = ndk_glue::native_activity();
    let vm = unsafe { jni::JavaVM::from_raw(activity.vm()) }.unwrap_or_else(|e| panic!("{e}"));
    let env = vm.attach_current_thread().unwrap_or_else(|e| panic!("{e}"));

    let window = env
        .call_method(
            unsafe { JObject::from_raw(activity.activity()) },
            "getWindow",
            "()Landroid/view/Window;",
            &[],
        )
        .unwrap_or_else(|e| panic!("{e}"))
        .l()
        .unwrap_or_else(|e| panic!("{e}"));
    let decor_view = env
        .call_method(window, "getDecorView", "()Landroid/view/View;", &[])
        .unwrap_or_else(|e| panic!("{e}"))
        .l()
        .unwrap_or_else(|e| panic!("{e}"));

    let flag_fullscreen = env
        .get_static_field(decor_view, "SYSTEM_UI_FLAG_FULLSCREEN", "I")
        .unwrap_or_else(|e| panic!("{e}"))
        .i()
        .unwrap_or_else(|e| panic!("{e}"));
    let flag_hide_navigation = env
        .get_static_field(decor_view, "SYSTEM_UI_FLAG_HIDE_NAVIGATION", "I")
        .unwrap_or_else(|e| panic!("{e}"))
        .i()
        .unwrap_or_else(|e| panic!("{e}"));
    let flag_immersive_sticky = env
        .get_static_field(decor_view, "SYSTEM_UI_FLAG_IMMERSIVE_STICKY", "I")
        .unwrap_or_else(|e| panic!("{e}"))
        .i()
        .unwrap_or_else(|e| panic!("{e}"));
    let flag = JValue::Int(flag_fullscreen | flag_hide_navigation | flag_immersive_sticky);

    env.call_method(decor_view, "setSystemUiVisibility", "(I)V", &[flag])
        .unwrap_or_else(|e| panic!("{e}"));
}
