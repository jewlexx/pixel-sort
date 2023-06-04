use clap::Parser;
use image::{GenericImageView, ImageFormat, RgbaImage};
use rayon::prelude::*;

use crate::colors::SingleColor;

mod args;
mod colors;

// threshold values to determine sorting start and end pixels
// using the absolute rgb value
// r*g*b = 255*255*255 = 16581375
// 0 = white
// -16581375 = black
// sort all pixels whiter than the threshold
const WHITE_VALUE: i32 = -12345678;
// sort all pixels blacker than the threshold
const BLACK_VALUE: i32 = -3456789;
// using the brightness value
// sort all pixels brighter than the threshold
const BRIGHT_VALUE: i32 = 127;
// sort all pixels darker than the threshold
const DARK_VALUE: i32 = 223;

fn main() {
    let args = args::Args::parse();

    let img = image::open(args.input).expect("valid input image");

    if args.pure {
        let pixels = img.pixels();

        let mut pec = pixels.collect::<Vec<_>>();

        println!("Sorting");
        let spin = indicatif::ProgressBar::new_spinner();
        spin.enable_steady_tick(std::time::Duration::from_millis(100));

        pec.par_sort_unstable_by(|pixel, last| match args.method {
            args::Method::Luminance => pixel.get_luminance().cmp(&last.get_luminance()),
            args::Method::Absolute => pixel.get_absolute().cmp(&last.get_absolute()),
            args::Method::Hue => pixel.get_hue().partial_cmp(&last.get_hue()).unwrap(),
        });

        spin.finish_with_message("Finished sorting image");

        let spin = indicatif::ProgressBar::new_spinner();
        spin.set_message("Cleaning up and saving image");
        spin.enable_steady_tick(std::time::Duration::from_millis(100));

        let mut sorted_img: RgbaImage = RgbaImage::new(img.width(), img.height());

        for mut p in sorted_img.pixels_mut() {
            let pixel = pec.pop().unwrap();
            p.0 = pixel.2 .0;
        }

        if let Some(out_path) = args.out {
            sorted_img.save(out_path).unwrap();
        } else {
            let format = image::guess_format(img.as_bytes()).unwrap_or(ImageFormat::Png);
            sorted_img.save_with_format("output", format).unwrap();
        }

        spin.finish();
    }
}
