use pagurus::failure::Failure;
use pagurus::{failure::OrFail, spatial::Size, Game, Result};
use pagurus_windows_system::{WindowsSystem, WindowsSystemBuilder};
use pixcil::event::InputId;
use pixcil::game::PixcilGame;
use pixcil::io::InputNumber;
use std::cell::RefCell;
use std::path::PathBuf;
use windows::Win32::Foundation::{GetLastError, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HWND,
        System::Com::{
            CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
        },
        UI::Shell::{
            Common::COMDLG_FILTERSPEC, FileOpenDialog, FileSaveDialog, IFileOpenDialog,
            IFileSaveDialog, SIGDN_FILESYSPATH,
        },
    },
};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).or_fail()?;
    }

    let mut game = PixcilGame::default();
    let mut system = WindowsSystemBuilder::new("Pixcil")
        .window_size(Some(Size::from_wh(1200, 800)))
        .enable_audio(false)
        .build()
        .or_fail()?;
    game.initialize(&mut system).or_fail()?;

    let mut workspace_name = None;
    loop {
        let event = system.next_event();
        let do_continue = game.handle_event(&mut system, event).or_fail()?;
        if !do_continue {
            break;
        }

        let request = game.query(&mut system, "nextIoRequest").or_fail()?;
        if request.is_empty() {
            continue;
        }
        handle_io_request(
            &mut system,
            &mut game,
            &mut workspace_name,
            serde_json::from_slice(&request).or_fail()?,
        )
        .or_fail()?;
    }
    Ok(())
}

fn handle_io_request(
    system: &mut WindowsSystem,
    game: &mut PixcilGame,
    workspace_name: &mut Option<String>,
    req: pixcil::io::IoRequest,
) -> Result<()> {
    match req {
        pixcil::io::IoRequest::SaveWorkspace => {
            handle_save_workspace(system, game, workspace_name).or_fail()?;
        }
        pixcil::io::IoRequest::LoadWorkspace => {
            handle_load_workspace(system, game, workspace_name).or_fail()?;
        }
        pixcil::io::IoRequest::ImportImage => {
            handle_import_image(system, game).or_fail()?;
        }
        pixcil::io::IoRequest::InputNumber { id } => {
            handle_input_number(system, game, id).or_fail()?;
        }
    }
    Ok(())
}

fn handle_input_number(
    system: &mut WindowsSystem,
    game: &mut PixcilGame,
    id: InputId,
) -> Result<()> {
    unsafe {
        let template = InputTextDialog::new();
        let ret = DialogBoxIndirectParamA(
            None,
            std::mem::transmute(&template),
            system.window().hwnd(),
            Some(input_text_dialog_proc),
            None,
        );
        std::mem::drop(template);

        if ret < 0 {
            return Err(Failure::new(format!(
                "Cannot open dialog: error_code={}",
                GetLastError().0
            )));
        }

        let buf = INPUT_TEXT_BUF.with(|buf| {
            buf.borrow()
                .iter()
                .copied()
                .take_while(|&b| b != 0)
                .collect::<Vec<_>>()
        });
        if !buf.is_empty() {
            if let Ok(number) = String::from_utf8(buf) {
                game.command(
                    system,
                    "notifyInputNumber",
                    &serde_json::to_vec(&InputNumber { id, number }).or_fail()?,
                )
                .or_fail()?;
            }
        }
    }
    Ok(())
}

fn handle_save_workspace(
    system: &mut WindowsSystem,
    game: &mut PixcilGame,
    workspace_name: &mut Option<String>,
) -> Result<()> {
    unsafe {
        let dialog: IFileSaveDialog =
            CoCreateInstance(&FileSaveDialog, None, CLSCTX_INPROC_SERVER).or_fail()?;
        dialog
            .SetFileTypes(&[COMDLG_FILTERSPEC {
                pszName: windows::w!("Pixcil workspace files (*.png)").into(),
                pszSpec: windows::w!("*.png").into(),
            }])
            .or_fail()?;
        dialog
            .SetTitle(windows::w!("Save Pixcil workspace"))
            .or_fail()?;
        dialog.SetDefaultExtension(windows::w!("png")).or_fail()?;
        if let Some(name) = workspace_name {
            let name = name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect::<Vec<_>>();
            dialog
                .SetFileName(PCWSTR::from_raw(name.as_ptr()))
                .or_fail()?;
        }
        if dialog.Show(system.window().hwnd()).is_err() {
            return Ok(());
        }

        let result = dialog.GetResult().or_fail()?;
        let path = PathBuf::from(
            result
                .GetDisplayName(SIGDN_FILESYSPATH)
                .or_fail()?
                .to_string()
                .or_fail()?,
        );
        if let Some(name) = path.file_name() {
            *workspace_name = Some(name.to_str().or_fail()?.to_owned());
        }

        let data = game.query(system, "workspacePng").or_fail()?;
        std::fs::write(path, data).or_fail()?;

        Ok(())
    }
}

fn handle_load_workspace(
    system: &mut WindowsSystem,
    game: &mut PixcilGame,
    workspace_name: &mut Option<String>,
) -> Result<()> {
    let result = file_open_dialog(
        system,
        windows::w!("Load Pixcil workspace").into(),
        windows::w!("Pixcil workspace files (*.png)").into(),
    )
    .or_fail()?;
    if let Some(path) = result {
        let data = std::fs::read(&path).or_fail()?;
        game.command(system, "loadWorkspace", &data).or_fail()?;
        if let Some(name) = path.file_name() {
            *workspace_name = Some(name.to_str().or_fail()?.to_owned());
        }
    }
    Ok(())
}

fn handle_import_image(system: &mut WindowsSystem, game: &mut PixcilGame) -> Result<()> {
    let result = file_open_dialog(
        system,
        windows::w!("Load image file").into(),
        windows::w!("PNG files (*.png)").into(),
    )
    .or_fail()?;
    if let Some(path) = result {
        let data = std::fs::read(path).or_fail()?;
        game.command(system, "importImage", &data).or_fail()?;
    }
    Ok(())
}

fn file_open_dialog(
    system: &WindowsSystem,
    title: PCWSTR,
    file_type: PCWSTR,
) -> Result<Option<PathBuf>> {
    unsafe {
        let dialog: IFileOpenDialog =
            CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER).or_fail()?;
        dialog
            .SetFileTypes(&[COMDLG_FILTERSPEC {
                pszName: file_type,
                pszSpec: windows::w!("*.png").into(),
            }])
            .or_fail()?;
        dialog.SetTitle(title).or_fail()?;
        if dialog.Show(system.window().hwnd()).is_err() {
            return Ok(None);
        }

        let result = dialog.GetResult().or_fail()?;
        let path = result.GetDisplayName(SIGDN_FILESYSPATH).or_fail()?;
        Ok(Some(PathBuf::from(path.to_string().or_fail()?)))
    }
}

#[repr(C)]
struct Dialog {
    template: DLGTEMPLATE,
    menu_resource: u16,
    window_class: u16,
    title: [u16; 15],
}

#[repr(C, align(4))]
struct InputTextDialog {
    dialog: Dialog,
    text_box: TextBox,
    ok_button: OkButton,
    cancel_button: CancelButton,
}

impl InputTextDialog {
    fn new() -> Self {
        let size = Size::from_wh(88, 36);

        let template = DLGTEMPLATE {
            style: (WS_POPUP | WS_BORDER | WS_DLGFRAME).0 | DS_CENTER as u32,
            cx: size.width as i16,
            cy: size.height as i16,
            cdit: 3,
            ..Default::default()
        };
        let dialog = Dialog {
            template,
            menu_resource: 0, // No menu
            window_class: 0,  // Predefined dialog box class (by default)
            title: [
                'E' as u16, 'n' as u16, 't' as u16, 'e' as u16, 'r' as u16, ' ' as u16, 'a' as u16,
                ' ' as u16, 'n' as u16, 'u' as u16, 'm' as u16, 'b' as u16, 'e' as u16, 'r' as u16,
                0,
            ],
        };
        let ok_button = OkButton::new();
        let cancel_button = CancelButton::new();
        let text_box = TextBox::new();
        Self {
            dialog,
            ok_button,
            cancel_button,
            text_box,
        }
    }
}

#[repr(C, align(4))]
struct OkButton {
    item: DLGITEMTEMPLATE,
    window_class: u16,
    window_class_value: u16,
    text: [u16; 3],
    creation_data: u16,
}

impl OkButton {
    const ID: u16 = 1;

    fn new() -> Self {
        let item = DLGITEMTEMPLATE {
            id: Self::ID,
            style: (WS_CHILD | WS_VISIBLE | WS_TABSTOP).0 | BS_DEFPUSHBUTTON as u32,
            x: 5,
            y: 20,
            cx: 35,
            cy: 12,
            ..Default::default()
        };
        Self {
            item,
            window_class: 0xFFFF,
            window_class_value: 0x0080, // Button
            text: ['O' as u16, 'K' as u16, 0],
            creation_data: 0,
        }
    }
}

#[repr(C, align(4))]
struct CancelButton {
    item: DLGITEMTEMPLATE,
    window_class: u16,
    window_class_value: u16,
    text: [u16; 7],
    creation_data: u16,
}

impl CancelButton {
    const ID: u16 = 2;

    fn new() -> Self {
        let item = DLGITEMTEMPLATE {
            id: Self::ID,
            style: (WS_CHILD | WS_VISIBLE | WS_TABSTOP).0,
            x: 48,
            y: 20,
            cx: 35,
            cy: 12,
            ..Default::default()
        };
        Self {
            item,
            window_class: 0xFFFF,
            window_class_value: 0x0080, // Button
            text: [
                'C' as u16, 'a' as u16, 'n' as u16, 'c' as u16, 'e' as u16, 'l' as u16, 0,
            ],
            creation_data: 0,
        }
    }
}

#[repr(C, align(4))]
struct TextBox {
    item: DLGITEMTEMPLATE,
    window_class: u16,
    window_class_value: u16,
    text: [u16; 1],
    creation_data: u16,
}

impl TextBox {
    const ID: u16 = 3;

    fn new() -> Self {
        let item = DLGITEMTEMPLATE {
            id: Self::ID,
            style: (WS_CHILD | WS_VISIBLE | WS_BORDER | WS_TABSTOP).0,
            x: 4,
            y: 4,
            cx: 80,
            cy: 12,
            ..Default::default()
        };
        Self {
            item,
            window_class: 0xFFFF,
            window_class_value: 0x0081, // Edit
            text: [0],
            creation_data: 0,
        }
    }
}

const TRUE: isize = 1;
const FALSE: isize = 0;

std::thread_local! {
    static INPUT_TEXT_BUF: RefCell<[u8; 128]> = RefCell::new([0; 128]);
}

unsafe extern "system" fn input_text_dialog_proc(
    hdlg: HWND,
    message: u32,
    wparam: WPARAM,
    _lparam: LPARAM,
) -> isize {
    if message != WM_COMMAND {
        return FALSE;
    }

    match wparam.0 as u16 {
        OkButton::ID => {
            INPUT_TEXT_BUF.with(|buf| {
                GetDlgItemTextA(hdlg, TextBox::ID as i32, &mut *buf.borrow_mut());
            });
            EndDialog(hdlg, TRUE);
            return TRUE;
        }
        CancelButton::ID => {
            INPUT_TEXT_BUF.with(|buf| buf.borrow_mut()[0] = 0);
            EndDialog(hdlg, TRUE);
            return TRUE;
        }
        _ => {}
    }
    FALSE
}
