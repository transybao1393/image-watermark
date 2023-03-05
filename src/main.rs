//- module root
use clap::{
    Args, 
    Parser,
    Subcommand,
};

mod services;
use services::image_processing::{
    WatermarkInput, 
    add_watermark_by_image_ratio
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct ImageWatermarkArgs {
    /// watermark type
    #[clap(subcommand)]
    watermark_type: WatermarkType
}

// Step 1: Choose image to text
#[derive(Debug, Subcommand)]
enum WatermarkType {
    /// Image-in-image watermark
    Image(ImageCommand),

    /// Text-in-image watermark
    Text(TextCommand),
}

// Step 2: Choose Image
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

// Step 3: Choose Text
#[derive(Debug, Args)]
struct TextCommand {
    /// Main image absolute path
    #[arg(short = 'i', long = "absolute_path")]
    image_text_absolute_path: String,

    /// Watermark custom text
    #[arg(short = 't', long)]
    custom_text: String,

    /// File output path
    #[arg(short = 'o', long = "output_path")]
    image_text_output_path: String,
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
            println!("image_absolute_path {:?}, watermark_image_absolute_path {:?}, output_path {:?}", image_absolute_path, watermark_image_absolute_path, image_output_path);
            let watermark_input = WatermarkInput {
                image_absolute_path: image_absolute_path.to_owned(),
                watermark_image_absolute_path: watermark_image_absolute_path.to_owned(),
                output_path: image_output_path.to_owned()
            };
            match add_watermark_by_image_ratio(&watermark_input) {
                Err(e) => panic!("Image-to-image processing failed with error {}", e),
                Ok(msg) => println!("{}", msg)
            }
        },
        WatermarkType::Text(
            TextCommand { 
                image_text_absolute_path, 
                custom_text, 
                image_text_output_path }
        ) => {
            println!("image_absolute_path {:?}, watermark_image_absolute_path {:?}, output_path {:?}", image_text_absolute_path, custom_text, image_text_output_path);
        },
    }



    // let imageCommand = "";
    // let TextCommand = "";

    // let watermark_input = WatermarkInput {
    //     image_absolute_path: args.image_absolute_path.to_owned(),
    //     watermark_image_absolute_path: args.watermark_image_absolute_path.to_owned(),
    //     output_path: args.output_path.to_owned()
    // };
    // match add_watermark_by_image_ratio(&watermark_input) {
    //     Err(e) => panic!("photon function for image processing failed with error {}", e),
    //     Ok(msg) => println!("{}", msg)
    // }
}

#[test]
fn test_arg_list() {
    let args = ImageWatermarkArgs::parse();
    println!("args {:?}", args);
    
    let x = true;
    assert!(x, "x wasn't true!")
}