use photon_rs::native::{open_image, save_image};
use std::path::PathBuf;
use photon_rs::multiple::watermark;
use photon_rs::transform::SamplingFilter;
// use photon_rs::transform::seam_carve;
use photon_rs::transform::resize;
use photon_rs::PhotonImage;
use std::path::Path;
use std::ffi::OsStr;

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

enum SupportedImageTypes {
    JPEG,
    JPG,
    PNG,
    TIFF,
    BMP
}

impl WatermarkInput {
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
    
    fn image_type_valid() {
        //- image support type: jpeg, jpg, png, tiff, bmp
        //- check if file is image and type is in enum
    }

}

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

    //- main image open
    let mut main_image: PhotonImage = match open_image(&watermark_input.image_absolute_path) {
        Ok(photon_image) => photon_image,
        Err(e) => panic!("Error when open main image with error {}", e)
    };

    //- watermark image resize
    //- FIXME: Finding ideal ratio/percentage of resize dimension for that image
    let resized_watermark_image: PhotonImage = match open_image(&watermark_input.watermark_image_absolute_path) {
        Ok(photon_image_instance) => resize(&photon_image_instance,
             (photon_image_instance.get_width() as f64 * 0.4).floor() as u32, 
             (photon_image_instance.get_height() as f64 * 0.4).floor() as u32, 
             SamplingFilter::Nearest),
        Err(e) => panic!("Error when open watermark image with error {}", e)
    };
    
    //- assert macro, non-recoverable error, panic
    assert!(watermark_input.file_path_validate(), "Files or output path is invalid!");

    //- calculate (x, y) for watermark image over main image
    let image_coords = generate_watermark_center_coords(&main_image, &resized_watermark_image);
    
    //- watermark
    watermark(&mut main_image, &resized_watermark_image, image_coords.left, image_coords.top);

    //- output consume
    let mut output_with_file = watermark_input.output_path.to_owned();
    let output_file_name = "/processed_image.jpeg"; //- TODO: Need to add more validation here
    output_with_file.push_str(output_file_name);

    //- save to output path
    save_image(main_image, &output_with_file);
    Ok(&watermark_input.output_path)
}

fn generate_watermark_center_coords (main_image: &PhotonImage, watermark_image: &PhotonImage) -> ImageCoords {
    let main_image_info = ImageDimension {
        width: main_image.get_width(),
        height: main_image.get_height()
    };

    let watermark_image_info = ImageDimension {
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