use std::fs::File;
use std::io::{BufWriter, Write, stderr};

mod vec3;
use vec3::{Vec3,Color,Point3};
mod ray;
use ray::{Ray};

// take a ray as input -> calculate its colour -> return black 
fn ray_color(r: &Ray) -> Color{
    //Color::new(0.0,0.0,0.0)
    //let unit_dir = r.direction().unit();
    let unit_dir = Vec3::unit(r.direction());
    let a = 0.5*(unit_dir.y() + 1.0);
    Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a
}

fn main() -> std::io::Result<()> {
    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    // new error log buffer class
    let mut clog = BufWriter::new(stderr());

    //----image ppm generator --------
    let aspect_ratio = 16.0/9.0;
    let img_width = 256;


    let mut img_height = (img_width as f64 / aspect_ratio) as i32;
    if img_height < 1 {
        img_height = 1;
    }
    
    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;

    let viewport_width = viewport_height * ((img_width) as f64 /img_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewportU = Vec3::new(viewport_width, 0.0, 0.0);
    let viewportV = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_deltaU = viewportU / img_width as f64;
    let pixel_deltaV = viewportV / img_height as f64;

    //starting pt: upper left pixel
    let pixel_upperleft = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewportU/2.0 - viewportV/2.0;
    let Q = pixel_upperleft + (pixel_deltaU + pixel_deltaV) * 0.5 as f64;


    //Renderer
    writeln!(writer, "P3")?;
    writeln!(writer, "{img_width} {img_height}")?;
    writeln!(writer, "255")?;
    
    for j in 0..img_height {
        writeln!(clog, "Scan lines remaining: {}", img_height - j)?;
        clog.flush()?;
        for i in 0..img_width {
            // let r = i as f64 / (img_width - 1) as f64;
            // let g = j as f64 / (img_height - 1) as f64;
            // let b = 0.0;
            let pixel_center = Q + (pixel_deltaU * i as f64) + (pixel_deltaV * j as f64);
            let ray_direction = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_direction);

            let pixel = ray_color(&r);
            Vec3::write_color(pixel,&mut writer)?;
        }
    }
    writeln!(clog, "Done!!")?;
    clog.flush()?;
    writer.flush()?;
    Ok(())
}
