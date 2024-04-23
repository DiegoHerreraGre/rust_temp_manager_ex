use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> { // Use `Result` as the return type
    // Correct file path for macOS not known, you must replace `temp_file_path` with valid path according to your system
    let temp_file_path = Path::new("/path/to/temperature/file/on/macOS");
    let mut temp_file = File::open(temp_file_path)?; // The `?` operator will propagate the error up to caller

    let mut cpu_temp_string = String::new();
    temp_file.read_to_string(&mut cpu_temp_string)?;

    // Handle the Result returned by `parse`
    let cpu_temp: u32 = match cpu_temp_string.trim().parse() {
        Ok(temp) => temp,
        Err(_) => {
            eprintln!("Failed to parse temperature!");
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse temperature!"));
        },
    };

    let cpu_temp_celsius = cpu_temp as f32 / 1000.0;
    println!("CPU Temperature: {:.2}°C", cpu_temp_celsius);

    Ok(())
}