use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use image::DynamicImage;
use ravif::*;
use reqwest::Client;
use rgb::RGBA;
use std::collections::HashMap;

fn convert_to_avif(img: &DynamicImage) -> Result<EncodedImage> {
    let encoder = ravif::Encoder::new();
    let _ = encoder.clone().with_quality(80.0);
    let _ = encoder.clone().with_speed(10);
    let _ = encoder.with_internal_color_space(ravif::ColorSpace::YCbCr);
    let rgba_image = img.to_rgba8();
    let width = rgba_image.width();
    let height = rgba_image.height();
    let mut pixels = Vec::with_capacity((width * height).try_into().unwrap());
    for pixel in rgba_image.pixels() {
        let rgba = RGBA::new(pixel[0], pixel[1], pixel[2], pixel[3]);
        pixels.push(rgb::RGB::new(rgba.r, rgba.g, rgba.b));
    }
    //pub fn new(buf: Container, width: usize, height: usize) -> Self
    let buffer = Img::new(
        pixels.as_slice(),
        width.try_into().unwrap(),
        height.try_into().unwrap(),
    );
    let encoded_image = ravif::Encoder::new()
        .with_quality(80.0)
        .with_speed(10)
        .with_internal_color_space(ravif::ColorSpace::YCbCr)
        .encode_rgb(buffer)?;
    Ok(encoded_image)
}

async fn convert_and_resize_image(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let image_url = query.get("url").expect("Image URL is required");
    let width: u32 = query
        .get("width")
        .unwrap_or(&"0".to_string())
        .parse()
        .unwrap_or(0);
    let height: u32 = query
        .get("height")
        .unwrap_or(&"0".to_string())
        .parse()
        .unwrap_or(0);

    let client = Client::new();
    let res = client
        .get(image_url)
        .send()
        .await
        .expect("Failed to send HTTP request");
    let bytes = res
        .bytes()
        .await
        .expect("Failed to read HTTP response body");

    let img = image::load_from_memory(&bytes).expect("Failed to load image");

    let resized_img = if width > 0 && height > 0 {
        img.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    let avif_img = convert_to_avif(&resized_img).expect("Failed to convert image to AVIF");

    HttpResponse::Ok()
        .content_type("image/avif")
        .body(avif_img.avif_file)
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "To convert and resize an image, make a GET request to /convert with the following query parameters: \n\
        url: The URL of the image to convert. \n\
        width: The desired width of the image. If not provided, the original width will be used. \n\
        height: The desired height of the image. If not provided, the original height will be used.",
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/convert", web::get().to(convert_and_resize_image))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}