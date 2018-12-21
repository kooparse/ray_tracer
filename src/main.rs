use std::fs::File;
use std::io::LineWriter;
use std::io::Write;

const OUTPUT_NAME: &str = "output.ppm";
const FILE_PATH: &str = "assets/";
const NB_COLUMN: i32 = 100;
const NB_ROW: i32 = 200;

fn main() -> std::io::Result<()> {
    // First, some cleanup.
    let _ = std::fs::remove_dir_all(FILE_PATH);

    // Create our empty file.
    let _ = std::fs::create_dir(FILE_PATH)?;
    let file = File::create(format!("{}{}", FILE_PATH, OUTPUT_NAME))?;
    let mut file = LineWriter::new(file);

    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", NB_ROW, NB_COLUMN).as_bytes())?;
    file.write_all(b"255\n")?;

    for _ in 0..NB_COLUMN - 1 {
        for _ in 0..NB_ROW - 1 {
            file.write_all(b"255 0 255\n")?;
        }
    }

    Ok(())
}
