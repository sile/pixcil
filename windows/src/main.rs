use pagurus::{failure::OrFail, spatial::Size, Game, Result};
use pagurus_windows_system::{WindowsSystem, WindowsSystemBuilder};
use pixcil::game::PixcilGame;
use std::path::PathBuf;
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
        .window_size(Some(Size::from_wh(1200, 600)))
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
            println!("input-number: {id:?}");
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
