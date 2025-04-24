use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_dir = String::new(); // Входная папка

    print!("Введите путь к папке: ");
    std::io::stdout().lock().flush().expect("Ошибка");
    std::io::stdin()
        .read_line(&mut input_dir)
        .expect("Не удалось обработать строку");

    input_dir = input_dir.trim().to_string();

    let audio_files = find_audio_files(input_dir.as_str())?;
    let mut i = 0;
    for file_path in audio_files {
        i += 1;
        println!("[{}]: {}", i, file_path.display());
    }

    Ok(())
}

fn find_audio_files(dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut audio_files = Vec::new();
    let audio_extensions = ["mp3", "flac", "wav", "ogg", "m4a"];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            audio_files.extend(find_audio_files(path.to_str().unwrap())?);
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if audio_extensions.contains(&ext.to_lowercase().as_str()) {
                audio_files.push(path);
            }
        }
    }

    Ok(audio_files)
}
