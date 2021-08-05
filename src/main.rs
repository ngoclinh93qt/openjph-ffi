fn main() {
      poke();
}

#[allow(bad_style)]
mod ffi;

pub use crate::ffi::*;
use image::*;

fn poke() {
      unsafe {
            let img = image::open("./tests/labe.ppm").unwrap();
            let width = img.width();
            let dimentsion = img.dimensions();

            let img8 = img.into_rgb8();
            let mut data = img8.into_raw() as Vec<u8>;
            let mut d = ojph_htj2kcompress{_address:1};
            let out = d.encodedao(data.as_mut_ptr(), dimentsion.0 as usize, dimentsion.1 as usize,  false);
      }
}
