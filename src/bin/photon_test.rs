use photon_rs::native::{open_image, save_image};
use std::path::PathBuf;

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use std::ffi::OsStr;
#[allow(unused_imports)]
use image::GenericImageView;

#[allow(dead_code)]
fn main() {
    //- need to set this for debugging only
    // env::set_var("RUST_BACKTRACE", "1"); //- 1 or full

    //- image crate test
    //- FIXME: Should this be into a macro
    // let relative_path = PathBuf::from("assets/images/test3.jpeg");
    // let mut absolute_path = std::env::current_dir().unwrap(); //- current project path
    // absolute_path.push(relative_path);
    // let t_string = absolute_path.into_os_string().into_string().unwrap();
    // println!("t_string {}", &t_string);

    let relative_path_string = String::from("assets/images/test2.jpeg");
    let absolute_path = get_image_from_relative_path(&relative_path_string);

    let mut img = open_image(&absolute_path).expect("File cant be open");

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    save_image(img, "processed_image.jpeg");
    // Ok(())
    
}

#[allow(dead_code)]
fn photon_test_func(absolute_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    //- FIXME: Add try catch and error handling for this
    let mut img = open_image(absolute_path).expect("File cant be open");

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    save_image(img, "processed_image.jpeg");
    Ok(())
}

//- relative_path = "assets/images/test3.jpeg"
#[allow(dead_code)]
fn get_image_from_relative_path(relative_path: &String) -> String {
    // let absolute_image_path = "/Users/macintoshhd/Documents/projects/rust/image-watermark/assets/images/test3.jpeg";
    
    let relative_path = PathBuf::from(&relative_path); //- pass a reference to avoid lifetime drop
    let mut absolute_path = std::env::current_dir().unwrap(); //- current project path
    absolute_path.push(relative_path);
    
    //- return
    absolute_path.into_os_string().into_string().unwrap()
}