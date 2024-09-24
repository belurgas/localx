use std::{
    fs::{self, File},
    io::copy,
    path::Path,
};

use reqwest::get;

pub async fn download_flf_file() -> Result<(), Box<dyn std::error::Error>> {
    // Ссылка на FLF файл с GitHub
    let url = "https://raw.githubusercontent.com/xero/figlet-fonts/master/ANSI%20Regular.flf";

    let dir_path = Path::new(r"C:/Program Files/LocalX/Fonts");

    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
        // Сделать логирование
        println!("Каталог создан: {:?}", dir_path);
    }

    // Полный путь к файлу
    let file_path = dir_path.join("ANSI.flf");

    // Проверяем существует ли файл
    if file_path.exists() {
        // Файл уже существует (Логирование)
        println!("Файд уже существует: {:?}", file_path);
        return Ok(());
    }

    // Скачиваем файл с github
    let response = get(url).await?;
    if response.status().is_success() {
        // Открываем файл для записи
        let mut file = File::create(&file_path)?;
        copy(&mut response.bytes().await?.as_ref(), &mut file)?;
        println!("Файл успешно загружен: {:?}", file_path);
    } else {
        println!("Ошибка при загрузке файла: {}", response.status());
    }
    Ok(())
}
