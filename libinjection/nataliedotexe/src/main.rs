use std::path::PathBuf;
use std::io::stdin;
use argh::FromArgs;
use color_eyre::{ eyre::eyre, Report, Result };

use widestring::{ u16cstr, U16CString, U16String };
use windows::{
    core::{ PCWSTR, PCSTR, PWSTR },
    Win32::{
        System::{
            LibraryLoader::{ GetModuleHandleW, GetProcAddress },
            Diagnostics::Debug::{
                WriteProcessMemory,
                FORMAT_MESSAGE_FROM_SYSTEM,
                FORMAT_MESSAGE_IGNORE_INSERTS,
                FormatMessageW,
            },
            Memory::{ MEM_COMMIT, PAGE_EXECUTE_READWRITE, VirtualAllocEx, MEM_RESERVE },
            Threading::{
                OpenProcess,
                PROCESS_QUERY_INFORMATION,
                PROCESS_CREATE_THREAD,
                PROCESS_VM_OPERATION,
                PROCESS_VM_READ,
                PROCESS_VM_WRITE,
                CreateRemoteThread,
            },
        },
        Foundation::GetLastError,
    },
};

// Init parse args
/// This Doc Comment is NON-OPTIONAL!
#[derive(FromArgs, Debug)]
struct Args {
    /// the Process ID to inject into
    #[argh(positional)]
    pid: u32,
}

fn main() -> Result<(), Report> {
    color_eyre::install()?; // idk
    let args: Args = argh::from_env(); // actually read args
    unsafe {
        do_crimes(args)?;
    } // all the code (unsafe, unsurprisingly)
    Ok(())
}

unsafe fn get_last_error() -> Report {
    let error_code = GetLastError().0;
    let mut buf = vec![0u16, 16384];
    let len = FormatMessageW(
        FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
        Some(std::ptr::null()),
        error_code,
        0,
        PWSTR(buf.as_mut_ptr()),
        buf.len() as u32,
        Some(std::ptr::null_mut())
    ) as usize;
    buf.truncate(len);
    let message = U16String::from_vec(buf).to_string_lossy();
    eyre!("win32 error 0x{error_code:x} ({error_code}): {message}")
}

unsafe fn do_crimes(args: Args) -> Result<(), Report> {
    // println!("Press enter to begin...");
    // stdin().read_line(&mut String::new())?;

    let pid = args.pid;

    let access =
        PROCESS_CREATE_THREAD |
        PROCESS_QUERY_INFORMATION |
        PROCESS_VM_OPERATION |
        PROCESS_VM_READ |
        PROCESS_VM_WRITE;

    let proc = OpenProcess(access, false, pid)?;
    println!("Opened Process {proc:?}");

    let target_addr = VirtualAllocEx(
        proc,
        Some(std::ptr::null_mut()),
        1024,
        MEM_RESERVE | MEM_COMMIT,
        PAGE_EXECUTE_READWRITE
    );
    if target_addr.is_null() {
        return Err(get_last_error());
    }

    let dll_path_buf = PathBuf::from("../natalib/target/debug/natalib.dll").canonicalize()?;
    let dll_path = U16CString::from_os_str(dll_path_buf)?;

    // println!("Press enter to write dll code to injection site...");
    // stdin().read_line(&mut String::new())?;
    let mut written = 0;
    WriteProcessMemory(
        proc,
        target_addr,
        dll_path.as_ptr() as _,
        (dll_path.len() + 1) * std::mem::size_of::<u16>(),
        Some(&mut written)
    ).ok()?;
    assert!(written > 0);

    let kernel32 = GetModuleHandleW(PCWSTR(u16cstr!("kernel32.dll").as_ptr()))?;
    println!("kernel32 handle: {kernel32:?}");

    let load_library_w = GetProcAddress(kernel32, PCSTR("LoadLibraryW\0".as_ptr())).ok_or(
        eyre!("LoadLibraryW not found!")
    )? as *const ();
    println!("LoadLibraryW address: {load_library_w:p}");

    println!("Press enter to wreak havoc...");
    stdin().read_line(&mut String::new())?;

    let mut tid: u32 = 0;
    let thread = CreateRemoteThread(
        proc,
        Some(std::ptr::null()),
        0,
        Some(std::mem::transmute(load_library_w)),
        Some(target_addr),
        0,
        Some(&mut tid)
    )?;

    println!("Created thread {thread:?}, tid = {tid:?}");
    Ok(())
}