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
            //println!("DDDDDDD{:?}", (*out_de).cur_ptr );
           // println!("DDDDDDD{:?}", (*out_de).buf  );

          
          //  let slice = unsafe { std::slice::from_raw_parts((*out_de).get_dao(), 6220800) };
            //println!("DDDDDDD{:?}", slice.len()  );
            //println!("DDDDDDDdfefr {:?}" ,slice);
          // let buf = (*out_de).get_dao();
          //let slice = unsafe { std::slice::from_raw_parts(buf, 6220800) };
           // println!("DDDDDDD{:?}", slice[6220799] );

            //println!("DDDDDDD{:?}", *((*out_de).get_dao().offset(1)) );

            //image::save_buffer("image.ppm", slice, 1920, 1080, image::ColorType::Rgb8).unwrap();

            //let mut bytes = std::fs::write("./tests/daoii.ppm",slice).expect("no file found");
            
      
      }
}
