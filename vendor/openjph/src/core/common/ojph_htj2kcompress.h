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
    mem_outfile encode(const uint8_t* data, size_t width, size_t height, bool isSigned) ;
    OJPH_EXPORT
    const ui8* encodedao(const uint8_t* data, size_t width, size_t height, bool isSigned) ;
   
  };
}

#endif