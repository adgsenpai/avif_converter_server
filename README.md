# Rust Image Conversion and Resizing Service 

This project is a web service that allows you to convert and resize images. It uses the Actix Web framework and the ravif library for AVIF image conversion.

## Installation

Make sure you have Rust and Cargo installed on your system. You can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

You also need to have `nasm` installed on your path you can get it here https://www.nasm.us

Clone the repository to your local machine:

```bash
git clone https://github.com/adgsenpai/avif_converter_server
cd avif_converter_server
```

Build the project:

```bash
cargo build --release
```

## Usage

To start the image conversion and resizing service, run the following command:

```bash
cargo run --release
```

By default, the service will bind to `0.0.0.0:8080`. You can change the binding address and port in the `main` function of the `main.rs` file.

## API Endpoints

### Convert and Resize an Image

To convert and resize an image, make a GET request to `/convert` with the following query parameters:

- `url`: The URL of the image to convert.
- `width`: The desired width of the image. If not provided, the original width will be used.
- `height`: The desired height of the image. If not provided, the original height will be used.

Example:

```bash
curl "http://localhost:8080/convert?url=https://example.com/image.jpg&width=300&height=200"
```

This will convert the image at the specified URL to AVIF format, resize it to the specified dimensions (if provided), and return the converted image.

## Dependencies

- [Actix Web](https://actix.rs/): A powerful and efficient web framework for Rust.
- [ravif](https://github.com/kornelski/rav1e): A Rust library for AVIF image encoding.
- [reqwest](https://github.com/seanmonstar/reqwest): An HTTP client for Rust.
- [image](https://github.com/image-rs/image): A crate for decoding and encoding various image formats.
- [rgb](https://github.com/linebender/rgb): A crate for working with RGB colors.
- [anyhow](https://github.com/dtolnay/anyhow): A Rust library for handling errors with ease.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.