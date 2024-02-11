use std::ffi::OsStr;
use std::path::PathBuf;
use windows::core::HSTRING;
use windows::Win32::{
    Foundation::ERROR_FILE_NOT_FOUND,
    Storage::FileSystem::GetFullPathNameW,
    System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED},
    UI::Shell::{
        FileOperation, IFileOperation, IShellItem, SHCreateItemFromParsingName, FOF_NO_UI,
    },
};

pub fn resolve_path(path: &OsStr) -> PathBuf {
    // Canonicalize is fine on Linux and MacOS. Windows uses UNC and that messes up IFileOperation.
    // TODO: Check if canonicalize is actually fine, it might error on non-existent paths.
    #[cfg(not(windows))]
    {
        return PathBuf::from(path).canonicalize().unwrap();
    }

    #[cfg(windows)]
    unsafe {
        let size = GetFullPathNameW::<&HSTRING>(&HSTRING::from(path.to_str().unwrap()), None, None);
        let mut buffer = vec![0u16; size as usize];
        GetFullPathNameW::<&HSTRING>(
            &HSTRING::from(path.to_str().unwrap()),
            Some(buffer.as_mut_slice()),
            None,
        );

        let expanded_path = String::from_utf16_lossy(&buffer);
        // Strip nulls from utf16 string because PathBuf hates them apparently
        PathBuf::from(expanded_path.trim_end_matches('\u{0}'))
    }
}

pub fn delete_path(path: &PathBuf) -> windows::core::Result<()> {
    // No-op on non-Windows platforms. Might replace with normal file delete later on as a fallback.
    #[cfg(not(windows))]
    {
        eprintln!("This program is only intended for Windows.");
        std::process::exit(1);
    }
    /*
    Initialize COM using multi-threaded apartment model.
    It's *technically* meant for GUI threads, according to https://learn.microsoft.com/en-us/windows/win32/learnwin32/initializing-the-com-library,
    however it seems to just shave off ~2-3ms compared to normal multi-threaded, at least on Windows 11.
    */
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;
    }

    if !path.try_exists().unwrap_or(false) {
        return Err(ERROR_FILE_NOT_FOUND.into());
    }

    // Silly windows conversions...
    let string_path = HSTRING::from(path.to_str().unwrap());

    // Create ShellItem from path, IFileOperation operates on ShellItems
    unsafe {
        let shell_item =
            SHCreateItemFromParsingName::<&HSTRING, Option<_>, IShellItem>(&string_path, None)?;

        // File operation instance, with ugly CoCreateInstance generics because Windows API be like that
        let file_operation =
            CoCreateInstance::<Option<_>, IFileOperation>(&FileOperation, None, CLSCTX_ALL)?;

        // Recursive by default! Can be disabled with https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-setoperationflags#fof_norecursion-0x1000
        // We just need to prevent the confirmation dialog from showing. FOF_NO_UI is actually from an older Win32 API and not documented by the IFileOperation interface, but still works as expected.
        file_operation.SetOperationFlags(FOF_NO_UI)?;
        file_operation.DeleteItem(&shell_item, None)?;
        file_operation.PerformOperations()?;
    }

    Ok(())
}
