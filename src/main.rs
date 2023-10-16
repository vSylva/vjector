//! # 中文
//!
//! [![Crates.io](https://img.shields.io/crates/v/vjector)](https://crates.io/crates/vjector)
//!
//! "vjector"是一个命令行实用程序，允许您将DLL注入目标进程或卸载已加载的DLL
//!
//! ```usage
//! Usage: vjector.exe <i|inject|e|eject> <executable> <library>
//!
//! Example: vjector.exe i test.exe test.dll
//! Example: vjector.exe e test.exe test.dll
//! ```
//!
//! # English
//!
//! [![Crates.io](https://img.shields.io/crates/v/vjector)](https://crates.io/crates/vjector)
//!
//! "vjector" is a command-line utility that allows you to inject a DLL into a target process or unload a DLL that is already loaded
//!
//! ```usage
//! Usage: vjector.exe <i|inject|e|eject> <executable> <library>
//!
//! Example: vjector.exe i test.exe test.dll
//! Example: vjector.exe e test.exe test.dll
//! ```

const HELP_TEXT: &'static str = "Usage: vjector.exe <i|inject|e|eject> <executable> <library>

Example: vjector.exe i test.exe test.dll
Example: vjector.exe e test.exe test.dll
";

struct ProcessHandle(*mut core::ffi::c_void);

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        vcheat::process::close_handle_unchecked(self.0);
    }
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 3 {
        println!("{}", HELP_TEXT);
        return;
    }

    match args[0].as_str() {
        "i" | "inject" | "e" | "eject" => (),
        _ => {
            return println!(
                "The first argument is invalid. It must be 'i', 'inject', 'e', or 'eject'"
            )
        }
    }

    let process_info = match vcheat::process::get_process_info(&args[1]) {
        Ok(ok) => ok,
        _ => return println!("Failed to retrieve process information"),
    };

    let process_handle = match vcheat::process::open_process(process_info.id) {
        Ok(ok) => ProcessHandle(ok),
        _ => return println!("Failed to obtain handle to the process"),
    };

    if args[0] == "i" || args[0] == "inject" {
        if let Err(_) = vcheat::module::inject_dll(process_handle.0, &args[2]) {
            println!("Failed to inject DLL into the process");

            return;
        }
    }

    if args[0] == "e" || args[0] == "eject" {
        let module_info = match vcheat::module::get_module_info(process_info.id, &args[2]) {
            Ok(ok) => ok,
            _ => return println!("Failed to retrieve module information"),
        };

        if let Err(_) = vcheat::module::eject_dll(process_handle.0, module_info.handle) {
            println!("Failed to unload DLL from the process");
            return;
        }
    }
}
