use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;

struct PPMImage {
    width: u32,
    height: u32,
    data: Vec<Vec<PPMPixel>>
}

struct PPMPixel {
    r: u32,
    g: u32,
    b: u32
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
    file.write(b"255\n")?;
    for row in image.data.iter() {
        for el in row.iter() {
            file.write(format!("{} {} {} ", el.r, el.g, el.b).as_bytes())?;
        }
        file.write(b"\n")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn file_is_saved() {
        let img = PPMImage{width: 3, height: 2, data: vec![vec![PPMPixel{r: 1, g: 1, b: 1}, PPMPixel{r: 0, g: 0, b: 0}, PPMPixel{r: 0, g: 1, b: 0}],
        vec![PPMPixel{r: 0, g: 0, b: 0}, PPMPixel{r: 1, g: 1, b: 1}, PPMPixel{r: 0, g: 1, b: 0}],]};
        let _ = save_ppm_image(&img, "test.ppm");

        assert!(Path::new("test.ppm").exists());

        // Cleanup
        // fs::remove_file("test.png").unwrap();
    }

    #[test]
    fn file_is_saved_correctly() {
        let img = PPMImage{width: 3, height: 2, data: vec![vec![PPMPixel{r: 1, g: 1, b: 1}, PPMPixel{r: 0, g: 0, b: 0}, PPMPixel{r: 0, g: 1, b: 0}],
                                                           vec![PPMPixel{r: 0, g: 0, b: 0}, PPMPixel{r: 1, g: 1, b: 1}, PPMPixel{r: 0, g: 1, b: 0}],]};
        let _ = save_ppm_image(&img, "test.ppm");
    }

}