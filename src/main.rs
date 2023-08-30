use std::fs;
use csv::Reader;
use std::path::PathBuf;
use std::env;


fn main() {
    let files = get_csv_files_in_current_folder();
    let document_column = get_document_column();


    for file in files {
        if let Some(values) = process_csv_file(&file, &document_column) {
            // Perform further operations with the extracted non-empty values
            // println!("{:?}", values);
            for value in values {
                let (document_name, download_url) = parse_document_entry(&value);
                println!("Downloading {:?} from URL: {:?}",document_name,download_url);
                download_document(&document_name, &download_url);
            }
        }
    }
}
fn download_document(document_name: &str, download_url: &str) {
    let response = reqwest::blocking::get(download_url)
        .expect("Failed to send request for download.");

    let mut file = fs::File::create(document_name)
        .expect("Failed to create file for download.");

    let mut content = response.bytes().expect("Failed to read response content.");

    std::io::copy(&mut content.as_ref(), &mut file)
        .expect("Failed to write download content to file.");
}
fn parse_document_entry(entry: &str) -> (String, String) {
    let start_pos = entry.find('(').unwrap();
    let end_pos = entry.find(')').unwrap();

    let document_name = entry[..start_pos].trim().to_owned();
    let download_url = entry[start_pos + 1..end_pos].trim().to_owned();

    (document_name, download_url)
}
fn get_csv_files_in_current_folder() -> Vec<PathBuf> {
    let mut csv_files = Vec::new();

    let entries = fs::read_dir(".")
        .expect("Failed to read directory.");

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "csv" {
                    csv_files.push(path);
                }
            }
        }
    }

    csv_files
}

fn get_document_column() -> String {
    match env::args().nth(1) {
        Some(column) => column,
        None => String::from("Document available"),
    }
}


fn process_csv_file(file_path: &PathBuf, document_column: &str) -> Option<Vec<String>> {
    let file = fs::File::open(file_path)
        .expect("Failed to open CSV file.");

    let mut csv_reader = Reader::from_reader(file);

    let headers = csv_reader.headers()
        .expect("Failed to read CSV headers.");

    let document_idx = headers.iter().position(|header| header == document_column);

    if let Some(idx) = document_idx {
        let values: Vec<String> = csv_reader.records()
            .filter_map(|record| record.ok())
            .filter_map(|record| record.get(idx).map(|value| value.to_owned()))
            .filter(|value| !value.is_empty())
            .collect();

        if !values.is_empty() {
            Some(values)
        } else {
            None
        }
    } else {
        None
    }
}