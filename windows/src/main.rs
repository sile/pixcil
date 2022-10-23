use pagurus::{failure::OrFail, spatial::Size, Game, Result};
use pagurus_windows_system::{WindowsSystem, WindowsSystemBuilder};
use pixcil::game::PixcilGame;
use std::path::PathBuf;
use windows::Win32::Foundation::{LPARAM, WPARAM};
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
        pixcil::io::IoRequest::InputNumber { id } => unsafe {
            let mut template: DLGTEMPLATE = std::mem::zeroed();
            template.style = (WS_POPUP | WS_BORDER | WS_SYSMENU | WS_CAPTION).0; // DS_MODALFRAME |
            template.cx = 200;
            template.cy = 100;
            DialogBoxIndirectParamA(None, &template, None, Some(password_proc), None);
        },
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
        if dialog.Show(HWND::default()).is_err() {
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

fn file_open_dialog(title: PCWSTR, file_type: PCWSTR) -> Result<Option<PathBuf>> {
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
        if dialog.Show(HWND::default()).is_err() {
            return Ok(None);
        }

        let result = dialog.GetResult().or_fail()?;
        let path = result.GetDisplayName(SIGDN_FILESYSPATH).or_fail()?;
        Ok(Some(PathBuf::from(path.to_string().or_fail()?)))
    }
}

unsafe extern "system" fn password_proc(
    hdlg: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> isize {
    match message {
        WM_INITDIALOG => {
            println!("init");
        }
        WM_COMMAND => {
            println!("command");
        }
        _ => {
            //return DefDlgProcA(hwnd, message, wparam, lparam).0;
            println!("message: {message:?}");
        }
    }
    0
}

// INT_PTR CALLBACK password_proc(HWND hDlg, UINT message, WPARAM wParam, LPARAM lParam)
// {
//     TCHAR lpszPassword[16];
//     WORD cchPassword;

//     switch (message)
//     {
//         case WM_INITDIALOG:
//             // Set password character to a plus sign (+)
//             SendDlgItemMessage(hDlg,
//                                IDE_PASSWORDEDIT,
//                                EM_SETPASSWORDCHAR,
//                                (WPARAM) '+',
//                                (LPARAM) 0);

//             // Set the default push button to "Cancel."
//             SendMessage(hDlg,
//                         DM_SETDEFID,
//                         (WPARAM) IDCANCEL,
//                         (LPARAM) 0);

//             return TRUE;

//         case WM_COMMAND:
//             // Set the default push button to "OK" when the user enters text.
//             if(HIWORD (wParam) == EN_CHANGE &&
//                                 LOWORD(wParam) == IDE_PASSWORDEDIT)
//             {
//                 SendMessage(hDlg,
//                             DM_SETDEFID,
//                             (WPARAM) IDOK,
//                             (LPARAM) 0);
//             }
//             switch(wParam)
//             {
//                 case IDOK:
//                     // Get number of characters.
//                     cchPassword = (WORD) SendDlgItemMessage(hDlg,
//                                                             IDE_PASSWORDEDIT,
//                                                             EM_LINELENGTH,
//                                                             (WPARAM) 0,
//                                                             (LPARAM) 0);
//                     if (cchPassword >= 16)
//                     {
//                         MessageBox(hDlg,
//                                    L"Too many characters.",
//                                    L"Error",
//                                    MB_OK);

//                         EndDialog(hDlg, TRUE);
//                         return FALSE;
//                     }
//                     else if (cchPassword == 0)
//                     {
//                         MessageBox(hDlg,
//                                    L"No characters entered.",
//                                    L"Error",
//                                    MB_OK);

//                         EndDialog(hDlg, TRUE);
//                         return FALSE;
//                     }

//                     // Put the number of characters into first word of buffer.
//                     *((LPWORD)lpszPassword) = cchPassword;

//                     // Get the characters.
//                     SendDlgItemMessage(hDlg,
//                                        IDE_PASSWORDEDIT,
//                                        EM_GETLINE,
//                                        (WPARAM) 0,       // line 0
//                                        (LPARAM) lpszPassword);

//                     // Null-terminate the string.
//                     lpszPassword[cchPassword] = 0;

//                     MessageBox(hDlg,
//                                lpszPassword,
//                                L"Did it work?",
//                                MB_OK);

//                     // Call a local password-parsing function.
//                     ParsePassword(lpszPassword);

//                     EndDialog(hDlg, TRUE);
//                     return TRUE;

//                 case IDCANCEL:
//                     EndDialog(hDlg, TRUE);
//                     return TRUE;
//             }
//             return 0;
//     }
//     return FALSE;
// }
