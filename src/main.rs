fn main() {
      poke();
}

#[allow(bad_style)]
mod ffi;
use std::time::{Duration, Instant};
pub use crate::ffi::*;
use image::*;
use std::fs::*;

fn poke() {
      unsafe {
            let img = image::open("./tests/lab.jpg").unwrap();
            let width = img.width();
            let dimentsion = img.dimensions();

            let img8 = img.into_rgb8();
            let mut data = img8.into_raw() as Vec<u8>;

            let mut bytes = std::fs::read("./tests/labe.ppm").expect("no file found");

            println!("{}",bytes.len());
            let now = Instant::now();

            let mut d = ojph_htj2kcompress{_address:1};
            let out = d.encodedao(data.as_mut_ptr(), dimentsion.0 as usize, dimentsion.1 as usize,  false);

            let elapase = now.elapsed();
            println!("{}",elapase.as_millis());


            let mut decoder = ojph_htj2kdecompress{_address:3};
            let mut decode_inputfile = std::fs::read("./tests/daocuongx.j2c").expect("no file found");
            let size = decode_inputfile.len();

            let out_de = decoder.decode(decode_inputfile.as_mut_ptr(),size as usize );

      // println!("DDDDDDD{}",out.buf_size)
      }
}
