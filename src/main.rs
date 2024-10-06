use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;


fn example1(output_file: PathBuf) -> io::Result<()> {
    let image_width: u16 = 256;
    let image_height: u16 = 256;


    let f = File::create(output_file)?;
    let mut writer = io::BufWriter::new(f);
    
    writer.write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for y in 0..image_height {
        for x in 0..image_width {
            let r = (x as f32) / ((image_width - 1) as f32);
            let g = (y as f32) / ((image_height -1) as f32);
            let b = 0f32;

            let r_int: u8 = (r * 255.99) as u8;
            let g_int: u8 = (g * 255.99) as u8;
            let b_int: u8 = (b * 255.99) as u8;

            writer.write(format!("{r_int} {g_int} {b_int}\n").as_bytes())?;
        }
    }

    Ok(())
}


fn main() -> io::Result<()> {
    let output_folder = PathBuf::from_str("output/").expect("Failed to set output folder/");
    
    // Check if the folder already exists
    if fs::metadata(output_folder.clone()).is_ok() {
        println!("Folder already exists.");
    } else {
        // Create a new directory
        if fs::create_dir(output_folder.as_path()).is_ok() {
            println!("Folder created successfully.");
        } else {
            eprintln!("Failed to create folder");
        }
    }

    example1(output_folder.join("example1.ppm"))?;

    Ok(())
}
