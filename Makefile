run-image:
	cargo run --bin image_watermark image -i image_path_here -o output_path_here --watermark-image-absolute-path watermark_image_path
run-text:
	cargo run --bin image_watermark text -i image_path_here -t custom_text
build-main:
	cargo build --bin image_watermark