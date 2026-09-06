use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
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
    Ok(())
}
