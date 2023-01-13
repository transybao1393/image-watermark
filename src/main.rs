//- module root
use clap::Parser;

mod services;
use services::image_processing::{
    WatermarkInput, 
    add_watermark_by_image_ratio
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Main image absolute path
    #[arg(short, long)]
    image_absolute_path: String,

    /// Watermark image absolute path
    #[arg(short, long)]
    watermark_image_absolute_path: String,

    /// File output path
    #[arg(short, long)]
    output_path: String
}

fn main() {
    let args = Args::parse();

    let watermark_input = WatermarkInput {
        image_absolute_path: args.image_absolute_path.to_owned(),
        watermark_image_absolute_path: args.watermark_image_absolute_path.to_owned(),
        output_path: args.output_path.to_owned()
    };
    match add_watermark_by_image_ratio(&watermark_input) {
        Err(e) => panic!("photon function for image processing failed with error {}", e),
        Ok(msg) => println!("{}", msg)
    }
}