use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use image::DynamicImage;
use ravif::*;
use reqwest::Client;
use rgb::RGBA;
use std::collections::HashMap;
use std::sync::{Arc, Mutex}; // For thread-safe shared state
use actix_files::Files;

// Type alias for the cache
type Cache = Arc<Mutex<HashMap<String, Vec<u8>>>>;

fn convert_to_avif(img: &DynamicImage) -> Result<EncodedImage> {
    let rgba_image = img.to_rgba8();
    let width = rgba_image.width();
    let height = rgba_image.height();
    let mut pixels = Vec::with_capacity((width * height).try_into().unwrap());
    
    for pixel in rgba_image.pixels() {
        let rgba = RGBA::new(pixel[0], pixel[1], pixel[2], pixel[3]);
        pixels.push(rgb::RGB::new(rgba.r, rgba.g, rgba.b));
    }
    
    let buffer = Img::new(
        pixels.as_slice(),
        width.try_into().unwrap(),
        height.try_into().unwrap(),
    );

    // Create a new encoder and chain the method calls
    let encoded_image = ravif::Encoder::new()
        .with_quality(80.0)
        .with_speed(10)
        .with_internal_color_space(ravif::ColorSpace::YCbCr)
        .encode_rgb(buffer)?;

    Ok(encoded_image)
}

async fn convert_and_resize_image(
    query: web::Query<HashMap<String, String>>,
    cache: web::Data<Cache>,
) -> impl Responder {
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

    // Create a unique cache key based on the URL and dimensions
    let cache_key = format!("{}:{}:{}", image_url, width, height);
    
    // Check if the result is in the cache
    {
        let cache_lock = cache.lock().unwrap();
        if let Some(cached_image) = cache_lock.get(&cache_key) {
            return HttpResponse::Ok()
                .content_type("image/avif")
                .body(cached_image.clone());
        }
    }

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
    
    // Cache the resulting image
    {
        let mut cache_lock = cache.lock().unwrap();
        cache_lock.insert(cache_key.clone(), avif_img.avif_file.clone());
    }

    HttpResponse::Ok()
        .content_type("image/avif")
        .body(avif_img.avif_file)
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../index.html"))    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a shared cache instance
    let cache: Cache = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cache.clone())) // Share cache across requests
            .route("/", web::get().to(index))
            .service(Files::new("/static", "static").show_files_listing())
            .route("/convert", web::get().to(convert_and_resize_image))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
