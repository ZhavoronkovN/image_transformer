use image::io::Reader as ImageReader;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::time::{SystemTime};
use std::io::Write;
use std::io::Read;
use weezl::BitOrder as LZWBitOrder;
use weezl::encode::Encoder as LZWEncoder;

struct RawImage(Vec<u8>);
struct Compresser
{
    name : String,
    func : fn (RawImage) -> Result<RawImage, std::io::Error>
}
#[derive(PartialEq,Copy,Clone)]
enum RgbColors
{
    Red, Green, Blue, All
}

fn main() {
    println!("Initialization");
    let now = SystemTime::now();
    let working_file = "images/defaults/default.bmp".to_string();
    let empty_comp = Compresser {name : "standard".to_string(), func: empty_compresser};
    let lzw_comp = Compresser {name : "LZW".to_string(), func: lzw_compresser};
    let rle_comp = Compresser {name : "RLE".to_string(), func : rle_compresser};
    println!("\n1) Transforming tiff\n");
    transform(&working_file, &"images/compression/lzw.tiff".to_string(), &lzw_comp).unwrap();
    println!("\n2) Transforming jpeg\n");
    transform(&working_file, &"images/compression/standard.jpeg".to_string(), &empty_comp).unwrap();
    println!("\n3) Transforming bmp\n");
    transform(&working_file, &"images/compression/rle.bmp".to_string(), &rle_comp).unwrap();

    println!("\n4) Removing colors bmp");
    remove_color(&"images/defaults/default.bmp".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/bmp/all.bmp".to_string(),RgbColors::All).expect("Can't");
    remove_color(&"images/defaults/default.bmp".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/bmp/red.bmp".to_string(),RgbColors::Red).expect("Can't");
    remove_color(&"images/defaults/default.bmp".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/bmp/blue.bmp".to_string(),RgbColors::Blue).expect("Can't");
    remove_color(&"images/defaults/default.bmp".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/bmp/green.bmp".to_string(),RgbColors::Green).expect("Can't");
    
    println!("\n5) Removing colors tiff");
    remove_color(&"images/defaults/default.tiff".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/tiff/all.tiff".to_string(),RgbColors::All).expect("Can't");
    remove_color(&"images/defaults/default.tiff".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/tiff/red.tiff".to_string(),RgbColors::Red).expect("Can't");
    remove_color(&"images/defaults/default.tiff".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/tiff/blue.tiff".to_string(),RgbColors::Blue).expect("Can't");
    remove_color(&"images/defaults/default.tiff".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/tiff/green.tiff".to_string(),RgbColors::Green).expect("Can't");
    
    println!("\n6) Removing colors jpeg");
    remove_color(&"images/defaults/default.jpeg".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/jpeg/all.jpeg".to_string(),RgbColors::All).expect("Can't");
    remove_color(&"images/defaults/default.jpeg".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/jpeg/red.jpeg".to_string(),RgbColors::Red).expect("Can't");
    remove_color(&"images/defaults/default.jpeg".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/jpeg/blue.jpeg".to_string(),RgbColors::Blue).expect("Can't");
    remove_color(&"images/defaults/default.jpeg".to_string(),&"images/defaults/default.jpeg".to_string(),&"images/color_removed/jpeg/green.jpeg".to_string(),RgbColors::Green).expect("Can't");

    println!("Time passed : {}s", now.elapsed().unwrap().as_secs())
}

fn empty_compresser(image : RawImage) -> Result<RawImage, std::io::Error>
{
    Ok(image)
}

fn lzw_compresser(image : RawImage) -> Result<RawImage, std::io::Error>
{
    let mut compressed = vec!();
    let mut enc = LZWEncoder::with_tiff_size_switch(LZWBitOrder::Lsb, 8);
    let result = enc.into_stream(&mut compressed).encode(&image.0[..]).status;
    match result
    {
        Ok(()) => Ok(RawImage(compressed)),
        _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unable to compress data using LZW method"))
    }
}

fn rle_compresser(image : RawImage) -> Result<RawImage, std::io::Error>
{
    let mut compressed = String::new();
    let delta_image = delta_compresser(&"images/defaults/default.bmp".to_string())?;
    let mut last_number = delta_image[0].clone();
    let mut count = vec![1,1,1];
    for numbers in &delta_image[1..]
    {
        for i in 1..3
        {
            if numbers[i] == last_number[i]
            {
                count[i] +=1;
            }
            else
            {
                compressed.push_str(format!("|{}*{}", &count[i], last_number[i]).as_str());
                count[i] = 1;
                last_number[i] = numbers[i];
            }
        }
    }
    Ok(RawImage(compressed.as_bytes().to_vec()))
}

fn delta_compresser(image : &String) -> Result<Vec<[i16; 3]>, std::io::Error>
{
    let mut first_image = ImageReader::open(&image).expect("Can't open first image").decode().expect("Can't decode first image").to_rgb8();;
    let mut pixels = first_image.pixels_mut();
    let mut compressed : Vec<[i16; 3]> = vec!();
    let mut last_pix = vec![255 as i16,255 as i16,255 as i16];
    for pix in pixels
    {
        compressed.push([(last_pix[0] - pix[0] as i16).clone(), (last_pix[1] - pix[1] as i16).clone(), (last_pix[2] - pix[2] as i16).clone()]);
        last_pix = vec![pix[0] as i16, pix[1] as i16, pix[2] as i16];
    }
    Ok(compressed)
}

fn transform(from : &String, to : &String, comp : &Compresser) -> Result<(),std::io::Error>
{
    let format = Path::new(&to).extension().unwrap().to_string_lossy();
    let mut now = SystemTime::now();
    let open_image = ImageReader::open(&from).expect("Can't open image");
    println!("Opening {} image. Time elapsed : {}ms", &format, now.elapsed().unwrap().as_millis());
    now = SystemTime::now();
    let image = open_image.decode().expect("Can't decode image");
    println!("Decoding {} image. Time elapsed : {}ms", &format, now.elapsed().unwrap().as_millis());
    now = SystemTime::now();
    image.save(&to).expect("Unable to save image");
    println!("Saving {} image without compression. Time elapsed : {}ms", &format, now.elapsed().unwrap().as_millis());
    let uncompr_size = fs::metadata(&to).unwrap().len() as f64;
    let mut bytes : Vec<u8> = Vec::new();
    let mut file = File::open(&to).expect("Cannot open file");
    file.read_to_end(&mut bytes).expect("Cannot read file");
    now = SystemTime::now();
    let compressed = (comp.func)(RawImage(bytes))?;
    let mut file_out = std::fs::File::create(&to).expect("Cannot create output file");
    file_out.write_all(&compressed.0).expect("Cannot write bytes to file");
    println!("Saving {} image with {} compression. Time elapsed : {}ms", &format, &comp.name, now.elapsed().unwrap().as_millis());
    let compr_size = fs::metadata(&to).unwrap().len() as f64;
    println!("compression rate {}%",((100 as f64 - (compr_size * 100 as f64/uncompr_size).round()) as i16 +100) % 100);
    Ok(())
}

fn remove_color (first : &String, second : &String, output : &String, what : RgbColors) -> Result<(), std::io::Error>
{
    let mut first_image = ImageReader::open(&first).expect("Can't open first image").decode().expect("Can't decode first image").to_rgb8();
    let second_image = ImageReader::open(&second).expect("Can't open second image").decode().expect("Can't decode second image").to_rgb8();
    let mut mse : i64 = 0;
    let mut lost : i64 = 0;
    let mut count : i64 = 0;
    for (fp,sp) in first_image.pixels_mut().zip(second_image.pixels())
    {
        if what.clone() == RgbColors::All
        {
            for i in 0..3
            {
                let temp = fp[i].clone();
                let val = fp[i] as i32 - sp[i] as i32;
                mse += val as i64 * val as i64;
                fp[i] = fp[i].wrapping_sub(sp[i]);
                lost += temp as i64 - fp[i] as i64;
            }
        }
        else
        {
            let temp = fp[what.clone() as usize].clone();
            fp[0] = 0;
            fp[1] = 0;
            fp[2] = 0;
            let val = temp as i32 - sp[what.clone() as usize] as i32;
            mse += val as i64 * val as i64;
            fp[what.clone() as usize] = temp.wrapping_sub(sp[what.clone() as usize]);
            lost += temp as i64 - fp[what.clone() as usize] as i64; 
        }
        count +=1;
    }
    mse /= count;
    println!("For file : {}\n1) MSE : {}\n2) Lost : {}", &output, mse, lost);
    first_image.save(&output).expect("Error while saving image");
    Ok(())
}