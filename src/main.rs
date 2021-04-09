mod utils;
mod pallete;
// mod logger;

use image::{imageops, ImageBuffer, GenericImageView};
use clap::{AppSettings, Clap};
#[derive(Clap)]
#[clap(version = "0.1", author = "4t145 <u4t145@163.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    // pixelize {path} {max-width} {max-height} -p {pallete path} -o {outdir}
    path: String,
    width: u32,
    height: u32,
    #[clap(short, long)]
    pallete: String,
    #[clap(short, long, default_value = "./")]
    outdir: String,
    #[clap(short, long)]
    method: Option<String>
}

fn main() -> Result<(), String>{
    use std::path::Path;
    let opts: Opts = Opts::parse();
    // check options

    println!("\ncheck options...");
    let outdir = Path::new(&opts.outdir);
    if !outdir.is_dir() {
        return Err(format!("ERROR: 输出路径:{} 不合法!", &opts.outdir));
    }
    use imageops::FilterType::*;
    let filter = 
    if let Some(method) = opts.method {
        match method.as_str() {
            "n"|"nearest" => Nearest,
            "t"|"triangle" => Triangle,
            "c"|"catmull-rom" => CatmullRom,
            "g"|"gaussian" => Gaussian,
            "l"|"lanczos3" => Lanczos3,
            _ => {
                println!("⚠WARN: 未知的滤镜:{}, 使用默认的高斯滤镜", method);
                Gaussian
            }
        } 
    } else {
        Gaussian
    };
    println!("✔️all is well");

    println!("\nreading pallete...");
    let pallete_path = Path::new(&opts.pallete);
    let p = pallete::get_pallete(pallete_path)?;
    println!("✔️completed");

    println!("\nopening image...");
    let input_path = Path::new(&opts.path);
    let img = image::open(input_path).expect("ERROR: 打开文件raw.png错误, 应检查该文件是否存在?");
    let (w,h) = img.dimensions();
    println!("✔️raw img:\t width:{}, height:{}", w, h);

    println!("\nresizing image...");
    let resized = img.resize(opts.width, opts.height, filter);
    let (r_w, r_h) = resized.dimensions();
    let mut imgbuf = ImageBuffer::new(r_w, r_h);
    println!("✔️resized img:\t width:{}, height:{}", r_w, r_h);

    println!("\nprocessing...");
    let mut stat = std::collections::HashMap::<usize, usize>::new();
    for x in 0..r_w {
        for y in 0..r_h {
            let c = resized.get_pixel(x, y);
            let (m, ind) = utils::map_color(c, &p);
            if let Some(count) = stat.get_mut(&ind) {
                *count+=1;
            } else {
                stat.insert(ind, 1);
            }
            *imgbuf.get_pixel_mut(x, y) = m;
        }
    }
    println!("✔️completed");

    println!("\noutputing result...");
    let outdir_str = outdir.to_str().unwrap_or("./");
    let name = input_path.file_stem().unwrap().to_str().unwrap_or("output");

    let resized_path = format!("{o}/{n}_resized.png", o=outdir_str, n=name.to_owned());
    let pixelized_path = format!("{o}/{n}_pixelized.png", o=outdir_str, n=name.to_owned());
    resized.save(&resized_path).map_err(|e|format!("ERROR: 保存文件{}错误, {}", resized_path, e))?;
    imgbuf.save(&pixelized_path).map_err(|e|format!("ERROR: 保存文件{}错误, {}", pixelized_path, e))?;
    println!("✔️completed");

    println!("\n #索引色统计:");
    stat.iter().for_each(|(ind,count)|println!("颜色:{}, 使用{}次", ind+1, count));
    Ok(())
}




