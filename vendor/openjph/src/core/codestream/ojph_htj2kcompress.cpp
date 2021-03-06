
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



namespace ojph {



  int read(const line_buf* line, const uint8_t* data, int width, int row, int column) {
    int index = 3 * width * row;
    const uint8_t* temp_buf = &data[index];
    const ui8* sp = (ui8*)temp_buf + column;
    si32* dp = line->i32;
    for (int i = width; i > 0; --i, sp += 3)
      *dp++ = (si32)*sp;
    return width;
  }

  output_data  htj2kcompress::encodefullquality(const uint8_t* data, size_t width, size_t height) {

    ojph::codestream codestream;
    ojph::param_siz siz = codestream.access_siz();
    siz.set_image_extent(ojph::point(width, height));
    int num_comps = 3;
    siz.set_num_components(num_comps);
    for (int c = 0; c < num_comps; ++c)
      siz.set_component(c, ojph::point(1, 1), 8, false);
    siz.set_image_offset(ojph::point(0, 0));
    siz.set_tile_size(ojph::size(0, 0));
    siz.set_tile_offset(ojph::point(0, 0));
    ojph::param_cod cod = codestream.access_cod();

    cod.set_num_decomposition(5);
    cod.set_block_dims(64, 64);


    const char* progOrders[] = { "LRCP", "RLCP", "RPCL", "PCRL", "CPRL" };
    cod.set_progression_order(progOrders[4]);
    cod.set_color_transform(true);
    cod.set_reversible(true);
    //codestream.access_qcd().set_irrev_quant(0.5);
    mem_outfile output;
    output.open();
    codestream.set_planar(false);
    codestream.write_headers(&output);


    const size_t bytesPerPixel = 3;
    int next_comp;
    ojph::line_buf* cur_line = codestream.exchange(NULL, next_comp);


    for (size_t y = 0; y < height; y++)
    {
      for (size_t c = 0; c < siz.get_num_components(); c++)
      {

        assert(c == next_comp);


        read(cur_line, data, width, y, next_comp);

        cur_line = codestream.exchange(cur_line, next_comp);


      }

    }

    // cleanup
    codestream.flush();
    codestream.close();


    si64 file_size = output.get_size();

    ui8* outbf = (ui8*)malloc(file_size);
    memcpy(outbf, output.get_data(), file_size);
    output.flush();
    output.close();
    output_data out;

    out.data = outbf;
    out.len = file_size;

    return out;
  }

  output_data  htj2kcompress::encodewithqualityrate(const uint8_t* data, size_t width, size_t height, float_t rate) {

   ojph::codestream codestream;
    ojph::param_siz siz = codestream.access_siz();
    siz.set_image_extent(ojph::point(width, height));
    int num_comps = 3;
    siz.set_num_components(num_comps);
    for (int c = 0; c < num_comps; ++c)
      siz.set_component(c, ojph::point(1, 1), 8, false);
    siz.set_image_offset(ojph::point(0, 0));
    siz.set_tile_size(ojph::size(0, 0));
    siz.set_tile_offset(ojph::point(0, 0));
    ojph::param_cod cod = codestream.access_cod();

    cod.set_num_decomposition(5);
    cod.set_block_dims(64, 64);


    const char* progOrders[] = { "LRCP", "RLCP", "RPCL", "PCRL", "CPRL" };
    cod.set_progression_order(progOrders[4]);
    cod.set_color_transform(true);
    cod.set_reversible(false);
    codestream.access_qcd().set_irrev_quant(rate);
    mem_outfile output;
    output.open();
    codestream.set_planar(false);
    codestream.write_headers(&output);


    const size_t bytesPerPixel = 3;
    int next_comp;
    ojph::line_buf* cur_line = codestream.exchange(NULL, next_comp);


    for (size_t y = 0; y < height; y++)
    {
      for (size_t c = 0; c < siz.get_num_components(); c++)
      {

        assert(c == next_comp);


        read(cur_line, data, width, y, next_comp);

        cur_line = codestream.exchange(cur_line, next_comp);


      }

    }



    // cleanup
    codestream.flush();
    codestream.close();


    si64 file_size = output.get_size();

    ui8* outbf = (ui8*)malloc(file_size);
    memcpy(outbf, output.get_data(), file_size);
    output.flush();
    output.close();
    output_data out;

    out.data = outbf;
    out.len = file_size;

    return out;
  }

  const ui8* htj2kcompress::encodedao(const uint8_t* data, size_t width, size_t height, bool isSigned) {

    ojph::codestream codestream;
    ojph::param_siz siz = codestream.access_siz();
    siz.set_image_extent(ojph::point(width, height));
    int num_comps = 3;
    siz.set_num_components(num_comps);
    for (int c = 0; c < num_comps; ++c)
      siz.set_component(c, ojph::point(1, 1), 8, false);
    siz.set_image_offset(ojph::point(0, 0));
    siz.set_tile_size(ojph::size(0, 0));
    siz.set_tile_offset(ojph::point(0, 0));
    ojph::param_cod cod = codestream.access_cod();

    cod.set_num_decomposition(5);
    cod.set_block_dims(64, 64);


    const char* progOrders[] = { "LRCP", "RLCP", "RPCL", "PCRL", "CPRL" };
    cod.set_progression_order(progOrders[4]);
    cod.set_color_transform(true);
    cod.set_reversible(false);
    codestream.access_qcd().set_irrev_quant(0.5);
    mem_outfile output;
    output.open();
    codestream.set_planar(false);
    codestream.write_headers(&output);


    const size_t bytesPerPixel = 3;
    int next_comp;
    ojph::line_buf* cur_line = codestream.exchange(NULL, next_comp);


    for (size_t y = 0; y < height; y++)
    {
      for (size_t c = 0; c < siz.get_num_components(); c++)
      {

        assert(c == next_comp);


        read(cur_line, data, width, y, next_comp);

        cur_line = codestream.exchange(cur_line, next_comp);


      }

    }



    // cleanup
    codestream.flush();
    codestream.close();


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
      siz.set_component(c, ojph::point(1, 1), 8, false);
    siz.set_image_offset(ojph::point(0, 0));
    siz.set_tile_size(ojph::size(0, 0));
    siz.set_tile_offset(ojph::point(0, 0));
    ojph::param_cod cod = codestream.access_cod();

    cod.set_num_decomposition(5);
    cod.set_block_dims(64, 64);


    const char* progOrders[] = { "LRCP", "RLCP", "RPCL", "PCRL", "CPRL" };
    cod.set_progression_order(progOrders[4]);
    cod.set_color_transform(true);
    cod.set_reversible(false);
    codestream.access_qcd().set_irrev_quant(0.5);
    mem_outfile output;
    output.open();
    codestream.set_planar(false);
    codestream.write_headers(&output);


    const size_t bytesPerPixel = 3;
    int next_comp;
    ojph::line_buf* cur_line = codestream.exchange(NULL, next_comp);


    for (size_t y = 0; y < height; y++)
    {
      for (size_t c = 0; c < siz.get_num_components(); c++)
      {

        assert(c == next_comp);


        read(cur_line, data, width, y, next_comp);

        cur_line = codestream.exchange(cur_line, next_comp);


      }

    }



    // cleanup
    codestream.flush();
    codestream.close();


    return output;
  }


}
