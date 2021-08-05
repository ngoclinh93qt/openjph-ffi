
#include "ojph_arg.h"
#include "ojph_mem.h"
#include "ojph_file.h"
#include "ojph_codestream.h"
#include "ojph_params.h"
#include "ojph_message.h"
#include "ojph_htj2kcompress.h"
#include <fstream> 
#include <exception>
#include <memory>
//#include "ojph_img_io.h"




namespace ojph {

    const ui8* htj2kcompress::encodedao(const uint8_t* data, size_t width, size_t height, bool isSigned) {
    printf ("width (%d) \n", width );
    printf ("height (%d) \n", height );
   // ojph::ppm_in ppm;
    //ojph::image_in_base *base = NULL;
    //ppm.open("./tests/labe.ppm");
    //base = &ppm;

    
    ojph::codestream codestream;
    ojph::param_siz siz = codestream.access_siz();
    siz.set_image_extent(ojph::point(width, height));
    int num_comps = 3;
    siz.set_num_components(num_comps);
    for (int c = 0; c < num_comps; ++c)
        siz.set_component(c, ojph::point(1, 1), 8, false);
    siz.set_image_offset(ojph::point(0, 0));
    siz.set_tile_size(ojph::size(0,0));
    siz.set_tile_offset(ojph::point(0, 0));
    ojph::param_cod cod = codestream.access_cod();

    cod.set_num_decomposition(5);
    cod.set_block_dims(64,64);
    

    const char* progOrders[] = {"LRCP", "RLCP", "RPCL", "PCRL", "CPRL"};
    cod.set_progression_order(progOrders[4]);
    cod.set_color_transform(true);
    cod.set_reversible(false);
    codestream.access_qcd().set_irrev_quant(0.5);
    mem_outfile output;
    j2c_outfile daoout;
    daoout.open("daocuongx.j2c");
    output.open();
    codestream.set_planar(false);
    codestream.write_headers(&daoout);


    const size_t bytesPerPixel = 3;
    int next_comp;
    ojph::line_buf* cur_line = codestream.exchange(NULL, next_comp);
        printf ("height (%d) \n", siz.get_num_components() );


     for (size_t y = 0; y < height; y++)
    {
      for (size_t c = 0; c < siz.get_num_components(); c++)
      {       
           int* dp = cur_line->i32;

            uint16_t* pIn = (uint16_t*)(data + (y * width * bytesPerPixel));
            for(size_t x=0; x < width; x++) {
             //printf ("data (%d) \n", x );

              *dp++ = *pIn++;
            }
            cur_line = codestream.exchange(cur_line, next_comp);

      }

    }
   // outfile.flush();
  // outfile.close();
  
     // cleanup
    codestream.flush();
    codestream.close();
    printf ("size (%d) \n", output.tell() );

    return output.get_data();  
    }

   mem_outfile htj2kcompress::encode(const uint8_t* data, size_t width, size_t height, bool isSigned) 
   
   {
    ojph::codestream codestream;
    ojph::param_siz siz = codestream.access_siz();
    siz.set_image_extent(ojph::point(width, height));
    int num_comps = 3;
    siz.set_num_components(num_comps);
    for (int c = 0; c < num_comps; ++c)
        siz.set_component(c, ojph::point(width, height), 8, isSigned);
    siz.set_image_offset(ojph::point(0, 0));
    siz.set_tile_size(ojph::size(0,0));
    siz.set_tile_offset(ojph::point(0, 0));
    ojph::param_cod cod = codestream.access_cod();

    cod.set_num_decomposition(5);
    cod.set_block_dims(64,64);
    

    const char* progOrders[] = {"LRCP", "RLCP", "RPCL", "PCRL", "CPRL"};
    cod.set_progression_order(progOrders[2]);
    cod.set_color_transform(false);
    cod.set_reversible(true);
    //codestream.access_qcd().set_irrev_quant(0);
    mem_outfile output;
    codestream.set_planar(false);
    codestream.write_headers(&output);


    const size_t bytesPerPixel = 3;
    int next_comp;
    ojph::line_buf* cur_line = codestream.exchange(NULL, next_comp);
    siz = codestream.access_siz();

     for (size_t y = 0; y < height; y++)
    {
      for (size_t c = 0; c < siz.get_num_components(); c++)
      {       
           int* dp = cur_line->i32;

            uint16_t* pIn = (uint16_t*)(data + (y * width * bytesPerPixel));
            for(size_t x=0; x < width; x++) {
              *dp++ = *pIn++;
            }
          cur_line = codestream.exchange(cur_line, next_comp);

      }

    }

     // cleanup
    codestream.flush();
    codestream.close();
    return output;
   }


}
