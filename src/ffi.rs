/* automatically generated by rust-bindgen 0.59.1 */
use std::os::raw::*;
pub type size_t = usize;
use libc::FILE;
///
pub type ojph_ui8 = u8;
pub type ojph_si8 = i8;
pub type ojph_ui16 = u16;
pub type ojph_si16 = i16;
pub type ojph_ui32 = u32;
pub type ojph_si32 = i32;
pub type ojph_ui64 = u64;
pub type ojph_si64 = i64;
extern "C" {
    ///
    #[link_name = "\u{1}__ZN4ojph13cpu_ext_levelEv"]
    pub fn ojph_cpu_ext_level() -> c_int;
}
#[repr(C)]
pub struct ojph_outfile_base__bindgen_vtable(c_void);
///
#[repr(C)]
#[derive(Debug)]
pub struct ojph_outfile_base {
    pub vtable_: *const ojph_outfile_base__bindgen_vtable,
}
///
#[repr(C)]
#[derive(Debug)]
pub struct ojph_j2c_outfile {
    pub _base: ojph_outfile_base,
    pub fh: *mut FILE,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph11j2c_outfile4openEPKc"]
    pub fn ojph_j2c_outfile_open(
        this: *mut ojph_j2c_outfile,
        filename: *const c_char,
    );
}
impl ojph_j2c_outfile {
    #[inline]
    pub unsafe fn open(&mut self, filename: *const c_char) {
        ojph_j2c_outfile_open(self, filename)
    }
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph11j2c_outfile5writeEPKvm"]
    pub fn ojph_j2c_outfile_write(
        this: *mut c_void,
        ptr: *const c_void,
        size: size_t,
    ) -> size_t;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph11j2c_outfile4tellEv"]
    pub fn ojph_j2c_outfile_tell(this: *mut c_void) -> ojph_si64;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph11j2c_outfile5flushEv"]
    pub fn ojph_j2c_outfile_flush(this: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph11j2c_outfile5closeEv"]
    pub fn ojph_j2c_outfile_close(this: *mut c_void);
}
///  @brief mem_outfile stores encoded j2k codestreams in memory
///
///  This code was first developed by Chris Hafey https://github.com/chafey
///  I took the code and integrated with OpenJPH, with some modifications.
///
///  This class serves as a memory-based file storage.
///  For example, generated j2k codestream is stored in memory
///  instead of a conventional file. The memory buffer associated with
///  this class grows with the addition of new data.
///
///  memory data can be accessed using get_data()
#[repr(C)]
pub struct ojph_mem_outfile {
    pub _base: ojph_outfile_base,
    pub is_open: bool,
    pub buf_size: size_t,
    pub buf: *mut ojph_ui8,
    pub cur_ptr: *mut ojph_ui8,
}
extern "C" {
    ///  Call this function to open a memory file.
    ///
    ///  This function creates a memory buffer to be used for storing
    ///  the generated j2k codestream.
    ///
    ///  * 'initial_size' —  is the initial memory buffer size.
    ///         The default value is 2^16.
    #[link_name = "\u{1}__ZN4ojph11mem_outfile4openEm"]
    pub fn ojph_mem_outfile_open(this: *mut ojph_mem_outfile, initial_size: size_t);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph11mem_outfile10get_bufferEv"]
    pub fn ojph_mem_outfile_get_buffer(this: *mut ojph_mem_outfile) -> *const ojph_ui8;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph11mem_outfile8get_sizeEv"]
    pub fn ojph_mem_outfile_get_size(this: *mut ojph_mem_outfile) -> ojph_si64;
}
extern "C" {
    ///  A constructor
    #[link_name = "\u{1}__ZN4ojph11mem_outfileC1Ev"]
    pub fn ojph_mem_outfile_mem_outfile(this: *mut ojph_mem_outfile);
}
impl ojph_mem_outfile {
    #[inline]
    pub unsafe fn open(&mut self, initial_size: size_t) {
        ojph_mem_outfile_open(self, initial_size)
    }
    #[inline]
    pub unsafe fn get_buffer(&mut self) -> *const ojph_ui8 {
        ojph_mem_outfile_get_buffer(self)
    }
    #[inline]
    pub unsafe fn get_size(&mut self) -> ojph_si64 {
        ojph_mem_outfile_get_size(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        ojph_mem_outfile_mem_outfile(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    ///  A destructor
    #[link_name = "\u{1}__ZN4ojph11mem_outfileD1Ev"]
    pub fn ojph_mem_outfile_mem_outfile_destructor(this: *mut ojph_mem_outfile);
}
extern "C" {
    ///  Call this function to write data to the memory file.
    ///
    ///  This function adds new data to the memory file.  The memory buffer
    ///  of the file grows as needed.
    ///
    ///  * 'ptr' —  is the address of the new data.
    ///  * 'size' —  the number of bytes in the new data.
    #[link_name = "\u{1}__ZN4ojph11mem_outfile5writeEPKvm"]
    pub fn ojph_mem_outfile_write(
        this: *mut c_void,
        ptr: *const c_void,
        size: size_t,
    ) -> size_t;
}
extern "C" {
    /// Call this function to close the file and deallocate memory
    ///
    ///  The object can be used again after calling close
    #[link_name = "\u{1}__ZN4ojph11mem_outfile5closeEv"]
    pub fn ojph_mem_outfile_close(this: *mut c_void);
}
#[repr(C)]
pub struct ojph_infile_base__bindgen_vtable(c_void);
///
#[repr(C)]
#[derive(Debug)]
pub struct ojph_infile_base {
    pub vtable_: *const ojph_infile_base__bindgen_vtable,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ojph_infile_base_seek {
    OJPH_SEEK_SET = 0,
    OJPH_SEEK_CUR = 1,
    OJPH_SEEK_END = 2,
}
///
#[repr(C)]
#[derive(Debug)]
pub struct ojph_j2c_infile {
    pub _base: ojph_infile_base,
    pub fh: *mut FILE,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10j2c_infile4openEPKc"]
    pub fn ojph_j2c_infile_open(
        this: *mut ojph_j2c_infile,
        filename: *const c_char,
    );
}
impl ojph_j2c_infile {
    #[inline]
    pub unsafe fn open(&mut self, filename: *const c_char) {
        ojph_j2c_infile_open(self, filename)
    }
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10j2c_infile4readEPvm"]
    pub fn ojph_j2c_infile_read(
        this: *mut c_void,
        ptr: *mut c_void,
        size: size_t,
    ) -> size_t;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10j2c_infile4seekExNS_11infile_base4seekE"]
    pub fn ojph_j2c_infile_seek(
        this: *mut c_void,
        offset: ojph_si64,
        origin: ojph_infile_base_seek,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10j2c_infile4tellEv"]
    pub fn ojph_j2c_infile_tell(this: *mut c_void) -> ojph_si64;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10j2c_infile5closeEv"]
    pub fn ojph_j2c_infile_close(this: *mut c_void);
}
///
#[repr(C)]
pub struct ojph_mem_infile {
    pub _base: ojph_infile_base,
    pub data: *const ojph_ui8,
    pub cur_ptr: *const ojph_ui8,
    pub size: size_t,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10mem_infile4openEPKhm"]
    pub fn ojph_mem_infile_open(this: *mut ojph_mem_infile, data: *const ojph_ui8, size: size_t);
}
impl ojph_mem_infile {
    #[inline]
    pub unsafe fn open(&mut self, data: *const ojph_ui8, size: size_t) {
        ojph_mem_infile_open(self, data, size)
    }
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10mem_infile4readEPvm"]
    pub fn ojph_mem_infile_read(
        this: *mut c_void,
        ptr: *mut c_void,
        size: size_t,
    ) -> size_t;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10mem_infile4seekExNS_11infile_base4seekE"]
    pub fn ojph_mem_infile_seek(
        this: *mut c_void,
        offset: ojph_si64,
        origin: ojph_infile_base_seek,
    ) -> c_int;
}
///
#[repr(C)]
pub struct ojph_mem_fixed_allocator {
    pub store: *mut c_void,
    pub avail_data: *mut c_void,
    pub avail_obj: *mut c_void,
    pub size_data: size_t,
    pub size_obj: size_t,
    pub avail_size_obj: size_t,
    pub avail_size_data: size_t,
}
///
#[repr(C)]
pub struct ojph_line_buf {
    pub size: size_t,
    pub pre_size: c_int,
    pub __bindgen_anon_1: ojph_line_buf__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ojph_line_buf__bindgen_ty_1 {
    pub i32_: *mut ojph_si32,
    pub f32_: *mut f32,
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_coded_lists {
    pub next_list: *mut ojph_coded_lists,
    pub buf_size: c_int,
    pub avail_size: c_int,
    pub buf: *mut ojph_ui8,
}
///
#[repr(C)]
#[derive(Debug)]
pub struct ojph_mem_elastic_allocator {
    pub store: *mut ojph_mem_elastic_allocator_stores_list,
    pub cur_store: *mut ojph_mem_elastic_allocator_stores_list,
    pub total_allocated: c_int,
    pub chunk_size: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_mem_elastic_allocator_stores_list {
    pub next_store: *mut ojph_mem_elastic_allocator_stores_list,
    pub available: c_int,
    pub data: *mut c_char,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph21mem_elastic_allocator10get_bufferEiRPNS_11coded_listsE"]
    pub fn ojph_mem_elastic_allocator_get_buffer(
        this: *mut ojph_mem_elastic_allocator,
        needed_bytes: c_int,
        p: *mut *mut ojph_coded_lists,
    );
}
impl ojph_mem_elastic_allocator {
    #[inline]
    pub unsafe fn get_buffer(
        &mut self,
        needed_bytes: c_int,
        p: *mut *mut ojph_coded_lists,
    ) {
        ojph_mem_elastic_allocator_get_buffer(self, needed_bytes, p)
    }
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_size {
    pub w: ojph_si32,
    pub h: ojph_si32,
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_point {
    pub x: ojph_si32,
    pub y: ojph_si32,
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_rect {
    pub org: ojph_point,
    pub siz: ojph_size,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_local_param_siz {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_local_param_cod {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_local_param_qcd {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_local_param_qcc {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_local_param_cap {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_local_codestream {
    _unused: [u8; 0],
}
extern "C" {
    ///
    #[link_name = "\u{1}__ZN4ojph5local31init_colour_transform_functionsEv"]
    pub fn ojph_local_init_colour_transform_functions();
}
extern "C" {
    ///
    #[link_name = "\u{1}__ZN4ojph5local32init_wavelet_transform_functionsEv"]
    pub fn ojph_local_init_wavelet_transform_functions();
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_param_siz {
    pub state: *mut ojph_local_param_siz,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_siz16set_image_extentENS_5pointE"]
    pub fn ojph_param_siz_set_image_extent(this: *mut ojph_param_siz, extent: ojph_point);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_siz13set_tile_sizeENS_4sizeE"]
    pub fn ojph_param_siz_set_tile_size(this: *mut ojph_param_siz, s: ojph_size);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_siz16set_image_offsetENS_5pointE"]
    pub fn ojph_param_siz_set_image_offset(this: *mut ojph_param_siz, offset: ojph_point);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_siz15set_tile_offsetENS_5pointE"]
    pub fn ojph_param_siz_set_tile_offset(this: *mut ojph_param_siz, offset: ojph_point);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_siz18set_num_componentsEi"]
    pub fn ojph_param_siz_set_num_components(
        this: *mut ojph_param_siz,
        num_comps: c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_siz13set_componentEiRKNS_5pointEib"]
    pub fn ojph_param_siz_set_component(
        this: *mut ojph_param_siz,
        comp_num: c_int,
        downsampling: *const ojph_point,
        bit_depth: c_int,
        is_signed: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz16get_image_extentEv"]
    pub fn ojph_param_siz_get_image_extent(this: *const ojph_param_siz) -> ojph_point;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz16get_image_offsetEv"]
    pub fn ojph_param_siz_get_image_offset(this: *const ojph_param_siz) -> ojph_point;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz13get_tile_sizeEv"]
    pub fn ojph_param_siz_get_tile_size(this: *const ojph_param_siz) -> ojph_size;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz15get_tile_offsetEv"]
    pub fn ojph_param_siz_get_tile_offset(this: *const ojph_param_siz) -> ojph_point;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz18get_num_componentsEv"]
    pub fn ojph_param_siz_get_num_components(this: *const ojph_param_siz) -> ojph_si32;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz13get_bit_depthEi"]
    pub fn ojph_param_siz_get_bit_depth(
        this: *const ojph_param_siz,
        comp_num: ojph_si32,
    ) -> ojph_si32;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz9is_signedEi"]
    pub fn ojph_param_siz_is_signed(this: *const ojph_param_siz, comp_num: ojph_si32) -> bool;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz16get_downsamplingEi"]
    pub fn ojph_param_siz_get_downsampling(
        this: *const ojph_param_siz,
        comp_num: ojph_si32,
    ) -> ojph_point;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz15get_recon_widthEi"]
    pub fn ojph_param_siz_get_recon_width(
        this: *const ojph_param_siz,
        comp_num: c_int,
    ) -> ojph_ui32;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_siz16get_recon_heightEi"]
    pub fn ojph_param_siz_get_recon_height(
        this: *const ojph_param_siz,
        comp_num: c_int,
    ) -> ojph_ui32;
}
impl ojph_param_siz {
    #[inline]
    pub unsafe fn set_image_extent(&mut self, extent: ojph_point) {
        ojph_param_siz_set_image_extent(self, extent)
    }
    #[inline]
    pub unsafe fn set_tile_size(&mut self, s: ojph_size) {
        ojph_param_siz_set_tile_size(self, s)
    }
    #[inline]
    pub unsafe fn set_image_offset(&mut self, offset: ojph_point) {
        ojph_param_siz_set_image_offset(self, offset)
    }
    #[inline]
    pub unsafe fn set_tile_offset(&mut self, offset: ojph_point) {
        ojph_param_siz_set_tile_offset(self, offset)
    }
    #[inline]
    pub unsafe fn set_num_components(&mut self, num_comps: c_int) {
        ojph_param_siz_set_num_components(self, num_comps)
    }
    #[inline]
    pub unsafe fn set_component(
        &mut self,
        comp_num: c_int,
        downsampling: *const ojph_point,
        bit_depth: c_int,
        is_signed: bool,
    ) {
        ojph_param_siz_set_component(self, comp_num, downsampling, bit_depth, is_signed)
    }
    #[inline]
    pub unsafe fn get_image_extent(&self) -> ojph_point {
        ojph_param_siz_get_image_extent(self)
    }
    #[inline]
    pub unsafe fn get_image_offset(&self) -> ojph_point {
        ojph_param_siz_get_image_offset(self)
    }
    #[inline]
    pub unsafe fn get_tile_size(&self) -> ojph_size {
        ojph_param_siz_get_tile_size(self)
    }
    #[inline]
    pub unsafe fn get_tile_offset(&self) -> ojph_point {
        ojph_param_siz_get_tile_offset(self)
    }
    #[inline]
    pub unsafe fn get_num_components(&self) -> ojph_si32 {
        ojph_param_siz_get_num_components(self)
    }
    #[inline]
    pub unsafe fn get_bit_depth(&self, comp_num: ojph_si32) -> ojph_si32 {
        ojph_param_siz_get_bit_depth(self, comp_num)
    }
    #[inline]
    pub unsafe fn is_signed(&self, comp_num: ojph_si32) -> bool {
        ojph_param_siz_is_signed(self, comp_num)
    }
    #[inline]
    pub unsafe fn get_downsampling(&self, comp_num: ojph_si32) -> ojph_point {
        ojph_param_siz_get_downsampling(self, comp_num)
    }
    #[inline]
    pub unsafe fn get_recon_width(&self, comp_num: c_int) -> ojph_ui32 {
        ojph_param_siz_get_recon_width(self, comp_num)
    }
    #[inline]
    pub unsafe fn get_recon_height(&self, comp_num: c_int) -> ojph_ui32 {
        ojph_param_siz_get_recon_height(self, comp_num)
    }
}
#[repr(C)]
pub struct ojph_output_data {
    pub data: *mut c_void,
    pub len: size_t,
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_param_cod {
    pub state: *mut ojph_local_param_cod,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_cod21set_num_decompositionEj"]
    pub fn ojph_param_cod_set_num_decomposition(
        this: *mut ojph_param_cod,
        num_decompositions: ojph_ui32,
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_cod14set_block_dimsEii"]
    pub fn ojph_param_cod_set_block_dims(
        this: *mut ojph_param_cod,
        width: c_int,
        height: c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_cod17set_precinct_sizeEiPNS_4sizeE"]
    pub fn ojph_param_cod_set_precinct_size(
        this: *mut ojph_param_cod,
        num_levels: c_int,
        precinct_size: *mut ojph_size,
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_cod21set_progression_orderEPKc"]
    pub fn ojph_param_cod_set_progression_order(
        this: *mut ojph_param_cod,
        name: *const c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_cod19set_color_transformEb"]
    pub fn ojph_param_cod_set_color_transform(this: *mut ojph_param_cod, color_transform: bool);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_cod14set_reversibleEb"]
    pub fn ojph_param_cod_set_reversible(this: *mut ojph_param_cod, reversible: bool);
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod22get_num_decompositionsEv"]
    pub fn ojph_param_cod_get_num_decompositions(
        this: *const ojph_param_cod,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod14get_block_dimsEv"]
    pub fn ojph_param_cod_get_block_dims(this: *const ojph_param_cod) -> ojph_size;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod18get_log_block_dimsEv"]
    pub fn ojph_param_cod_get_log_block_dims(this: *const ojph_param_cod) -> ojph_size;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod13is_reversibleEv"]
    pub fn ojph_param_cod_is_reversible(this: *const ojph_param_cod) -> bool;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod17get_precinct_sizeEi"]
    pub fn ojph_param_cod_get_precinct_size(
        this: *const ojph_param_cod,
        level_num: c_int,
    ) -> ojph_size;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod21get_log_precinct_sizeEi"]
    pub fn ojph_param_cod_get_log_precinct_size(
        this: *const ojph_param_cod,
        level_num: c_int,
    ) -> ojph_size;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod21get_progression_orderEv"]
    pub fn ojph_param_cod_get_progression_order(
        this: *const ojph_param_cod,
    ) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod31get_progression_order_as_stringEv"]
    pub fn ojph_param_cod_get_progression_order_as_string(
        this: *const ojph_param_cod,
    ) -> *const c_char;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod14get_num_layersEv"]
    pub fn ojph_param_cod_get_num_layers(this: *const ojph_param_cod) -> c_int;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod24is_using_color_transformEv"]
    pub fn ojph_param_cod_is_using_color_transform(this: *const ojph_param_cod) -> bool;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod19packets_may_use_sopEv"]
    pub fn ojph_param_cod_packets_may_use_sop(this: *const ojph_param_cod) -> bool;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph9param_cod15packets_use_ephEv"]
    pub fn ojph_param_cod_packets_use_eph(this: *const ojph_param_cod) -> bool;
}
impl ojph_param_cod {
    #[inline]
    pub unsafe fn set_num_decomposition(&mut self, num_decompositions: ojph_ui32) {
        ojph_param_cod_set_num_decomposition(self, num_decompositions)
    }
    #[inline]
    pub unsafe fn set_block_dims(
        &mut self,
        width: c_int,
        height: c_int,
    ) {
        ojph_param_cod_set_block_dims(self, width, height)
    }
    #[inline]
    pub unsafe fn set_precinct_size(
        &mut self,
        num_levels: c_int,
        precinct_size: *mut ojph_size,
    ) {
        ojph_param_cod_set_precinct_size(self, num_levels, precinct_size)
    }
    #[inline]
    pub unsafe fn set_progression_order(&mut self, name: *const c_char) {
        ojph_param_cod_set_progression_order(self, name)
    }
    #[inline]
    pub unsafe fn set_color_transform(&mut self, color_transform: bool) {
        ojph_param_cod_set_color_transform(self, color_transform)
    }
    #[inline]
    pub unsafe fn set_reversible(&mut self, reversible: bool) {
        ojph_param_cod_set_reversible(self, reversible)
    }
    #[inline]
    pub unsafe fn get_num_decompositions(&self) -> c_int {
        ojph_param_cod_get_num_decompositions(self)
    }
    #[inline]
    pub unsafe fn get_block_dims(&self) -> ojph_size {
        ojph_param_cod_get_block_dims(self)
    }
    #[inline]
    pub unsafe fn get_log_block_dims(&self) -> ojph_size {
        ojph_param_cod_get_log_block_dims(self)
    }
    #[inline]
    pub unsafe fn is_reversible(&self) -> bool {
        ojph_param_cod_is_reversible(self)
    }
    #[inline]
    pub unsafe fn get_precinct_size(&self, level_num: c_int) -> ojph_size {
        ojph_param_cod_get_precinct_size(self, level_num)
    }
    #[inline]
    pub unsafe fn get_log_precinct_size(&self, level_num: c_int) -> ojph_size {
        ojph_param_cod_get_log_precinct_size(self, level_num)
    }
    #[inline]
    pub unsafe fn get_progression_order(&self) -> c_int {
        ojph_param_cod_get_progression_order(self)
    }
    #[inline]
    pub unsafe fn get_progression_order_as_string(&self) -> *const c_char {
        ojph_param_cod_get_progression_order_as_string(self)
    }
    #[inline]
    pub unsafe fn get_num_layers(&self) -> c_int {
        ojph_param_cod_get_num_layers(self)
    }
    #[inline]
    pub unsafe fn is_using_color_transform(&self) -> bool {
        ojph_param_cod_is_using_color_transform(self)
    }
    #[inline]
    pub unsafe fn packets_may_use_sop(&self) -> bool {
        ojph_param_cod_packets_may_use_sop(self)
    }
    #[inline]
    pub unsafe fn packets_use_eph(&self) -> bool {
        ojph_param_cod_packets_use_eph(self)
    }
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_param_qcd {
    pub state: *mut ojph_local_param_qcd,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9param_qcd15set_irrev_quantEf"]
    pub fn ojph_param_qcd_set_irrev_quant(this: *mut ojph_param_qcd, delta: f32);
}
impl ojph_param_qcd {
    #[inline]
    pub unsafe fn set_irrev_quant(&mut self, delta: f32) {
        ojph_param_qcd_set_irrev_quant(self, delta)
    }
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_param_qcc {
    pub state: *mut ojph_local_param_qcc,
}
///
#[repr(C)]
#[derive(Debug)]
pub struct ojph_codestream {
    pub state: *mut ojph_local_codestream,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream3daoEv"]
    pub fn ojph_codestream_dao() -> ojph_codestream;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream10set_planarEb"]
    pub fn ojph_codestream_set_planar(this: *mut ojph_codestream, planar: bool);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream11set_profileEPKc"]
    pub fn ojph_codestream_set_profile(
        this: *mut ojph_codestream,
        s: *const c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream13write_headersEPNS_12outfile_baseE"]
    pub fn ojph_codestream_write_headers(this: *mut ojph_codestream, file: *mut ojph_outfile_base);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream8exchangeEPNS_8line_bufERi"]
    pub fn ojph_codestream_exchange(
        this: *mut ojph_codestream,
        line: *mut ojph_line_buf,
        next_component: *mut c_int,
    ) -> *mut ojph_line_buf;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream5flushEv"]
    pub fn ojph_codestream_flush(this: *mut ojph_codestream);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream17enable_resilienceEv"]
    pub fn ojph_codestream_enable_resilience(this: *mut ojph_codestream);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream12read_headersEPNS_11infile_baseE"]
    pub fn ojph_codestream_read_headers(this: *mut ojph_codestream, file: *mut ojph_infile_base);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream25restrict_input_resolutionEii"]
    pub fn ojph_codestream_restrict_input_resolution(
        this: *mut ojph_codestream,
        skipped_res_for_data: c_int,
        skipped_res_for_recon: c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream6createEv"]
    pub fn ojph_codestream_create(this: *mut ojph_codestream);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream4pullERi"]
    pub fn ojph_codestream_pull(
        this: *mut ojph_codestream,
        comp_num: *mut c_int,
    ) -> *mut ojph_line_buf;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream5closeEv"]
    pub fn ojph_codestream_close(this: *mut ojph_codestream);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream10access_sizEv"]
    pub fn ojph_codestream_access_siz(this: *mut ojph_codestream) -> ojph_param_siz;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream10access_codEv"]
    pub fn ojph_codestream_access_cod(this: *mut ojph_codestream) -> ojph_param_cod;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestream10access_qcdEv"]
    pub fn ojph_codestream_access_qcd(this: *mut ojph_codestream) -> ojph_param_qcd;
}
extern "C" {
    #[link_name = "\u{1}__ZNK4ojph10codestream9is_planarEv"]
    pub fn ojph_codestream_is_planar(this: *const ojph_codestream) -> bool;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestreamC1Ev"]
    pub fn ojph_codestream_codestream(this: *mut ojph_codestream);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph10codestreamD1Ev"]
    pub fn ojph_codestream_codestream_destructor(this: *mut ojph_codestream);
}
impl ojph_codestream {
    #[inline]
    pub unsafe fn dao() -> ojph_codestream {
        ojph_codestream_dao()
    }
    #[inline]
    pub unsafe fn set_planar(&mut self, planar: bool) {
        ojph_codestream_set_planar(self, planar)
    }
    #[inline]
    pub unsafe fn set_profile(&mut self, s: *const c_char) {
        ojph_codestream_set_profile(self, s)
    }
    #[inline]
    pub unsafe fn write_headers(&mut self, file: *mut ojph_outfile_base) {
        ojph_codestream_write_headers(self, file)
    }
    #[inline]
    pub unsafe fn exchange(
        &mut self,
        line: *mut ojph_line_buf,
        next_component: *mut c_int,
    ) -> *mut ojph_line_buf {
        ojph_codestream_exchange(self, line, next_component)
    }
    #[inline]
    pub unsafe fn flush(&mut self) {
        ojph_codestream_flush(self)
    }
    #[inline]
    pub unsafe fn enable_resilience(&mut self) {
        ojph_codestream_enable_resilience(self)
    }
    #[inline]
    pub unsafe fn read_headers(&mut self, file: *mut ojph_infile_base) {
        ojph_codestream_read_headers(self, file)
    }
    #[inline]
    pub unsafe fn restrict_input_resolution(
        &mut self,
        skipped_res_for_data: c_int,
        skipped_res_for_recon: c_int,
    ) {
        ojph_codestream_restrict_input_resolution(self, skipped_res_for_data, skipped_res_for_recon)
    }
    #[inline]
    pub unsafe fn create(&mut self) {
        ojph_codestream_create(self)
    }
    #[inline]
    pub unsafe fn pull(&mut self, comp_num: *mut c_int) -> *mut ojph_line_buf {
        ojph_codestream_pull(self, comp_num)
    }
    #[inline]
    pub unsafe fn close(&mut self) {
        ojph_codestream_close(self)
    }
    #[inline]
    pub unsafe fn access_siz(&mut self) -> ojph_param_siz {
        ojph_codestream_access_siz(self)
    }
    #[inline]
    pub unsafe fn access_cod(&mut self) -> ojph_param_cod {
        ojph_codestream_access_cod(self)
    }
    #[inline]
    pub unsafe fn access_qcd(&mut self) -> ojph_param_qcd {
        ojph_codestream_access_qcd(self)
    }
    #[inline]
    pub unsafe fn is_planar(&self) -> bool {
        ojph_codestream_is_planar(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        ojph_codestream_codestream(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        ojph_codestream_codestream_destructor(self)
    }
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_argument {
    pub arg: *mut c_char,
    pub index: c_int,
}
///
#[repr(C)]
#[derive(Debug)]
pub struct ojph_cli_interpreter {
    pub argv: *mut *mut c_char,
    pub argc: c_int,
    pub avail_store: [ojph_ui8; 16usize],
    pub avail: *mut ojph_ui8,
}
#[repr(C)]
pub struct ojph_cli_interpreter_arg_inter_base__bindgen_vtable(c_void);
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_cli_interpreter_arg_inter_base {
    pub vtable_: *const ojph_cli_interpreter_arg_inter_base__bindgen_vtable,
}
#[repr(i32)]
///
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ojph_OJPH_MSG_LEVEL {
    NO_MSG = 0,
    INFO = 1,
    WARN = 2,
    ERROR = 3,
}
#[repr(C)]
pub struct ojph_message_base__bindgen_vtable(c_void);
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_message_base {
    pub vtable_: *const ojph_message_base__bindgen_vtable,
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_message_info {
    pub _base: ojph_message_base,
}
extern "C" {
    ///
    #[link_name = "\u{1}__ZN4ojph15set_info_streamEP7__sFILE"]
    pub fn ojph_set_info_stream(s: *mut FILE);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph14configure_infoEPNS_12message_infoE"]
    pub fn ojph_configure_info(info: *mut ojph_message_info);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph8get_infoEv"]
    pub fn ojph_get_info() -> *mut ojph_message_info;
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_message_warning {
    pub _base: ojph_message_base,
}
extern "C" {
    ///
    #[link_name = "\u{1}__ZN4ojph18set_warning_streamEP7__sFILE"]
    pub fn ojph_set_warning_stream(s: *mut FILE);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph17configure_warningEPNS_15message_warningE"]
    pub fn ojph_configure_warning(warn: *mut ojph_message_warning);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph11get_warningEv"]
    pub fn ojph_get_warning() -> *mut ojph_message_warning;
}
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ojph_message_error {
    pub _base: ojph_message_base,
}
extern "C" {
    ///
    #[link_name = "\u{1}__ZN4ojph16set_error_streamEP7__sFILE"]
    pub fn ojph_set_error_stream(s: *mut FILE);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph15configure_errorEPNS_13message_errorE"]
    pub fn ojph_configure_error(error: *mut ojph_message_error);
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph9get_errorEv"]
    pub fn ojph_get_error() -> *mut ojph_message_error;
}
#[repr(C)]
#[derive(Debug)]
pub struct ojph_htj2kcompress {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph13htj2kcompress6encodeEPKhmmb"]
    pub fn ojph_htj2kcompress_encode(
        this: *mut ojph_htj2kcompress,
        data: *const u8,
        width: size_t,
        height: size_t,
        isSigned: bool,
    ) -> ojph_mem_outfile;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph13htj2kcompress9encodedaoEPKhmmb"]
    pub fn ojph_htj2kcompress_encodedao(
        this: *mut ojph_htj2kcompress,
        data: *const u8,
        width: size_t,
        height: size_t,
        isSigned: bool,
    ) -> *const ojph_ui8;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph13htj2kcompress17encodefullqualityEPKhmm"]
    pub fn ojph_htj2kcompress_encodefullquality(
        this: *mut ojph_htj2kcompress,
        data: *const u8,
        width: size_t,
        height: size_t,
    ) -> ojph_output_data;
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph13htj2kcompress21encodewithqualityrateEPKhmmf"]
    pub fn ojph_htj2kcompress_encodewithqualityrate(
        this: *mut ojph_htj2kcompress,
        data: *const u8,
        width: size_t,
        height: size_t,
        quality: f32,
    ) -> ojph_output_data;
}
impl ojph_htj2kcompress {
    #[inline]
    pub unsafe fn encode(
        &mut self,
        data: *const u8,
        width: size_t,
        height: size_t,
        isSigned: bool,
    ) -> ojph_mem_outfile {
        ojph_htj2kcompress_encode(self, data, width, height, isSigned)
    }
    #[inline]
    pub unsafe fn encodedao(
        &mut self,
        data: *const u8,
        width: size_t,
        height: size_t,
        isSigned: bool,
    ) -> *const ojph_ui8 {
        ojph_htj2kcompress_encodedao(self, data, width, height, isSigned)
    }
    #[inline]
    pub unsafe fn encodefullquality(
        &mut self,
        data: *const u8,
        width: size_t,
        height: size_t,
    ) -> ojph_output_data {
        ojph_htj2kcompress_encodefullquality(self, data, width, height)
    }
    #[inline]
    pub unsafe fn encodewithqualityrate(
        &mut self,
        data: *const u8,
        width: size_t,
        height: size_t,
        quality: f32,
    ) -> ojph_output_data {
        ojph_htj2kcompress_encodewithqualityrate(self, data, width, height, quality)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct ojph_htj2kdecompress {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}__ZN4ojph15htj2kdecompress6decodeEPKhm"]
    pub fn ojph_htj2kdecompress_decode(
        this: *mut ojph_htj2kdecompress,
        data: *const u8,
        size: size_t,
    ) -> ojph_output_data;
}
impl ojph_htj2kdecompress {
    #[inline]
    pub unsafe fn decode(&mut self, data: *const u8, size: size_t) -> ojph_output_data {
        ojph_htj2kdecompress_decode(self, data, size)
    }
}
