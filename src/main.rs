fn main() {
    
}



#[allow(bad_style)]
mod ffi;

pub use crate::ffi::*;
use image::*;

#[test]
fn poke() {

    unsafe {
      let img = image::open("./tests/lab.jpg").unwrap();

      let dimentsion = img.dimensions();
  
      let img16 = img.into_rgb8();
      let data = img16.into_raw() as Vec<u8>;

      let codestream = ojph_codestream {
            state: std::mem::uninitialized()
      };

      let siz = codestream.access_siz();
    }

}
