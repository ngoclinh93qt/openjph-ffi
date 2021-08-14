
#include "ojph_arg.h"
#include "ojph_mem.h"
#include "ojph_file.h"
#include "ojph_codestream.h"
#include "ojph_params.h"
#include "ojph_message.h"
#include "ojph_htj2kdecompress.h"
#include <fstream> 
#include <exception>
#include <memory>
#include <iostream>
#include <cstdio>

namespace ojph {

  int write(const line_buf* line, mem_outfile* outfile, ui8* buffer, int width, int comp_num) {

    int bit_depth = 8;
    int max_val = (1 << bit_depth) - 1;
    const si32* sp = line->i32;

    ui8* dp = buffer + comp_num;

    for (int i = width; i > 0; --i, dp += 3)
    {
      int val = *sp++;
      val = val >= 0 ? val : 0;
      val = val <= max_val ? val : max_val;
      *dp = (ui8)val;

    }


    if (comp_num == 2)
    {

      int s = outfile->write(buffer, 3 * width);

    }

    return 0;
  }

  output_data htj2kdecompress::decode(const uint8_t* data, size_t size)
  {

    ojph::mem_infile mem_infile;
    mem_infile.open(data, size);
    ojph::codestream codestream;
    ojph::mem_outfile mem_outfile;
    mem_outfile.open();
    codestream.enable_resilience();
    codestream.read_headers(&mem_infile);
    codestream.restrict_input_resolution(0,
      0);
    codestream.create();
    codestream.set_planar(false);

    ojph::param_siz siz = codestream.access_siz();

    ui32 height = siz.get_recon_height(0);
    ui32 width = siz.get_recon_width(0);
    ui8* buffer = (ui8*)malloc(3 * width);
    char g[50];
    int header_len = sprintf(g, "P6\n%d %d\n%d\n", width, height, 255);

    mem_outfile.write(g, header_len);

    for (int i = 0; i < height; ++i)
    {
      for (int c = 0; c < siz.get_num_components(); ++c)
      {
        int comp_num;

        ojph::line_buf* line = codestream.pull(comp_num);

        assert(comp_num == c);

        int d = write(line, &mem_outfile, buffer, width, comp_num);

      }
    }
    int file_size = 3 * width * height + header_len;
    ui8* outbf = (ui8*)malloc(file_size);
    memcpy(outbf, mem_outfile.get_data(), file_size);


    mem_outfile.flush();
    mem_outfile.close();

    codestream.close();

    free(buffer);
    
    output_data d;
    d.data = outbf;
    d.len = file_size;
    return d;
  }
}