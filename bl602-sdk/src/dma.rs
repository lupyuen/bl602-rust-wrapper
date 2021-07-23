/* automatically generated by rust-bindgen 0.58.1 */

use
super::*;

pub const BL_DMA_ITEM_CTRL_MAGIC_IRQ: u32 = 2353639424;
pub const BL_DMA_ITEM_CTRL_MAGIC_NOIRQ: u32 = 206155776;
pub const BL_DMA_ITEM_CTRL_MAGIC_IRQ_CLR: u32 = 2147483647;
pub const BL_DMA_ITEM_CTRL_MAGIC_IRQ_SET: u32 = 2147483648;
pub const BL_DMA_ITEM_BITS_SRC_BURST_COUNT_01: u32 = 0;
pub const BL_DMA_ITEM_BITS_SRC_BURST_COUNT_04: u32 = 4096;
pub const BL_DMA_ITEM_BITS_SRC_BURST_COUNT_08: u32 = 8192;
pub const BL_DMA_ITEM_BITS_SRC_BURST_COUNT_16: u32 = 12288;
pub const BL_DMA_ITEM_BITS_DST_BURST_COUNT_01: u32 = 0;
pub const BL_DMA_ITEM_BITS_DST_BURST_COUNT_04: u32 = 32768;
pub const BL_DMA_ITEM_BITS_DST_BURST_COUNT_08: u32 = 65536;
pub const BL_DMA_ITEM_BITS_DST_BURST_COUNT_16: u32 = 98304;
pub const BL_DMA_ITEM_BITS_SRC_WIDTH_1BYTE: u32 = 0;
pub const BL_DMA_ITEM_BITS_SRC_WIDTH_2BYTE: u32 = 262144;
pub const BL_DMA_ITEM_BITS_SRC_WIDTH_4BYTE: u32 = 524288;
pub const BL_DMA_ITEM_BITS_DST_WIDTH_1BYTE: u32 = 0;
pub const BL_DMA_ITEM_BITS_DST_WIDTH_2BYTE: u32 = 2097152;
pub const BL_DMA_ITEM_BITS_DST_WIDTH_4BYTE: u32 = 4194304;
pub const BL_DMA_ITEM_BITS_SRC_AUTO_INCR_ENABLE: u32 = 67108864;
pub const BL_DMA_ITEM_BITS_DST_AUTO_INCR_ENABLE: u32 = 134217728;
pub const BL_DMA_ITEM_BITS_IRQ_ENABLE: u32 = 2147483648;
pub type __uint8_t = ::cty::c_uchar;
pub type __uint32_t = ::cty::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utils_list_hdr {
    pub next: *mut utils_list_hdr,
}
impl Default for utils_list_hdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bl_dma_item {
    pub item: utils_list_hdr,
    pub cb: ::core::option::Option<unsafe extern "C" fn(arg: *mut ::cty::c_void)>,
    pub arg: *mut ::cty::c_void,
    pub src: u32,
    pub dst: u32,
    pub next: u32,
    pub ctrl: u32,
}
impl Default for bl_dma_item {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_copy(item: *mut bl_dma_item);
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_init();
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_test();
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_int_clear(ch: ::cty::c_int) -> ::cty::c_int;
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_update_memsrc(ch: u8, src: u32, len: u32);
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_update_memdst(ch: u8, dst: u32, len: u32);
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_irq_register(
        channel: ::cty::c_int,
        tc_handler: *mut ::cty::c_void,
        interr_handler: *mut ::cty::c_void,
        ctx: *mut ::cty::c_void,
    ) -> ::cty::c_int;
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_irq_unregister(channel: ::cty::c_int) -> ::cty::c_int;
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_find_node_by_channel(channel: ::cty::c_int) -> *mut ::cty::c_void;
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_find_ctx_by_channel(channel: ::cty::c_int) -> *mut ::cty::c_void;
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_mem_malloc(size: u32) -> *mut ::cty::c_void;
}
#[safe_wrap(_)] extern "C" {
    pub fn bl_dma_mem_free(ptr: *mut ::cty::c_void);
}