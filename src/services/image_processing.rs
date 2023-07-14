use core::panic;
use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;
use image::ImageResult;
//- photon_rs
use photon_rs::native::{open_image, save_image};
use photon_rs::multiple::watermark;
use photon_rs::transform::SamplingFilter;
// use photon_rs::transform::seam_carve;
use photon_rs::transform::resize;
use photon_rs::PhotonImage;

//- imageproc
use imageproc::drawing::{draw_text_mut, text_size};

//- image
use image::Rgba;

//- rusttype
use rusttype::{Font, Scale};

#[derive(Debug, PartialEq, PartialOrd)]
struct ImageDimension {
    width: u32,
    height: u32
}

#[derive(Debug)]
struct ImageCoords {
    top: u32,
    left: u32
}

#[derive(Debug)]
pub struct WatermarkInput {
    //- required
    pub image_absolute_path: String,
    pub watermark_image_absolute_path: String,
    pub output_path: String
}

#[derive(Debug)]
pub struct TextWatermarkInput {
    //- required
    pub image_absolute_path: String,
    pub custom_text: String,
}

impl WatermarkInput {
    //- FIXME: Combine fix_path_validate and image_type_validate
    fn file_path_validate(&self) -> bool {
        let new_main_image = Path::new(&self.image_absolute_path);
        let new_watermark_image = Path::new(&self.watermark_image_absolute_path);
        let new_output_path = Path::new(&self.output_path);

        if !new_main_image.is_file() |  
        !new_watermark_image.is_file() |
        !new_output_path.is_dir()
        {
            false
        } else {
            true
        }
    }
    
    #[allow(dead_code)]
    fn image_type_validate(&self) -> usize {
        //- FIXME: Can be use function chaining
        //- Suggested: create bool array => check if main image extension is supported => record to array 
        //- => check if watermark image extension is valid => record result to array 
        //- => calculation base on last result
        let main_image_extension = Path::new(&self.image_absolute_path).extension().unwrap();
        let watermark_image_extension = Path::new(&self.watermark_image_absolute_path).extension().unwrap(); 
        
        let supported_types: [&str; 5] = ["jpeg", "jpg", "png", "tiff", "bmp"];
        //- FIXME: Should not iterate 2 times
        let is_support_main_image_type = supported_types.iter().any(|&s| s == &main_image_extension.to_owned());
        let is_support_watermark_image_type = supported_types.iter().any(|&s| s == &watermark_image_extension.to_owned());
        
        let actual_arr: Vec<bool> = vec![is_support_main_image_type, is_support_watermark_image_type];
        let expected_arr = vec![true, true];
        let matching = expected_arr.iter().zip(&actual_arr).filter(|&(expected_arr, actual_arr)| expected_arr == actual_arr).count();
        matching
    }

}

/// inner unit test
#[test]
fn is_file_extension_supported() {
    let main_image_path = "/Users/macintoshhd/Documents/projects/rust/image-watermark/assets/images/test4.apng";
    let watermark_image_path = "/Users/macintoshhd/Documents/projects/rust/image-watermark/assets/images/watermark.jpeg";
    let watermark_input = WatermarkInput {
        image_absolute_path: main_image_path.to_owned(),
        watermark_image_absolute_path: watermark_image_path.to_owned(),
        output_path: String::from("")
    };
    let actual_result: usize = watermark_input.image_type_validate();
    assert_eq!(2, actual_result)
}

//- TODO: Small support for file relative path in the next release
#[allow(dead_code)]
fn abs_path(p: &str) -> Option<String> {
    shellexpand::full(p)
        .ok()
        .and_then(|x| Path::new(OsStr::new(x.as_ref())).canonicalize().ok())
        .and_then(|p| p.into_os_string().into_string().ok())
}

//- FIXME: Should this be into a macro?
#[allow(dead_code)]
fn sample_image_processing(relative_path: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let relative_path_string = String::from(relative_path);
    let absolute_path = get_image_from_relative_path(&relative_path_string);

    //- image processing
    let mut img = open_image(&absolute_path)?;

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    //- sample watermark
    let watermark_image = open_image("assets/images/signature.png").expect("File should open");
    watermark(&mut img, &watermark_image, 30_u32, 40_u32);
    save_image(img, "processed_image.jpeg");
    Ok("Image processing successfully!")
}

pub fn add_watermark_by_image_ratio(watermark_input: &WatermarkInput) -> Result<&str, Box<dyn std::error::Error>> {
    //- TODO: Save image to tmp folder?

    //- assert macro, non-recoverable error, panic
    assert!(watermark_input.file_path_validate(), "Files or output path is invalid!");
    assert_eq!(2, watermark_input.image_type_validate(), "Image type invalid or not support!");

    //- multithreading
    use std::thread;
    // use std::sync::mpsc::channel;
    use std::sync::mpsc::{Sender, channel};
    let (tx, rx) = channel();
    let tx: Sender<PhotonImage> = tx.clone();

    //- applying smart pointers
    use std::sync::{Arc, Mutex};
    let new_watermark_input: WatermarkInput = WatermarkInput {
        image_absolute_path: watermark_input.image_absolute_path.to_owned(),
        watermark_image_absolute_path: watermark_input.watermark_image_absolute_path.to_owned(),
        output_path: watermark_input.output_path.to_owned()
    };
    let watermark_input_arc = Arc::new(Mutex::new(new_watermark_input));
    let watermark_input_arc_clone = Arc::clone(&watermark_input_arc);
    let watermark_input_arc_clone_2 = Arc::clone(&watermark_input_arc);

    //- open image handling
    thread::spawn(move || {
        let watermark_input_mutex_arc = watermark_input_arc_clone.lock().unwrap();
        let main_image: PhotonImage = match open_image(&watermark_input_mutex_arc.image_absolute_path) {
            Ok(photon_image) => photon_image,
            Err(e) => panic!("Error when open main image with error {}", e)
        };

        //- if success, sending main image to next thread
        tx.send(main_image).unwrap(); //- unwrap() handle error
        drop(watermark_input_mutex_arc);
    });
    
    //- resize watermark image handling
    let resize_watermark_threading = thread::spawn(move || {
        let watermark_input_mutex_arc = watermark_input_arc_clone_2.lock().unwrap();
        
        //- output data build
        let output_with_file = &mut watermark_input_mutex_arc.output_path.to_owned();
        let output_file_name = "/processed_image.jpeg"; //- TODO: Need to add more validation here
        output_with_file.push_str(output_file_name);
        
        //- resize watermark image processing
        let resized_watermark_image: PhotonImage = match open_image(&watermark_input_mutex_arc.watermark_image_absolute_path) {
            Ok(photon_image_instance) => resize(&photon_image_instance,
                 (photon_image_instance.get_width() as f64 * 0.4).floor() as u32, 
                 (photon_image_instance.get_height() as f64 * 0.4).floor() as u32, 
                 SamplingFilter::Nearest),
            Err(e) => panic!("Error when open watermark image with error {}", e)
        };

        //- unlock mutex for another thread to process
        drop(watermark_input_mutex_arc);

        //- receive main image from previous thread
        let mut main_image: PhotonImage = rx.recv().unwrap();
        //- calculate (x, y) for watermark image over main image
        let image_coords: ImageCoords = generate_watermark_center_coords(&main_image, &resized_watermark_image);
        
        //- watermark
        watermark(&mut main_image, &resized_watermark_image, image_coords.left, image_coords.top);

        //- save to output path
        save_image(main_image, &output_with_file);
    });

    // main_image_threading.join().unwrap();
    resize_watermark_threading.join().unwrap();
    Ok("Success! Please check output path")
}

fn generate_watermark_center_coords (main_image: &PhotonImage, watermark_image: &PhotonImage) -> ImageCoords {
    let main_image_info: ImageDimension = ImageDimension {
        width: main_image.get_width(),
        height: main_image.get_height()
    };

    let watermark_image_info: ImageDimension = ImageDimension {
        width: watermark_image.get_width(),
        height: watermark_image.get_height()
    };

    //- TODO: check if width and height of watermark is larger than main image or not
    if watermark_image_info > main_image_info {
        panic!("Cannot get center coords because watermark dimension is larger than the original image dimension");
    }

    let left = (main_image_info.width / 2) - (watermark_image_info.width / 2);
    let top = (main_image_info.height / 2) - (watermark_image_info.height / 2);
    ImageCoords {
        top,
        left
    }
}

//- This part is actually get the current project dir
fn get_image_from_relative_path(relative_path: &str) -> String {
    //- FIXME: Need to upgrade: add relative path cleanup before converting
    let relative_path = PathBuf::from(&relative_path); //- pass a reference to avoid lifetime drop
    let mut absolute_path = std::env::current_dir().unwrap(); //- current project path
    absolute_path.push(relative_path);
    
    //- return
    absolute_path.into_os_string().into_string().unwrap()
}

pub fn add_text_to_image(text_watermark_input: &TextWatermarkInput) -> ImageResult<()> {
    //- get image extension
    let mut img = match image::open(&text_watermark_input.image_absolute_path) {
        Ok(img) => img,
        Err(e) => panic!("{}", e)
    };

    //- fonts
    let font: &[u8] = include_bytes!("./fonts/DejaVuSans.ttf") as &[u8];
    let font = Font::try_from_bytes(font).unwrap();

    //- scale
    let scale = Scale {
        x: img.width() as f32 * 0.05,
        y: img.height() as f32 * 0.05,
    };

    //- center coordination
    let (text_w, text_h) = text_size(scale, &font, &text_watermark_input.custom_text);

    //- text width validation
    if (text_w as f32) > (img.width() as f32 * 0.8) {
        panic!("Custom text is too long")
    }

    let main_image_info = ImageDimension {
        width: img.width(),
        height: img.height()
    };

    let left = (main_image_info.width / 2) - (text_w as u32 / 2);
    let top = (main_image_info.height / 2) - (text_h as u32 / 2);

    //- imageproc: draw_text_mut
    //- black: Rgba([0u8, 0u8, 0u8, 255u8])
    draw_text_mut(&mut img, 
        Rgba([255u8, 255u8, 255u8, 0.1 as u8]), 
        left as i32, 
        top as i32, 
        scale, 
        &font, 
        &text_watermark_input.custom_text);

    //- save image
    let path = Path::new(&text_watermark_input.image_absolute_path);
    img.save(path)
}



