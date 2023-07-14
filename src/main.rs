mod services;

//- module root
use clap::{
    Args, 
    Parser,
    Subcommand,
};

use services::image_processing::{
    WatermarkInput, 
    TextWatermarkInput,
    add_watermark_by_image_ratio,
    add_text_to_image
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct ImageWatermarkArgs {
    #[clap(subcommand)]
    watermark_type: WatermarkType
}

#[derive(Debug, Subcommand)]
enum WatermarkType {
    /// Image-in-image watermark
    Image(ImageCommand),

    /// Text-in-image watermark
    Text(TextCommand),
}

#[derive(Debug, Args)]
struct ImageCommand {
    /// Main image absolute path
    #[arg(short = 'i', long)]
    image_absolute_path: String,

    /// Watermark image absolute path
    #[arg(short, long)]
    watermark_image_absolute_path: String,

    /// File output path
    #[arg(short = 'o', long)]
    image_output_path: String,
}

#[derive(Debug, Args)]
struct TextCommand {
    /// Main image absolute path
    #[arg(short = 'i', long = "absolute_path")]
    image_text_absolute_path: String,

    /// Watermark custom text
    #[arg(short = 't', long)]
    custom_text: String,
}

fn main() {
    let args = ImageWatermarkArgs::parse();

    match &args.watermark_type {
        WatermarkType::Image(
            ImageCommand {
                image_absolute_path,
                watermark_image_absolute_path,
                image_output_path
            }
        ) => {
            let watermark_input = WatermarkInput {
                image_absolute_path: image_absolute_path.to_owned(),
                watermark_image_absolute_path: watermark_image_absolute_path.to_owned(),
                output_path: image_output_path.to_owned()
            };

            //- performance accelerate
            // use std::time::Instant;
            // let start = Instant::now();
            match add_watermark_by_image_ratio(&watermark_input) {
                Err(e) => panic!("Image-to-image processing failed with error {}", e),
                Ok(msg) => println!("{}", msg)
            }
            // let duration = start.elapsed();
            // println!("Time elapsed in add_watermark_by_image_ratio() is: {:?}", duration);
        },
        WatermarkType::Text(
            TextCommand { 
                image_text_absolute_path, 
                custom_text, 
            }
        ) => {
            let text_watermark_input = TextWatermarkInput {
                image_absolute_path: image_text_absolute_path.to_owned(),
                custom_text: custom_text.to_owned(),
            };
            match add_text_to_image(&text_watermark_input) {
                Ok(t) => println!("{:?}", t),
                Err(image_error) => println!("Text-to-image processing failed with error {:?}", image_error)
            }
        },
    }
}