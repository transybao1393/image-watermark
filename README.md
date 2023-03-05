# Rust image watermark
Add image watermark CLI program for multiple image format (jpeg, jpg, png, tiff, bmp, etc.) manipulation, browser WASM build support.

## Table of contents

-   [Requirements](#requirements)
-   [Features](#requirements)
-   [Status](#requirements)
-   [Roadmap](#roadmap)
    -   [Objectives for 2023](#objectives-for-2023)
    -   [Objectives for 2024](#objectives-for-2024)
-   [Getting Started](#getting-started)
    -   [Dependencies](#dependencies)
    -   [Installing](#installing)
    -   [Executing program](#executing-program)
    -   [Help](#help)
    -   [Authors](#authors)
    -   [Version History](#version-history)
    -   [License](#license)
    -   [Acknowledgments](#acknowledgments)

<!-- tocstop -->

## Requirements
* Rust version 1.66.0

## Features
* Image-in-image watermark
* Text-in-image watermark

## Status
This library currently is in IN-PROGRESS status. 
Goals:
* Single image-in-image support
* Single text-in-image support
* Wide range image data types support
* Relative path support
* Batch/folder image-in-image support

## Roadmap
### Objectives for Q1+Q2/2023
* Single image-in-image support
* Single text-in-image support
* Wide range image data types support
* Relative path support

### Objectives for Q3+Q4/2023
* Batch/folder image-in-image support

## Getting Started

### Dependencies

* clap
* image
* imageproc

### Installing

* No additional library required to install

### Executing program

* How to run this CLI
```
image_watermark -i <absolute_main_image_path> -w <absolute_watermark_image_path> -o <absolute_output_path>
```

## Help

Get all useful command 
```
image_watermark --help
```

## Authors

* [Johnathan](https://github.com/transybao1393)

## Version History

* 1.0.0
    * Image-in-image watermark added
    * Auto resize and align watermark over main image
    * See [commit change]() or See [release history]()

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details