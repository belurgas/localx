// use core::str;
// use std::process::Command;


use app::{handlers::handle_event, widgets::app::{App, RunningState}};
// use chardet::detect;
// use encoding_rs::{IBM866, WINDOWS_1251};
use tokio::runtime::Runtime;
use os_info;

mod app;
mod tui;

fn main() -> color_eyre::Result<()> {
    // Определяем тип системы
    let os = os_info::get();
    let os_type = os.os_type();
    match os_type {
        os_info::Type::Windows => {
            // Запускаем установку шрифтов
            let handle = std::thread::spawn(|| {
                let rt = Runtime::new().unwrap();
                let _ = rt.block_on(app::handlers::flf_grub::download_flf_file());
            });

            handle.join().unwrap();
            println!("Задача завершена");
        },
        _ => {
            println!("Простите, но {} пока не поддерживается", os_type)
        }
    }

    tui::install_panic_hook();

    let mut terminal = tui::init_terminal()?;
    // Инициализация структур
    let mut app = App::new();

    // Запуск цикла
    while app.running_state != RunningState::Done {
        // Рисуем App с помощью имплементрированной view функции (изменить)
        terminal.draw(|f| app.view(f))?;

        // Обработка ивентов
        let mut event = handle_event()?; // Импортирован из asd (в первой версии)
        while event.is_some() {
            event = app.update(event.unwrap()) // Передана структура виджетов в обновления (переделать)
        }
    }
    tui::restore_terminal()?;
    Ok(())
}
