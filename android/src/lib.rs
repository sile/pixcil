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
    let activity_instance = unsafe { JObject::from_raw(activity.activity()) };
    let window = env
        .call_method(
            activity_instance,
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

fn start_file_open() {
    let activity = ndk_glue::native_activity();
    let vm = unsafe { jni::JavaVM::from_raw(activity.vm()) }.unwrap_or_else(|e| panic!("{e}"));
    let env = vm.attach_current_thread().unwrap_or_else(|e| panic!("{e}"));

    let activity = ndk_glue::native_activity();
    let activity_instance = unsafe { JObject::from_raw(activity.activity()) };

    let intent_class = env
        .find_class("android/content/Intent")
        .unwrap_or_else(|e| panic!("{e}"));
    let action_create_document = env
        .get_static_field(intent_class, "ACTION_GET_CONTENT", "Ljava/lang/String;")
        //.get_static_field(intent_class, "ACTION_OPEN_DOCUMENT", "Ljava/lang/String;")
        //.get_static_field(intent_class, "ACTION_CREATE_DOCUMENT", "Ljava/lang/String;")
        .unwrap_or_else(|e| panic!("{e}"));
    let category_openable = env
        .get_static_field(intent_class, "CATEGORY_OPENABLE", "Ljava/lang/String;")
        .unwrap_or_else(|e| panic!("{e}"));

    let intent = env
        .new_object(
            intent_class,
            "(Ljava/lang/String;)V",
            &[action_create_document],
        )
        .unwrap_or_else(|e| panic!("{e}"));
    env.call_method(
        intent,
        "addCategory",
        "(Ljava/lang/String;)Landroid/content/Intent;",
        &[category_openable],
    )
    .unwrap_or_else(|e| panic!("{e}"));
    env.call_method(
        intent,
        "setType",
        "(Ljava/lang/String;)Landroid/content/Intent;",
        &[env
            .new_string("image/png")
            .unwrap_or_else(|e| panic!("{e}"))
            .into()],
    )
    .unwrap_or_else(|e| panic!("{e}"));

    // FIXME: Cannot get the result as it's not possible to override
    //        the `Activity.onActivityResult` method in pure Rust.
    env.call_method(
        activity_instance,
        "startActivityForResult",
        "(Landroid/content/Intent;I)V",
        &[intent.clone().into(), 100.into()],
    )
    .unwrap_or_else(|e| panic!("{e}"));
}
