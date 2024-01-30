# Project Overview
This project is a Rust web application built using the Actix web framework. It allows users to convert and resize images from the internet to the AVIF format. The code includes functions to handle HTTP requests, image processing, and AVIF conversion.

## Dependencies
- `actix-web`: A popular Rust web framework.
- `anyhow`: A Rust library for flexible error handling.
- `image`: A library for image processing.
- `ravif`: A Rust library for AVIF image encoding.
- `reqwest`: A Rust HTTP client.
- `rgb`: A Rust library for RGB colors.

# Code Structure
The code is organized into several functions:

## `convert_to_avif`
This function takes a `DynamicImage` as input and converts it to AVIF format using the `ravif` library. It performs the following steps:
- Configures the AVIF encoder with quality and speed settings.
- Converts the image to RGBA format.
- Iterates through the pixels and converts them to RGB format.
- Creates an `Img` buffer and encodes it to AVIF.

## `convert_and_resize_image`
This asynchronous function handles HTTP requests to convert and resize images. It expects query parameters including the image URL, width, and height. The steps are as follows:
- Extracts query parameters from the request.
- Sends an HTTP request to fetch the image from the provided URL.
- Reads the image bytes.
- Loads the image from memory.
- Resizes the image if width and height parameters are provided.
- Calls `convert_to_avif` to convert the resized image to AVIF.
- Responds with the AVIF image.

## `index`
A simple function that provides information about how to use the API. It responds with a description of the available endpoints and query parameters.

## `main`
The entry point of the application. It configures the Actix web server, defines routes, and starts the server on port 8080.

# Usage
To use this application, make a GET request to the following endpoints:

- `/`: Provides usage instructions.
- `/convert`: Convert and resize an image by providing the `url`, `width`, and `height` query parameters.

# Building and Running
You can run the application using `cargo`:

```bash
cargo run
```

This will start the server on `http://0.0.0.0:8080`.

# Example Request
To convert and resize an image, you can use a URL like this:

```http
GET http://localhost:8080/convert?url=<image_url>&width=<desired_width>&height=<desired_height>
```

Replace `<image_url>`, `<desired_width>`, and `<desired_height>` with your image URL and desired dimensions.

# Error Handling
Make sure to handle error cases gracefully, especially when dealing with network requests or image processing, as shown in the code.

# Further Improvements
Consider adding error handling for various scenarios, logging, and potentially supporting other image formats.
