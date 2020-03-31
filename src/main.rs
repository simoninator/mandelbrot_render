extern crate crossbeam;
extern crate num;
extern crate probability;
extern crate slice_of_array;

use ::slice_of_array::prelude::*;
use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use probability::prelude::Continuous;
use probability::prelude::Gaussian;
use std::fs::File;
use std::f64;
use std::str::FromStr;

/// Try to determina if 'c' is in the Mandelbrot set, using at most 'limit' iterations to decide.
///
/// If 'c' is not a mebmer, return 'Some(i)', where 'i' is the number of iterations it took for 'c'
/// to leave the circle of radius two centered on the origin.
/// If'c' seems to be a member (more precisely, if we reached the iteration limit without being able to prove that 'c'
/// is not a member), return 'None'.

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parse a pair of floating-point number separated by a comma as a comples number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

/// Calculate the RGB value of the mandelbrot set.
///
/// The RGB value is calculated by three gaussian fits. For each color of red,
/// green and blue one gaussian is calculated. Each input value creates the superset
/// of the three gaussians and therefore three RGB colors
///
/// The mean value of the three gaussian are the limit divided by three and shifted to the left
/// by a sixth of the mean. 
/// The variance is calculated by Full Width Half Mean (FWHM) approxiamtion.
fn calculate_rgb(limit: u32, value: f64) -> (u8, u8, u8) {
    let fwhm = limit as f64 / 2.0;
    let sigma = fwhm / 2.3548; // see Gaussian Full Width Half Maximum Approximation

    let base_point_blue = limit as f64 / 6.0;
    let base_point_green = base_point_blue + limit as f64 / 3.0;
    let base_point_red = base_point_green + limit as f64 / 3.0;

    let red_gaussian = Gaussian::new(base_point_red, sigma);
    let green_gaussian = Gaussian::new(base_point_green, sigma);
    let blue_gaussian = Gaussian::new(base_point_blue, sigma);

    let scale = limit as f64 * (2.0 * f64::consts::PI * sigma.powi(2)).sqrt();

    let red = scale * red_gaussian.density(value);
    let green = scale * green_gaussian.density(value);
    let blue = scale * blue_gaussian.density(value);

    (red as u8, green as u8, blue as u8)
}

#[test]
fn test_get_rgb() {
    assert_eq!(calculate_rgb(255, 42.5), (1, 74, 255));
    assert_eq!(calculate_rgb(255, 127.5), (74, 255, 74));
    assert_eq!(calculate_rgb(255, 212.5), (255, 74, 1));
    assert_eq!(calculate_rgb(255, 85.0), (15, 187, 187));
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [(u8, u8, u8)],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    let limit = 255;
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, limit) {
                None => (40, 40, 40),
                Some(count) => calculate_rgb(limit, count as f64),
            };
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    Ok(())
}

use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT"
        )
        .unwrap();

        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0,0.20",
            args[0]
        )
        .unwrap();

        std::process::exit(1);
    }

    let bounds = parse_pair::<usize>(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![(0, 0, 0); bounds.0 * bounds.1];

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [(u8, u8, u8)]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move || render(band, band_bounds, band_upper_left, band_lower_right));
            }
        });
    }

    let rgb_image = pixels
        .iter()
        .map(|&x| [x.0 as u8, x.1 as u8, x.2 as u8])
        .collect::<Vec<_>>();

    write_image(&args[1], rgb_image.flat(), bounds).expect("error writing PNG file");
}
