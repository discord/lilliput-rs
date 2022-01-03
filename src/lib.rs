use std::collections::HashMap;
use std::time::Duration;

enum PixelType {
    EightBitOneCh,
    EightBitFourCh,
}

enum ImageType {
    Jpeg,
    Png,
    Gif,
    Webp,
}

enum ImageOrientation {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    LeftTop,
    RightTop,
    RightBottom,
    LeftBottom,
}

struct ImageHeader {
    width: u32,
    height: u32,
    pixel_type: PixelType,
    orientation: ImageOrientation,
    num_frames: u64,
}

enum ImageOptionsResizeMethod {
    NoResize,
    Fit,
    Resize,
}

struct ImageOptions {
    image_type: ImageType,
    width: u32,
    height: u32,
    resize_method: ImageOptionsResizeMethod,
    normalize_orientation: bool,
    encode_options: HashMap<String, String>,
    max_encode_frames: u64,
    max_encode_duration: Duration,
}

struct Framebuffer {
    buf: Vec<u8>,
    width: u32,
    height: u32,
    pixel_type: PixelType,
}

struct ImageOps {
    frames: Vec<*mut Framebuffer>,
    frame_index: usize,
}

fn main() {
    println!("Hello, world!");
}
