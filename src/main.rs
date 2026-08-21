use raylib::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write, stderr};
use std::process;

mod vec3;
use vec3::{Vec3,Colour,Point3,dot};
mod ray;
use ray::{Ray};

fn hit_sphere(center : Point3, radius: f64, r: &Ray) -> Option<f64>{
    let oc = center - *r.origin();
    let a = dot(*r.direction(), *r.direction());
    let b = -2.0 * dot(*r.direction(), oc);
    let c = dot(oc, oc) - radius * radius;

    let D = b * b - 4.0 * a * c;
    if D < 0.0 {
        None 
    }
    else{
        Some((-b-D.sqrt()) / (2.0 * a))
    }
}
// take a ray as input -> calculate its colour -> return black 
fn ray_color(r: &Ray) -> Colour{
    //Color::new(0.0,0.0,0.0)
    let sph = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r);
    if let Some(t) = sph {
        let N = Vec3::unit(&(r.at(t) - Vec3::new(0.0,0.0,-1.0))) * 1.0;
        Colour::new(N.x()+1.0, N.y()+1.0, N.z()+1.0) * 0.5
        
    }
    else {
        let unit_dir = Vec3::unit(r.direction());
        let a = 0.5*(unit_dir.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0)*(1.0-a) + Colour::new(0.5, 0.7, 1.0)*a

    }
    //let unit_dir = r.direction().unit();
    // let unit_dir = Vec3::unit(r.direction());
    // let a = 0.5*(unit_dir.y() + 1.0);
    // Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a
}

fn raytracer() -> std::io::Result<()> {
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

fn ppmgenerator() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <file_path>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let p3 = &lines[0];
    let mut parts = lines[1].split_whitespace();
    let width : i32 = parts.next().expect("width not available").parse().expect("some issue with width");
    let height : i32 = parts.next().expect("height not available").parse().expect("some issue with height");
    let max_color: f32 = lines[2].trim().parse().expect("some issue with max color value");

    let rem = lines[3..].join(" ");
    let mut tokens = rem.split_whitespace();
    let mut pixels: Vec<Color> = Vec::with_capacity((width * height) as usize);

    while let (Some(r_str), Some(g_str), Some(b_str)) = (tokens.next(), tokens.next(), tokens.next()) {
        let r_raw: f32 = r_str.parse().unwrap_or(0.0);
        let g_raw: f32 = g_str.parse().unwrap_or(0.0);
        let b_raw: f32 = b_str.parse().unwrap_or(0.0);

        // Normalize color values
        let r = ((r_raw / max_color) * 255.0) as u8;
        let g = ((g_raw / max_color) * 255.0) as u8;
        let b = ((b_raw / max_color) * 255.0) as u8;

        pixels.push(Color::new(r, g, b, 255));
    }

    let (mut rl, thread) = raylib::init().size(width,height).title("ppm viewer").build();

    rl.set_target_fps(60);
    while !rl.window_should_close(){
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        let mut y_offset = 20;
        //for line in &lines {

            //d.draw_text(line, 20, y_offset, 18, Color::LIGHTGRAY);
            //y_offset+=22;
            for y in 0..height {
                for x in 0..width {
                    let index = (x + y * width) as usize;
                    if let Some(&color) = pixels.get(index) {
                        d.draw_pixel(x, y, color);
                    }
                }
            }
     

    }

    Ok(())

}

fn main() -> std::io::Result<()> {
    raytracer()?;
    ppmgenerator()?;

    Ok(())
}
