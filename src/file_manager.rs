use std::fs::File;
use std::io::Read;

pub struct FileManager<'a> {
    file_path: &'a str,
}

impl<'a> FileManager<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self { file_path }
    }

    fn read_to_string(&self, file: &mut File) -> crate::StdResult<String> {
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;

        Ok(file_string)
    }

    pub fn read_file(&self) -> Option<String> {
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

    pub fn write_to_file(self, data: String) {}
}
