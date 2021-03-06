#!/bin/bash

bindgen --opaque-type=FILE \
        --blacklist-type='^_.*' \
        --whitelist-function='^ojph.*' \
        --whitelist-type='^ojph.*' \
        --rust-target=nightly \
        --rustified-enum='.*' \
        --no-layout-tests  \
        vendor/openjph/src/core/ojph_wrapped.h -- -std=c++11 -x c++ -I./vendor/openjph/src/core/codestream -I./vendor/openjph/src/core/coding -I./vendor/openjph/src/core/common -I./vendor/openjph/src/core/transform |
        sed -E "s/pub type FILE.*/use libc::FILE;/;
                s/ @param +([^ ]+)/ * '\1' — /;
                s/#\\[doc = \"([^\"]*)\"\\]/\\/\\/\\/\1/;
                s/pub type size_t.*/pub type size_t = usize;/;
                2s/^/use std::os::raw::*;/;
                s/::std::os::raw:://g" > src/ffi.rs
