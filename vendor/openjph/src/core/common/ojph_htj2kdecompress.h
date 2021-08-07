#ifndef OJPH_HTJ2KDECOMPRESS_H
#define OJPH_HTJ2KDECOMPRESS_H

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

class htj2kdecompress 
  {
  public:

    OJPH_EXPORT
    htj2kdecompress() {}

    OJPH_EXPORT
    ~htj2kdecompress() { }

    OJPH_EXPORT
    mem_outfile decode(const uint8_t* data, size_t width, int height) ;
   
  };
}

#endif