use std::fs::{remove_file, File};
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use crate::StdResult;

pub struct FileManager<'a> {
    file_path: &'a str,
}

impl<'a> FileManager<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self { file_path }
    }

    fn check_if_file_exists(&self) -> bool {
        Path::new(self.file_path).exists()
    }

    fn read_to_string(&self, file: &mut File) -> StdResult<String> {
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;

        Ok(file_string)
    }

    fn create_file(&self) -> StdResult<File> {
        let file = File::create(self.file_path)?;
        Ok(file)
    }

    pub fn delete_file(&self) -> StdResult<()> {
        remove_file(self.file_path)?;
        Ok(())
    }

    pub fn get_file_string(&self) -> Option<String> {
        use std::io::ErrorKind;

        let file = File::open(self.file_path);

        match file {
            Ok(mut file) => {
                let file_string = self.read_to_string(&mut file);

                match file_string {
                    Ok(file_string) => Some(file_string),
                    Err(err) => panic!(
                        "Unexpected error while reading file data to string: {}",
                        err
                    ),
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => None,
                err => panic!("Unexpected error while opening file: {}", err),
            },
        }
    }

    pub fn write_to_file(&self, data: String) -> StdResult<()> {
        let file_exists = self.check_if_file_exists();

        if file_exists {
            self.delete_file()?;
        }

        let file = self.create_file()?;
        let mut file_bufer_writer = BufWriter::new(file);

        file_bufer_writer.write_all(data.as_bytes())?;

        Ok(())
    }
}
