use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;

struct PPMImage {
    width: u32,
    height: u32,
    data: Vec<Vec<u8>>
}

fn save_ppm_image(image: &PPMImage, filename: &str) -> std::io::Result<()> {
    let path = Path::new(filename);

    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why.to_string()),
        Ok(file) => file,
    };

    let mut file = LineWriter::new(file);
    file.write(b"P3\n")?;
    file.write(format!("{} {}\n", image.width, image.height).as_bytes())?;
    for row in image.data.iter() {
        for el in row.iter() {
            file.write(format!("{} ", el).as_bytes())?;
        }
        file.write(b"\n")?;
    }

    match file.write_all(image.data.concat().as_slice()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why.to_string()),
        Ok(_) => println!("successfully wrote to {}", path.display()),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn file_is_saved() {
        let img = PPMImage{width: 3, height: 3, data: vec![vec![1, 2, 3], vec![4, 5, 6]]};
        let _ = save_ppm_image(&img, "test.png");

        assert!(Path::new("test.png").exists());

        // Cleanup
        fs::remove_file("test.png").unwrap();
    }

    #[test]
    fn file_is_saved_correctly() {
        let img = PPMImage{width: 3, height: 3, data: vec![vec![1, 2, 3], vec![4, 5, 6]]};
        let _ = save_ppm_image(&img, "test.png");
    }

}