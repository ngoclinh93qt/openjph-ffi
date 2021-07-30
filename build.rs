use std::path::Path;

fn main() {
    let mut cc = cc::Build::new();
    let coredir = Path::new("vendor/openjph/src/core");
    let codestreamdir = Path::new("vendor/openjph/src/core/codestream");
    let codingdir = Path::new("vendor/openjph/src/core/coding");
    let otherdir = Path::new("vendor/openjph/src/core/others");
    let transformdir = Path::new("vendor/openjph/src/core/transform");
    let commondir = Path::new("vendor/openjph/src/core/common");

    cc.include(coredir);
    cc.include(codestreamdir);
    cc.include(codingdir);
    cc.include(otherdir);
    cc.include(transformdir);
    cc.include(commondir);
    cc.cpp(true);
    cc.cpp_set_stdlib("c++");
    cc.flag("-std=c++17");

    let files = [
          "codestream/ojph_codestream.cpp",
          "codestream/ojph_params.cpp",
          "coding/ojph_block_decoder.cpp",
          "coding/ojph_block_encoder.cpp",
          "others/ojph_arch.cpp",
          "others/ojph_file.cpp",
          "others/ojph_mem.cpp",
          "others/ojph_message.cpp",
          "transform/ojph_colour.cpp",
          "transform/ojph_colour_sse.cpp",
          "transform/ojph_colour_sse2.cpp",
          "transform/ojph_transform_sse.cpp",
          "transform/ojph_transform_sse2.cpp",
          "transform/ojph_transform.cpp",

    ];
    for file in files.iter() {
        cc.file(coredir.join(file));
    }
    cc.compile("openjph");

}
