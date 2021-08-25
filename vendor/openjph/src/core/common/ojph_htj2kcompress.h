#ifndef OJPH_HTJ2KCOMPRESS_H
#define OJPH_HTJ2KCOMPRESS_H

#include <cstdlib>
#include <cstdio>
#include <ctime>
#include <iostream>

#include "ojph_arg.h"
#include "ojph_mem.h"
#include "ojph_file.h"
#include "ojph_codestream.h"
#include "ojph_params.h"
#include "ojph_message.h"
#include "ojph_arch.h"

namespace ojph {
  class mem_outfile;
  class htj2kcompress
  {
  public:

    OJPH_EXPORT
      htj2kcompress() {}

    OJPH_EXPORT
      ~htj2kcompress() { }

    OJPH_EXPORT
      mem_outfile encode(const uint8_t* data, size_t width, size_t height, bool isSigned);
    OJPH_EXPORT
     int encodedao(const uint8_t* data, size_t width, size_t height, bool isSigned);

    OJPH_EXPORT
      output_data  encodefullquality(const uint8_t* data, size_t width, size_t height);

    OJPH_EXPORT
      output_data  encodewithqualityrate(const uint8_t* data, size_t width, size_t height, float_t rate) ;
    };
  }

#endif