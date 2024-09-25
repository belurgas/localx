use super::windows::locale::get_user_locale;

// Локализация программы
pub mod en;
pub mod ru;

pub enum Local {
    Russian,
    English,
}

pub fn set_locale_language() -> Local {
    let get_locale = get_user_locale();
    if get_locale == "ru-RU" {
        Local::Russian
    } else {
        Local::English
    }
}
