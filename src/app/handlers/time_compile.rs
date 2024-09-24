// Временный файл который подлежит удалению
// Требуется для реализации состояния CrossCompile

use os_info;

pub fn get_os_info() -> (String, String, String, String) {
    let info = os_info::get();
    // let os = info.edition();

    let arch = match info.architecture() {
        Some(arch) => String::from(arch),
        None => String::from("Not Identefined Architecture"),
    };

    (info.os_type().to_string(), info.version().to_string(), info.bitness().to_string(), arch)
    }
