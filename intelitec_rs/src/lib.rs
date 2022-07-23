#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]

//! This crate exposes two apis for these tasks.
//!


/*! This crate provides a basic set of functionality for greeting people
 * and computing The greate common divisor of two integers
 * # Examples
 * ```
 *  use intelitec_rs;
 *  let x = 24;
 *  let y = 8;
 *  let gcd = intelitec_rs::great_common_divisor(x,y);
 * ```
 */


pub mod logic_gates;

/// Prints a very simple greenting message.
pub fn greet() {
    println!("Hello from MY_CRATE from Rusters!");
}

/// Computes the common divisor of two integers.
///
/// # Examples
/// ```
/// use intelitec_rs;
/// assert_eq!(intelitec_rs::great_common_divisor(24,8), 8);
/// ```
pub fn great_common_divisor(x:usize, y:usize) -> usize {
    if y == 0 {
        x
    }else {
        great_common_divisor(y, x % y)
    }
}

/// Rotating an image to 90 degrees.
///
/// # Examples
/// ```
/// use intelitec_rs;
/// use std::path::Path;
/// let path = Path::new("../../../assets/images/dell.jpeg");
/// //my_crate::image_rotation(path);
///
/// ```
pub fn image_rotation(path: &std::path::Path) {
    let img = image::open(path).unwrap();
    let rotated = img.rotate90();
    rotated.save(path).unwrap();
}

pub fn slow_fibonacci(nth: usize) -> u64 {
    if nth <= 1 {
        return nth as u64;
    }else {
        return slow_fibonacci(nth - 1) + slow_fibonacci(nth - 2);
    }
}

pub fn fast_fibonacci(nth: usize) -> u64 {
    let mut first = 0;
    let mut second = 1;
    let mut result = 0;
    for _ in 1..nth {
        result = first + second;
        first = second;
        second = result;
    }
    result
}


