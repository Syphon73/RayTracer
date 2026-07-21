use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);

    let img_width = 256;
    let img_height = 256;

    writeln!(writer, "P3")?;
    writeln!(writer, "{img_width} {img_height}")?;
    writeln!(writer, "255")?;
    
    
    for j in 0..img_height {
        for i in 0..img_width {
            let r = i as f64 / (img_width - 1) as f64;
            let g = j as f64 / (img_height - 1) as f64;
            let b = 0.0;

            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;

            writeln!(writer, "{ir} {ig} {ib}")?;
        }
    }


    writer.flush()?;
    Ok(())
}
