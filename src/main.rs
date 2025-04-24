use id3::{Tag, TagLike};
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Конфигурация (можно заменить на аргументы командной строки)
    let mut input_dir = String::new(); // Входная папка
    let mut output_dir = String::new(); // Выходная папка

    print!("Введите путь к входной папке: ");
    std::io::stdout().lock().flush().expect("Ошибка");
    std::io::stdin()
        .read_line(&mut input_dir)
        .expect("Не удалось обработать строку");

    print!("Введите путь к выходной папке: ");
    std::io::stdout().lock().flush().expect("Ошибка");
    std::io::stdin()
        .read_line(&mut output_dir)
        .expect("Не удалось обработать строку");

    input_dir = input_dir.trim().to_string();
    output_dir = output_dir.trim().to_string();

    // Создаем выходную папку, если её нет
    fs::create_dir_all(output_dir.as_str())?;

    // Собираем все аудиофайлы
    let audio_files = find_audio_files(input_dir.as_str())?;
    println!("Найдено {} аудиофайлов", audio_files.len());

    // Храним уникальные треки (по названию и автору)
    let mut unique_tracks = HashSet::new();
    let mut tracks_info = Vec::new();

    for file_path in audio_files {
        if let Some(tag) = Tag::read_from_path(&file_path).ok() {
            let title = tag.title().unwrap_or("Неизвестный трек").to_string();
            let artist = tag
                .artist()
                .unwrap_or("Неизвестный исполнитель")
                .to_string();

            let file_name = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let track_key = (title.clone(), artist.clone());

            if !unique_tracks.contains(&track_key) {
                unique_tracks.insert(track_key);
                tracks_info.push((artist, title, file_path, file_name));
            }
        }
    }

    println!("Уникальных треков: {}", tracks_info.len());

    // Группируем по исполнителям
    let mut artist_map = std::collections::HashMap::new();
    for (artist, title, path, file_name) in tracks_info {
        artist_map
            .entry(artist)
            .or_insert_with(Vec::new)
            .push((title, path, file_name));
    }

    // Создаем папки для исполнителей и перемещаем файлы
    for (artist, tracks) in artist_map {
        if tracks.len() > 1 {
            let artist_dir = Path::new(output_dir.as_str()).join(&artist);
            fs::create_dir_all(&artist_dir)?;

            for (_, src_path, file_name) in tracks {
                let dest_path = artist_dir.join(&file_name);

                if let Err(e) = fs::copy(&src_path, &dest_path) {
                    eprintln!("Ошибка при копировании {}: {}", src_path.display(), e);
                    continue;
                }
                println!(
                    "Скопировано: {} -> {}",
                    src_path.display(),
                    dest_path.display()
                );
            }
        } else {
            // Для исполнителей с одним треком копируем прямо в выходную папку
            let (_, src_path, file_name) = &tracks[0];
            let dest_path = Path::new(output_dir.as_str()).join(file_name);

            if let Err(e) = fs::copy(src_path, &dest_path) {
                eprintln!("Ошибка при копировании {}: {}", src_path.display(), e);
                continue;
            }
            println!(
                "Скопировано: {} -> {}",
                src_path.display(),
                dest_path.display()
            );
        }
    }

    Ok(())
}

// Поиск аудиофайлов в директории и поддиректориях
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
