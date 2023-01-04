use photon_rs::native::{open_image, save_image};
use std::path::PathBuf;

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use std::ffi::OsStr;

fn main() {
    //- need to set this for debugging only
    //- Can be move this into general
    env::set_var("RUST_BACKTRACE", "1"); //- 1 or full

    match sample_image_processing("assets/images/test2.jpeg") {
        Err(e) => panic!("photon function for image processing failed with error {}", e),
        Ok(msg) => println!("{}", msg)
    }
    
}

//- FIXME: Should this be into a macro?
#[allow(dead_code)]
fn sample_image_processing(relative_path: &str) -> Result<&str, Box<dyn std::error::Error>> {
    // let relative_path_string = String::from("assets/images/test2.jpeg");

    let relative_path_string = String::from(relative_path);
    let absolute_path = get_image_from_relative_path(&relative_path_string);

    //- image processing
    let mut img = open_image(&absolute_path)?;

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    //- FIXME: Need to upgrade to support custom output path
    //- TODO: Need to save image into tmp folder "/tmp"?
    // let tmp_folder_name_path = String::from_str("/tmp");
    save_image(img, "processed_image.jpeg");
    Ok("Image processing successfully!")
}

#[allow(dead_code)]
fn add_default_watermark() -> (){
    //- TODO: add default watermark with fixed text
}

#[allow(dead_code)]
fn add_custom_watermark(__watermark_text: &str) -> (){
    //- TODO: add custom watermark with fixed text
}

//- relative to absolute
//- relative_path = "assets/images/test3.jpeg"
fn get_image_from_relative_path(relative_path: &String) -> String {
    //- FIXME: Need to upgrade: add relative path cleanup before converting
    // let absolute_image_path = "/Users/macintoshhd/Documents/projects/rust/image-watermark/assets/images/test3.jpeg";
    
    let relative_path = PathBuf::from(&relative_path); //- pass a reference to avoid lifetime drop
    let mut absolute_path = std::env::current_dir().unwrap(); //- current project path
    absolute_path.push(relative_path);
    
    //- return
    absolute_path.into_os_string().into_string().unwrap()
}

//- TODO: Image Correction
//- TODO: Image Resizing
