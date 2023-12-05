use std::path::Path;

use clap::Parser;
use anyhow::Result;
use image::ImageFormat;
use url::Url;
use web2image::web2image;

#[derive(Parser, Debug)]
#[clap(author="", version="0.1")]
struct Opts {
    // output file
    #[clap(short, long,  default_value = "/tmp/snapshot.jpg", value_parser = valid_filename)]
    output: String,
    // url to cpature
    #[clap(short, long, value_parser = valid_url)]
    url: String,
}

fn valid_filename(name: &str) -> Result<String, String>  {
    let path = Path::new(name);
    let parent = path.parent().and_then(|p|p.is_dir().then(||p));
    let ext = get_image_format(&path);
    if parent.is_none() || ext.is_none() {
        return Err("File path must be exists and file must be jpg, jpeg or png.".into());
    }
    Ok(name.to_string())
}

fn valid_url(url: &str) -> Result<String, String>  {
    match Url::parse(url) {
        Ok(_) => Ok(url.to_string()),
        Err(_) => Err("You must provide a valid url.".into()),
    }
}

fn get_image_format(path: &Path) -> Option<ImageFormat> {
    path.extension()
        .and_then(|p| p.to_str())
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
                "png" => Some(ImageFormat::Png),
                _ => None,
            }
        })
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:#?}", opts);

    let format = get_image_format(Path::new(&opts.output)).unwrap();
    web2image(&opts.url, &opts.output, format).unwrap()
}
