
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



namespace ojph {
 int write(const line_buf* line, mem_outfile outfile,ui8* buffer, int width, int row, int comp_num) {

       ojph::ui8* temp_buf = buffer;
        const si32 *sp = line->i32;
      int bit_depth = 8;
      int max_val = (1<<bit_depth) - 1;
        ojph::ui8* dp = temp_buf + comp_num;

        for (int i = width; i > 0; --i, dp += 3)
        {
          int val = *sp++;
          val = val >= 0 ? val : 0;
          val = val <= max_val ? val : max_val;
          *dp = (ui8) val;
        }

        if (comp_num == 2)
      {


        outfile.write(buffer,3*width);
        
      }

        return width;
    }
    mem_outfile htj2kdecompress::decode(const uint8_t* data, size_t size) 
{

    ojph::mem_infile mem_infile;
    mem_infile.open(data,size);
    ojph::codestream codestream;
    ojph::mem_outfile mem_outfile;

       codestream.enable_resilience();
      codestream.read_headers(&mem_infile);
      codestream.restrict_input_resolution(0, 
        0);
      codestream.create();
    
       ojph::param_siz siz = codestream.access_siz();
      ui8* buffer  = (ui8*)malloc(65536);
      ui32 height = siz.get_recon_height(0);
      ui32 width = siz.get_recon_height(0);

      for (int i = 0; i < height; ++i)
      {
        for (int c = 0; c < siz.get_num_components(); ++c)
        {
          int comp_num;
          ojph::line_buf *line = codestream.pull(comp_num);
          assert(comp_num == c);
          write(line, mem_outfile,buffer ,width,i,comp_num);
        }
      }

      free(buffer);
       return  mem_outfile;   
    }
}