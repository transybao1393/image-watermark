use std::path::Path;

/// integration test
#[cfg(test)]
mod tests {
    //- test cases
    //- Test if path is valid or invalid
    //- Test if file is not found
    #[allow(dead_code)]
    fn test_file_path_validate() {
        // let main_image_path = "/Users/macintoshhd/Documents/projects/rust/image-watermark/assets/images/test4.jpeg";
        // let watermark_image_path = "/Users/macintoshhd/Documents/projects/rust/image-watermark/assets/images/watermark.png";
        // let output_path = "/Users/macintoshhd/Documents/projects/rust/image-watermark/assets/images";

        // let watermark_input = WatermarkInput {
        //     image_absolute_path: main_image_path.to_owned(),
        //     watermark_image_absolute_path: watermark_image_path.to_owned(),
        //     output_path: output_path.to_owned()
        // };
        // //- non-recoverable return
        // assert!(watermark_input.file_path_validate(), "Files or output path is invalid!");
        // println!("Do something else...");
    }

    #[allow(dead_code)]
    fn test_absolute_relative_path() {
        //- test relative path to absolute path
        // let relative_path_sample = "~/Documents/projects/rust/image-watermark/assets/images/test4.jpeg";
        // let relative_path_sample2 = "../../assets/images/test2.jpeg";
        // let absolute_path = match abs_path(relative_path_sample2) {
        //     None => panic!("Cannot convert"),
        //     Some(absolute_path) => absolute_path
        // };
    }
}