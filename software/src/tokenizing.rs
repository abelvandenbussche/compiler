use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter},
    path::Path,
};

/// Reads a files contents and tokenizes it.
/// Outputs the tokens into a seperate file
pub fn tokenize(file: &Path) -> io::Result<()> {
    let reader = BufReader::new(File::open(file)?);
    // Creating the output file path
    let dir = file.parent();
    let file_name = file.file_stem();
    let output_path = if dir.is_some() && file_name.is_some() {
        dir.unwrap().with_file_name(format!(
            "{}_tokens.txt",
            file_name.unwrap().to_string_lossy()
        ))
    } else {
        return Err(io::Error::other("Invalid file path"));
    };
    let mut writer = BufWriter::new(File::options().write(true).create(true).open(output_path)?);

    let mut chunk = String::new();
    for line in reader.lines() {
        for c in line?.chars() {
            chunk.push(c);

            // Checking if the chunk is meaningfull
        }
    }

    Ok(())
}

/// Internal test module for tokenizing
const TEST_FILE_DIR: &str = "test_files";
mod tests {
    use std::path::PathBuf;

    use crate::tokenizing::TEST_FILE_DIR;

    fn token_test1() {}
}
