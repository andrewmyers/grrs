use anyhow::Ok;
use anyhow::{Result};
use std::io::{BufRead, BufReader};

pub fn find_matches<T: std::io::Read>(file_buffer: BufReader<T>, pattern: &str, mut writer: impl std::io::Write) -> Result<()>{
    let mut line_num = 1;

    for line in file_buffer.lines() {
        let contents = line.unwrap();
        if contents.contains(pattern) {
            writeln!(writer, "{}: {}", line_num, contents)?;
        }

        line_num += 1;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Cursor, Seek, SeekFrom, Write};

    #[test]
    fn find_a_match() {

        let mut result = Vec::new();

        let test_string = b"lorem ipsum\ndolor sit amet\nipsum lorem";

        let mut cursor = Cursor::new(Vec::new());
        cursor.write(test_string).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        find_matches(BufReader::new(cursor.clone()), "lorem", &mut result).unwrap();
        assert_eq!(result, b"1: lorem ipsum\n3: ipsum lorem\n");
    }
}