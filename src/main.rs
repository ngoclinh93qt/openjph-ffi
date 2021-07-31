fn main() {}

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

            // Setup image size parameters
            let mut codestream = ojph_codestream {
                  state: std::mem::uninitialized(),
            };
            let mut num_comps = 3;
            let mut siz = codestream.access_siz();
            let point = ojph_point {
                  x: dimentsion.0 as i32,
                  y: dimentsion.1 as i32,
            };

            let downSamples = vec![ojph_point { x: 0, y: 0 }];

            siz.set_image_extent(point);
            siz.set_num_components(num_comps);

            for i in 0..num_comps {
                  siz.set_component(i, &point, 8, true);
            }

            siz.set_image_offset(ojph_point { x: 0, y: 0 });
            siz.set_tile_size(ojph_size { w: 0, h: 0 });
            siz.set_tile_offset(ojph_point { x: 0, y: 0 });

            // Setup image size parameters

            let mut cod = codestream.access_cod();
            cod.set_num_decomposition(6);
            cod.set_block_dims(64, 64);

            let mut precincts = vec![ojph_size { w: 0, h: 0 }; 8];

            for i in 0..precincts.len() {
                  precincts[i].w = 0;
                  precincts[i].h = 0;
            }

            cod.set_precinct_size(precincts.len() as i32, &mut ojph_size { w: 0, h: 0 });
            // cod.set_progression_order();
            let lossless_ = true;
            let isUsingColorTransform_ = true;
            cod.set_color_transform(lossless_);
            cod.set_reversible(isUsingColorTransform_);
            if (!lossless_) {
                  codestream.access_qcd().set_irrev_quant(0f32);
            }

            codestream.set_planar(isUsingColorTransform_ == false);
            let mut encoded = ojph_mem_outfile::new();
            codestream.write_headers(&mut encoded._base);

            // Encode the image
            let bytes_per_ixel = 8;
            let next_component_ptr = std::mem::uninitialized();

            let mut cur_line = ojph_codestream_exchange(
                  &mut codestream,
                  std::mem::uninitialized(),
                  next_component_ptr,
            );
            siz = codestream.access_siz();
            let height = siz.get_image_extent().y - siz.get_image_offset().y;

            for i in 0..height {
                  for j in 0..siz.get_num_components() {

                  }
                  cur_line = codestream.exchange(cur_line, next_component_ptr);
            }

            codestream.flush();
            codestream.close();
      }
}
