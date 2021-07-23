fname: "bl_adc_init", namespace: "bl_adc"
fname: "bl_adc_dma_init", namespace: "bl_adc"
fname: "bl_adc_start", namespace: "bl_adc"
fname: "bl_adc_gpio_init", namespace: "bl_adc"
fname: "bl_adc_get_channel_by_gpio", namespace: "bl_adc"
fname: "bl_adc_freq_init", namespace: "bl_adc"
fname: "bl_adc_parse_data", namespace: "bl_adc"
fname: "bl_dma_copy", namespace: "bl_dma"
fname: "bl_dma_init", namespace: "bl_dma"
fname: "bl_dma_test", namespace: "bl_dma"
fname: "bl_dma_int_clear", namespace: "bl_dma"
fname: "bl_dma_update_memsrc", namespace: "bl_dma"
fname: "bl_dma_update_memdst", namespace: "bl_dma"
fname: "bl_dma_irq_register", namespace: "bl_dma"
fname: "bl_dma_irq_unregister", namespace: "bl_dma"
fname: "bl_dma_find_node_by_channel", namespace: "bl_dma"
fname: "bl_dma_find_ctx_by_channel", namespace: "bl_dma"
fname: "bl_dma_mem_malloc", namespace: "bl_dma"
fname: "bl_dma_mem_free", namespace: "bl_dma"
fname: "bl_gpio_enable_output", namespace: "bl_gpio"
fname: "bl_gpio_enable_input", namespace: "bl_gpio"
fname: "bl_gpio_output_set", namespace: "bl_gpio"
fname: "bl_gpio_input_get", namespace: "bl_gpio"
fname: "bl_gpio_input_get_value", namespace: "bl_gpio"
fname: "bl_gpio_int_clear", namespace: "bl_gpio"
fname: "bl_gpio_intmask", namespace: "bl_gpio"
fname: "bl_gpio_register", namespace: "bl_gpio"
fname: "i2c_set_freq", namespace: "i2c"
fname: "i2c_gpio_init", namespace: "i2c"
fname: "i2c_clear_status", namespace: "i2c"
fname: "i2c_transfer_start", namespace: "i2c"
fname: "PWM_Channel_Init", namespace: "PWM"
fname: "PWM_Channel_Update", namespace: "PWM"
fname: "PWM_Channel_Set_Div", namespace: "PWM"
fname: "PWM_Channel_Set_Threshold1", namespace: "PWM"
fname: "PWM_Channel_Set_Threshold2", namespace: "PWM"
fname: "PWM_Channel_Set_Period", namespace: "PWM"
fname: "PWM_Channel_Get", namespace: "PWM"
fname: "PWM_IntMask", namespace: "PWM"
fname: "PWM_Channel_Enable", namespace: "PWM"
fname: "PWM_Channel_Disable", namespace: "PWM"
fname: "PWM_Int_Callback_Install", namespace: "PWM"
fname: "bl_pwm_init", namespace: "bl_pwm"
fname: "bl_pwm_start", namespace: "bl_pwm"
fname: "bl_pwm_stop", namespace: "bl_pwm"
fname: "bl_pwm_set_freq", namespace: "bl_pwm"
fname: "bl_pwm_set_duty", namespace: "bl_pwm"
fname: "bl_pwm_get_duty", namespace: "bl_pwm"
fname: "hal_spi_transfer", namespace: "hal_spi"
fname: "spi_init", namespace: "spi"
fname: "UART_Init", namespace: "UART"
fname: "UART_DeInit", namespace: "UART"
fname: "UART_FifoConfig", namespace: "UART"
fname: "UART_IrConfig", namespace: "UART"
fname: "UART_Enable", namespace: "UART"
fname: "UART_Disable", namespace: "UART"
fname: "UART_SetTxDataLength", namespace: "UART"
fname: "UART_SetRxDataLength", namespace: "UART"
fname: "UART_SetRxTimeoutValue", namespace: "UART"
fname: "UART_SetDeglitchCount", namespace: "UART"
fname: "UART_SetBaudrate", namespace: "UART"
fname: "UART_SetRtsValue", namespace: "UART"
fname: "UART_ClrRtsValue", namespace: "UART"
fname: "UART_TxFreeRun", namespace: "UART"
fname: "UART_AutoBaudDetection", namespace: "UART"
fname: "UART_TxFifoClear", namespace: "UART"
fname: "UART_RxFifoClear", namespace: "UART"
fname: "UART_IntMask", namespace: "UART"
fname: "UART_IntClear", namespace: "UART"
fname: "UART_Int_Callback_Install", namespace: "UART"
fname: "UART_SendData", namespace: "UART"
fname: "UART_SendDataBlock", namespace: "UART"
fname: "UART_ReceiveData", namespace: "UART"
fname: "UART_GetAutoBaudCount", namespace: "UART"
fname: "UART_GetTxFifoCount", namespace: "UART"
fname: "UART_GetRxFifoCount", namespace: "UART"
fname: "UART_GetIntStatus", namespace: "UART"
fname: "UART_GetTxBusBusyStatus", namespace: "UART"
fname: "UART_GetRxBusBusyStatus", namespace: "UART"
fname: "UART_GetOverflowStatus", namespace: "UART"
fname: "bl_uart_gpio_init", namespace: "bl_uart"
fname: "bl_uart_init", namespace: "bl_uart"
fname: "bl_uart_debug_early_init", namespace: "bl_uart"
fname: "bl_uart_early_init", namespace: "bl_uart"
fname: "bl_uart_int_rx_enable", namespace: "bl_uart"
fname: "bl_uart_int_rx_disable", namespace: "bl_uart"
fname: "bl_uart_int_tx_enable", namespace: "bl_uart"
fname: "bl_uart_int_tx_disable", namespace: "bl_uart"
fname: "bl_uart_string_send", namespace: "bl_uart"
fname: "bl_uart_flush", namespace: "bl_uart"
fname: "bl_uart_getdefconfig", namespace: "bl_uart"
fname: "bl_uart_setconfig", namespace: "bl_uart"
fname: "bl_uart_setbaud", namespace: "bl_uart"
fname: "bl_uart_data_send", namespace: "bl_uart"
fname: "bl_uart_datas_send", namespace: "bl_uart"
fname: "bl_uart_data_recv", namespace: "bl_uart"
fname: "bl_uart_int_enable", namespace: "bl_uart"
fname: "bl_uart_int_disable", namespace: "bl_uart"
fname: "bl_uart_int_rx_notify_register", namespace: "bl_uart"
fname: "bl_uart_int_tx_notify_register", namespace: "bl_uart"
fname: "bl_uart_int_rx_notify_unregister", namespace: "bl_uart"
fname: "bl_uart_int_tx_notify_unregister", namespace: "bl_uart"
fname: "hal_wifi_start_firmware_task", namespace: "hal_wifi"
fname: "wifi_mgmr_psk_cal", namespace: "wifi"
fname: "wifi_mgmr_drv_init", namespace: "wifi"
fname: "wifi_mgmr_init", namespace: "wifi"
fname: "wifi_mgmr_start", namespace: "wifi"
fname: "wifi_mgmr_start_background", namespace: "wifi"
fname: "wifi_mgmr_get_wifi_channel_conf", namespace: "wifi"
fname: "wifi_mgmr_sta_enable", namespace: "wifi"
fname: "wifi_mgmr_sta_disable", namespace: "wifi"
fname: "wifi_mgmr_sta_mac_set", namespace: "wifi"
fname: "wifi_mgmr_sta_mac_get", namespace: "wifi"
fname: "wifi_mgmr_sta_ip_get", namespace: "wifi"
fname: "wifi_mgmr_sta_ip_set", namespace: "wifi"
fname: "wifi_mgmr_sta_dns_get", namespace: "wifi"
fname: "wifi_mgmr_sta_ip_unset", namespace: "wifi"
fname: "wifi_mgmr_sta_connect", namespace: "wifi"
fname: "wifi_mgmr_sta_disconnect", namespace: "wifi"
fname: "wifi_mgmr_sta_powersaving", namespace: "wifi"
fname: "wifi_mgmr_sta_autoconnect_enable", namespace: "wifi"
fname: "wifi_mgmr_sta_autoconnect_disable", namespace: "wifi"
fname: "wifi_mgmr_sta_ssid_set", namespace: "wifi"
fname: "wifi_mgmr_sta_psk_set", namespace: "wifi"
fname: "wifi_mgmr_sta_connect_ind_stat_get", namespace: "wifi"
fname: "wifi_mgmr_ap_enable", namespace: "wifi"
fname: "wifi_mgmr_ap_mac_set", namespace: "wifi"
fname: "wifi_mgmr_ap_mac_get", namespace: "wifi"
fname: "wifi_mgmr_ap_ip_get", namespace: "wifi"
fname: "wifi_mgmr_ap_stop", namespace: "wifi"
fname: "wifi_mgmr_ap_start", namespace: "wifi"
fname: "wifi_mgmr_ap_sta_cnt_get", namespace: "wifi"
fname: "wifi_mgmr_ap_sta_info_get", namespace: "wifi"
fname: "wifi_mgmr_ap_sta_delete", namespace: "wifi"
fname: "wifi_mgmr_ap_set_gateway", namespace: "wifi"
fname: "wifi_mgmr_sniffer_enable", namespace: "wifi"
fname: "wifi_mgmr_sniffer_disable", namespace: "wifi"
fname: "wifi_mgmr_rate_config", namespace: "wifi"
fname: "wifi_mgmr_conf_max_sta", namespace: "wifi"
fname: "wifi_mgmr_sniffer_register", namespace: "wifi"
fname: "wifi_mgmr_sniffer_unregister", namespace: "wifi"
fname: "wifi_mgmr_state_get", namespace: "wifi"
fname: "wifi_mgmr_status_code_get", namespace: "wifi"
fname: "wifi_mgmr_rssi_get", namespace: "wifi"
fname: "wifi_mgmr_channel_get", namespace: "wifi"
fname: "wifi_mgmr_channel_set", namespace: "wifi"
fname: "wifi_mgmr_all_ap_scan", namespace: "wifi"
fname: "wifi_mgmr_scan_filter_hidden_ssid", namespace: "wifi"
fname: "wifi_mgmr_scan", namespace: "wifi"
fname: "wifi_mgmr_cfg_req", namespace: "wifi"
fname: "wifi_mgmr_scan_complete_callback", namespace: "wifi"
fname: "wifi_mgmr_cli_scanlist", namespace: "wifi"
fname: "wifi_mgmr_cli_init", namespace: "wifi"
fname: "wifi_mgmr_scan_ap", namespace: "wifi"
fname: "wifi_mgmr_scan_ap_all", namespace: "wifi"
fname: "wifi_mgmr_raw_80211_send", namespace: "wifi"
fname: "wifi_mgmr_set_country_code", namespace: "wifi"
fname: "wifi_mgmr_ext_dump_needed", namespace: "wifi"
fname: "wifi_mgmr_status_code_str", namespace: "wifi"
fname: "wifi_mgmr_event_notify", namespace: "wifi"
fname: "wifi_mgmr_state_get_internal", namespace: "wifi"
fname: "wifi_mgmr_status_code_clean_internal", namespace: "wifi"
fname: "wifi_mgmr_status_code_get_internal", namespace: "wifi"
fname: "wifi_mgmr_set_country_code_internal", namespace: "wifi"
fname: "wifi_mgmr_ap_sta_cnt_get_internal", namespace: "wifi"
fname: "wifi_mgmr_ap_sta_info_get_internal", namespace: "wifi"
fname: "wifi_mgmr_ap_sta_delete_internal", namespace: "wifi"
fname: "wifi_mgmr_scan_complete_notify", namespace: "wifi"
fname: "wifi_mgmr_auth_to_str", namespace: "wifi"
fname: "wifi_mgmr_cipher_to_str", namespace: "wifi"
fname: "wifi_mgmr_api_fw_tsen_reload", namespace: "wifi"
fname: "wifi_mgmr_scan_item_is_timeout", namespace: "wifi"
#![feature(prelude_import)]
//!  Rust Wrapper for BL602 IoT SDK. See "The RISC-V BL602 Book" <https://lupyuen.github.io/articles/book>
#![no_std]
#[prelude_import]
use core::prelude::rust_2018::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
//  Use the Rust Core Library instead of the Rust Standard Library, which is not compatible with embedded systems

use bl602_macros::safe_wrap;
use result::*;

///////////////////////////////////////////////////////////////////////////////
//  Wrapper for BL602 HAL generated by `bindgen` and `safe_wrap`

/// ADC HAL for BL602. See <https://lupyuen.github.io/articles/book#adc-on-bl602>
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod adc {








    //  Import the Rust Core Library
    //  For `PanicInfo` type used by `panic` function
    //  For converting `str` to `String`

    ///////////////////////////////////////////////////////////////////////////////
    //  Wrapper for NimBLE Porting Layer
    //  TODO: Generate with `bindgen` and `safe_wrap`

    //  `&str` is a reference to a string slice, similar to `const char *` in C

    //  Import C Function

    //  Convert `str` to `String`, which similar to `char [64]` in C
    //  `mut` because we will modify it
    //  If it exceeds 64 chars, halt with an error

    //  Terminate the string with null, since we will be passing to C
    //  If we exceed 64 chars, halt with an error

    //  Convert the null-terminated string to a pointer

    //  Call the C function
    //  Flag this code as unsafe because we're calling a C function

    //  No semicolon `;` here, so the value returned by the C function will be passed to our caller

    //  Number of milliseconds
    //  Returns the number of ticks (uint32_t)
    //  Import C Function

    //  Call the C function
    //  Flag this code as unsafe because we're calling a C function

    //  No semicolon `;` here, so the value returned by the C function will be passed to our caller

    //  Number of ticks to sleep
    //  Import C Function

    //  Call the C function
    //  Flag this code as unsafe because we're calling a C function


    ///////////////////////////////////////////////////////////////////////////////
    //  Wrapper Types



    //  Allow type names to have non-camel case




    //  TODO

    //  Strn may be copied

    //  StrnRep may be copied

    //  Last byte must be 0.



    //  Last byte must be 0.
    //  Don't count the terminating null.
    //  Look for the null termination.
    //  String too long


    //  Last byte must be 0.

    //  Last byte must be 0.
    //  Not implemented

    //  Last byte must be 0.

    //  Last byte must be 0.




    ///////////////////////////////////////////////////////////////////////////////
    //  Test Functions

    //  Don't mangle the function name
    //  Declare `extern "C"` because it will be called by BL602 firmware
    //  Result to be returned to command-line interface
    //  Length of command line (int)
    //  Number of command line args (int)
    //  Array of command line args (char **)
    //  Show a message on the serial console

    //  PineCone Blue LED is connected on BL602 GPIO 11
    //  `u8` is 8-bit unsigned integer

    //  Configure the LED GPIO for output (instead of input)
    //  No pullup, no pulldown
    //  Halt on error

    //  Blink the LED 5 times
    //  Iterates 10 times from 0 to 9 (`..` excludes 10)

    //  Toggle the LED GPIO between 0 (on) and 1 (off)
    //  Set the GPIO output (from BL602 GPIO HAL)
    //  GPIO pin number
    //  0 for low, 1 for high
    //  Halt on error

    //  Sleep 1 second
    //  Sleep by number of ticks (from NimBLE Porting Layer)
    //  Convert 1,000 milliseconds to ticks (from NimBLE Porting Layer)

    //  Return to the BL602 command-line interface

    //  `!` means that panic handler will never return
    //  TODO: Implement the complete panic handler like this:
    //  https://github.com/lupyuen/pinetime-rust-BL602/blob/master/rust/app/src/lib.rs#L115-L146

    //  For now we display a message

    //  Loop forever, do not pass go, do not collect $200
    use super::*;
    pub const ADC_DMA_CHANNEL: u32 = 1;
    pub const ADC_CHANNEL_MAX: u32 = 12;
    pub type __int32_t = ::cty::c_int;
    pub type __uint32_t = ::cty::c_uint;
    pub type bl_adc_cb_t =
     ::core::option::Option<unsafe extern "C" fn(mode: ::cty::c_int,
                                                 data_ptr: *mut u32,
                                                 data_size: u32)>;
    #[repr(C)]
    pub struct adc_ctx {
        pub mode: ::cty::c_int,
        pub channel_data: *mut u32,
        pub adc_lli: *mut ::cty::c_void,
        pub lli_flag: ::cty::c_int,
        pub chan_init_table: u32,
        pub data_size: u32,
        pub cb: bl_adc_cb_t,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for adc_ctx { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for adc_ctx {
        #[inline]
        fn clone(&self) -> adc_ctx {
            {
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<*mut u32>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<bl_adc_cb_t>;
                *self
            }
        }
    }
    impl Default for adc_ctx {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type adc_ctx_t = adc_ctx;
    #[doc =
      "Init an ADC Channel. See `bl_adc_init` in \"init_adc\" <https://github.com/lupyuen/bl_iot_sdk/blob/adc/customer_app/sdk_app_adc2/sdk_app_adc2/demo.c>"]
    pub fn init(mode: ::cty::c_int, gpio_num: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_adc_init(mode: ::cty::c_int, gpio_num: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_adc_init(mode as ::cty::c_int, gpio_num as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Init DMA for the ADC Channel. See `bl_adc_dma_init` in \"init_adc\" <https://github.com/lupyuen/bl_iot_sdk/blob/adc/customer_app/sdk_app_adc2/sdk_app_adc2/demo.c>"]
    pub fn dma_init(mode: ::cty::c_int, data_num: u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_adc_dma_init(mode: ::cty::c_int, data_num: u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_adc_dma_init(mode as ::cty::c_int, data_num as u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Start reading the ADC Channel via DMA. See `bl_adc_start` in \"init_adc\" <https://github.com/lupyuen/bl_iot_sdk/blob/adc/customer_app/sdk_app_adc2/sdk_app_adc2/demo.c>"]
    pub fn start() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_adc_start()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_adc_start();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Configure the GPIO Pin as ADC Input (no pullup, no pulldown). See `bl_adc_gpio_init` in \"init_adc\" <https://github.com/lupyuen/bl_iot_sdk/blob/adc/customer_app/sdk_app_adc2/sdk_app_adc2/demo.c>"]
    pub fn gpio_init(gpio_num: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_adc_gpio_init(gpio_num: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_adc_gpio_init(gpio_num as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Get the ADC Channel Number for the GPIO Pin. See `bl_adc_get_channel_by_gpio` in \"init_adc\" <https://github.com/lupyuen/bl_iot_sdk/blob/adc/customer_app/sdk_app_adc2/sdk_app_adc2/demo.c>"]
    pub fn get_channel_by_gpio(gpio_num: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_adc_get_channel_by_gpio(gpio_num: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_adc_get_channel_by_gpio(gpio_num as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Init the ADC Channel Frequency. See `bl_adc_freq_init` in \"init_adc\" <https://github.com/lupyuen/bl_iot_sdk/blob/adc/customer_app/sdk_app_adc2/sdk_app_adc2/demo.c>"]
    pub fn freq_init(mode: ::cty::c_int, freq: u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_adc_freq_init(mode: ::cty::c_int, freq: u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_adc_freq_init(mode as ::cty::c_int, freq as u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc = "Parse the ADC Samples that have been read."]
    pub fn parse_data(parr: *mut u32, data_size: ::cty::c_int,
                      channel: ::cty::c_int, raw_flag: ::cty::c_int)
     -> BlResult<i32> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_adc_parse_data(parr: *mut u32, data_size: ::cty::c_int,
                                     channel: ::cty::c_int,
                                     raw_flag: ::cty::c_int)
            -> i32;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_adc_parse_data(parr as *mut u32, data_size as ::cty::c_int,
                                  channel as ::cty::c_int,
                                  raw_flag as ::cty::c_int);
            "----------Result----------";
            Ok(res)
        }
    }
}
/// DMA HAL for BL602. See <https://lupyuen.github.io/articles/book#dma-on-bl602>
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod dma {
    use super::*;
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
    pub struct utils_list_hdr {
        pub next: *mut utils_list_hdr,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for utils_list_hdr { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for utils_list_hdr {
        #[inline]
        fn clone(&self) -> utils_list_hdr {
            {
                let _: ::core::clone::AssertParamIsClone<*mut utils_list_hdr>;
                *self
            }
        }
    }
    impl Default for utils_list_hdr {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub struct bl_dma_item {
        pub item: utils_list_hdr,
        pub cb: ::core::option::Option<unsafe extern "C" fn(arg:
                                                                *mut ::cty::c_void)>,
        pub arg: *mut ::cty::c_void,
        pub src: u32,
        pub dst: u32,
        pub next: u32,
        pub ctrl: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for bl_dma_item { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for bl_dma_item {
        #[inline]
        fn clone(&self) -> bl_dma_item {
            {
                let _: ::core::clone::AssertParamIsClone<utils_list_hdr>;
                let _:
                        ::core::clone::AssertParamIsClone<::core::option::Option<unsafe extern "C" fn(arg:
                                                                                                          *mut ::cty::c_void)>>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    impl Default for bl_dma_item {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub fn copy(item: *mut bl_dma_item) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_copy(item: *mut bl_dma_item);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_dma_copy(item as *mut bl_dma_item);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn init() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_init();
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_dma_init();
            "----------Result----------";
            Ok(())
        }
    }
    pub fn test() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_test();
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_dma_test();
            "----------Result----------";
            Ok(())
        }
    }
    pub fn int_clear(ch: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_int_clear(ch: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_dma_int_clear(ch as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn update_memsrc(ch: u8, src: u32, len: u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_update_memsrc(ch: u8, src: u32, len: u32);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_dma_update_memsrc(ch as u8, src as u32, len as u32);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn update_memdst(ch: u8, dst: u32, len: u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_update_memdst(ch: u8, dst: u32, len: u32);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_dma_update_memdst(ch as u8, dst as u32, len as u32);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn irq_register(channel: ::cty::c_int, tc_handler: Ptr,
                        interr_handler: Ptr, ctx: Ptr) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_irq_register(channel: ::cty::c_int,
                                       tc_handler: *mut ::cty::c_void,
                                       interr_handler: *mut ::cty::c_void,
                                       ctx: *mut ::cty::c_void)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_dma_irq_register(channel as ::cty::c_int,
                                    tc_handler as *mut ::cty::c_void,
                                    interr_handler as *mut ::cty::c_void,
                                    ctx as *mut ::cty::c_void);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn irq_unregister(channel: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_irq_unregister(channel: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_dma_irq_unregister(channel as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn find_node_by_channel(channel: ::cty::c_int)
     -> BlResult<*mut ::cty::c_void> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_find_node_by_channel(channel: ::cty::c_int)
            -> *mut ::cty::c_void;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_dma_find_node_by_channel(channel as ::cty::c_int);
            "----------Result----------";
            Ok(res)
        }
    }
    #[doc =
      "Get the DMA Context for an ADC Channel. See `bl_dma_find_ctx_by_channel` in \"init_adc\" <https://github.com/lupyuen/bl_iot_sdk/blob/adc/customer_app/sdk_app_adc2/sdk_app_adc2/demo.c>"]
    pub fn find_ctx_by_channel(channel: ::cty::c_int)
     -> BlResult<*mut ::cty::c_void> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_find_ctx_by_channel(channel: ::cty::c_int)
            -> *mut ::cty::c_void;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_dma_find_ctx_by_channel(channel as ::cty::c_int);
            "----------Result----------";
            Ok(res)
        }
    }
    pub fn mem_malloc(size: u32) -> BlResult<*mut ::cty::c_void> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_mem_malloc(size: u32)
            -> *mut ::cty::c_void;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_dma_mem_malloc(size as u32);
            "----------Result----------";
            Ok(res)
        }
    }
    pub fn mem_free(ptr: Ptr) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_dma_mem_free(ptr: *mut ::cty::c_void);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_dma_mem_free(ptr as *mut ::cty::c_void);
            "----------Result----------";
            Ok(())
        }
    }
}
/// GPIO HAL for BL602. See <https://lupyuen.github.io/articles/book#gpio-on-bl602>
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod gpio {
    use super::*;
    pub type __uint8_t = ::cty::c_uchar;
    #[repr(C)]
    pub struct _gpio_ctx_desc {
        pub next: *mut _gpio_ctx_desc,
        pub gpio_handler: ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                          *mut ::cty::c_void)>,
        pub arg: *mut ::cty::c_void,
        pub gpioPin: u8,
        pub intCtrlMod: u8,
        pub intTrgMod: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for _gpio_ctx_desc { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for _gpio_ctx_desc {
        #[inline]
        fn clone(&self) -> _gpio_ctx_desc {
            {
                let _: ::core::clone::AssertParamIsClone<*mut _gpio_ctx_desc>;
                let _:
                        ::core::clone::AssertParamIsClone<::core::option::Option<unsafe extern "C" fn(arg1:
                                                                                                          *mut ::cty::c_void)>>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    impl Default for _gpio_ctx_desc {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type gpio_ctx_t = _gpio_ctx_desc;
    #[doc =
      "Configure a GPIO Pin for Output Mode. See `bl_gpio_enable_output` in \"Enable GPIO\" <https://lupyuen.github.io/articles/led#enable-gpio>"]
    pub fn enable_output(pin: u8, pullup: u8, pulldown: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_enable_output(pin: u8, pullup: u8, pulldown: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_gpio_enable_output(pin as u8, pullup as u8,
                                      pulldown as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Configure a GPIO Pin for Input Mode. See `bl_gpio_enable_input` in \"Enable GPIO\" <https://lupyuen.github.io/articles/led#enable-gpio>"]
    pub fn enable_input(pin: u8, pullup: u8, pulldown: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_enable_input(pin: u8, pullup: u8, pulldown: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_gpio_enable_input(pin as u8, pullup as u8, pulldown as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Set the output value of a GPIO Pin. See `bl_gpio_output_set` in \"Read and Write GPIO\" <https://lupyuen.github.io/articles/led#read-and-write-gpio>"]
    pub fn output_set(pin: u8, value: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_output_set(pin: u8, value: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_gpio_output_set(pin as u8, value as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Get the input value of a GPIO Pin, by reference. See `bl_gpio_input_get` in \"Read and Write GPIO\" <https://lupyuen.github.io/articles/led#read-and-write-gpio>"]
    pub fn input_get(pin: u8, value: *mut u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_input_get(pin: u8, value: *mut u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_gpio_input_get(pin as u8, value as *mut u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Get the input value of GPIO Pin. See `bl_gpio_input_get_value` in \"Read and Write GPIO\" <https://lupyuen.github.io/articles/led#read-and-write-gpio>"]
    pub fn input_get_value(pin: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_input_get_value(pin: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_gpio_input_get_value(pin as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Clear GPIO Interrupt. See `bl_gpio_int_clear` in \"GPIO Interrupts\" <https://lupyuen.github.io/articles/led#gpio-interrupts>"]
    pub fn int_clear(gpioPin: u8, intClear: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_int_clear(gpioPin: u8, intClear: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_gpio_int_clear(gpioPin as u8, intClear as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Set GPIO Interrupt Mask. See `bl_gpio_intmask` in \"GPIO Interrupts\" <https://lupyuen.github.io/articles/led#gpio-interrupts>"]
    pub fn intmask(gpiopin: u8, mask: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_intmask(gpiopin: u8, mask: u8);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_gpio_intmask(gpiopin as u8, mask as u8);
            "----------Result----------";
            Ok(())
        }
    }
    #[doc =
      "Register GPIO Interrupt. See `bl_gpio_register` in \"GPIO Interrupts\" <https://lupyuen.github.io/articles/led#gpio-interrupts>"]
    pub fn register(pstnode: *mut gpio_ctx_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_gpio_register(pstnode: *mut gpio_ctx_t);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_gpio_register(pstnode as *mut gpio_ctx_t);
            "----------Result----------";
            Ok(())
        }
    }
}
/// I2C HAL for BL602. See <https://lupyuen.github.io/articles/book#i2c-on-bl602>
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod i2c {
    use super::*;
    pub const I2C_M_READ: u32 = 1;
    pub const I2C_M_WRITE: u32 = 0;
    pub const I2C_M_BLOCK: u32 = 0;
    pub const I2C_M_NO_BLOCK: u32 = 1;
    pub type __uint8_t = ::cty::c_uchar;
    pub type __uint16_t = ::cty::c_ushort;
    pub type __uint32_t = ::cty::c_uint;
    #[repr(C)]
    pub struct i2c_msg {
        pub addr: u16,
        pub direct: u8,
        pub subflag: u8,
        pub subaddr: u32,
        pub sublen: u8,
        pub len: u32,
        pub buf: *mut u8,
        pub event: ::cty::c_int,
        pub idex: ::cty::c_int,
        pub block: ::cty::c_int,
        pub stop: ::cty::c_int,
        pub ins_num: ::cty::c_int,
        pub i2cx: ::cty::c_int,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for i2c_msg { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for i2c_msg {
        #[inline]
        fn clone(&self) -> i2c_msg {
            {
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<*mut u8>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                *self
            }
        }
    }
    impl Default for i2c_msg {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type i2c_msg_t = i2c_msg;
    #[doc =
      "Set the Frequency of an I2C Port. See `i2c_set_freq` in \"Assign I2C Pins and set I2C Frequency\" <https://lupyuen.github.io/articles/i2c#assign-i2c-pins-and-set-i2c-frequency>"]
    pub fn set_freq(freq: ::cty::c_int, i2cx: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn i2c_set_freq(freq: ::cty::c_int, i2cx: ::cty::c_int);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            i2c_set_freq(freq as ::cty::c_int, i2cx as ::cty::c_int);
            "----------Result----------";
            Ok(())
        }
    }
    #[doc =
      "Init an I2C Port. See `i2c_gpio_init` in \"Assign I2C Pins and set I2C Frequency\" <https://lupyuen.github.io/articles/i2c#assign-i2c-pins-and-set-i2c-frequency>"]
    pub fn gpio_init(i2cx: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn i2c_gpio_init(i2cx: ::cty::c_int);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            i2c_gpio_init(i2cx as ::cty::c_int);
            "----------Result----------";
            Ok(())
        }
    }
    #[doc =
      "Clear the I2C Error Status for an I2C Port. See `i2c_clear_status` in \"Stop I2C Read\" <https://lupyuen.github.io/articles/i2c#stop-i2c-read>"]
    pub fn clear_status(i2cx: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn i2c_clear_status(i2cx: ::cty::c_int);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            i2c_clear_status(i2cx as ::cty::c_int);
            "----------Result----------";
            Ok(())
        }
    }
    #[doc =
      "Start an I2C Data Transfer. See `i2c_transfer_start` in \"Start I2C Transfer\" <https://lupyuen.github.io/articles/i2c#start-i2c-transfer>"]
    pub fn transfer_start(pstmsg: *mut i2c_msg_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn i2c_transfer_start(pstmsg: *mut i2c_msg_t);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            i2c_transfer_start(pstmsg as *mut i2c_msg_t);
            "----------Result----------";
            Ok(())
        }
    }
}
/// PWM HAL for BL602. See <https://lupyuen.github.io/articles/book#pwm-on-bl602>
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod pwm {
    use super::*;
    #[repr(C)]
    pub struct __BindgenBitfieldUnit<Storage> {
        storage: Storage,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::marker::Copy> ::core::marker::Copy for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::clone::Clone> ::core::clone::Clone for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn clone(&self) -> __BindgenBitfieldUnit<Storage> {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                __BindgenBitfieldUnit{storage:
                                          ::core::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::fmt::Debug> ::core::fmt::Debug for
     __BindgenBitfieldUnit<Storage> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                                                  "__BindgenBitfieldUnit");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                        "storage",
                                                        &&(*__self_0_0));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::default::Default> ::core::default::Default for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn default() -> __BindgenBitfieldUnit<Storage> {
            __BindgenBitfieldUnit{storage:
                                      ::core::default::Default::default(),}
        }
    }
    impl <Storage> ::core::marker::StructuralEq for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::Eq> ::core::cmp::Eq for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::core::cmp::AssertParamIsEq<Storage>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::hash::Hash> ::core::hash::Hash for
     __BindgenBitfieldUnit<Storage> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::Ord> ::core::cmp::Ord for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn cmp(&self, other: &__BindgenBitfieldUnit<Storage>)
         -> ::core::cmp::Ordering {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    match ::core::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal =>
                        ::core::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    impl <Storage> ::core::marker::StructuralPartialEq for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::PartialEq> ::core::cmp::PartialEq for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn eq(&self, other: &__BindgenBitfieldUnit<Storage>) -> bool {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &__BindgenBitfieldUnit<Storage>) -> bool {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn partial_cmp(&self, other: &__BindgenBitfieldUnit<Storage>)
         -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        =>
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
    }
    impl <Storage> __BindgenBitfieldUnit<Storage> {
        #[inline]
        pub const fn new(storage: Storage) -> Self { Self{storage,} }
    }
    impl <Storage> __BindgenBitfieldUnit<Storage> where Storage: AsRef<[u8]> +
     AsMut<[u8]> {
        #[inline]
        pub fn get_bit(&self, index: usize) -> bool {
            if true {
                if !(index / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: index / 8 < self.storage.as_ref().len()")
                };
            };
            let byte_index = index / 8;
            let byte = self.storage.as_ref()[byte_index];
            let bit_index = if false { 7 - (index % 8) } else { index % 8 };
            let mask = 1 << bit_index;
            byte & mask == mask
        }
        #[inline]
        pub fn set_bit(&mut self, index: usize, val: bool) {
            if true {
                if !(index / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: index / 8 < self.storage.as_ref().len()")
                };
            };
            let byte_index = index / 8;
            let byte = &mut self.storage.as_mut()[byte_index];
            let bit_index = if false { 7 - (index % 8) } else { index % 8 };
            let mask = 1 << bit_index;
            if val { *byte |= mask; } else { *byte &= !mask; }
        }
        #[inline]
        pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
            if true {
                if !(bit_width <= 64) {
                    ::core::panicking::panic("assertion failed: bit_width <= 64")
                };
            };
            if true {
                if !(bit_offset / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: bit_offset / 8 < self.storage.as_ref().len()")
                };
            };
            if true {
                if !((bit_offset + (bit_width as usize)) / 8 <=
                         self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()")
                };
            };
            let mut val = 0;
            for i in 0..(bit_width as usize) {
                if self.get_bit(i + bit_offset) {
                    let index =
                        if false { bit_width as usize - 1 - i } else { i };
                    val |= 1 << index;
                }
            }
            val
        }
        #[inline]
        pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
            if true {
                if !(bit_width <= 64) {
                    ::core::panicking::panic("assertion failed: bit_width <= 64")
                };
            };
            if true {
                if !(bit_offset / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: bit_offset / 8 < self.storage.as_ref().len()")
                };
            };
            if true {
                if !((bit_offset + (bit_width as usize)) / 8 <=
                         self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()")
                };
            };
            for i in 0..(bit_width as usize) {
                let mask = 1 << i;
                let val_bit_is_set = val & mask == mask;
                let index =
                    if false { bit_width as usize - 1 - i } else { i };
                self.set_bit(index + bit_offset, val_bit_is_set);
            }
        }
    }
    pub const PWM_INT_CONFIG_OFFSET: u32 = 0;
    pub const PWM_INTERRUPT_STS_POS: u32 = 0;
    pub const PWM_INTERRUPT_STS_LEN: u32 = 6;
    pub const PWM_INTERRUPT_STS_MSK: u32 = 63;
    pub const PWM_INTERRUPT_STS_UMSK: i32 = -64;
    pub const PWM_INT_CLEAR_POS: u32 = 8;
    pub const PWM_INT_CLEAR_LEN: u32 = 6;
    pub const PWM_INT_CLEAR_MSK: u32 = 16128;
    pub const PWM_INT_CLEAR_UMSK: i32 = -16129;
    pub const PWM0_CLKDIV_OFFSET: u32 = 32;
    pub const PWM_CLK_DIV_POS: u32 = 0;
    pub const PWM_CLK_DIV_LEN: u32 = 16;
    pub const PWM_CLK_DIV_MSK: u32 = 65535;
    pub const PWM_CLK_DIV_UMSK: i32 = -65536;
    pub const PWM0_THRE1_OFFSET: u32 = 36;
    pub const PWM_THRE1_POS: u32 = 0;
    pub const PWM_THRE1_LEN: u32 = 16;
    pub const PWM_THRE1_MSK: u32 = 65535;
    pub const PWM_THRE1_UMSK: i32 = -65536;
    pub const PWM0_THRE2_OFFSET: u32 = 40;
    pub const PWM_THRE2_POS: u32 = 0;
    pub const PWM_THRE2_LEN: u32 = 16;
    pub const PWM_THRE2_MSK: u32 = 65535;
    pub const PWM_THRE2_UMSK: i32 = -65536;
    pub const PWM0_PERIOD_OFFSET: u32 = 44;
    pub const PWM_PERIOD_POS: u32 = 0;
    pub const PWM_PERIOD_LEN: u32 = 16;
    pub const PWM_PERIOD_MSK: u32 = 65535;
    pub const PWM_PERIOD_UMSK: i32 = -65536;
    pub const PWM0_CONFIG_OFFSET: u32 = 48;
    pub const PWM_REG_CLK_SEL_POS: u32 = 0;
    pub const PWM_REG_CLK_SEL_LEN: u32 = 2;
    pub const PWM_REG_CLK_SEL_MSK: u32 = 3;
    pub const PWM_REG_CLK_SEL_UMSK: i32 = -4;
    pub const PWM_OUT_INV_POS: u32 = 2;
    pub const PWM_OUT_INV_LEN: u32 = 1;
    pub const PWM_OUT_INV_MSK: u32 = 4;
    pub const PWM_OUT_INV_UMSK: i32 = -5;
    pub const PWM_STOP_MODE_POS: u32 = 3;
    pub const PWM_STOP_MODE_LEN: u32 = 1;
    pub const PWM_STOP_MODE_MSK: u32 = 8;
    pub const PWM_STOP_MODE_UMSK: i32 = -9;
    pub const PWM_SW_FORCE_VAL_POS: u32 = 4;
    pub const PWM_SW_FORCE_VAL_LEN: u32 = 1;
    pub const PWM_SW_FORCE_VAL_MSK: u32 = 16;
    pub const PWM_SW_FORCE_VAL_UMSK: i32 = -17;
    pub const PWM_SW_MODE_POS: u32 = 5;
    pub const PWM_SW_MODE_LEN: u32 = 1;
    pub const PWM_SW_MODE_MSK: u32 = 32;
    pub const PWM_SW_MODE_UMSK: i32 = -33;
    pub const PWM_STOP_EN_POS: u32 = 6;
    pub const PWM_STOP_EN_LEN: u32 = 1;
    pub const PWM_STOP_EN_MSK: u32 = 64;
    pub const PWM_STOP_EN_UMSK: i32 = -65;
    pub const PWM_STS_TOP_POS: u32 = 7;
    pub const PWM_STS_TOP_LEN: u32 = 1;
    pub const PWM_STS_TOP_MSK: u32 = 128;
    pub const PWM_STS_TOP_UMSK: i32 = -129;
    pub const PWM0_INTERRUPT_OFFSET: u32 = 52;
    pub const PWM_INT_PERIOD_CNT_POS: u32 = 0;
    pub const PWM_INT_PERIOD_CNT_LEN: u32 = 16;
    pub const PWM_INT_PERIOD_CNT_MSK: u32 = 65535;
    pub const PWM_INT_PERIOD_CNT_UMSK: i32 = -65536;
    pub const PWM_INT_ENABLE_POS: u32 = 16;
    pub const PWM_INT_ENABLE_LEN: u32 = 1;
    pub const PWM_INT_ENABLE_MSK: u32 = 65536;
    pub const PWM_INT_ENABLE_UMSK: i32 = -65537;
    pub const PWM1_CLKDIV_OFFSET: u32 = 64;
    pub const PWM1_THRE1_OFFSET: u32 = 68;
    pub const PWM1_THRE2_OFFSET: u32 = 72;
    pub const PWM1_PERIOD_OFFSET: u32 = 76;
    pub const PWM1_CONFIG_OFFSET: u32 = 80;
    pub const PWM1_INTERRUPT_OFFSET: u32 = 84;
    pub const PWM2_CLKDIV_OFFSET: u32 = 96;
    pub const PWM2_THRE1_OFFSET: u32 = 100;
    pub const PWM2_THRE2_OFFSET: u32 = 104;
    pub const PWM2_PERIOD_OFFSET: u32 = 108;
    pub const PWM2_CONFIG_OFFSET: u32 = 112;
    pub const PWM2_INTERRUPT_OFFSET: u32 = 116;
    pub const PWM3_CLKDIV_OFFSET: u32 = 128;
    pub const PWM3_THRE1_OFFSET: u32 = 132;
    pub const PWM3_THRE2_OFFSET: u32 = 136;
    pub const PWM3_PERIOD_OFFSET: u32 = 140;
    pub const PWM3_CONFIG_OFFSET: u32 = 144;
    pub const PWM3_INTERRUPT_OFFSET: u32 = 148;
    pub const PWM4_CLKDIV_OFFSET: u32 = 160;
    pub const PWM4_THRE1_OFFSET: u32 = 164;
    pub const PWM4_THRE2_OFFSET: u32 = 168;
    pub const PWM4_PERIOD_OFFSET: u32 = 172;
    pub const PWM4_CONFIG_OFFSET: u32 = 176;
    pub const PWM4_INTERRUPT_OFFSET: u32 = 180;
    pub const PWM_CLKDIV_OFFSET: u32 = 0;
    pub const PWM_THRE1_OFFSET: u32 = 4;
    pub const PWM_THRE2_OFFSET: u32 = 8;
    pub const PWM_PERIOD_OFFSET: u32 = 12;
    pub const PWM_CONFIG_OFFSET: u32 = 16;
    pub const PWM_INTERRUPT_OFFSET: u32 = 20;
    pub const PWM_CHANNEL_OFFSET: u32 = 32;
    pub const BL_PWM_XTAL_CLK: u32 = 40000000;
    pub const BL_PWM_BUS_BCLK: u32 = 80000000;
    pub const BL_PWM_CLK: u32 = 40000000;
    pub type __uint8_t = ::cty::c_uchar;
    pub type __uint16_t = ::cty::c_ushort;
    pub type __int32_t = ::cty::c_int;
    pub type __uint32_t = ::cty::c_uint;
    #[repr(C)]
    pub struct pwm_reg {
        pub pwm_int_config: pwm_reg__bindgen_ty_1,
        pub RESERVED0x4: [u8; 28usize],
        pub pwm0_clkdiv: pwm_reg__bindgen_ty_2,
        pub pwm0_thre1: pwm_reg__bindgen_ty_3,
        pub pwm0_thre2: pwm_reg__bindgen_ty_4,
        pub pwm0_period: pwm_reg__bindgen_ty_5,
        pub pwm0_config: pwm_reg__bindgen_ty_6,
        pub pwm0_interrupt: pwm_reg__bindgen_ty_7,
        pub RESERVED0x38: [u8; 8usize],
        pub pwm1_clkdiv: pwm_reg__bindgen_ty_8,
        pub pwm1_thre1: pwm_reg__bindgen_ty_9,
        pub pwm1_thre2: pwm_reg__bindgen_ty_10,
        pub pwm1_period: pwm_reg__bindgen_ty_11,
        pub pwm1_config: pwm_reg__bindgen_ty_12,
        pub pwm1_interrupt: pwm_reg__bindgen_ty_13,
        pub RESERVED0x58: [u8; 8usize],
        pub pwm2_clkdiv: pwm_reg__bindgen_ty_14,
        pub pwm2_thre1: pwm_reg__bindgen_ty_15,
        pub pwm2_thre2: pwm_reg__bindgen_ty_16,
        pub pwm2_period: pwm_reg__bindgen_ty_17,
        pub pwm2_config: pwm_reg__bindgen_ty_18,
        pub pwm2_interrupt: pwm_reg__bindgen_ty_19,
        pub RESERVED0x78: [u8; 8usize],
        pub pwm3_clkdiv: pwm_reg__bindgen_ty_20,
        pub pwm3_thre1: pwm_reg__bindgen_ty_21,
        pub pwm3_thre2: pwm_reg__bindgen_ty_22,
        pub pwm3_period: pwm_reg__bindgen_ty_23,
        pub pwm3_config: pwm_reg__bindgen_ty_24,
        pub pwm3_interrupt: pwm_reg__bindgen_ty_25,
        pub RESERVED0x98: [u8; 8usize],
        pub pwm4_clkdiv: pwm_reg__bindgen_ty_26,
        pub pwm4_thre1: pwm_reg__bindgen_ty_27,
        pub pwm4_thre2: pwm_reg__bindgen_ty_28,
        pub pwm4_period: pwm_reg__bindgen_ty_29,
        pub pwm4_config: pwm_reg__bindgen_ty_30,
        pub pwm4_interrupt: pwm_reg__bindgen_ty_31,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg {
        #[inline]
        fn clone(&self) -> pwm_reg {
            {
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_1>;
                let _: ::core::clone::AssertParamIsClone<[u8; 28usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_2>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_3>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_4>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_5>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_6>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_7>;
                let _: ::core::clone::AssertParamIsClone<[u8; 8usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_8>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_9>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_10>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_11>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_12>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_13>;
                let _: ::core::clone::AssertParamIsClone<[u8; 8usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_14>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_15>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_16>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_17>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_18>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_19>;
                let _: ::core::clone::AssertParamIsClone<[u8; 8usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_20>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_21>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_22>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_23>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_24>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_25>;
                let _: ::core::clone::AssertParamIsClone<[u8; 8usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_26>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_27>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_28>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_29>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_30>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_reg__bindgen_ty_31>;
                *self
            }
        }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_1 {
        pub BF: pwm_reg__bindgen_ty_1__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_1 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_1__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_1__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_1__bindgen_ty_1 {
            pwm_reg__bindgen_ty_1__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_1__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_1__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_1__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_1__bindgen_ty_1 {
        #[inline]
        pub fn pwm_interrupt_sts(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 6u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_interrupt_sts(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 6u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_6_7(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_6_7(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_int_clear(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 6u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_clear(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 6u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_14_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(14usize, 18u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_14_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(14usize, 18u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_interrupt_sts: u32, reserved_6_7: u32,
                              pwm_int_clear: u32, reserved_14_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 6u8,
                                        {
                                            let pwm_interrupt_sts: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_interrupt_sts)
                                                };
                                            pwm_interrupt_sts as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 2u8,
                                        {
                                            let reserved_6_7: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_6_7)
                                                };
                                            reserved_6_7 as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 6u8,
                                        {
                                            let pwm_int_clear: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_clear)
                                                };
                                            pwm_int_clear as u64
                                        });
            __bindgen_bitfield_unit.set(14usize, 18u8,
                                        {
                                            let reserved_14_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_14_31)
                                                };
                                            reserved_14_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_1 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_2 {
        pub BF: pwm_reg__bindgen_ty_2__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_2 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_2 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_2 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_2__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_2__bindgen_ty_1 {
            pwm_reg__bindgen_ty_2__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_2__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_2__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        pub fn pwm_clk_div(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_clk_div(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_clk_div: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_clk_div: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_clk_div)
                                                };
                                            pwm_clk_div as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_2 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_3 {
        pub BF: pwm_reg__bindgen_ty_3__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_3 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_3 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_3 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_3__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_3__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_3__bindgen_ty_1 {
            pwm_reg__bindgen_ty_3__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_3__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_3__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_3__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_3__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre1(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre1(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre1: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre1: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre1)
                                                };
                                            pwm_thre1 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_3 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_4 {
        pub BF: pwm_reg__bindgen_ty_4__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_4 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_4 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_4 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_4__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_4__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_4__bindgen_ty_1 {
            pwm_reg__bindgen_ty_4__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_4__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_4__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_4__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_4__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre2(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre2(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre2: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre2: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre2)
                                                };
                                            pwm_thre2 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_4 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_5 {
        pub BF: pwm_reg__bindgen_ty_5__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_5 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_5 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_5 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_5__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_5__bindgen_ty_1 {
            pwm_reg__bindgen_ty_5__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_5__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_5__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        pub fn pwm_period(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_period(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_period: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_period: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_period)
                                                };
                                            pwm_period as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_5 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_6 {
        pub BF: pwm_reg__bindgen_ty_6__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_6 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_6 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_6 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_6__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_6__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_6__bindgen_ty_1 {
            pwm_reg__bindgen_ty_6__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_6__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_6__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_6__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_6__bindgen_ty_1 {
        #[inline]
        pub fn reg_clk_sel(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reg_clk_sel(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_out_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_out_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_force_val(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_force_val(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sts_top(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sts_top(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(reg_clk_sel: u32, pwm_out_inv: u32,
                              pwm_stop_mode: u32, pwm_sw_force_val: u32,
                              pwm_sw_mode: u32, pwm_stop_en: u32,
                              pwm_sts_top: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 2u8,
                                        {
                                            let reg_clk_sel: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reg_clk_sel)
                                                };
                                            reg_clk_sel as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let pwm_out_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_out_inv)
                                                };
                                            pwm_out_inv as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let pwm_stop_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_mode)
                                                };
                                            pwm_stop_mode as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let pwm_sw_force_val: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_force_val)
                                                };
                                            pwm_sw_force_val as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let pwm_sw_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_mode)
                                                };
                                            pwm_sw_mode as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let pwm_stop_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_en)
                                                };
                                            pwm_stop_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let pwm_sts_top: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sts_top)
                                                };
                                            pwm_sts_top as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_6 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_7 {
        pub BF: pwm_reg__bindgen_ty_7__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_7 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_7 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_7 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_7__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_7__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_7__bindgen_ty_1 {
            pwm_reg__bindgen_ty_7__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_7__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_7__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_7__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_7__bindgen_ty_1 {
        #[inline]
        pub fn pwm_int_period_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_period_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_int_enable(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_enable(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_17_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(17usize, 15u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_17_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(17usize, 15u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_int_period_cnt: u32, pwm_int_enable: u32,
                              reserved_17_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_int_period_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_period_cnt)
                                                };
                                            pwm_int_period_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 1u8,
                                        {
                                            let pwm_int_enable: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_enable)
                                                };
                                            pwm_int_enable as u64
                                        });
            __bindgen_bitfield_unit.set(17usize, 15u8,
                                        {
                                            let reserved_17_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_17_31)
                                                };
                                            reserved_17_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_7 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_8 {
        pub BF: pwm_reg__bindgen_ty_8__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_8 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_8 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_8 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_8__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_8__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_8__bindgen_ty_1 {
            pwm_reg__bindgen_ty_8__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_8__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_8__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_8__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_8__bindgen_ty_1 {
        #[inline]
        pub fn pwm_clk_div(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_clk_div(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_clk_div: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_clk_div: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_clk_div)
                                                };
                                            pwm_clk_div as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_8 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_9 {
        pub BF: pwm_reg__bindgen_ty_9__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_9 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_9 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_9 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_9__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_9__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_9__bindgen_ty_1 {
            pwm_reg__bindgen_ty_9__bindgen_ty_1{_bitfield_align_1:
                                                    ::core::default::Default::default(),
                                                _bitfield_1:
                                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_9__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_9__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_9__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_9__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre1(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre1(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre1: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre1: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre1)
                                                };
                                            pwm_thre1 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_9 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_10 {
        pub BF: pwm_reg__bindgen_ty_10__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_10 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_10 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_10 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_10__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_10__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_10__bindgen_ty_1 {
            pwm_reg__bindgen_ty_10__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_10__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_10__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_10__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_10__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre2(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre2(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre2: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre2: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre2)
                                                };
                                            pwm_thre2 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_10 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_11 {
        pub BF: pwm_reg__bindgen_ty_11__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_11 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_11 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_11 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_11__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_11__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_11__bindgen_ty_1 {
            pwm_reg__bindgen_ty_11__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_11__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_11__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_11__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_11__bindgen_ty_1 {
        #[inline]
        pub fn pwm_period(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_period(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_period: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_period: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_period)
                                                };
                                            pwm_period as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_11 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_12 {
        pub BF: pwm_reg__bindgen_ty_12__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_12 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_12 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_12 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_12__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_12__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_12__bindgen_ty_1 {
            pwm_reg__bindgen_ty_12__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_12__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_12__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_12__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_12__bindgen_ty_1 {
        #[inline]
        pub fn reg_clk_sel(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reg_clk_sel(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_out_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_out_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_force_val(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_force_val(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sts_top(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sts_top(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(reg_clk_sel: u32, pwm_out_inv: u32,
                              pwm_stop_mode: u32, pwm_sw_force_val: u32,
                              pwm_sw_mode: u32, pwm_stop_en: u32,
                              pwm_sts_top: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 2u8,
                                        {
                                            let reg_clk_sel: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reg_clk_sel)
                                                };
                                            reg_clk_sel as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let pwm_out_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_out_inv)
                                                };
                                            pwm_out_inv as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let pwm_stop_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_mode)
                                                };
                                            pwm_stop_mode as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let pwm_sw_force_val: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_force_val)
                                                };
                                            pwm_sw_force_val as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let pwm_sw_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_mode)
                                                };
                                            pwm_sw_mode as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let pwm_stop_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_en)
                                                };
                                            pwm_stop_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let pwm_sts_top: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sts_top)
                                                };
                                            pwm_sts_top as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_12 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_13 {
        pub BF: pwm_reg__bindgen_ty_13__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_13 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_13 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_13 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_13__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_13__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_13__bindgen_ty_1 {
            pwm_reg__bindgen_ty_13__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_13__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_13__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_13__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_13__bindgen_ty_1 {
        #[inline]
        pub fn pwm_int_period_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_period_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_int_enable(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_enable(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_17_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(17usize, 15u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_17_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(17usize, 15u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_int_period_cnt: u32, pwm_int_enable: u32,
                              reserved_17_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_int_period_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_period_cnt)
                                                };
                                            pwm_int_period_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 1u8,
                                        {
                                            let pwm_int_enable: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_enable)
                                                };
                                            pwm_int_enable as u64
                                        });
            __bindgen_bitfield_unit.set(17usize, 15u8,
                                        {
                                            let reserved_17_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_17_31)
                                                };
                                            reserved_17_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_13 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_14 {
        pub BF: pwm_reg__bindgen_ty_14__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_14 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_14 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_14 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_14__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_14__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_14__bindgen_ty_1 {
            pwm_reg__bindgen_ty_14__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_14__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_14__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_14__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_14__bindgen_ty_1 {
        #[inline]
        pub fn pwm_clk_div(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_clk_div(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_clk_div: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_clk_div: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_clk_div)
                                                };
                                            pwm_clk_div as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_14 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_15 {
        pub BF: pwm_reg__bindgen_ty_15__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_15 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_15 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_15 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_15__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_15__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_15__bindgen_ty_1 {
            pwm_reg__bindgen_ty_15__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_15__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_15__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_15__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_15__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre1(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre1(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre1: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre1: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre1)
                                                };
                                            pwm_thre1 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_15 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_16 {
        pub BF: pwm_reg__bindgen_ty_16__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_16 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_16 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_16 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_16__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_16__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_16__bindgen_ty_1 {
            pwm_reg__bindgen_ty_16__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_16__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_16__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_16__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_16__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre2(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre2(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre2: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre2: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre2)
                                                };
                                            pwm_thre2 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_16 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_17 {
        pub BF: pwm_reg__bindgen_ty_17__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_17 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_17 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_17 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_17__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_17__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_17__bindgen_ty_1 {
            pwm_reg__bindgen_ty_17__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_17__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_17__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_17__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_17__bindgen_ty_1 {
        #[inline]
        pub fn pwm_period(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_period(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_period: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_period: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_period)
                                                };
                                            pwm_period as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_17 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_18 {
        pub BF: pwm_reg__bindgen_ty_18__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_18 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_18 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_18 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_18__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_18__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_18__bindgen_ty_1 {
            pwm_reg__bindgen_ty_18__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_18__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_18__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_18__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_18__bindgen_ty_1 {
        #[inline]
        pub fn reg_clk_sel(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reg_clk_sel(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_out_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_out_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_force_val(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_force_val(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sts_top(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sts_top(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(reg_clk_sel: u32, pwm_out_inv: u32,
                              pwm_stop_mode: u32, pwm_sw_force_val: u32,
                              pwm_sw_mode: u32, pwm_stop_en: u32,
                              pwm_sts_top: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 2u8,
                                        {
                                            let reg_clk_sel: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reg_clk_sel)
                                                };
                                            reg_clk_sel as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let pwm_out_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_out_inv)
                                                };
                                            pwm_out_inv as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let pwm_stop_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_mode)
                                                };
                                            pwm_stop_mode as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let pwm_sw_force_val: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_force_val)
                                                };
                                            pwm_sw_force_val as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let pwm_sw_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_mode)
                                                };
                                            pwm_sw_mode as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let pwm_stop_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_en)
                                                };
                                            pwm_stop_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let pwm_sts_top: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sts_top)
                                                };
                                            pwm_sts_top as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_18 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_19 {
        pub BF: pwm_reg__bindgen_ty_19__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_19 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_19 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_19 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_19__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_19__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_19__bindgen_ty_1 {
            pwm_reg__bindgen_ty_19__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_19__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_19__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_19__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_19__bindgen_ty_1 {
        #[inline]
        pub fn pwm_int_period_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_period_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_int_enable(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_enable(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_17_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(17usize, 15u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_17_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(17usize, 15u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_int_period_cnt: u32, pwm_int_enable: u32,
                              reserved_17_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_int_period_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_period_cnt)
                                                };
                                            pwm_int_period_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 1u8,
                                        {
                                            let pwm_int_enable: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_enable)
                                                };
                                            pwm_int_enable as u64
                                        });
            __bindgen_bitfield_unit.set(17usize, 15u8,
                                        {
                                            let reserved_17_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_17_31)
                                                };
                                            reserved_17_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_19 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_20 {
        pub BF: pwm_reg__bindgen_ty_20__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_20 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_20 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_20 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_20__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_20__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_20__bindgen_ty_1 {
            pwm_reg__bindgen_ty_20__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_20__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_20__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_20__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_20__bindgen_ty_1 {
        #[inline]
        pub fn pwm_clk_div(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_clk_div(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_clk_div: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_clk_div: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_clk_div)
                                                };
                                            pwm_clk_div as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_20 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_21 {
        pub BF: pwm_reg__bindgen_ty_21__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_21 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_21 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_21 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_21__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_21__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_21__bindgen_ty_1 {
            pwm_reg__bindgen_ty_21__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_21__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_21__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_21__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_21__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre1(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre1(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre1: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre1: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre1)
                                                };
                                            pwm_thre1 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_21 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_22 {
        pub BF: pwm_reg__bindgen_ty_22__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_22 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_22 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_22 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_22__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_22__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_22__bindgen_ty_1 {
            pwm_reg__bindgen_ty_22__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_22__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_22__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_22__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_22__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre2(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre2(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre2: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre2: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre2)
                                                };
                                            pwm_thre2 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_22 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_23 {
        pub BF: pwm_reg__bindgen_ty_23__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_23 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_23 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_23 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_23__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_23__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_23__bindgen_ty_1 {
            pwm_reg__bindgen_ty_23__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_23__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_23__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_23__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_23__bindgen_ty_1 {
        #[inline]
        pub fn pwm_period(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_period(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_period: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_period: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_period)
                                                };
                                            pwm_period as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_23 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_24 {
        pub BF: pwm_reg__bindgen_ty_24__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_24 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_24 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_24 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_24__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_24__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_24__bindgen_ty_1 {
            pwm_reg__bindgen_ty_24__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_24__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_24__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_24__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_24__bindgen_ty_1 {
        #[inline]
        pub fn reg_clk_sel(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reg_clk_sel(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_out_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_out_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_force_val(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_force_val(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sts_top(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sts_top(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(reg_clk_sel: u32, pwm_out_inv: u32,
                              pwm_stop_mode: u32, pwm_sw_force_val: u32,
                              pwm_sw_mode: u32, pwm_stop_en: u32,
                              pwm_sts_top: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 2u8,
                                        {
                                            let reg_clk_sel: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reg_clk_sel)
                                                };
                                            reg_clk_sel as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let pwm_out_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_out_inv)
                                                };
                                            pwm_out_inv as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let pwm_stop_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_mode)
                                                };
                                            pwm_stop_mode as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let pwm_sw_force_val: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_force_val)
                                                };
                                            pwm_sw_force_val as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let pwm_sw_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_mode)
                                                };
                                            pwm_sw_mode as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let pwm_stop_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_en)
                                                };
                                            pwm_stop_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let pwm_sts_top: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sts_top)
                                                };
                                            pwm_sts_top as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_24 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_25 {
        pub BF: pwm_reg__bindgen_ty_25__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_25 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_25 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_25 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_25__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_25__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_25__bindgen_ty_1 {
            pwm_reg__bindgen_ty_25__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_25__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_25__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_25__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_25__bindgen_ty_1 {
        #[inline]
        pub fn pwm_int_period_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_period_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_int_enable(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_enable(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_17_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(17usize, 15u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_17_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(17usize, 15u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_int_period_cnt: u32, pwm_int_enable: u32,
                              reserved_17_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_int_period_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_period_cnt)
                                                };
                                            pwm_int_period_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 1u8,
                                        {
                                            let pwm_int_enable: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_enable)
                                                };
                                            pwm_int_enable as u64
                                        });
            __bindgen_bitfield_unit.set(17usize, 15u8,
                                        {
                                            let reserved_17_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_17_31)
                                                };
                                            reserved_17_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_25 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_26 {
        pub BF: pwm_reg__bindgen_ty_26__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_26 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_26 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_26 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_26__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_26__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_26__bindgen_ty_1 {
            pwm_reg__bindgen_ty_26__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_26__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_26__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_26__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_26__bindgen_ty_1 {
        #[inline]
        pub fn pwm_clk_div(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_clk_div(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_clk_div: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_clk_div: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_clk_div)
                                                };
                                            pwm_clk_div as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_26 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_27 {
        pub BF: pwm_reg__bindgen_ty_27__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_27 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_27 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_27 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_27__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_27__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_27__bindgen_ty_1 {
            pwm_reg__bindgen_ty_27__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_27__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_27__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_27__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_27__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre1(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre1(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre1: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre1: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre1)
                                                };
                                            pwm_thre1 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_27 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_28 {
        pub BF: pwm_reg__bindgen_ty_28__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_28 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_28 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_28 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_28__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_28__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_28__bindgen_ty_1 {
            pwm_reg__bindgen_ty_28__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_28__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_28__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_28__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_28__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre2(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre2(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre2: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre2: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre2)
                                                };
                                            pwm_thre2 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_28 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_29 {
        pub BF: pwm_reg__bindgen_ty_29__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_29 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_29 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_29 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_29__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_29__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_29__bindgen_ty_1 {
            pwm_reg__bindgen_ty_29__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_29__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_29__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_29__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_29__bindgen_ty_1 {
        #[inline]
        pub fn pwm_period(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_period(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_period: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_period: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_period)
                                                };
                                            pwm_period as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_29 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_30 {
        pub BF: pwm_reg__bindgen_ty_30__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_30 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_30 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_30 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_30__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_30__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_30__bindgen_ty_1 {
            pwm_reg__bindgen_ty_30__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_30__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_30__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_30__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_30__bindgen_ty_1 {
        #[inline]
        pub fn reg_clk_sel(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reg_clk_sel(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_out_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_out_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_force_val(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_force_val(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sts_top(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sts_top(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(reg_clk_sel: u32, pwm_out_inv: u32,
                              pwm_stop_mode: u32, pwm_sw_force_val: u32,
                              pwm_sw_mode: u32, pwm_stop_en: u32,
                              pwm_sts_top: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 2u8,
                                        {
                                            let reg_clk_sel: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reg_clk_sel)
                                                };
                                            reg_clk_sel as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let pwm_out_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_out_inv)
                                                };
                                            pwm_out_inv as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let pwm_stop_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_mode)
                                                };
                                            pwm_stop_mode as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let pwm_sw_force_val: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_force_val)
                                                };
                                            pwm_sw_force_val as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let pwm_sw_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_mode)
                                                };
                                            pwm_sw_mode as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let pwm_stop_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_en)
                                                };
                                            pwm_stop_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let pwm_sts_top: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sts_top)
                                                };
                                            pwm_sts_top as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_30 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_reg__bindgen_ty_31 {
        pub BF: pwm_reg__bindgen_ty_31__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_31 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_31 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_31 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_reg__bindgen_ty_31__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for pwm_reg__bindgen_ty_31__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_reg__bindgen_ty_31__bindgen_ty_1 {
            pwm_reg__bindgen_ty_31__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_reg__bindgen_ty_31__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_reg__bindgen_ty_31__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_reg__bindgen_ty_31__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_reg__bindgen_ty_31__bindgen_ty_1 {
        #[inline]
        pub fn pwm_int_period_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_period_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_int_enable(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_enable(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_17_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(17usize, 15u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_17_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(17usize, 15u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_int_period_cnt: u32, pwm_int_enable: u32,
                              reserved_17_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_int_period_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_period_cnt)
                                                };
                                            pwm_int_period_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 1u8,
                                        {
                                            let pwm_int_enable: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_enable)
                                                };
                                            pwm_int_enable as u64
                                        });
            __bindgen_bitfield_unit.set(17usize, 15u8,
                                        {
                                            let reserved_17_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_17_31)
                                                };
                                            reserved_17_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_reg__bindgen_ty_31 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    impl Default for pwm_reg {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type pwm_reg_t = pwm_reg;
    #[repr(C)]
    pub struct pwm_channel_reg {
        pub pwm_clkdiv: pwm_channel_reg__bindgen_ty_1,
        pub pwm_thre1: pwm_channel_reg__bindgen_ty_2,
        pub pwm_thre2: pwm_channel_reg__bindgen_ty_3,
        pub pwm_period: pwm_channel_reg__bindgen_ty_4,
        pub pwm_config: pwm_channel_reg__bindgen_ty_5,
        pub pwm_interrupt: pwm_channel_reg__bindgen_ty_6,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg {
        #[inline]
        fn clone(&self) -> pwm_channel_reg {
            {
                let _:
                        ::core::clone::AssertParamIsClone<pwm_channel_reg__bindgen_ty_1>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_channel_reg__bindgen_ty_2>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_channel_reg__bindgen_ty_3>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_channel_reg__bindgen_ty_4>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_channel_reg__bindgen_ty_5>;
                let _:
                        ::core::clone::AssertParamIsClone<pwm_channel_reg__bindgen_ty_6>;
                *self
            }
        }
    }
    #[repr(C)]
    pub union pwm_channel_reg__bindgen_ty_1 {
        pub BF: pwm_channel_reg__bindgen_ty_1__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_1 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_channel_reg__bindgen_ty_1__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for
     pwm_channel_reg__bindgen_ty_1__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_channel_reg__bindgen_ty_1__bindgen_ty_1 {
            pwm_channel_reg__bindgen_ty_1__bindgen_ty_1{_bitfield_align_1:
                                                            ::core::default::Default::default(),
                                                        _bitfield_1:
                                                            ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_1__bindgen_ty_1
     {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_1__bindgen_ty_1
     {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_1__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_channel_reg__bindgen_ty_1__bindgen_ty_1 {
        #[inline]
        pub fn pwm_clk_div(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_clk_div(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_clk_div: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_clk_div: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_clk_div)
                                                };
                                            pwm_clk_div as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_channel_reg__bindgen_ty_1 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_channel_reg__bindgen_ty_2 {
        pub BF: pwm_channel_reg__bindgen_ty_2__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_2 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_2 {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_2 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_channel_reg__bindgen_ty_2__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for
     pwm_channel_reg__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_channel_reg__bindgen_ty_2__bindgen_ty_1 {
            pwm_channel_reg__bindgen_ty_2__bindgen_ty_1{_bitfield_align_1:
                                                            ::core::default::Default::default(),
                                                        _bitfield_1:
                                                            ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_2__bindgen_ty_1
     {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_2__bindgen_ty_1
     {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_2__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_channel_reg__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre1(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre1(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre1: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre1: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre1)
                                                };
                                            pwm_thre1 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_channel_reg__bindgen_ty_2 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_channel_reg__bindgen_ty_3 {
        pub BF: pwm_channel_reg__bindgen_ty_3__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_3 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_3 {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_3 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_channel_reg__bindgen_ty_3__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for
     pwm_channel_reg__bindgen_ty_3__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_channel_reg__bindgen_ty_3__bindgen_ty_1 {
            pwm_channel_reg__bindgen_ty_3__bindgen_ty_1{_bitfield_align_1:
                                                            ::core::default::Default::default(),
                                                        _bitfield_1:
                                                            ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_3__bindgen_ty_1
     {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_3__bindgen_ty_1
     {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_3__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_channel_reg__bindgen_ty_3__bindgen_ty_1 {
        #[inline]
        pub fn pwm_thre2(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_thre2(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_thre2: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_thre2: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_thre2)
                                                };
                                            pwm_thre2 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_channel_reg__bindgen_ty_3 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_channel_reg__bindgen_ty_4 {
        pub BF: pwm_channel_reg__bindgen_ty_4__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_4 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_4 {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_4 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_channel_reg__bindgen_ty_4__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for
     pwm_channel_reg__bindgen_ty_4__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_channel_reg__bindgen_ty_4__bindgen_ty_1 {
            pwm_channel_reg__bindgen_ty_4__bindgen_ty_1{_bitfield_align_1:
                                                            ::core::default::Default::default(),
                                                        _bitfield_1:
                                                            ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_4__bindgen_ty_1
     {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_4__bindgen_ty_1
     {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_4__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_channel_reg__bindgen_ty_4__bindgen_ty_1 {
        #[inline]
        pub fn pwm_period(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_period(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_period: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_period: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_period)
                                                };
                                            pwm_period as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_channel_reg__bindgen_ty_4 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_channel_reg__bindgen_ty_5 {
        pub BF: pwm_channel_reg__bindgen_ty_5__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_5 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_5 {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_5 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_channel_reg__bindgen_ty_5__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for
     pwm_channel_reg__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_channel_reg__bindgen_ty_5__bindgen_ty_1 {
            pwm_channel_reg__bindgen_ty_5__bindgen_ty_1{_bitfield_align_1:
                                                            ::core::default::Default::default(),
                                                        _bitfield_1:
                                                            ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_5__bindgen_ty_1
     {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_5__bindgen_ty_1
     {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_5__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_channel_reg__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        pub fn reg_clk_sel(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reg_clk_sel(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_out_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_out_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_force_val(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_force_val(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sw_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sw_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_stop_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_stop_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_sts_top(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_sts_top(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(reg_clk_sel: u32, pwm_out_inv: u32,
                              pwm_stop_mode: u32, pwm_sw_force_val: u32,
                              pwm_sw_mode: u32, pwm_stop_en: u32,
                              pwm_sts_top: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 2u8,
                                        {
                                            let reg_clk_sel: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reg_clk_sel)
                                                };
                                            reg_clk_sel as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let pwm_out_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_out_inv)
                                                };
                                            pwm_out_inv as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let pwm_stop_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_mode)
                                                };
                                            pwm_stop_mode as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let pwm_sw_force_val: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_force_val)
                                                };
                                            pwm_sw_force_val as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let pwm_sw_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sw_mode)
                                                };
                                            pwm_sw_mode as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let pwm_stop_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_stop_en)
                                                };
                                            pwm_stop_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let pwm_sts_top: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_sts_top)
                                                };
                                            pwm_sts_top as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_channel_reg__bindgen_ty_5 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union pwm_channel_reg__bindgen_ty_6 {
        pub BF: pwm_channel_reg__bindgen_ty_6__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_6 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_6 {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_6 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct pwm_channel_reg__bindgen_ty_6__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for
     pwm_channel_reg__bindgen_ty_6__bindgen_ty_1 {
        #[inline]
        fn default() -> pwm_channel_reg__bindgen_ty_6__bindgen_ty_1 {
            pwm_channel_reg__bindgen_ty_6__bindgen_ty_1{_bitfield_align_1:
                                                            ::core::default::Default::default(),
                                                        _bitfield_1:
                                                            ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pwm_channel_reg__bindgen_ty_6__bindgen_ty_1
     {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pwm_channel_reg__bindgen_ty_6__bindgen_ty_1
     {
        #[inline]
        fn clone(&self) -> pwm_channel_reg__bindgen_ty_6__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl pwm_channel_reg__bindgen_ty_6__bindgen_ty_1 {
        #[inline]
        pub fn pwm_int_period_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_period_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn pwm_int_enable(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_pwm_int_enable(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_17_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(17usize, 15u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_17_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(17usize, 15u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(pwm_int_period_cnt: u32, pwm_int_enable: u32,
                              reserved_17_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let pwm_int_period_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_period_cnt)
                                                };
                                            pwm_int_period_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 1u8,
                                        {
                                            let pwm_int_enable: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(pwm_int_enable)
                                                };
                                            pwm_int_enable as u64
                                        });
            __bindgen_bitfield_unit.set(17usize, 15u8,
                                        {
                                            let reserved_17_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_17_31)
                                                };
                                            reserved_17_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for pwm_channel_reg__bindgen_ty_6 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    impl Default for pwm_channel_reg {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type pwm_channel_reg_t = pwm_channel_reg;
    pub const BL_Err_Type_SUCCESS: BL_Err_Type = 0;
    pub const BL_Err_Type_ERROR: BL_Err_Type = 1;
    pub const BL_Err_Type_TIMEOUT: BL_Err_Type = 2;
    #[doc = " @brief Error type definition"]
    pub type BL_Err_Type = ::cty::c_uint;
    pub const BL_Mask_Type_UNMASK: BL_Mask_Type = 0;
    pub const BL_Mask_Type_MASK: BL_Mask_Type = 1;
    #[doc = " @brief Mask type definition"]
    pub type BL_Mask_Type = ::cty::c_uint;
    #[doc = "  @brief Interrupt callback function type"]
    pub type intCallback_Type =
     ::core::option::Option<unsafe extern "C" fn()>;
    #[doc = "< PWM Channel 0 define"]
    pub const PWM_CH_ID_Type_PWM_CH0: PWM_CH_ID_Type = 0;
    #[doc = "< PWM Channel 1 define"]
    pub const PWM_CH_ID_Type_PWM_CH1: PWM_CH_ID_Type = 1;
    #[doc = "< PWM Channel 2 define"]
    pub const PWM_CH_ID_Type_PWM_CH2: PWM_CH_ID_Type = 2;
    #[doc = "< PWM Channel 3 define"]
    pub const PWM_CH_ID_Type_PWM_CH3: PWM_CH_ID_Type = 3;
    #[doc = "< PWM Channel 4 define"]
    pub const PWM_CH_ID_Type_PWM_CH4: PWM_CH_ID_Type = 4;
    #[doc = "<"]
    pub const PWM_CH_ID_Type_PWM_CH_MAX: PWM_CH_ID_Type = 5;
    #[doc = "  @brief PWM No. type definition"]
    pub type PWM_CH_ID_Type = ::cty::c_uint;
    #[doc = "< PWM Clock source :XTAL CLK"]
    pub const PWM_Clk_Type_PWM_CLK_XCLK: PWM_Clk_Type = 0;
    #[doc = "< PWM Clock source :Bus CLK"]
    pub const PWM_Clk_Type_PWM_CLK_BCLK: PWM_Clk_Type = 1;
    #[doc = "< PWM Clock source :32K CLK"]
    pub const PWM_Clk_Type_PWM_CLK_32K: PWM_Clk_Type = 2;
    #[doc = "  @brief PWM Clock definition"]
    pub type PWM_Clk_Type = ::cty::c_uint;
    #[doc = "< PWM stop abrupt select define"]
    pub const PWM_Stop_Mode_Type_PWM_STOP_ABRUPT: PWM_Stop_Mode_Type = 0;
    #[doc = "< PWM stop graceful select define"]
    pub const PWM_Stop_Mode_Type_PWM_STOP_GRACEFUL: PWM_Stop_Mode_Type = 1;
    #[doc = "  @brief PWM Stop Mode definition"]
    pub type PWM_Stop_Mode_Type = ::cty::c_uint;
    #[doc = "< PWM normal polarity mode define"]
    pub const PWM_Polarity_Type_PWM_POL_NORMAL: PWM_Polarity_Type = 0;
    #[doc = "< PWM invert polarity mode define"]
    pub const PWM_Polarity_Type_PWM_POL_INVERT: PWM_Polarity_Type = 1;
    #[doc = "  @brief PWM mode type def"]
    pub type PWM_Polarity_Type = ::cty::c_uint;
    #[doc = "< PWM Pulse count interrupt define"]
    pub const PWM_INT_Type_PWM_INT_PULSE_CNT: PWM_INT_Type = 0;
    #[doc = "<"]
    pub const PWM_INT_Type_PWM_INT_ALL: PWM_INT_Type = 1;
    #[doc = "  @brief PWM interrupt type def"]
    pub type PWM_INT_Type = ::cty::c_uint;
    #[doc = "  @brief PWM configuration structure type definition"]
    #[repr(C)]
    pub struct PWM_CH_CFG_Type {
        #[doc = "< PWM channel"]
        pub ch: PWM_CH_ID_Type,
        #[doc = "< PWM Clock"]
        pub clk: PWM_Clk_Type,
        #[doc = "< PWM Stop Mode"]
        pub stopMode: PWM_Stop_Mode_Type,
        #[doc = "< PWM mode type"]
        pub pol: PWM_Polarity_Type,
        #[doc = "< PWM clkDiv num"]
        pub clkDiv: u16,
        #[doc = "< PWM period set"]
        pub period: u16,
        #[doc = "< PWM threshold1 num"]
        pub threshold1: u16,
        #[doc = "< PWM threshold2 num"]
        pub threshold2: u16,
        #[doc = "< PWM interrupt pulse count"]
        pub intPulseCnt: u16,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PWM_CH_CFG_Type { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PWM_CH_CFG_Type {
        #[inline]
        fn clone(&self) -> PWM_CH_CFG_Type {
            {
                let _: ::core::clone::AssertParamIsClone<PWM_CH_ID_Type>;
                let _: ::core::clone::AssertParamIsClone<PWM_Clk_Type>;
                let _: ::core::clone::AssertParamIsClone<PWM_Stop_Mode_Type>;
                let _: ::core::clone::AssertParamIsClone<PWM_Polarity_Type>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                *self
            }
        }
    }
    impl Default for PWM_CH_CFG_Type {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    extern "C" {
        pub fn PWM_Channel_Init(chCfg: *mut PWM_CH_CFG_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn PWM_Channel_Update(ch: u8, period: u16, threshold1: u16,
                                  threshold2: u16);
    }
    extern "C" {
        pub fn PWM_Channel_Set_Div(ch: u8, div: u16);
    }
    extern "C" {
        pub fn PWM_Channel_Set_Threshold1(ch: u8, threshold1: u16);
    }
    extern "C" {
        pub fn PWM_Channel_Set_Threshold2(ch: u8, threshold2: u16);
    }
    extern "C" {
        pub fn PWM_Channel_Set_Period(ch: u8, period: u16);
    }
    extern "C" {
        pub fn PWM_Channel_Get(ch: u8, period: *mut u16, threshold1: *mut u16,
                               threshold2: *mut u16);
    }
    extern "C" {
        pub fn PWM_IntMask(ch: u8, intType: PWM_INT_Type,
                           intMask: BL_Mask_Type);
    }
    extern "C" {
        pub fn PWM_Channel_Enable(ch: u8);
    }
    extern "C" {
        pub fn PWM_Channel_Disable(ch: u8);
    }
    extern "C" {
        pub fn PWM_Int_Callback_Install(intType: u32,
                                        cbFun: intCallback_Type);
    }
    #[doc =
      "Designate a GPIO Pin as a PWM Channel. See `bl_pwm_init` in \"Initialise PWM\" <https://lupyuen.github.io/articles/led#initialise-pwm>"]
    pub fn init(id: u8, pin: u8, freq: u32) -> BlResult<i32> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_pwm_init(id: u8, pin: u8, freq: u32)
            -> i32;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_pwm_init(id as u8, pin as u8, freq as u32);
            "----------Result----------";
            Ok(res)
        }
    }
    #[doc =
      "Start a PWM Channel. See `bl_pwm_start` in \"PWM Operation\" <https://lupyuen.github.io/articles/led#pwm-operation>"]
    pub fn start(id: u8) -> BlResult<i32> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_pwm_start(id: u8)
            -> i32;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_pwm_start(id as u8);
            "----------Result----------";
            Ok(res)
        }
    }
    #[doc =
      "Stop a PWM Channel. See `bl_pwm_stop` in \"PWM Operation\" <https://lupyuen.github.io/articles/led#pwm-operation>"]
    pub fn stop(id: u8) -> BlResult<i32> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_pwm_stop(id: u8)
            -> i32;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_pwm_stop(id as u8);
            "----------Result----------";
            Ok(res)
        }
    }
    #[doc =
      "Set the Frequency of a PWM Channel. See `bl_pwm_set_freq` in \"PWM Frequency and Duty Cycle\" <https://lupyuen.github.io/articles/led#pwm-frequency-and-duty-cycle>"]
    pub fn set_freq(id: u8, freq: u32) -> BlResult<i32> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_pwm_set_freq(id: u8, freq: u32)
            -> i32;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_pwm_set_freq(id as u8, freq as u32);
            "----------Result----------";
            Ok(res)
        }
    }
    #[doc =
      "Set the Duty Cycle of a PWM Channel. See `bl_pwm_set_duty` in \"PWM Frequency and Duty Cycle\" <https://lupyuen.github.io/articles/led#pwm-frequency-and-duty-cycle>"]
    pub fn set_duty(id: u8, duty: f32) -> BlResult<i32> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_pwm_set_duty(id: u8, duty: f32)
            -> i32;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_pwm_set_duty(id as u8, duty as f32);
            "----------Result----------";
            Ok(res)
        }
    }
    #[doc =
      "Get the Duty Cycle of a PWM Channel. See `bl_pwm_get_duty` in \"PWM Frequency and Duty Cycle\" <https://lupyuen.github.io/articles/led#pwm-frequency-and-duty-cycle>"]
    pub fn get_duty(id: u8, p_duty: *mut f32) -> BlResult<i32> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_pwm_get_duty(id: u8, p_duty: *mut f32)
            -> i32;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_pwm_get_duty(id as u8, p_duty as *mut f32);
            "----------Result----------";
            Ok(res)
        }
    }
}
/// SPI HAL for BL602. See <https://lupyuen.github.io/articles/book#spi-on-bl602>
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod spi {
    use super::*;
    pub const HAL_SPI_MODE_MASTER: u32 = 1;
    pub const HAL_SPI_MODE_SLAVE: u32 = 2;
    pub type __uint8_t = ::cty::c_uchar;
    pub type __uint16_t = ::cty::c_ushort;
    pub type __int32_t = ::cty::c_int;
    pub type __uint32_t = ::cty::c_uint;
    #[repr(C)]
    pub struct spi_config_t {
        pub mode: u8,
        pub freq: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for spi_config_t {
        #[inline]
        fn default() -> spi_config_t {
            spi_config_t{mode: ::core::default::Default::default(),
                         freq: ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for spi_config_t { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for spi_config_t {
        #[inline]
        fn clone(&self) -> spi_config_t {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    #[repr(C)]
    pub struct spi_dev_t {
        pub port: u8,
        pub config: spi_config_t,
        pub priv_: *mut ::cty::c_void,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for spi_dev_t { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for spi_dev_t {
        #[inline]
        fn clone(&self) -> spi_dev_t {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<spi_config_t>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                *self
            }
        }
    }
    impl Default for spi_dev_t {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[doc =
      "Execute an SPI Data Transfer. See `hal_spi_transfer` in \"Execute the SPI Transfers\" <https://lupyuen.github.io/articles/spi#execute-the-spi-transfers>"]
    pub fn transfer(spi_dev: *mut spi_dev_t, xfer: Ptr, size: u8)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn hal_spi_transfer(spi_dev: *mut spi_dev_t,
                                    xfer: *mut ::cty::c_void, size: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                hal_spi_transfer(spi_dev as *mut spi_dev_t,
                                 xfer as *mut ::cty::c_void, size as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Initialise an SPI Port. See `spi_init` in \"Initialise SPI Port\" <https://lupyuen.github.io/articles/spi#initialise-spi-port>"]
    pub fn init(spi: *mut spi_dev_t, port: u8, mode: u8, polar_phase: u8,
                freq: u32, tx_dma_ch: u8, rx_dma_ch: u8, pin_clk: u8,
                pin_cs: u8, pin_mosi: u8, pin_miso: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn spi_init(spi: *mut spi_dev_t, port: u8, mode: u8,
                            polar_phase: u8, freq: u32, tx_dma_ch: u8,
                            rx_dma_ch: u8, pin_clk: u8, pin_cs: u8,
                            pin_mosi: u8, pin_miso: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                spi_init(spi as *mut spi_dev_t, port as u8, mode as u8,
                         polar_phase as u8, freq as u32, tx_dma_ch as u8,
                         rx_dma_ch as u8, pin_clk as u8, pin_cs as u8,
                         pin_mosi as u8, pin_miso as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
}
/// UART HAL for BL602. See <https://lupyuen.github.io/articles/book#uart-on-bl602>
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod uart {
    use super::*;
    #[repr(C)]
    pub struct __BindgenBitfieldUnit<Storage> {
        storage: Storage,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::marker::Copy> ::core::marker::Copy for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::clone::Clone> ::core::clone::Clone for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn clone(&self) -> __BindgenBitfieldUnit<Storage> {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                __BindgenBitfieldUnit{storage:
                                          ::core::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::fmt::Debug> ::core::fmt::Debug for
     __BindgenBitfieldUnit<Storage> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                                                  "__BindgenBitfieldUnit");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                        "storage",
                                                        &&(*__self_0_0));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::default::Default> ::core::default::Default for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn default() -> __BindgenBitfieldUnit<Storage> {
            __BindgenBitfieldUnit{storage:
                                      ::core::default::Default::default(),}
        }
    }
    impl <Storage> ::core::marker::StructuralEq for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::Eq> ::core::cmp::Eq for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::core::cmp::AssertParamIsEq<Storage>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::hash::Hash> ::core::hash::Hash for
     __BindgenBitfieldUnit<Storage> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::Ord> ::core::cmp::Ord for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn cmp(&self, other: &__BindgenBitfieldUnit<Storage>)
         -> ::core::cmp::Ordering {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    match ::core::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal =>
                        ::core::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    impl <Storage> ::core::marker::StructuralPartialEq for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::PartialEq> ::core::cmp::PartialEq for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn eq(&self, other: &__BindgenBitfieldUnit<Storage>) -> bool {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &__BindgenBitfieldUnit<Storage>) -> bool {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn partial_cmp(&self, other: &__BindgenBitfieldUnit<Storage>)
         -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        =>
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
    }
    impl <Storage> __BindgenBitfieldUnit<Storage> {
        #[inline]
        pub const fn new(storage: Storage) -> Self { Self{storage,} }
    }
    impl <Storage> __BindgenBitfieldUnit<Storage> where Storage: AsRef<[u8]> +
     AsMut<[u8]> {
        #[inline]
        pub fn get_bit(&self, index: usize) -> bool {
            if true {
                if !(index / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: index / 8 < self.storage.as_ref().len()")
                };
            };
            let byte_index = index / 8;
            let byte = self.storage.as_ref()[byte_index];
            let bit_index = if false { 7 - (index % 8) } else { index % 8 };
            let mask = 1 << bit_index;
            byte & mask == mask
        }
        #[inline]
        pub fn set_bit(&mut self, index: usize, val: bool) {
            if true {
                if !(index / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: index / 8 < self.storage.as_ref().len()")
                };
            };
            let byte_index = index / 8;
            let byte = &mut self.storage.as_mut()[byte_index];
            let bit_index = if false { 7 - (index % 8) } else { index % 8 };
            let mask = 1 << bit_index;
            if val { *byte |= mask; } else { *byte &= !mask; }
        }
        #[inline]
        pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
            if true {
                if !(bit_width <= 64) {
                    ::core::panicking::panic("assertion failed: bit_width <= 64")
                };
            };
            if true {
                if !(bit_offset / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: bit_offset / 8 < self.storage.as_ref().len()")
                };
            };
            if true {
                if !((bit_offset + (bit_width as usize)) / 8 <=
                         self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()")
                };
            };
            let mut val = 0;
            for i in 0..(bit_width as usize) {
                if self.get_bit(i + bit_offset) {
                    let index =
                        if false { bit_width as usize - 1 - i } else { i };
                    val |= 1 << index;
                }
            }
            val
        }
        #[inline]
        pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
            if true {
                if !(bit_width <= 64) {
                    ::core::panicking::panic("assertion failed: bit_width <= 64")
                };
            };
            if true {
                if !(bit_offset / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: bit_offset / 8 < self.storage.as_ref().len()")
                };
            };
            if true {
                if !((bit_offset + (bit_width as usize)) / 8 <=
                         self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()")
                };
            };
            for i in 0..(bit_width as usize) {
                let mask = 1 << i;
                let val_bit_is_set = val & mask == mask;
                let index =
                    if false { bit_width as usize - 1 - i } else { i };
                self.set_bit(index + bit_offset, val_bit_is_set);
            }
        }
    }
    pub const UART_UTX_CONFIG_OFFSET: u32 = 0;
    pub const UART_CR_UTX_EN_POS: u32 = 0;
    pub const UART_CR_UTX_EN_LEN: u32 = 1;
    pub const UART_CR_UTX_EN_MSK: u32 = 1;
    pub const UART_CR_UTX_EN_UMSK: i32 = -2;
    pub const UART_CR_UTX_CTS_EN_POS: u32 = 1;
    pub const UART_CR_UTX_CTS_EN_LEN: u32 = 1;
    pub const UART_CR_UTX_CTS_EN_MSK: u32 = 2;
    pub const UART_CR_UTX_CTS_EN_UMSK: i32 = -3;
    pub const UART_CR_UTX_FRM_EN_POS: u32 = 2;
    pub const UART_CR_UTX_FRM_EN_LEN: u32 = 1;
    pub const UART_CR_UTX_FRM_EN_MSK: u32 = 4;
    pub const UART_CR_UTX_FRM_EN_UMSK: i32 = -5;
    pub const UART_CR_UTX_PRT_EN_POS: u32 = 4;
    pub const UART_CR_UTX_PRT_EN_LEN: u32 = 1;
    pub const UART_CR_UTX_PRT_EN_MSK: u32 = 16;
    pub const UART_CR_UTX_PRT_EN_UMSK: i32 = -17;
    pub const UART_CR_UTX_PRT_SEL_POS: u32 = 5;
    pub const UART_CR_UTX_PRT_SEL_LEN: u32 = 1;
    pub const UART_CR_UTX_PRT_SEL_MSK: u32 = 32;
    pub const UART_CR_UTX_PRT_SEL_UMSK: i32 = -33;
    pub const UART_CR_UTX_IR_EN_POS: u32 = 6;
    pub const UART_CR_UTX_IR_EN_LEN: u32 = 1;
    pub const UART_CR_UTX_IR_EN_MSK: u32 = 64;
    pub const UART_CR_UTX_IR_EN_UMSK: i32 = -65;
    pub const UART_CR_UTX_IR_INV_POS: u32 = 7;
    pub const UART_CR_UTX_IR_INV_LEN: u32 = 1;
    pub const UART_CR_UTX_IR_INV_MSK: u32 = 128;
    pub const UART_CR_UTX_IR_INV_UMSK: i32 = -129;
    pub const UART_CR_UTX_BIT_CNT_D_POS: u32 = 8;
    pub const UART_CR_UTX_BIT_CNT_D_LEN: u32 = 3;
    pub const UART_CR_UTX_BIT_CNT_D_MSK: u32 = 1792;
    pub const UART_CR_UTX_BIT_CNT_D_UMSK: i32 = -1793;
    pub const UART_CR_UTX_BIT_CNT_P_POS: u32 = 12;
    pub const UART_CR_UTX_BIT_CNT_P_LEN: u32 = 2;
    pub const UART_CR_UTX_BIT_CNT_P_MSK: u32 = 12288;
    pub const UART_CR_UTX_BIT_CNT_P_UMSK: i32 = -12289;
    pub const UART_CR_UTX_LEN_POS: u32 = 16;
    pub const UART_CR_UTX_LEN_LEN: u32 = 16;
    pub const UART_CR_UTX_LEN_MSK: u32 = 4294901760;
    pub const UART_CR_UTX_LEN_UMSK: i64 = -4294901761;
    pub const UART_URX_CONFIG_OFFSET: u32 = 4;
    pub const UART_CR_URX_EN_POS: u32 = 0;
    pub const UART_CR_URX_EN_LEN: u32 = 1;
    pub const UART_CR_URX_EN_MSK: u32 = 1;
    pub const UART_CR_URX_EN_UMSK: i32 = -2;
    pub const UART_CR_URX_RTS_SW_MODE_POS: u32 = 1;
    pub const UART_CR_URX_RTS_SW_MODE_LEN: u32 = 1;
    pub const UART_CR_URX_RTS_SW_MODE_MSK: u32 = 2;
    pub const UART_CR_URX_RTS_SW_MODE_UMSK: i32 = -3;
    pub const UART_CR_URX_RTS_SW_VAL_POS: u32 = 2;
    pub const UART_CR_URX_RTS_SW_VAL_LEN: u32 = 1;
    pub const UART_CR_URX_RTS_SW_VAL_MSK: u32 = 4;
    pub const UART_CR_URX_RTS_SW_VAL_UMSK: i32 = -5;
    pub const UART_CR_URX_ABR_EN_POS: u32 = 3;
    pub const UART_CR_URX_ABR_EN_LEN: u32 = 1;
    pub const UART_CR_URX_ABR_EN_MSK: u32 = 8;
    pub const UART_CR_URX_ABR_EN_UMSK: i32 = -9;
    pub const UART_CR_URX_PRT_EN_POS: u32 = 4;
    pub const UART_CR_URX_PRT_EN_LEN: u32 = 1;
    pub const UART_CR_URX_PRT_EN_MSK: u32 = 16;
    pub const UART_CR_URX_PRT_EN_UMSK: i32 = -17;
    pub const UART_CR_URX_PRT_SEL_POS: u32 = 5;
    pub const UART_CR_URX_PRT_SEL_LEN: u32 = 1;
    pub const UART_CR_URX_PRT_SEL_MSK: u32 = 32;
    pub const UART_CR_URX_PRT_SEL_UMSK: i32 = -33;
    pub const UART_CR_URX_IR_EN_POS: u32 = 6;
    pub const UART_CR_URX_IR_EN_LEN: u32 = 1;
    pub const UART_CR_URX_IR_EN_MSK: u32 = 64;
    pub const UART_CR_URX_IR_EN_UMSK: i32 = -65;
    pub const UART_CR_URX_IR_INV_POS: u32 = 7;
    pub const UART_CR_URX_IR_INV_LEN: u32 = 1;
    pub const UART_CR_URX_IR_INV_MSK: u32 = 128;
    pub const UART_CR_URX_IR_INV_UMSK: i32 = -129;
    pub const UART_CR_URX_BIT_CNT_D_POS: u32 = 8;
    pub const UART_CR_URX_BIT_CNT_D_LEN: u32 = 3;
    pub const UART_CR_URX_BIT_CNT_D_MSK: u32 = 1792;
    pub const UART_CR_URX_BIT_CNT_D_UMSK: i32 = -1793;
    pub const UART_CR_URX_DEG_EN_POS: u32 = 11;
    pub const UART_CR_URX_DEG_EN_LEN: u32 = 1;
    pub const UART_CR_URX_DEG_EN_MSK: u32 = 2048;
    pub const UART_CR_URX_DEG_EN_UMSK: i32 = -2049;
    pub const UART_CR_URX_DEG_CNT_POS: u32 = 12;
    pub const UART_CR_URX_DEG_CNT_LEN: u32 = 4;
    pub const UART_CR_URX_DEG_CNT_MSK: u32 = 61440;
    pub const UART_CR_URX_DEG_CNT_UMSK: i32 = -61441;
    pub const UART_CR_URX_LEN_POS: u32 = 16;
    pub const UART_CR_URX_LEN_LEN: u32 = 16;
    pub const UART_CR_URX_LEN_MSK: u32 = 4294901760;
    pub const UART_CR_URX_LEN_UMSK: i64 = -4294901761;
    pub const UART_BIT_PRD_OFFSET: u32 = 8;
    pub const UART_CR_UTX_BIT_PRD_POS: u32 = 0;
    pub const UART_CR_UTX_BIT_PRD_LEN: u32 = 16;
    pub const UART_CR_UTX_BIT_PRD_MSK: u32 = 65535;
    pub const UART_CR_UTX_BIT_PRD_UMSK: i32 = -65536;
    pub const UART_CR_URX_BIT_PRD_POS: u32 = 16;
    pub const UART_CR_URX_BIT_PRD_LEN: u32 = 16;
    pub const UART_CR_URX_BIT_PRD_MSK: u32 = 4294901760;
    pub const UART_CR_URX_BIT_PRD_UMSK: i64 = -4294901761;
    pub const UART_DATA_CONFIG_OFFSET: u32 = 12;
    pub const UART_CR_UART_BIT_INV_POS: u32 = 0;
    pub const UART_CR_UART_BIT_INV_LEN: u32 = 1;
    pub const UART_CR_UART_BIT_INV_MSK: u32 = 1;
    pub const UART_CR_UART_BIT_INV_UMSK: i32 = -2;
    pub const UART_UTX_IR_POSITION_OFFSET: u32 = 16;
    pub const UART_CR_UTX_IR_POS_S_POS: u32 = 0;
    pub const UART_CR_UTX_IR_POS_S_LEN: u32 = 16;
    pub const UART_CR_UTX_IR_POS_S_MSK: u32 = 65535;
    pub const UART_CR_UTX_IR_POS_S_UMSK: i32 = -65536;
    pub const UART_CR_UTX_IR_POS_P_POS: u32 = 16;
    pub const UART_CR_UTX_IR_POS_P_LEN: u32 = 16;
    pub const UART_CR_UTX_IR_POS_P_MSK: u32 = 4294901760;
    pub const UART_CR_UTX_IR_POS_P_UMSK: i64 = -4294901761;
    pub const UART_URX_IR_POSITION_OFFSET: u32 = 20;
    pub const UART_CR_URX_IR_POS_S_POS: u32 = 0;
    pub const UART_CR_URX_IR_POS_S_LEN: u32 = 16;
    pub const UART_CR_URX_IR_POS_S_MSK: u32 = 65535;
    pub const UART_CR_URX_IR_POS_S_UMSK: i32 = -65536;
    pub const UART_URX_RTO_TIMER_OFFSET: u32 = 24;
    pub const UART_CR_URX_RTO_VALUE_POS: u32 = 0;
    pub const UART_CR_URX_RTO_VALUE_LEN: u32 = 8;
    pub const UART_CR_URX_RTO_VALUE_MSK: u32 = 255;
    pub const UART_CR_URX_RTO_VALUE_UMSK: i32 = -256;
    pub const UART_INT_STS_OFFSET: u32 = 32;
    pub const UART_UTX_END_INT_POS: u32 = 0;
    pub const UART_UTX_END_INT_LEN: u32 = 1;
    pub const UART_UTX_END_INT_MSK: u32 = 1;
    pub const UART_UTX_END_INT_UMSK: i32 = -2;
    pub const UART_URX_END_INT_POS: u32 = 1;
    pub const UART_URX_END_INT_LEN: u32 = 1;
    pub const UART_URX_END_INT_MSK: u32 = 2;
    pub const UART_URX_END_INT_UMSK: i32 = -3;
    pub const UART_UTX_FIFO_INT_POS: u32 = 2;
    pub const UART_UTX_FIFO_INT_LEN: u32 = 1;
    pub const UART_UTX_FIFO_INT_MSK: u32 = 4;
    pub const UART_UTX_FIFO_INT_UMSK: i32 = -5;
    pub const UART_URX_FIFO_INT_POS: u32 = 3;
    pub const UART_URX_FIFO_INT_LEN: u32 = 1;
    pub const UART_URX_FIFO_INT_MSK: u32 = 8;
    pub const UART_URX_FIFO_INT_UMSK: i32 = -9;
    pub const UART_URX_RTO_INT_POS: u32 = 4;
    pub const UART_URX_RTO_INT_LEN: u32 = 1;
    pub const UART_URX_RTO_INT_MSK: u32 = 16;
    pub const UART_URX_RTO_INT_UMSK: i32 = -17;
    pub const UART_URX_PCE_INT_POS: u32 = 5;
    pub const UART_URX_PCE_INT_LEN: u32 = 1;
    pub const UART_URX_PCE_INT_MSK: u32 = 32;
    pub const UART_URX_PCE_INT_UMSK: i32 = -33;
    pub const UART_UTX_FER_INT_POS: u32 = 6;
    pub const UART_UTX_FER_INT_LEN: u32 = 1;
    pub const UART_UTX_FER_INT_MSK: u32 = 64;
    pub const UART_UTX_FER_INT_UMSK: i32 = -65;
    pub const UART_URX_FER_INT_POS: u32 = 7;
    pub const UART_URX_FER_INT_LEN: u32 = 1;
    pub const UART_URX_FER_INT_MSK: u32 = 128;
    pub const UART_URX_FER_INT_UMSK: i32 = -129;
    pub const UART_INT_MASK_OFFSET: u32 = 36;
    pub const UART_CR_UTX_END_MASK_POS: u32 = 0;
    pub const UART_CR_UTX_END_MASK_LEN: u32 = 1;
    pub const UART_CR_UTX_END_MASK_MSK: u32 = 1;
    pub const UART_CR_UTX_END_MASK_UMSK: i32 = -2;
    pub const UART_CR_URX_END_MASK_POS: u32 = 1;
    pub const UART_CR_URX_END_MASK_LEN: u32 = 1;
    pub const UART_CR_URX_END_MASK_MSK: u32 = 2;
    pub const UART_CR_URX_END_MASK_UMSK: i32 = -3;
    pub const UART_CR_UTX_FIFO_MASK_POS: u32 = 2;
    pub const UART_CR_UTX_FIFO_MASK_LEN: u32 = 1;
    pub const UART_CR_UTX_FIFO_MASK_MSK: u32 = 4;
    pub const UART_CR_UTX_FIFO_MASK_UMSK: i32 = -5;
    pub const UART_CR_URX_FIFO_MASK_POS: u32 = 3;
    pub const UART_CR_URX_FIFO_MASK_LEN: u32 = 1;
    pub const UART_CR_URX_FIFO_MASK_MSK: u32 = 8;
    pub const UART_CR_URX_FIFO_MASK_UMSK: i32 = -9;
    pub const UART_CR_URX_RTO_MASK_POS: u32 = 4;
    pub const UART_CR_URX_RTO_MASK_LEN: u32 = 1;
    pub const UART_CR_URX_RTO_MASK_MSK: u32 = 16;
    pub const UART_CR_URX_RTO_MASK_UMSK: i32 = -17;
    pub const UART_CR_URX_PCE_MASK_POS: u32 = 5;
    pub const UART_CR_URX_PCE_MASK_LEN: u32 = 1;
    pub const UART_CR_URX_PCE_MASK_MSK: u32 = 32;
    pub const UART_CR_URX_PCE_MASK_UMSK: i32 = -33;
    pub const UART_CR_UTX_FER_MASK_POS: u32 = 6;
    pub const UART_CR_UTX_FER_MASK_LEN: u32 = 1;
    pub const UART_CR_UTX_FER_MASK_MSK: u32 = 64;
    pub const UART_CR_UTX_FER_MASK_UMSK: i32 = -65;
    pub const UART_CR_URX_FER_MASK_POS: u32 = 7;
    pub const UART_CR_URX_FER_MASK_LEN: u32 = 1;
    pub const UART_CR_URX_FER_MASK_MSK: u32 = 128;
    pub const UART_CR_URX_FER_MASK_UMSK: i32 = -129;
    pub const UART_INT_CLEAR_OFFSET: u32 = 40;
    pub const UART_CR_UTX_END_CLR_POS: u32 = 0;
    pub const UART_CR_UTX_END_CLR_LEN: u32 = 1;
    pub const UART_CR_UTX_END_CLR_MSK: u32 = 1;
    pub const UART_CR_UTX_END_CLR_UMSK: i32 = -2;
    pub const UART_CR_URX_END_CLR_POS: u32 = 1;
    pub const UART_CR_URX_END_CLR_LEN: u32 = 1;
    pub const UART_CR_URX_END_CLR_MSK: u32 = 2;
    pub const UART_CR_URX_END_CLR_UMSK: i32 = -3;
    pub const UART_CR_URX_RTO_CLR_POS: u32 = 4;
    pub const UART_CR_URX_RTO_CLR_LEN: u32 = 1;
    pub const UART_CR_URX_RTO_CLR_MSK: u32 = 16;
    pub const UART_CR_URX_RTO_CLR_UMSK: i32 = -17;
    pub const UART_CR_URX_PCE_CLR_POS: u32 = 5;
    pub const UART_CR_URX_PCE_CLR_LEN: u32 = 1;
    pub const UART_CR_URX_PCE_CLR_MSK: u32 = 32;
    pub const UART_CR_URX_PCE_CLR_UMSK: i32 = -33;
    pub const UART_INT_EN_OFFSET: u32 = 44;
    pub const UART_CR_UTX_END_EN_POS: u32 = 0;
    pub const UART_CR_UTX_END_EN_LEN: u32 = 1;
    pub const UART_CR_UTX_END_EN_MSK: u32 = 1;
    pub const UART_CR_UTX_END_EN_UMSK: i32 = -2;
    pub const UART_CR_URX_END_EN_POS: u32 = 1;
    pub const UART_CR_URX_END_EN_LEN: u32 = 1;
    pub const UART_CR_URX_END_EN_MSK: u32 = 2;
    pub const UART_CR_URX_END_EN_UMSK: i32 = -3;
    pub const UART_CR_UTX_FIFO_EN_POS: u32 = 2;
    pub const UART_CR_UTX_FIFO_EN_LEN: u32 = 1;
    pub const UART_CR_UTX_FIFO_EN_MSK: u32 = 4;
    pub const UART_CR_UTX_FIFO_EN_UMSK: i32 = -5;
    pub const UART_CR_URX_FIFO_EN_POS: u32 = 3;
    pub const UART_CR_URX_FIFO_EN_LEN: u32 = 1;
    pub const UART_CR_URX_FIFO_EN_MSK: u32 = 8;
    pub const UART_CR_URX_FIFO_EN_UMSK: i32 = -9;
    pub const UART_CR_URX_RTO_EN_POS: u32 = 4;
    pub const UART_CR_URX_RTO_EN_LEN: u32 = 1;
    pub const UART_CR_URX_RTO_EN_MSK: u32 = 16;
    pub const UART_CR_URX_RTO_EN_UMSK: i32 = -17;
    pub const UART_CR_URX_PCE_EN_POS: u32 = 5;
    pub const UART_CR_URX_PCE_EN_LEN: u32 = 1;
    pub const UART_CR_URX_PCE_EN_MSK: u32 = 32;
    pub const UART_CR_URX_PCE_EN_UMSK: i32 = -33;
    pub const UART_CR_UTX_FER_EN_POS: u32 = 6;
    pub const UART_CR_UTX_FER_EN_LEN: u32 = 1;
    pub const UART_CR_UTX_FER_EN_MSK: u32 = 64;
    pub const UART_CR_UTX_FER_EN_UMSK: i32 = -65;
    pub const UART_CR_URX_FER_EN_POS: u32 = 7;
    pub const UART_CR_URX_FER_EN_LEN: u32 = 1;
    pub const UART_CR_URX_FER_EN_MSK: u32 = 128;
    pub const UART_CR_URX_FER_EN_UMSK: i32 = -129;
    pub const UART_STATUS_OFFSET: u32 = 48;
    pub const UART_STS_UTX_BUS_BUSY_POS: u32 = 0;
    pub const UART_STS_UTX_BUS_BUSY_LEN: u32 = 1;
    pub const UART_STS_UTX_BUS_BUSY_MSK: u32 = 1;
    pub const UART_STS_UTX_BUS_BUSY_UMSK: i32 = -2;
    pub const UART_STS_URX_BUS_BUSY_POS: u32 = 1;
    pub const UART_STS_URX_BUS_BUSY_LEN: u32 = 1;
    pub const UART_STS_URX_BUS_BUSY_MSK: u32 = 2;
    pub const UART_STS_URX_BUS_BUSY_UMSK: i32 = -3;
    pub const UART_STS_URX_ABR_PRD_OFFSET: u32 = 52;
    pub const UART_STS_URX_ABR_PRD_START_POS: u32 = 0;
    pub const UART_STS_URX_ABR_PRD_START_LEN: u32 = 16;
    pub const UART_STS_URX_ABR_PRD_START_MSK: u32 = 65535;
    pub const UART_STS_URX_ABR_PRD_START_UMSK: i32 = -65536;
    pub const UART_STS_URX_ABR_PRD_0X55_POS: u32 = 16;
    pub const UART_STS_URX_ABR_PRD_0X55_LEN: u32 = 16;
    pub const UART_STS_URX_ABR_PRD_0X55_MSK: u32 = 4294901760;
    pub const UART_STS_URX_ABR_PRD_0X55_UMSK: i64 = -4294901761;
    pub const UART_FIFO_CONFIG_0_OFFSET: u32 = 128;
    pub const UART_DMA_TX_EN_POS: u32 = 0;
    pub const UART_DMA_TX_EN_LEN: u32 = 1;
    pub const UART_DMA_TX_EN_MSK: u32 = 1;
    pub const UART_DMA_TX_EN_UMSK: i32 = -2;
    pub const UART_DMA_RX_EN_POS: u32 = 1;
    pub const UART_DMA_RX_EN_LEN: u32 = 1;
    pub const UART_DMA_RX_EN_MSK: u32 = 2;
    pub const UART_DMA_RX_EN_UMSK: i32 = -3;
    pub const UART_TX_FIFO_CLR_POS: u32 = 2;
    pub const UART_TX_FIFO_CLR_LEN: u32 = 1;
    pub const UART_TX_FIFO_CLR_MSK: u32 = 4;
    pub const UART_TX_FIFO_CLR_UMSK: i32 = -5;
    pub const UART_RX_FIFO_CLR_POS: u32 = 3;
    pub const UART_RX_FIFO_CLR_LEN: u32 = 1;
    pub const UART_RX_FIFO_CLR_MSK: u32 = 8;
    pub const UART_RX_FIFO_CLR_UMSK: i32 = -9;
    pub const UART_TX_FIFO_OVERFLOW_POS: u32 = 4;
    pub const UART_TX_FIFO_OVERFLOW_LEN: u32 = 1;
    pub const UART_TX_FIFO_OVERFLOW_MSK: u32 = 16;
    pub const UART_TX_FIFO_OVERFLOW_UMSK: i32 = -17;
    pub const UART_TX_FIFO_UNDERFLOW_POS: u32 = 5;
    pub const UART_TX_FIFO_UNDERFLOW_LEN: u32 = 1;
    pub const UART_TX_FIFO_UNDERFLOW_MSK: u32 = 32;
    pub const UART_TX_FIFO_UNDERFLOW_UMSK: i32 = -33;
    pub const UART_RX_FIFO_OVERFLOW_POS: u32 = 6;
    pub const UART_RX_FIFO_OVERFLOW_LEN: u32 = 1;
    pub const UART_RX_FIFO_OVERFLOW_MSK: u32 = 64;
    pub const UART_RX_FIFO_OVERFLOW_UMSK: i32 = -65;
    pub const UART_RX_FIFO_UNDERFLOW_POS: u32 = 7;
    pub const UART_RX_FIFO_UNDERFLOW_LEN: u32 = 1;
    pub const UART_RX_FIFO_UNDERFLOW_MSK: u32 = 128;
    pub const UART_RX_FIFO_UNDERFLOW_UMSK: i32 = -129;
    pub const UART_FIFO_CONFIG_1_OFFSET: u32 = 132;
    pub const UART_TX_FIFO_CNT_POS: u32 = 0;
    pub const UART_TX_FIFO_CNT_LEN: u32 = 6;
    pub const UART_TX_FIFO_CNT_MSK: u32 = 63;
    pub const UART_TX_FIFO_CNT_UMSK: i32 = -64;
    pub const UART_RX_FIFO_CNT_POS: u32 = 8;
    pub const UART_RX_FIFO_CNT_LEN: u32 = 6;
    pub const UART_RX_FIFO_CNT_MSK: u32 = 16128;
    pub const UART_RX_FIFO_CNT_UMSK: i32 = -16129;
    pub const UART_TX_FIFO_TH_POS: u32 = 16;
    pub const UART_TX_FIFO_TH_LEN: u32 = 5;
    pub const UART_TX_FIFO_TH_MSK: u32 = 2031616;
    pub const UART_TX_FIFO_TH_UMSK: i32 = -2031617;
    pub const UART_RX_FIFO_TH_POS: u32 = 24;
    pub const UART_RX_FIFO_TH_LEN: u32 = 5;
    pub const UART_RX_FIFO_TH_MSK: u32 = 520093696;
    pub const UART_RX_FIFO_TH_UMSK: i32 = -520093697;
    pub const UART_FIFO_WDATA_OFFSET: u32 = 136;
    pub const UART_FIFO_WDATA_POS: u32 = 0;
    pub const UART_FIFO_WDATA_LEN: u32 = 8;
    pub const UART_FIFO_WDATA_MSK: u32 = 255;
    pub const UART_FIFO_WDATA_UMSK: i32 = -256;
    pub const UART_FIFO_RDATA_OFFSET: u32 = 140;
    pub const UART_FIFO_RDATA_POS: u32 = 0;
    pub const UART_FIFO_RDATA_LEN: u32 = 8;
    pub const UART_FIFO_RDATA_MSK: u32 = 255;
    pub const UART_FIFO_RDATA_UMSK: i32 = -256;
    pub const UART_RX_FIFO_SIZE: u32 = 32;
    pub const UART_TX_FIFO_SIZE: u32 = 32;
    pub const UART_DEFAULT_RECV_TIMEOUT: u32 = 80;
    pub const BL_UART_BUFFER_SIZE_MIN: u32 = 128;
    pub const BL_UART_BUFFER_SIZE_MASK: u32 = 127;
    pub type __uint8_t = ::cty::c_uchar;
    pub type __uint16_t = ::cty::c_ushort;
    pub type __uint32_t = ::cty::c_uint;
    #[repr(C)]
    pub struct uart_reg {
        pub utx_config: uart_reg__bindgen_ty_1,
        pub urx_config: uart_reg__bindgen_ty_2,
        pub uart_bit_prd: uart_reg__bindgen_ty_3,
        pub data_config: uart_reg__bindgen_ty_4,
        pub utx_ir_position: uart_reg__bindgen_ty_5,
        pub urx_ir_position: uart_reg__bindgen_ty_6,
        pub urx_rto_timer: uart_reg__bindgen_ty_7,
        pub RESERVED0x1c: [u8; 4usize],
        pub uart_int_sts: uart_reg__bindgen_ty_8,
        pub uart_int_mask: uart_reg__bindgen_ty_9,
        pub uart_int_clear: uart_reg__bindgen_ty_10,
        pub uart_int_en: uart_reg__bindgen_ty_11,
        pub uart_status: uart_reg__bindgen_ty_12,
        pub sts_urx_abr_prd: uart_reg__bindgen_ty_13,
        pub RESERVED0x38: [u8; 72usize],
        pub uart_fifo_config_0: uart_reg__bindgen_ty_14,
        pub uart_fifo_config_1: uart_reg__bindgen_ty_15,
        pub uart_fifo_wdata: uart_reg__bindgen_ty_16,
        pub uart_fifo_rdata: uart_reg__bindgen_ty_17,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg {
        #[inline]
        fn clone(&self) -> uart_reg {
            {
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_1>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_2>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_3>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_4>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_5>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_6>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_7>;
                let _: ::core::clone::AssertParamIsClone<[u8; 4usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_8>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_9>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_10>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_11>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_12>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_13>;
                let _: ::core::clone::AssertParamIsClone<[u8; 72usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_14>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_15>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_16>;
                let _:
                        ::core::clone::AssertParamIsClone<uart_reg__bindgen_ty_17>;
                *self
            }
        }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_1 {
        pub BF: uart_reg__bindgen_ty_1__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_1 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_1__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_1__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_1__bindgen_ty_1 {
            uart_reg__bindgen_ty_1__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_1__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_1__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_1__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_1__bindgen_ty_1 {
        #[inline]
        pub fn cr_utx_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_cts_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_cts_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_frm_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_frm_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_3(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_3(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_prt_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_prt_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_prt_sel(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_prt_sel(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_ir_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_ir_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_ir_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_ir_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_bit_cnt_d(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 3u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_bit_cnt_d(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 3u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_11(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(11usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_11(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(11usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_bit_cnt_p(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(12usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_bit_cnt_p(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(12usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_14_15(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(14usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_14_15(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(14usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_len(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_len(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_utx_en: u32, cr_utx_cts_en: u32,
                              cr_utx_frm_en: u32, reserved_3: u32,
                              cr_utx_prt_en: u32, cr_utx_prt_sel: u32,
                              cr_utx_ir_en: u32, cr_utx_ir_inv: u32,
                              cr_utx_bit_cnt_d: u32, reserved_11: u32,
                              cr_utx_bit_cnt_p: u32, reserved_14_15: u32,
                              cr_utx_len: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let cr_utx_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_en)
                                                };
                                            cr_utx_en as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let cr_utx_cts_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_cts_en)
                                                };
                                            cr_utx_cts_en as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let cr_utx_frm_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_frm_en)
                                                };
                                            cr_utx_frm_en as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let reserved_3: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_3)
                                                };
                                            reserved_3 as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let cr_utx_prt_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_prt_en)
                                                };
                                            cr_utx_prt_en as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let cr_utx_prt_sel: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_prt_sel)
                                                };
                                            cr_utx_prt_sel as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let cr_utx_ir_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_ir_en)
                                                };
                                            cr_utx_ir_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let cr_utx_ir_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_ir_inv)
                                                };
                                            cr_utx_ir_inv as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 3u8,
                                        {
                                            let cr_utx_bit_cnt_d: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_bit_cnt_d)
                                                };
                                            cr_utx_bit_cnt_d as u64
                                        });
            __bindgen_bitfield_unit.set(11usize, 1u8,
                                        {
                                            let reserved_11: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_11)
                                                };
                                            reserved_11 as u64
                                        });
            __bindgen_bitfield_unit.set(12usize, 2u8,
                                        {
                                            let cr_utx_bit_cnt_p: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_bit_cnt_p)
                                                };
                                            cr_utx_bit_cnt_p as u64
                                        });
            __bindgen_bitfield_unit.set(14usize, 2u8,
                                        {
                                            let reserved_14_15: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_14_15)
                                                };
                                            reserved_14_15 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let cr_utx_len: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_len)
                                                };
                                            cr_utx_len as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_1 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_2 {
        pub BF: uart_reg__bindgen_ty_2__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_2 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_2 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_2 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_2__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_2__bindgen_ty_1 {
            uart_reg__bindgen_ty_2__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_2__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_2__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        pub fn cr_urx_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_rts_sw_mode(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_rts_sw_mode(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_rts_sw_val(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_rts_sw_val(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_abr_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_abr_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_prt_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_prt_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_prt_sel(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_prt_sel(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_ir_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_ir_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_ir_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_ir_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_bit_cnt_d(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 3u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_bit_cnt_d(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 3u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_deg_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(11usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_deg_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(11usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_deg_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(12usize, 4u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_deg_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(12usize, 4u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_len(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_len(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_urx_en: u32, cr_urx_rts_sw_mode: u32,
                              cr_urx_rts_sw_val: u32, cr_urx_abr_en: u32,
                              cr_urx_prt_en: u32, cr_urx_prt_sel: u32,
                              cr_urx_ir_en: u32, cr_urx_ir_inv: u32,
                              cr_urx_bit_cnt_d: u32, cr_urx_deg_en: u32,
                              cr_urx_deg_cnt: u32, cr_urx_len: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let cr_urx_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_en)
                                                };
                                            cr_urx_en as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let cr_urx_rts_sw_mode: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_rts_sw_mode)
                                                };
                                            cr_urx_rts_sw_mode as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let cr_urx_rts_sw_val: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_rts_sw_val)
                                                };
                                            cr_urx_rts_sw_val as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let cr_urx_abr_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_abr_en)
                                                };
                                            cr_urx_abr_en as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let cr_urx_prt_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_prt_en)
                                                };
                                            cr_urx_prt_en as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let cr_urx_prt_sel: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_prt_sel)
                                                };
                                            cr_urx_prt_sel as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let cr_urx_ir_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_ir_en)
                                                };
                                            cr_urx_ir_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let cr_urx_ir_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_ir_inv)
                                                };
                                            cr_urx_ir_inv as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 3u8,
                                        {
                                            let cr_urx_bit_cnt_d: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_bit_cnt_d)
                                                };
                                            cr_urx_bit_cnt_d as u64
                                        });
            __bindgen_bitfield_unit.set(11usize, 1u8,
                                        {
                                            let cr_urx_deg_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_deg_en)
                                                };
                                            cr_urx_deg_en as u64
                                        });
            __bindgen_bitfield_unit.set(12usize, 4u8,
                                        {
                                            let cr_urx_deg_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_deg_cnt)
                                                };
                                            cr_urx_deg_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let cr_urx_len: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_len)
                                                };
                                            cr_urx_len as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_2 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_3 {
        pub BF: uart_reg__bindgen_ty_3__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_3 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_3 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_3 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_3__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_3__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_3__bindgen_ty_1 {
            uart_reg__bindgen_ty_3__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_3__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_3__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_3__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_3__bindgen_ty_1 {
        #[inline]
        pub fn cr_utx_bit_prd(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_bit_prd(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_bit_prd(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_bit_prd(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_utx_bit_prd: u32, cr_urx_bit_prd: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let cr_utx_bit_prd: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_bit_prd)
                                                };
                                            cr_utx_bit_prd as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let cr_urx_bit_prd: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_bit_prd)
                                                };
                                            cr_urx_bit_prd as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_3 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_4 {
        pub BF: uart_reg__bindgen_ty_4__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_4 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_4 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_4 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_4__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_4__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_4__bindgen_ty_1 {
            uart_reg__bindgen_ty_4__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_4__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_4__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_4__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_4__bindgen_ty_1 {
        #[inline]
        pub fn cr_uart_bit_inv(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_uart_bit_inv(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_1_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 31u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_1_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 31u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_uart_bit_inv: u32, reserved_1_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let cr_uart_bit_inv: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_uart_bit_inv)
                                                };
                                            cr_uart_bit_inv as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 31u8,
                                        {
                                            let reserved_1_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_1_31)
                                                };
                                            reserved_1_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_4 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_5 {
        pub BF: uart_reg__bindgen_ty_5__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_5 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_5 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_5 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_5__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_5__bindgen_ty_1 {
            uart_reg__bindgen_ty_5__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_5__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_5__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        pub fn cr_utx_ir_pos_s(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_ir_pos_s(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_ir_pos_p(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_ir_pos_p(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_utx_ir_pos_s: u32, cr_utx_ir_pos_p: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let cr_utx_ir_pos_s: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_ir_pos_s)
                                                };
                                            cr_utx_ir_pos_s as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let cr_utx_ir_pos_p: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_ir_pos_p)
                                                };
                                            cr_utx_ir_pos_p as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_5 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_6 {
        pub BF: uart_reg__bindgen_ty_6__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_6 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_6 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_6 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_6__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_6__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_6__bindgen_ty_1 {
            uart_reg__bindgen_ty_6__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_6__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_6__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_6__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_6__bindgen_ty_1 {
        #[inline]
        pub fn cr_urx_ir_pos_s(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_ir_pos_s(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_16_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_16_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_urx_ir_pos_s: u32, reserved_16_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let cr_urx_ir_pos_s: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_ir_pos_s)
                                                };
                                            cr_urx_ir_pos_s as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let reserved_16_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_16_31)
                                                };
                                            reserved_16_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_6 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_7 {
        pub BF: uart_reg__bindgen_ty_7__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_7 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_7 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_7 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_7__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_7__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_7__bindgen_ty_1 {
            uart_reg__bindgen_ty_7__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_7__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_7__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_7__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_7__bindgen_ty_1 {
        #[inline]
        pub fn cr_urx_rto_value(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 8u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_rto_value(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 8u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_urx_rto_value: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 8u8,
                                        {
                                            let cr_urx_rto_value: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_rto_value)
                                                };
                                            cr_urx_rto_value as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_7 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_8 {
        pub BF: uart_reg__bindgen_ty_8__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_8 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_8 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_8 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_8__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_8__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_8__bindgen_ty_1 {
            uart_reg__bindgen_ty_8__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_8__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_8__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_8__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_8__bindgen_ty_1 {
        #[inline]
        pub fn utx_end_int(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_utx_end_int(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn urx_end_int(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_urx_end_int(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn utx_fifo_int(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_utx_fifo_int(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn urx_fifo_int(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_urx_fifo_int(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn urx_rto_int(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_urx_rto_int(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn urx_pce_int(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_urx_pce_int(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn utx_fer_int(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_utx_fer_int(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn urx_fer_int(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_urx_fer_int(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(utx_end_int: u32, urx_end_int: u32,
                              utx_fifo_int: u32, urx_fifo_int: u32,
                              urx_rto_int: u32, urx_pce_int: u32,
                              utx_fer_int: u32, urx_fer_int: u32,
                              reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let utx_end_int: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(utx_end_int)
                                                };
                                            utx_end_int as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let urx_end_int: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(urx_end_int)
                                                };
                                            urx_end_int as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let utx_fifo_int: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(utx_fifo_int)
                                                };
                                            utx_fifo_int as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let urx_fifo_int: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(urx_fifo_int)
                                                };
                                            urx_fifo_int as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let urx_rto_int: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(urx_rto_int)
                                                };
                                            urx_rto_int as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let urx_pce_int: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(urx_pce_int)
                                                };
                                            urx_pce_int as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let utx_fer_int: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(utx_fer_int)
                                                };
                                            utx_fer_int as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let urx_fer_int: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(urx_fer_int)
                                                };
                                            urx_fer_int as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_8 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_9 {
        pub BF: uart_reg__bindgen_ty_9__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_9 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_9 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_9 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_9__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_9__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_9__bindgen_ty_1 {
            uart_reg__bindgen_ty_9__bindgen_ty_1{_bitfield_align_1:
                                                     ::core::default::Default::default(),
                                                 _bitfield_1:
                                                     ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_9__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_9__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_9__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_9__bindgen_ty_1 {
        #[inline]
        pub fn cr_utx_end_mask(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_end_mask(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_end_mask(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_end_mask(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_fifo_mask(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_fifo_mask(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_fifo_mask(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_fifo_mask(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_rto_mask(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_rto_mask(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_pce_mask(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_pce_mask(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_fer_mask(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_fer_mask(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_fer_mask(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_fer_mask(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_utx_end_mask: u32, cr_urx_end_mask: u32,
                              cr_utx_fifo_mask: u32, cr_urx_fifo_mask: u32,
                              cr_urx_rto_mask: u32, cr_urx_pce_mask: u32,
                              cr_utx_fer_mask: u32, cr_urx_fer_mask: u32,
                              reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let cr_utx_end_mask: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_end_mask)
                                                };
                                            cr_utx_end_mask as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let cr_urx_end_mask: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_end_mask)
                                                };
                                            cr_urx_end_mask as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let cr_utx_fifo_mask: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_fifo_mask)
                                                };
                                            cr_utx_fifo_mask as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let cr_urx_fifo_mask: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_fifo_mask)
                                                };
                                            cr_urx_fifo_mask as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let cr_urx_rto_mask: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_rto_mask)
                                                };
                                            cr_urx_rto_mask as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let cr_urx_pce_mask: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_pce_mask)
                                                };
                                            cr_urx_pce_mask as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let cr_utx_fer_mask: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_fer_mask)
                                                };
                                            cr_utx_fer_mask as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let cr_urx_fer_mask: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_fer_mask)
                                                };
                                            cr_urx_fer_mask as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_9 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_10 {
        pub BF: uart_reg__bindgen_ty_10__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_10 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_10 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_10 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_10__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_10__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_10__bindgen_ty_1 {
            uart_reg__bindgen_ty_10__bindgen_ty_1{_bitfield_align_1:
                                                      ::core::default::Default::default(),
                                                  _bitfield_1:
                                                      ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_10__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_10__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_10__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_10__bindgen_ty_1 {
        #[inline]
        pub fn cr_utx_end_clr(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_end_clr(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_end_clr(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_end_clr(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn rsvd_2(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rsvd_2(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn rsvd_3(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rsvd_3(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_rto_clr(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_rto_clr(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_pce_clr(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_pce_clr(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn rsvd_6(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rsvd_6(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn rsvd_7(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rsvd_7(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_utx_end_clr: u32, cr_urx_end_clr: u32,
                              rsvd_2: u32, rsvd_3: u32, cr_urx_rto_clr: u32,
                              cr_urx_pce_clr: u32, rsvd_6: u32, rsvd_7: u32,
                              reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let cr_utx_end_clr: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_end_clr)
                                                };
                                            cr_utx_end_clr as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let cr_urx_end_clr: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_end_clr)
                                                };
                                            cr_urx_end_clr as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let rsvd_2: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rsvd_2)
                                                };
                                            rsvd_2 as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let rsvd_3: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rsvd_3)
                                                };
                                            rsvd_3 as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let cr_urx_rto_clr: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_rto_clr)
                                                };
                                            cr_urx_rto_clr as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let cr_urx_pce_clr: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_pce_clr)
                                                };
                                            cr_urx_pce_clr as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let rsvd_6: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rsvd_6)
                                                };
                                            rsvd_6 as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let rsvd_7: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rsvd_7)
                                                };
                                            rsvd_7 as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_10 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_11 {
        pub BF: uart_reg__bindgen_ty_11__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_11 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_11 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_11 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_11__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_11__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_11__bindgen_ty_1 {
            uart_reg__bindgen_ty_11__bindgen_ty_1{_bitfield_align_1:
                                                      ::core::default::Default::default(),
                                                  _bitfield_1:
                                                      ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_11__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_11__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_11__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_11__bindgen_ty_1 {
        #[inline]
        pub fn cr_utx_end_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_end_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_end_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_end_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_fifo_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_fifo_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_fifo_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_fifo_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_rto_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_rto_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_pce_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_pce_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_utx_fer_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_utx_fer_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn cr_urx_fer_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_cr_urx_fer_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(cr_utx_end_en: u32, cr_urx_end_en: u32,
                              cr_utx_fifo_en: u32, cr_urx_fifo_en: u32,
                              cr_urx_rto_en: u32, cr_urx_pce_en: u32,
                              cr_utx_fer_en: u32, cr_urx_fer_en: u32,
                              reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let cr_utx_end_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_end_en)
                                                };
                                            cr_utx_end_en as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let cr_urx_end_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_end_en)
                                                };
                                            cr_urx_end_en as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let cr_utx_fifo_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_fifo_en)
                                                };
                                            cr_utx_fifo_en as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let cr_urx_fifo_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_fifo_en)
                                                };
                                            cr_urx_fifo_en as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let cr_urx_rto_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_rto_en)
                                                };
                                            cr_urx_rto_en as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let cr_urx_pce_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_pce_en)
                                                };
                                            cr_urx_pce_en as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let cr_utx_fer_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_utx_fer_en)
                                                };
                                            cr_utx_fer_en as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let cr_urx_fer_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(cr_urx_fer_en)
                                                };
                                            cr_urx_fer_en as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_11 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_12 {
        pub BF: uart_reg__bindgen_ty_12__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_12 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_12 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_12 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_12__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_12__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_12__bindgen_ty_1 {
            uart_reg__bindgen_ty_12__bindgen_ty_1{_bitfield_align_1:
                                                      ::core::default::Default::default(),
                                                  _bitfield_1:
                                                      ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_12__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_12__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_12__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_12__bindgen_ty_1 {
        #[inline]
        pub fn sts_utx_bus_busy(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_sts_utx_bus_busy(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn sts_urx_bus_busy(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_sts_urx_bus_busy(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_2_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 30u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_2_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 30u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(sts_utx_bus_busy: u32, sts_urx_bus_busy: u32,
                              reserved_2_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let sts_utx_bus_busy: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(sts_utx_bus_busy)
                                                };
                                            sts_utx_bus_busy as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let sts_urx_bus_busy: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(sts_urx_bus_busy)
                                                };
                                            sts_urx_bus_busy as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 30u8,
                                        {
                                            let reserved_2_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_2_31)
                                                };
                                            reserved_2_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_12 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_13 {
        pub BF: uart_reg__bindgen_ty_13__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_13 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_13 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_13 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_13__bindgen_ty_1 {
        pub _bitfield_align_1: [u16; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_13__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_13__bindgen_ty_1 {
            uart_reg__bindgen_ty_13__bindgen_ty_1{_bitfield_align_1:
                                                      ::core::default::Default::default(),
                                                  _bitfield_1:
                                                      ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_13__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_13__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_13__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u16; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_13__bindgen_ty_1 {
        #[inline]
        pub fn sts_urx_abr_prd_start(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_sts_urx_abr_prd_start(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn sts_urx_abr_prd_0x55(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_sts_urx_abr_prd_0x55(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 16u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(sts_urx_abr_prd_start: u32,
                              sts_urx_abr_prd_0x55: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 16u8,
                                        {
                                            let sts_urx_abr_prd_start: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(sts_urx_abr_prd_start)
                                                };
                                            sts_urx_abr_prd_start as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 16u8,
                                        {
                                            let sts_urx_abr_prd_0x55: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(sts_urx_abr_prd_0x55)
                                                };
                                            sts_urx_abr_prd_0x55 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_13 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_14 {
        pub BF: uart_reg__bindgen_ty_14__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_14 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_14 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_14 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_14__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_14__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_14__bindgen_ty_1 {
            uart_reg__bindgen_ty_14__bindgen_ty_1{_bitfield_align_1:
                                                      ::core::default::Default::default(),
                                                  _bitfield_1:
                                                      ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_14__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_14__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_14__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_14__bindgen_ty_1 {
        #[inline]
        pub fn uart_dma_tx_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_uart_dma_tx_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn uart_dma_rx_en(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_uart_dma_rx_en(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn tx_fifo_clr(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_tx_fifo_clr(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn rx_fifo_clr(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rx_fifo_clr(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn tx_fifo_overflow(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_tx_fifo_overflow(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn tx_fifo_underflow(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_tx_fifo_underflow(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(5usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn rx_fifo_overflow(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rx_fifo_overflow(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn rx_fifo_underflow(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rx_fifo_underflow(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(7usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(uart_dma_tx_en: u32, uart_dma_rx_en: u32,
                              tx_fifo_clr: u32, rx_fifo_clr: u32,
                              tx_fifo_overflow: u32, tx_fifo_underflow: u32,
                              rx_fifo_overflow: u32, rx_fifo_underflow: u32,
                              reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let uart_dma_tx_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(uart_dma_tx_en)
                                                };
                                            uart_dma_tx_en as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let uart_dma_rx_en: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(uart_dma_rx_en)
                                                };
                                            uart_dma_rx_en as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let tx_fifo_clr: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(tx_fifo_clr)
                                                };
                                            tx_fifo_clr as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let rx_fifo_clr: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rx_fifo_clr)
                                                };
                                            rx_fifo_clr as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 1u8,
                                        {
                                            let tx_fifo_overflow: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(tx_fifo_overflow)
                                                };
                                            tx_fifo_overflow as u64
                                        });
            __bindgen_bitfield_unit.set(5usize, 1u8,
                                        {
                                            let tx_fifo_underflow: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(tx_fifo_underflow)
                                                };
                                            tx_fifo_underflow as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 1u8,
                                        {
                                            let rx_fifo_overflow: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rx_fifo_overflow)
                                                };
                                            rx_fifo_overflow as u64
                                        });
            __bindgen_bitfield_unit.set(7usize, 1u8,
                                        {
                                            let rx_fifo_underflow: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rx_fifo_underflow)
                                                };
                                            rx_fifo_underflow as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_14 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_15 {
        pub BF: uart_reg__bindgen_ty_15__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_15 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_15 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_15 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_15__bindgen_ty_1 {
        pub _bitfield_align_1: [u8; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_15__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_15__bindgen_ty_1 {
            uart_reg__bindgen_ty_15__bindgen_ty_1{_bitfield_align_1:
                                                      ::core::default::Default::default(),
                                                  _bitfield_1:
                                                      ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_15__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_15__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_15__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_15__bindgen_ty_1 {
        #[inline]
        pub fn tx_fifo_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 6u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_tx_fifo_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 6u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_6_7(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(6usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_6_7(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(6usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn rx_fifo_cnt(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 6u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rx_fifo_cnt(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 6u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_14_15(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(14usize, 2u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_14_15(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(14usize, 2u8, val as u64)
            }
        }
        #[inline]
        pub fn tx_fifo_th(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(16usize, 5u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_tx_fifo_th(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(16usize, 5u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_21_23(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(21usize, 3u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_21_23(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(21usize, 3u8, val as u64)
            }
        }
        #[inline]
        pub fn rx_fifo_th(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(24usize, 5u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_rx_fifo_th(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(24usize, 5u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_29_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(29usize, 3u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_29_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(29usize, 3u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(tx_fifo_cnt: u32, reserved_6_7: u32,
                              rx_fifo_cnt: u32, reserved_14_15: u32,
                              tx_fifo_th: u32, reserved_21_23: u32,
                              rx_fifo_th: u32, reserved_29_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 6u8,
                                        {
                                            let tx_fifo_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(tx_fifo_cnt)
                                                };
                                            tx_fifo_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(6usize, 2u8,
                                        {
                                            let reserved_6_7: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_6_7)
                                                };
                                            reserved_6_7 as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 6u8,
                                        {
                                            let rx_fifo_cnt: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rx_fifo_cnt)
                                                };
                                            rx_fifo_cnt as u64
                                        });
            __bindgen_bitfield_unit.set(14usize, 2u8,
                                        {
                                            let reserved_14_15: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_14_15)
                                                };
                                            reserved_14_15 as u64
                                        });
            __bindgen_bitfield_unit.set(16usize, 5u8,
                                        {
                                            let tx_fifo_th: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(tx_fifo_th)
                                                };
                                            tx_fifo_th as u64
                                        });
            __bindgen_bitfield_unit.set(21usize, 3u8,
                                        {
                                            let reserved_21_23: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_21_23)
                                                };
                                            reserved_21_23 as u64
                                        });
            __bindgen_bitfield_unit.set(24usize, 5u8,
                                        {
                                            let rx_fifo_th: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(rx_fifo_th)
                                                };
                                            rx_fifo_th as u64
                                        });
            __bindgen_bitfield_unit.set(29usize, 3u8,
                                        {
                                            let reserved_29_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_29_31)
                                                };
                                            reserved_29_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_15 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_16 {
        pub BF: uart_reg__bindgen_ty_16__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_16 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_16 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_16 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_16__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_16__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_16__bindgen_ty_1 {
            uart_reg__bindgen_ty_16__bindgen_ty_1{_bitfield_align_1:
                                                      ::core::default::Default::default(),
                                                  _bitfield_1:
                                                      ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_16__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_16__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_16__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_16__bindgen_ty_1 {
        #[inline]
        pub fn uart_fifo_wdata(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 8u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_uart_fifo_wdata(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 8u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(uart_fifo_wdata: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 8u8,
                                        {
                                            let uart_fifo_wdata: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(uart_fifo_wdata)
                                                };
                                            uart_fifo_wdata as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_16 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub union uart_reg__bindgen_ty_17 {
        pub BF: uart_reg__bindgen_ty_17__bindgen_ty_1,
        pub WORD: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_17 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_17 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_17 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    #[repr(align(4))]
    pub struct uart_reg__bindgen_ty_17__bindgen_ty_1 {
        pub _bitfield_align_1: [u32; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for uart_reg__bindgen_ty_17__bindgen_ty_1 {
        #[inline]
        fn default() -> uart_reg__bindgen_ty_17__bindgen_ty_1 {
            uart_reg__bindgen_ty_17__bindgen_ty_1{_bitfield_align_1:
                                                      ::core::default::Default::default(),
                                                  _bitfield_1:
                                                      ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for uart_reg__bindgen_ty_17__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for uart_reg__bindgen_ty_17__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> uart_reg__bindgen_ty_17__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<[u32; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 4usize]>>;
                *self
            }
        }
    }
    impl uart_reg__bindgen_ty_17__bindgen_ty_1 {
        #[inline]
        pub fn uart_fifo_rdata(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 8u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_uart_fifo_rdata(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 8u8, val as u64)
            }
        }
        #[inline]
        pub fn reserved_8_31(&self) -> u32 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(8usize, 24u8) as
                                           u32)
            }
        }
        #[inline]
        pub fn set_reserved_8_31(&mut self, val: u32) {
            unsafe {
                let val: u32 = ::core::mem::transmute(val);
                self._bitfield_1.set(8usize, 24u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(uart_fifo_rdata: u32, reserved_8_31: u32)
         -> __BindgenBitfieldUnit<[u8; 4usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 8u8,
                                        {
                                            let uart_fifo_rdata: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(uart_fifo_rdata)
                                                };
                                            uart_fifo_rdata as u64
                                        });
            __bindgen_bitfield_unit.set(8usize, 24u8,
                                        {
                                            let reserved_8_31: u32 =
                                                unsafe {
                                                    ::core::mem::transmute(reserved_8_31)
                                                };
                                            reserved_8_31 as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    impl Default for uart_reg__bindgen_ty_17 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    impl Default for uart_reg {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type uart_reg_t = uart_reg;
    pub const BL_Err_Type_SUCCESS: BL_Err_Type = 0;
    pub const BL_Err_Type_ERROR: BL_Err_Type = 1;
    pub const BL_Err_Type_TIMEOUT: BL_Err_Type = 2;
    #[doc = " @brief Error type definition"]
    pub type BL_Err_Type = ::cty::c_uint;
    pub const BL_Fun_Type_DISABLE: BL_Fun_Type = 0;
    pub const BL_Fun_Type_ENABLE: BL_Fun_Type = 1;
    #[doc = " @brief Functional type definition"]
    pub type BL_Fun_Type = ::cty::c_uint;
    pub const BL_Sts_Type_RESET: BL_Sts_Type = 0;
    pub const BL_Sts_Type_SET: BL_Sts_Type = 1;
    #[doc = " @brief Status type definition"]
    pub type BL_Sts_Type = ::cty::c_uint;
    pub const BL_Mask_Type_UNMASK: BL_Mask_Type = 0;
    pub const BL_Mask_Type_MASK: BL_Mask_Type = 1;
    #[doc = " @brief Mask type definition"]
    pub type BL_Mask_Type = ::cty::c_uint;
    #[doc = "  @brief Interrupt callback function type"]
    pub type intCallback_Type =
     ::core::option::Option<unsafe extern "C" fn()>;
    #[doc = "< UART0 port define"]
    pub const UART_ID_Type_UART0_ID: UART_ID_Type = 0;
    #[doc = "< UART1 port define"]
    pub const UART_ID_Type_UART1_ID: UART_ID_Type = 1;
    #[doc = "< UART MAX ID define"]
    pub const UART_ID_Type_UART_ID_MAX: UART_ID_Type = 2;
    #[doc = "  @brief UART port type definition"]
    pub type UART_ID_Type = ::cty::c_uint;
    #[doc = "< UART TX Direction"]
    pub const UART_Direction_Type_UART_TX: UART_Direction_Type = 0;
    #[doc = "< UART RX Direction"]
    pub const UART_Direction_Type_UART_RX: UART_Direction_Type = 1;
    #[doc = "< UART TX and RX Direction"]
    pub const UART_Direction_Type_UART_TXRX: UART_Direction_Type = 2;
    #[doc = "  @brief UART direction type definition"]
    pub type UART_Direction_Type = ::cty::c_uint;
    #[doc = "< UART parity none define"]
    pub const UART_Parity_Type_UART_PARITY_NONE: UART_Parity_Type = 0;
    #[doc = "< UART parity odd define"]
    pub const UART_Parity_Type_UART_PARITY_ODD: UART_Parity_Type = 1;
    #[doc = "< UART parity even define"]
    pub const UART_Parity_Type_UART_PARITY_EVEN: UART_Parity_Type = 2;
    #[doc = "  @brief UART parity type definition"]
    pub type UART_Parity_Type = ::cty::c_uint;
    #[doc = "< UART data bits length:5 bits"]
    pub const UART_DataBits_Type_UART_DATABITS_5: UART_DataBits_Type = 0;
    #[doc = "< UART data bits length:6 bits"]
    pub const UART_DataBits_Type_UART_DATABITS_6: UART_DataBits_Type = 1;
    #[doc = "< UART data bits length:7 bits"]
    pub const UART_DataBits_Type_UART_DATABITS_7: UART_DataBits_Type = 2;
    #[doc = "< UART data bits length:8 bits"]
    pub const UART_DataBits_Type_UART_DATABITS_8: UART_DataBits_Type = 3;
    #[doc = "  @brief UART data bits type definiton"]
    pub type UART_DataBits_Type = ::cty::c_uint;
    #[doc = "< UART data stop bits length:1 bits"]
    pub const UART_StopBits_Type_UART_STOPBITS_1: UART_StopBits_Type = 0;
    #[doc = "< UART data stop bits length:1.5 bits"]
    pub const UART_StopBits_Type_UART_STOPBITS_1_5: UART_StopBits_Type = 1;
    #[doc = "< UART data stop bits length:2 bits"]
    pub const UART_StopBits_Type_UART_STOPBITS_2: UART_StopBits_Type = 2;
    #[doc = "  @brief UART stop bits type definiton"]
    pub type UART_StopBits_Type = ::cty::c_uint;
    #[doc = "< UART each byte is send out LSB-first"]
    pub const UART_ByteBitInverse_Type_UART_LSB_FIRST:
     UART_ByteBitInverse_Type =
        0;
    #[doc = "< UART each byte is send out MSB-first"]
    pub const UART_ByteBitInverse_Type_UART_MSB_FIRST:
     UART_ByteBitInverse_Type =
        1;
    #[doc =
      "  @brief UART each data byte is send out LSB-first or MSB-first type definiton"]
    pub type UART_ByteBitInverse_Type = ::cty::c_uint;
    #[doc = "< UART auto baudrate detection using codeword 0x55"]
    pub const UART_AutoBaudDetection_Type_UART_AUTOBAUD_0X55:
     UART_AutoBaudDetection_Type =
        0;
    #[doc = "< UART auto baudrate detection using start bit"]
    pub const UART_AutoBaudDetection_Type_UART_AUTOBAUD_STARTBIT:
     UART_AutoBaudDetection_Type =
        1;
    #[doc =
      "  @brief UART auto baudrate detection using codeword 0x55 or start bit definiton"]
    pub type UART_AutoBaudDetection_Type = ::cty::c_uint;
    #[doc = "< UART tx transfer end interrupt"]
    pub const UART_INT_Type_UART_INT_TX_END: UART_INT_Type = 0;
    #[doc = "< UART rx transfer end interrupt"]
    pub const UART_INT_Type_UART_INT_RX_END: UART_INT_Type = 1;
    #[doc = "< UART tx fifo interrupt when tx fifo count reaches,auto clear"]
    pub const UART_INT_Type_UART_INT_TX_FIFO_REQ: UART_INT_Type = 2;
    #[doc = "< UART rx fifo interrupt when rx fifo count reaches,auto clear"]
    pub const UART_INT_Type_UART_INT_RX_FIFO_REQ: UART_INT_Type = 3;
    #[doc = "< UART rx time-out interrupt"]
    pub const UART_INT_Type_UART_INT_RTO: UART_INT_Type = 4;
    #[doc = "< UART rx parity check error interrupt"]
    pub const UART_INT_Type_UART_INT_PCE: UART_INT_Type = 5;
    #[doc = "< UART tx fifo overflow/underflow error interrupt"]
    pub const UART_INT_Type_UART_INT_TX_FER: UART_INT_Type = 6;
    #[doc = "< UART rx fifo overflow/underflow error interrupt"]
    pub const UART_INT_Type_UART_INT_RX_FER: UART_INT_Type = 7;
    #[doc = "< All the interrupt"]
    pub const UART_INT_Type_UART_INT_ALL: UART_INT_Type = 8;
    #[doc = "  @brief UART interrupt type definition"]
    pub type UART_INT_Type = ::cty::c_uint;
    #[doc = "< UART tx fifo overflow"]
    pub const UART_Overflow_Type_UART_TX_OVERFLOW: UART_Overflow_Type = 0;
    #[doc = "< UART tx fifo underflow"]
    pub const UART_Overflow_Type_UART_TX_UNDERFLOW: UART_Overflow_Type = 1;
    #[doc = "< UART rx fifo overflow"]
    pub const UART_Overflow_Type_UART_RX_OVERFLOW: UART_Overflow_Type = 2;
    #[doc = "< UART rx fifo underflow"]
    pub const UART_Overflow_Type_UART_RX_UNDERFLOW: UART_Overflow_Type = 3;
    #[doc = "  @brief UART overflow or underflow type definition"]
    pub type UART_Overflow_Type = ::cty::c_uint;
    #[doc = "  @brief UART configuration structure type definition"]
    #[repr(C)]
    pub struct UART_CFG_Type {
        #[doc = "< Uart module clock"]
        pub uartClk: u32,
        #[doc = "< Uart baudrate"]
        pub baudRate: u32,
        #[doc = "< Uart frame length of data bit"]
        pub dataBits: UART_DataBits_Type,
        #[doc = "< Uart frame length of stop bit"]
        pub stopBits: UART_StopBits_Type,
        #[doc = "< Uart parity check type"]
        pub parity: UART_Parity_Type,
        #[doc = "< Enable or disable tx CTS flow control"]
        pub ctsFlowControl: BL_Fun_Type,
        #[doc = "< Enable or disable rx input de-glitch function"]
        pub rxDeglitch: BL_Fun_Type,
        #[doc = "< Enable or disable rx RTS output SW control mode"]
        pub rtsSoftwareControl: BL_Fun_Type,
        #[doc = "< Uart each data byte is send out LSB-first or MSB-first"]
        pub byteBitInverse: UART_ByteBitInverse_Type,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for UART_CFG_Type { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for UART_CFG_Type {
        #[inline]
        fn clone(&self) -> UART_CFG_Type {
            {
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<UART_DataBits_Type>;
                let _: ::core::clone::AssertParamIsClone<UART_StopBits_Type>;
                let _: ::core::clone::AssertParamIsClone<UART_Parity_Type>;
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                let _:
                        ::core::clone::AssertParamIsClone<UART_ByteBitInverse_Type>;
                *self
            }
        }
    }
    impl Default for UART_CFG_Type {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[doc = "  @brief UART FIFO configuration structure type definition"]
    #[repr(C)]
    pub struct UART_FifoCfg_Type {
        #[doc =
          "< TX FIFO threshold, dma tx request will not be asserted if tx fifo count is less than this value"]
        pub txFifoDmaThreshold: u8,
        #[doc =
          "< RX FIFO threshold, dma rx request will not be asserted if rx fifo count is less than this value"]
        pub rxFifoDmaThreshold: u8,
        #[doc = "< Enable or disable tx dma req/ack interface"]
        pub txFifoDmaEnable: BL_Fun_Type,
        #[doc = "< Enable or disable rx dma req/ack interface"]
        pub rxFifoDmaEnable: BL_Fun_Type,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for UART_FifoCfg_Type { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for UART_FifoCfg_Type {
        #[inline]
        fn clone(&self) -> UART_FifoCfg_Type {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                *self
            }
        }
    }
    impl Default for UART_FifoCfg_Type {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[doc = "  @brief UART infrared configuration structure type definition"]
    #[repr(C)]
    pub struct UART_IrCfg_Type {
        #[doc = "< Enable or disable uart tx ir mode"]
        pub txIrEnable: BL_Fun_Type,
        #[doc = "< Enable or disable uart rx ir mode"]
        pub rxIrEnable: BL_Fun_Type,
        #[doc =
          "< Enable or disable inverse signal of uart tx output in ir mode"]
        pub txIrInverse: BL_Fun_Type,
        #[doc =
          "< Enable or disable inverse signal of uart rx input in ir mode"]
        pub rxIrInverse: BL_Fun_Type,
        #[doc = "< Set start position of uart tx ir pulse"]
        pub txIrPulseStart: u16,
        #[doc = "< Set stop position of uart tx ir pulse"]
        pub txIrPulseStop: u16,
        #[doc =
          "< Set start position of uart rx pulse recovered from ir signal"]
        pub rxIrPulseStart: u16,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for UART_IrCfg_Type { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for UART_IrCfg_Type {
        #[inline]
        fn clone(&self) -> UART_IrCfg_Type {
            {
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                let _: ::core::clone::AssertParamIsClone<BL_Fun_Type>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                *self
            }
        }
    }
    impl Default for UART_IrCfg_Type {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    extern "C" {
        pub fn UART_Init(uartId: UART_ID_Type, uartCfg: *mut UART_CFG_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_DeInit(uartId: UART_ID_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_FifoConfig(uartId: UART_ID_Type,
                               fifoCfg: *mut UART_FifoCfg_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_IrConfig(uartId: UART_ID_Type,
                             irCfg: *mut UART_IrCfg_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_Enable(uartId: UART_ID_Type, direct: UART_Direction_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_Disable(uartId: UART_ID_Type, direct: UART_Direction_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_SetTxDataLength(uartId: UART_ID_Type, length: u16)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_SetRxDataLength(uartId: UART_ID_Type, length: u16)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_SetRxTimeoutValue(uartId: UART_ID_Type, time: u8)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_SetDeglitchCount(uartId: UART_ID_Type, deglitchCnt: u8)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_SetBaudrate(uartId: UART_ID_Type,
                                autoBaudDet: UART_AutoBaudDetection_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_SetRtsValue(uartId: UART_ID_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_ClrRtsValue(uartId: UART_ID_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_TxFreeRun(uartId: UART_ID_Type, txFreeRun: BL_Fun_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_AutoBaudDetection(uartId: UART_ID_Type,
                                      autoBaud: BL_Fun_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_TxFifoClear(uartId: UART_ID_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_RxFifoClear(uartId: UART_ID_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_IntMask(uartId: UART_ID_Type, intType: UART_INT_Type,
                            intMask: BL_Mask_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_IntClear(uartId: UART_ID_Type, intType: UART_INT_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_Int_Callback_Install(uartId: UART_ID_Type,
                                         intType: UART_INT_Type,
                                         cbFun: intCallback_Type)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_SendData(uartId: UART_ID_Type, data: *mut u8, len: u32)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_SendDataBlock(uartId: UART_ID_Type, data: *mut u8,
                                  len: u32)
        -> BL_Err_Type;
    }
    extern "C" {
        pub fn UART_ReceiveData(uartId: UART_ID_Type, data: *mut u8,
                                maxLen: u32)
        -> u32;
    }
    extern "C" {
        pub fn UART_GetAutoBaudCount(uartId: UART_ID_Type,
                                     autoBaudDet: UART_AutoBaudDetection_Type)
        -> u16;
    }
    extern "C" {
        pub fn UART_GetTxFifoCount(uartId: UART_ID_Type)
        -> u8;
    }
    extern "C" {
        pub fn UART_GetRxFifoCount(uartId: UART_ID_Type)
        -> u8;
    }
    extern "C" {
        pub fn UART_GetIntStatus(uartId: UART_ID_Type, intType: UART_INT_Type)
        -> BL_Sts_Type;
    }
    extern "C" {
        pub fn UART_GetTxBusBusyStatus(uartId: UART_ID_Type)
        -> BL_Sts_Type;
    }
    extern "C" {
        pub fn UART_GetRxBusBusyStatus(uartId: UART_ID_Type)
        -> BL_Sts_Type;
    }
    extern "C" {
        pub fn UART_GetOverflowStatus(uartId: UART_ID_Type,
                                      overflow: UART_Overflow_Type)
        -> BL_Sts_Type;
    }
    pub type cb_uart_notify_t =
     ::core::option::Option<unsafe extern "C" fn(arg: *mut ::cty::c_void)>;
    pub fn gpio_init(id: u8, tx: u8, rx: u8, rts: u8, cts: u8,
                     baudrate: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_gpio_init(id: u8, tx: u8, rx: u8, rts: u8, cts: u8,
                                     baudrate: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_gpio_init(id as u8, tx as u8, rx as u8, rts as u8,
                                  cts as u8, baudrate as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Initialise a UART Port. See `bl_uart_init` in \"Initialise UART Port\" <https://lupyuen.github.io/articles/uart#initialise-uart-port>"]
    pub fn init(id: u8, tx_pin: u8, rx_pin: u8, cts_pin: u8, rts_pin: u8,
                baudrate: u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_init(id: u8, tx_pin: u8, rx_pin: u8, cts_pin: u8,
                                rts_pin: u8, baudrate: u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_init(id as u8, tx_pin as u8, rx_pin as u8,
                             cts_pin as u8, rts_pin as u8, baudrate as u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn debug_early_init(baudrate: u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_debug_early_init(baudrate: u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_debug_early_init(baudrate as u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn early_init(id: u8, tx_pin: u8, baudrate: u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_early_init(id: u8, tx_pin: u8, baudrate: u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_early_init(id as u8, tx_pin as u8, baudrate as u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_rx_enable(id: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_rx_enable(id: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_int_rx_enable(id as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_rx_disable(id: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_rx_disable(id: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_int_rx_disable(id as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_tx_enable(id: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_tx_enable(id: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_int_tx_enable(id as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_tx_disable(id: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_tx_disable(id: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_int_tx_disable(id as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn string_send(id: u8, data: *mut ::cty::c_char) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_string_send(id: u8, data: *mut ::cty::c_char)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_string_send(id as u8, data as *mut ::cty::c_char);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn flush(id: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_flush(id: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_flush(id as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn getdefconfig(id: u8, parity: *mut u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_getdefconfig(id: u8, parity: *mut u8);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_uart_getdefconfig(id as u8, parity as *mut u8);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn setconfig(id: u8, baudrate: u32, parity: UART_Parity_Type)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_setconfig(id: u8, baudrate: u32,
                                     parity: UART_Parity_Type);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_uart_setconfig(id as u8, baudrate as u32,
                              parity as UART_Parity_Type);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn setbaud(id: u8, baud: u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_setbaud(id: u8, baud: u32);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            bl_uart_setbaud(id as u8, baud as u32);
            "----------Result----------";
            Ok(())
        }
    }
    #[doc =
      "Transmit data to a UART Port. See `bl_uart_data_send` in \"Transmit Data\" <https://lupyuen.github.io/articles/uart#transmit-data>"]
    pub fn data_send(id: u8, data: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_data_send(id: u8, data: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_data_send(id as u8, data as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn datas_send(id: u8, data: *mut u8, len: ::cty::c_int)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_datas_send(id: u8, data: *mut u8,
                                      len: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_datas_send(id as u8, data as *mut u8,
                                   len as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Receive data from a UART Port (non-blocking). See `bl_uart_data_recv` in \"Receive Data\" <https://lupyuen.github.io/articles/uart#receive-data>"]
    pub fn data_recv(id: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_data_recv(id: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_data_recv(id as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_enable(id: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_enable(id: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_int_enable(id as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_disable(id: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_disable(id: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = bl_uart_int_disable(id as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_rx_notify_register(id: u8, cb: cb_uart_notify_t, arg: Ptr)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_rx_notify_register(id: u8,
                                                  cb: cb_uart_notify_t,
                                                  arg: *mut ::cty::c_void)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_int_rx_notify_register(id as u8,
                                               cb as cb_uart_notify_t,
                                               arg as *mut ::cty::c_void);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_tx_notify_register(id: u8, cb: cb_uart_notify_t, arg: Ptr)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_tx_notify_register(id: u8,
                                                  cb: cb_uart_notify_t,
                                                  arg: *mut ::cty::c_void)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_int_tx_notify_register(id as u8,
                                               cb as cb_uart_notify_t,
                                               arg as *mut ::cty::c_void);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_rx_notify_unregister(id: u8, cb: cb_uart_notify_t, arg: Ptr)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_rx_notify_unregister(id: u8,
                                                    cb: cb_uart_notify_t,
                                                    arg: *mut ::cty::c_void)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_int_rx_notify_unregister(id as u8,
                                                 cb as cb_uart_notify_t,
                                                 arg as *mut ::cty::c_void);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn int_tx_notify_unregister(id: u8, cb: cb_uart_notify_t, arg: Ptr)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn bl_uart_int_tx_notify_unregister(id: u8,
                                                    cb: cb_uart_notify_t,
                                                    arg: *mut ::cty::c_void)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                bl_uart_int_tx_notify_unregister(id as u8,
                                                 cb as cb_uart_notify_t,
                                                 arg as *mut ::cty::c_void);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
}
/// WiFi HAL for BL602. See <https://lupyuen.github.io/articles/book#wifi-on-bl602>
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod wifi {
    use super::*;
    #[repr(C)]
    pub struct __BindgenBitfieldUnit<Storage> {
        storage: Storage,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::marker::Copy> ::core::marker::Copy for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::clone::Clone> ::core::clone::Clone for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn clone(&self) -> __BindgenBitfieldUnit<Storage> {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                __BindgenBitfieldUnit{storage:
                                          ::core::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::fmt::Debug> ::core::fmt::Debug for
     __BindgenBitfieldUnit<Storage> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                                                  "__BindgenBitfieldUnit");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                        "storage",
                                                        &&(*__self_0_0));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::default::Default> ::core::default::Default for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn default() -> __BindgenBitfieldUnit<Storage> {
            __BindgenBitfieldUnit{storage:
                                      ::core::default::Default::default(),}
        }
    }
    impl <Storage> ::core::marker::StructuralEq for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::Eq> ::core::cmp::Eq for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::core::cmp::AssertParamIsEq<Storage>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::hash::Hash> ::core::hash::Hash for
     __BindgenBitfieldUnit<Storage> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                __BindgenBitfieldUnit { storage: ref __self_0_0 } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::Ord> ::core::cmp::Ord for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn cmp(&self, other: &__BindgenBitfieldUnit<Storage>)
         -> ::core::cmp::Ordering {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    match ::core::cmp::Ord::cmp(&(*__self_0_0),
                                                &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal =>
                        ::core::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    impl <Storage> ::core::marker::StructuralPartialEq for
     __BindgenBitfieldUnit<Storage> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::PartialEq> ::core::cmp::PartialEq for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn eq(&self, other: &__BindgenBitfieldUnit<Storage>) -> bool {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &__BindgenBitfieldUnit<Storage>) -> bool {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Storage: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for
     __BindgenBitfieldUnit<Storage> {
        #[inline]
        fn partial_cmp(&self, other: &__BindgenBitfieldUnit<Storage>)
         -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                __BindgenBitfieldUnit { storage: ref __self_1_0 } =>
                match *self {
                    __BindgenBitfieldUnit { storage: ref __self_0_0 } =>
                    match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                               &(*__self_1_0))
                        {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        =>
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
    }
    impl <Storage> __BindgenBitfieldUnit<Storage> {
        #[inline]
        pub const fn new(storage: Storage) -> Self { Self{storage,} }
    }
    impl <Storage> __BindgenBitfieldUnit<Storage> where Storage: AsRef<[u8]> +
     AsMut<[u8]> {
        #[inline]
        pub fn get_bit(&self, index: usize) -> bool {
            if true {
                if !(index / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: index / 8 < self.storage.as_ref().len()")
                };
            };
            let byte_index = index / 8;
            let byte = self.storage.as_ref()[byte_index];
            let bit_index = if false { 7 - (index % 8) } else { index % 8 };
            let mask = 1 << bit_index;
            byte & mask == mask
        }
        #[inline]
        pub fn set_bit(&mut self, index: usize, val: bool) {
            if true {
                if !(index / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: index / 8 < self.storage.as_ref().len()")
                };
            };
            let byte_index = index / 8;
            let byte = &mut self.storage.as_mut()[byte_index];
            let bit_index = if false { 7 - (index % 8) } else { index % 8 };
            let mask = 1 << bit_index;
            if val { *byte |= mask; } else { *byte &= !mask; }
        }
        #[inline]
        pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
            if true {
                if !(bit_width <= 64) {
                    ::core::panicking::panic("assertion failed: bit_width <= 64")
                };
            };
            if true {
                if !(bit_offset / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: bit_offset / 8 < self.storage.as_ref().len()")
                };
            };
            if true {
                if !((bit_offset + (bit_width as usize)) / 8 <=
                         self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()")
                };
            };
            let mut val = 0;
            for i in 0..(bit_width as usize) {
                if self.get_bit(i + bit_offset) {
                    let index =
                        if false { bit_width as usize - 1 - i } else { i };
                    val |= 1 << index;
                }
            }
            val
        }
        #[inline]
        pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
            if true {
                if !(bit_width <= 64) {
                    ::core::panicking::panic("assertion failed: bit_width <= 64")
                };
            };
            if true {
                if !(bit_offset / 8 < self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: bit_offset / 8 < self.storage.as_ref().len()")
                };
            };
            if true {
                if !((bit_offset + (bit_width as usize)) / 8 <=
                         self.storage.as_ref().len()) {
                    ::core::panicking::panic("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()")
                };
            };
            for i in 0..(bit_width as usize) {
                let mask = 1 << i;
                let val_bit_is_set = val & mask == mask;
                let index =
                    if false { bit_width as usize - 1 - i } else { i };
                self.set_bit(index + bit_offset, val_bit_is_set);
            }
        }
    }
    #[repr(C)]
    pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>,
                                         [T; 0]);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::core::default::Default> ::core::default::Default for
     __IncompleteArrayField<T> {
        #[inline]
        fn default() -> __IncompleteArrayField<T> {
            __IncompleteArrayField(::core::default::Default::default(),
                                   ::core::default::Default::default())
        }
    }
    impl <T> __IncompleteArrayField<T> {
        #[inline]
        pub const fn new() -> Self {
            __IncompleteArrayField(::core::marker::PhantomData, [])
        }
        #[inline]
        pub fn as_ptr(&self) -> *const T { self as *const _ as *const T }
        #[inline]
        pub fn as_mut_ptr(&mut self) -> *mut T { self as *mut _ as *mut T }
        #[inline]
        pub unsafe fn as_slice(&self, len: usize) -> &[T] {
            ::core::slice::from_raw_parts(self.as_ptr(), len)
        }
        #[inline]
        pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
            ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
        }
    }
    impl <T> ::core::fmt::Debug for __IncompleteArrayField<T> {
        fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>)
         -> ::core::fmt::Result {
            fmt.write_str("__IncompleteArrayField")
        }
    }
    pub const WIFI_MGMR_SCAN_ITEMS_MAX: u32 = 50;
    pub const WIFI_MGMR_PROFILES_MAX: u32 = 2;
    pub const WIFI_MGMR_MQ_MSG_SIZE: u32 = 224;
    pub const WIFI_MGMR_MQ_MSG_COUNT: u32 = 10;
    pub const WIFI_MGMR_CONNECT_IND_STAT_INFO_TYPE_IND_CONNECTION: u32 = 1;
    pub const WIFI_MGMR_CONNECT_IND_STAT_INFO_TYPE_IND_DISCONNECTION: u32 = 2;
    pub const WIFI_MGMR_PENDING_TASK_SCAN_BIT: u32 = 1;
    pub const WIFI_MGMR_FEATURES_SCAN_SAVE_HIDDEN_SSID: u32 = 1;
    pub const WIFI_MGMR_CONFIG_SCAN_ITEM_TIMEOUT: u32 = 15000;
    pub type size_t = ::cty::c_ulong;
    pub type __int8_t = ::cty::c_schar;
    pub type __uint8_t = ::cty::c_uchar;
    pub type __uint16_t = ::cty::c_ushort;
    pub type __int32_t = ::cty::c_int;
    pub type __uint32_t = ::cty::c_uint;
    pub type TaskFunction_t =
     ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::cty::c_void)>;
    pub type UBaseType_t = u32;
    pub type TickType_t = u32;
    #[repr(C)]
    pub struct xSTATIC_LIST_ITEM {
        pub xDummy2: TickType_t,
        pub pvDummy3: [*mut ::cty::c_void; 4usize],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for xSTATIC_LIST_ITEM { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for xSTATIC_LIST_ITEM {
        #[inline]
        fn clone(&self) -> xSTATIC_LIST_ITEM {
            {
                let _: ::core::clone::AssertParamIsClone<TickType_t>;
                let _:
                        ::core::clone::AssertParamIsClone<[*mut ::cty::c_void; 4usize]>;
                *self
            }
        }
    }
    impl Default for xSTATIC_LIST_ITEM {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type StaticListItem_t = xSTATIC_LIST_ITEM;
    #[repr(C)]
    pub struct xSTATIC_TIMER {
        pub pvDummy1: *mut ::cty::c_void,
        pub xDummy2: StaticListItem_t,
        pub xDummy3: TickType_t,
        pub pvDummy5: *mut ::cty::c_void,
        pub pvDummy6: TaskFunction_t,
        pub uxDummy7: UBaseType_t,
        pub ucDummy8: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for xSTATIC_TIMER { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for xSTATIC_TIMER {
        #[inline]
        fn clone(&self) -> xSTATIC_TIMER {
            {
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _: ::core::clone::AssertParamIsClone<StaticListItem_t>;
                let _: ::core::clone::AssertParamIsClone<TickType_t>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _: ::core::clone::AssertParamIsClone<TaskFunction_t>;
                let _: ::core::clone::AssertParamIsClone<UBaseType_t>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    impl Default for xSTATIC_TIMER {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type StaticTimer_t = xSTATIC_TIMER;
    #[repr(C)]
    pub struct xSTATIC_STREAM_BUFFER {
        pub uxDummy1: [size_t; 4usize],
        pub pvDummy2: [*mut ::cty::c_void; 3usize],
        pub ucDummy3: u8,
        pub uxDummy4: UBaseType_t,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for xSTATIC_STREAM_BUFFER { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for xSTATIC_STREAM_BUFFER {
        #[inline]
        fn clone(&self) -> xSTATIC_STREAM_BUFFER {
            {
                let _: ::core::clone::AssertParamIsClone<[size_t; 4usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[*mut ::cty::c_void; 3usize]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<UBaseType_t>;
                *self
            }
        }
    }
    impl Default for xSTATIC_STREAM_BUFFER {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type StaticStreamBuffer_t = xSTATIC_STREAM_BUFFER;
    pub type StaticMessageBuffer_t = StaticStreamBuffer_t;
    #[doc =
      "Start the WiFi Firmware Task. See `hal_wifi_start_firmware_task` in \"Start WiFi Firmware Task\" <https://lupyuen.github.io/articles/wifi#start-wifi-firmware-task>"]
    pub fn start_firmware_task() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn hal_wifi_start_firmware_task()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = hal_wifi_start_firmware_task();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub type u8_t = u8;
    pub type s8_t = i8;
    pub type u16_t = u16;
    pub type u32_t = u32;
    pub type err_t = s8_t;
    #[doc = " This is the aligned version of ip4_addr_t,"]
    #[doc = "used as local variable, on the stack, etc."]
    #[repr(C)]
    pub struct ip4_addr {
        pub addr: u32_t,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for ip4_addr {
        #[inline]
        fn default() -> ip4_addr {
            ip4_addr{addr: ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ip4_addr { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ip4_addr {
        #[inline]
        fn clone(&self) -> ip4_addr {
            { let _: ::core::clone::AssertParamIsClone<u32_t>; *self }
        }
    }
    #[doc =
      " ip4_addr_t uses a struct for convenience only, so that the same defines can"]
    #[doc = " operate both on ip4_addr_t as well as on ip4_addr_p_t."]
    pub type ip4_addr_t = ip4_addr;
    pub type ip_addr_t = ip4_addr_t;
    #[doc = " Main packet buffer struct"]
    #[repr(C)]
    pub struct pbuf {
        #[doc = " next pbuf in singly linked pbuf chain"]
        pub next: *mut pbuf,
        #[doc = " pointer to the actual data in the buffer"]
        pub payload: *mut ::cty::c_void,
        #[doc = " total length of this buffer and all next buffers in chain"]
        #[doc = " belonging to the same packet."]
        #[doc = ""]
        #[doc = " For non-queue packet chains this is the invariant:"]
        #[doc = " p->tot_len == p->len + (p->next? p->next->tot_len: 0)"]
        pub tot_len: u16_t,
        #[doc = " length of this buffer"]
        pub len: u16_t,
        #[doc = " a bit field indicating pbuf type and allocation sources"]
        #[doc =
          "(see PBUF_TYPE_FLAG_*, PBUF_ALLOC_FLAG_* and PBUF_TYPE_ALLOC_SRC_MASK)"]
        pub type_internal: u8_t,
        #[doc = " misc flags"]
        pub flags: u8_t,
        #[doc = " the reference count always equals the number of pointers"]
        #[doc =
          " that refer to this pbuf. This can be pointers from an application,"]
        #[doc = " the stack itself, or pbuf->next pointers from a chain."]
        pub ref_: u8_t,
        #[doc =
          " For incoming packets, this contains the input netif's index"]
        pub if_idx: u8_t,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for pbuf { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for pbuf {
        #[inline]
        fn clone(&self) -> pbuf {
            {
                let _: ::core::clone::AssertParamIsClone<*mut pbuf>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _: ::core::clone::AssertParamIsClone<u16_t>;
                let _: ::core::clone::AssertParamIsClone<u16_t>;
                let _: ::core::clone::AssertParamIsClone<u8_t>;
                let _: ::core::clone::AssertParamIsClone<u8_t>;
                let _: ::core::clone::AssertParamIsClone<u8_t>;
                let _: ::core::clone::AssertParamIsClone<u8_t>;
                *self
            }
        }
    }
    impl Default for pbuf {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[doc = " Delete a filter entry"]
    pub const netif_mac_filter_action_NETIF_DEL_MAC_FILTER:
     netif_mac_filter_action =
        0;
    #[doc = " Add a filter entry"]
    pub const netif_mac_filter_action_NETIF_ADD_MAC_FILTER:
     netif_mac_filter_action =
        1;
    #[doc =
      " MAC Filter Actions, these are passed to a netif's igmp_mac_filter or"]
    #[doc = " mld_mac_filter callback function."]
    pub type netif_mac_filter_action = ::cty::c_uint;
    #[doc =
      " Function prototype for netif->input functions. This function is saved as 'input'"]
    #[doc =
      " callback function in the netif struct. Call it when a packet has been received."]
    #[doc = ""]
    #[doc = " @param p The received packet, copied into a pbuf"]
    #[doc = " @param inp The netif which received the packet"]
    #[doc = " Return: ERR_OK if the packet was handled"]
    #[doc =
      "         != ERR_OK is the packet was NOT handled, in this case, the caller has"]
    #[doc = "                   to free the pbuf"]
    pub type netif_input_fn =
     ::core::option::Option<unsafe extern "C" fn(p: *mut pbuf,
                                                 inp: *mut netif) -> err_t>;
    #[doc =
      " Function prototype for netif->output functions. Called by lwIP when a packet"]
    #[doc =
      " shall be sent. For ethernet netif, set this to 'etharp_output' and set"]
    #[doc = " 'linkoutput'."]
    #[doc = ""]
    #[doc = " @param netif The netif which shall send a packet"]
    #[doc = " @param p The packet to send (p->payload points to IP header)"]
    #[doc = " @param ipaddr The IP address to which the packet shall be sent"]
    pub type netif_output_fn =
     ::core::option::Option<unsafe extern "C" fn(netif: *mut netif,
                                                 p: *mut pbuf,
                                                 ipaddr: *const ip4_addr_t)
                                -> err_t>;
    #[doc =
      " Function prototype for netif->linkoutput functions. Only used for ethernet"]
    #[doc =
      " netifs. This function is called by ARP when a packet shall be sent."]
    #[doc = ""]
    #[doc = " @param netif The netif which shall send a packet"]
    #[doc = " @param p The packet to send (raw ethernet packet)"]
    pub type netif_linkoutput_fn =
     ::core::option::Option<unsafe extern "C" fn(netif: *mut netif,
                                                 p: *mut pbuf) -> err_t>;
    #[doc =
      " Function prototype for netif status- or link-callback functions."]
    pub type netif_status_callback_fn =
     ::core::option::Option<unsafe extern "C" fn(netif: *mut netif)>;
    #[doc = " Function prototype for netif igmp_mac_filter functions"]
    pub type netif_igmp_mac_filter_fn =
     ::core::option::Option<unsafe extern "C" fn(netif: *mut netif,
                                                 group: *const ip4_addr_t,
                                                 action:
                                                     netif_mac_filter_action)
                                -> err_t>;
    #[doc = " Generic data structure used for all lwIP network interfaces."]
    #[doc =
      "  The following fields should be filled in by the initialization"]
    #[doc =
      "  function for the device driver: hwaddr_len, hwaddr[], mtu, flags"]
    #[repr(C)]
    pub struct netif {
        #[doc = " pointer to next in linked list"]
        pub next: *mut netif,
        #[doc = " IP address configuration in network byte order"]
        pub ip_addr: ip_addr_t,
        pub netmask: ip_addr_t,
        pub gw: ip_addr_t,
        #[doc = " This function is called by the network device driver"]
        #[doc = "  to pass a packet up the TCP/IP stack."]
        pub input: netif_input_fn,
        #[doc = " This function is called by the IP module when it wants"]
        #[doc =
          "  to send a packet on the interface. This function typically"]
        #[doc =
          "  first resolves the hardware address, then sends the packet."]
        #[doc =
          "  For ethernet physical layer, this is usually etharp_output()"]
        pub output: netif_output_fn,
        #[doc = " This function is called by ethernet_output() when it wants"]
        #[doc = "  to send a packet on the interface. This function outputs"]
        #[doc = "  the pbuf as-is on the link medium."]
        pub linkoutput: netif_linkoutput_fn,
        #[doc =
          " This function is called when the netif state is set to up or down"]
        pub status_callback: netif_status_callback_fn,
        #[doc =
          " This function is called when the netif link is set to up or down"]
        pub link_callback: netif_status_callback_fn,
        #[doc = " This field can be set by the device driver and could point"]
        #[doc = "  to state information for the device."]
        pub state: *mut ::cty::c_void,
        pub client_data: [*mut ::cty::c_void; 3usize],
        pub hostname: *const ::cty::c_char,
        #[doc = " maximum transfer unit (in bytes)"]
        pub mtu: u16_t,
        #[doc = " link level hardware address of this interface"]
        pub hwaddr: [u8_t; 6usize],
        #[doc = " number of bytes used in hwaddr"]
        pub hwaddr_len: u8_t,
        #[doc = " flags (@see @ref netif_flags)"]
        pub flags: u8_t,
        #[doc = " descriptive abbreviation"]
        pub name: [::cty::c_char; 2usize],
        #[doc =
          " number of this interface. Used for @ref if_api and @ref netifapi_netif,"]
        #[doc = " as well as for IPv6 zones"]
        pub num: u8_t,
        #[doc =
          " Number of Router Solicitation messages that remain to be sent."]
        pub rs_count: u8_t,
        #[doc =
          " This function could be called to add or delete an entry in the multicast"]
        #[doc = "filter table of the ethernet MAC."]
        pub igmp_mac_filter: netif_igmp_mac_filter_fn,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for netif { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for netif {
        #[inline]
        fn clone(&self) -> netif {
            {
                let _: ::core::clone::AssertParamIsClone<*mut netif>;
                let _: ::core::clone::AssertParamIsClone<ip_addr_t>;
                let _: ::core::clone::AssertParamIsClone<ip_addr_t>;
                let _: ::core::clone::AssertParamIsClone<ip_addr_t>;
                let _: ::core::clone::AssertParamIsClone<netif_input_fn>;
                let _: ::core::clone::AssertParamIsClone<netif_output_fn>;
                let _: ::core::clone::AssertParamIsClone<netif_linkoutput_fn>;
                let _:
                        ::core::clone::AssertParamIsClone<netif_status_callback_fn>;
                let _:
                        ::core::clone::AssertParamIsClone<netif_status_callback_fn>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _:
                        ::core::clone::AssertParamIsClone<[*mut ::cty::c_void; 3usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<*const ::cty::c_char>;
                let _: ::core::clone::AssertParamIsClone<u16_t>;
                let _: ::core::clone::AssertParamIsClone<[u8_t; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u8_t>;
                let _: ::core::clone::AssertParamIsClone<u8_t>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 2usize]>;
                let _: ::core::clone::AssertParamIsClone<u8_t>;
                let _: ::core::clone::AssertParamIsClone<u8_t>;
                let _:
                        ::core::clone::AssertParamIsClone<netif_igmp_mac_filter_fn>;
                *self
            }
        }
    }
    impl Default for netif {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub struct wifi_mgmr_ap_item {
        pub ssid: [::cty::c_char; 32usize],
        pub ssid_tail: [::cty::c_char; 1usize],
        pub ssid_len: u32,
        pub bssid: [u8; 6usize],
        pub channel: u8,
        pub auth: u8,
        pub rssi: i8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wifi_mgmr_ap_item {
        #[inline]
        fn default() -> wifi_mgmr_ap_item {
            wifi_mgmr_ap_item{ssid: ::core::default::Default::default(),
                              ssid_tail: ::core::default::Default::default(),
                              ssid_len: ::core::default::Default::default(),
                              bssid: ::core::default::Default::default(),
                              channel: ::core::default::Default::default(),
                              auth: ::core::default::Default::default(),
                              rssi: ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_ap_item { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_ap_item {
        #[inline]
        fn clone(&self) -> wifi_mgmr_ap_item {
            {
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 32usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 1usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<i8>;
                *self
            }
        }
    }
    pub type wifi_mgmr_ap_item_t = wifi_mgmr_ap_item;
    #[repr(C)]
    pub struct wifi_mgmr_sta_connect_ind_stat_info {
        pub status_code: u16,
        pub type_ind: u8,
        pub ssid: [::cty::c_char; 32usize],
        pub psk: [::cty::c_char; 65usize],
        pub pmk: [::cty::c_char; 64usize],
        pub bssid: [u8; 6usize],
        pub chan_freq: u16,
        pub chan_band: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_sta_connect_ind_stat_info { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_sta_connect_ind_stat_info {
        #[inline]
        fn clone(&self) -> wifi_mgmr_sta_connect_ind_stat_info {
            {
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 32usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 65usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 64usize]>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    impl Default for wifi_mgmr_sta_connect_ind_stat_info {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type wifi_mgmr_sta_connect_ind_stat_info_t =
     wifi_mgmr_sta_connect_ind_stat_info;
    #[repr(C)]
    pub struct wifi_sta_basic_info {
        pub sta_idx: u8,
        pub is_used: u8,
        pub sta_mac: [u8; 6usize],
        pub tsfhi: u32,
        pub tsflo: u32,
        pub rssi: ::cty::c_int,
        pub data_rate: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wifi_sta_basic_info {
        #[inline]
        fn default() -> wifi_sta_basic_info {
            wifi_sta_basic_info{sta_idx: ::core::default::Default::default(),
                                is_used: ::core::default::Default::default(),
                                sta_mac: ::core::default::Default::default(),
                                tsfhi: ::core::default::Default::default(),
                                tsflo: ::core::default::Default::default(),
                                rssi: ::core::default::Default::default(),
                                data_rate:
                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_sta_basic_info { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_sta_basic_info {
        #[inline]
        fn clone(&self) -> wifi_sta_basic_info {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    pub type wifi_sta_basic_info_t = wifi_sta_basic_info;
    pub type wifi_interface_t = *mut ::cty::c_void;
    pub type sniffer_cb_t =
     ::core::option::Option<unsafe extern "C" fn(env: *mut ::cty::c_void,
                                                 pkt: *mut u8,
                                                 len: ::cty::c_int)>;
    pub type scan_item_cb_t =
     ::core::option::Option<unsafe extern "C" fn(env:
                                                     *mut wifi_mgmr_ap_item_t,
                                                 param1: *mut u32,
                                                 item:
                                                     *mut wifi_mgmr_ap_item_t)>;
    pub type scan_complete_cb_t =
     ::core::option::Option<unsafe extern "C" fn(data: *mut ::cty::c_void,
                                                 param: *mut ::cty::c_void)>;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_UNKNOWN: WIFI_STATE_ENUM_LIST =
        0;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_IDLE: WIFI_STATE_ENUM_LIST = 1;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_CONNECTING: WIFI_STATE_ENUM_LIST
     =
        2;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_CONNECTED_IP_GETTING:
     WIFI_STATE_ENUM_LIST =
        3;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_CONNECTED_IP_GOT:
     WIFI_STATE_ENUM_LIST =
        4;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_DISCONNECT: WIFI_STATE_ENUM_LIST
     =
        5;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_WITH_AP_IDLE:
     WIFI_STATE_ENUM_LIST =
        17;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_WITH_AP_CONNECTING:
     WIFI_STATE_ENUM_LIST =
        18;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_WITH_AP_CONNECTED_IP_GETTING:
     WIFI_STATE_ENUM_LIST =
        19;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_WITH_AP_CONNECTED_IP_GOT:
     WIFI_STATE_ENUM_LIST =
        20;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_WITH_AP_DISCONNECT:
     WIFI_STATE_ENUM_LIST =
        21;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_IFDOWN: WIFI_STATE_ENUM_LIST =
        6;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_SNIFFER: WIFI_STATE_ENUM_LIST =
        7;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_PSK_ERROR: WIFI_STATE_ENUM_LIST
     =
        8;
    pub const WIFI_STATE_ENUM_LIST_WIFI_STATE_NO_AP_FOUND:
     WIFI_STATE_ENUM_LIST =
        9;
    pub type WIFI_STATE_ENUM_LIST = ::cty::c_uint;
    pub const WIFI_SCAN_DONE_EVENT_TYPE_WIFI_SCAN_DONE_EVENT_OK:
     WIFI_SCAN_DONE_EVENT_TYPE =
        0;
    pub const WIFI_SCAN_DONE_EVENT_TYPE_WIFI_SCAN_DONE_EVENT_BUSY:
     WIFI_SCAN_DONE_EVENT_TYPE =
        1;
    pub type WIFI_SCAN_DONE_EVENT_TYPE = ::cty::c_uint;
    #[repr(C)]
    pub struct wifi_conf {
        pub country_code: [::cty::c_char; 3usize],
        pub channel_nums: ::cty::c_int,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wifi_conf {
        #[inline]
        fn default() -> wifi_conf {
            wifi_conf{country_code: ::core::default::Default::default(),
                      channel_nums: ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_conf { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_conf {
        #[inline]
        fn clone(&self) -> wifi_conf {
            {
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 3usize]>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                *self
            }
        }
    }
    pub type wifi_conf_t = wifi_conf;
    pub fn mgmr_psk_cal(password: *mut ::cty::c_char,
                        ssid: *mut ::cty::c_char, ssid_len: ::cty::c_int,
                        output: *mut ::cty::c_char) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_psk_cal(password: *mut ::cty::c_char,
                                     ssid: *mut ::cty::c_char,
                                     ssid_len: ::cty::c_int,
                                     output: *mut ::cty::c_char)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_psk_cal(password as *mut ::cty::c_char,
                                  ssid as *mut ::cty::c_char,
                                  ssid_len as ::cty::c_int,
                                  output as *mut ::cty::c_char);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_drv_init(conf: *mut wifi_conf_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_drv_init(conf: *mut wifi_conf_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_drv_init(conf as *mut wifi_conf_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_init() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_init()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_init();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_start() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_start();
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            wifi_mgmr_start();
            "----------Result----------";
            Ok(())
        }
    }
    #[doc =
      "Start the WiFi Manager Task. See `wifi_mgmr_start_background` in \"Start WiFi Manager Task\" <https://lupyuen.github.io/articles/wifi#start-wifi-manager-task>"]
    pub fn mgmr_start_background(conf: *mut wifi_conf_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_start_background(conf: *mut wifi_conf_t);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            wifi_mgmr_start_background(conf as *mut wifi_conf_t);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn mgmr_get_wifi_channel_conf(wifi_chan_conf: *mut wifi_conf_t)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_get_wifi_channel_conf(wifi_chan_conf:
                                                       *mut wifi_conf_t);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            wifi_mgmr_get_wifi_channel_conf(wifi_chan_conf as
                                                *mut wifi_conf_t);
            "----------Result----------";
            Ok(())
        }
    }
    #[doc =
      "Enable the WiFi Client. See `wifi_mgmr_sta_enable` in \"Connect to WiFi Network\" <https://lupyuen.github.io/articles/wifi#connect-to-wifi-network>"]
    pub fn mgmr_sta_enable() -> BlResult<wifi_interface_t> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_enable()
            -> wifi_interface_t;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sta_enable();
            "----------Result----------";
            Ok(res)
        }
    }
    pub fn mgmr_sta_disable(interface: *mut wifi_interface_t)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_disable(interface: *mut wifi_interface_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_sta_disable(interface as *mut wifi_interface_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_mac_set(mac: *mut u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_mac_set(mac: *mut u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sta_mac_set(mac as *mut u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_mac_get(mac: *mut u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_mac_get(mac: *mut u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sta_mac_get(mac as *mut u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_ip_get(ip: *mut u32, gw: *mut u32, mask: *mut u32)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_ip_get(ip: *mut u32, gw: *mut u32,
                                        mask: *mut u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_sta_ip_get(ip as *mut u32, gw as *mut u32,
                                     mask as *mut u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_ip_set(ip: u32, mask: u32, gw: u32, dns1: u32, dns2: u32)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_ip_set(ip: u32, mask: u32, gw: u32,
                                        dns1: u32, dns2: u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_sta_ip_set(ip as u32, mask as u32, gw as u32,
                                     dns1 as u32, dns2 as u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_dns_get(dns1: *mut u32, dns2: *mut u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_dns_get(dns1: *mut u32, dns2: *mut u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_sta_dns_get(dns1 as *mut u32, dns2 as *mut u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_ip_unset() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_ip_unset()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sta_ip_unset();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    #[doc =
      "Connect to a WiFi Access Point. See `wifi_mgmr_sta_connect` in \"Connect to WiFi Network\" <https://lupyuen.github.io/articles/wifi#connect-to-wifi-network>"]
    pub fn mgmr_sta_connect(wifi_interface: *mut wifi_interface_t,
                            ssid: *mut ::cty::c_char, psk: *mut ::cty::c_char,
                            pmk: *mut ::cty::c_char, mac: *mut u8, band: u8,
                            freq: u16) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_connect(wifi_interface:
                                             *mut wifi_interface_t,
                                         ssid: *mut ::cty::c_char,
                                         psk: *mut ::cty::c_char,
                                         pmk: *mut ::cty::c_char,
                                         mac: *mut u8, band: u8, freq: u16)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_sta_connect(wifi_interface as *mut wifi_interface_t,
                                      ssid as *mut ::cty::c_char,
                                      psk as *mut ::cty::c_char,
                                      pmk as *mut ::cty::c_char,
                                      mac as *mut u8, band as u8,
                                      freq as u16);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_disconnect() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_disconnect()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sta_disconnect();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_powersaving(ps: ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_powersaving(ps: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sta_powersaving(ps as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_autoconnect_enable() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_autoconnect_enable()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sta_autoconnect_enable();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_autoconnect_disable() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_autoconnect_disable()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sta_autoconnect_disable();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sta_ssid_set(ssid: *mut ::cty::c_char) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_ssid_set(ssid: *mut ::cty::c_char);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            wifi_mgmr_sta_ssid_set(ssid as *mut ::cty::c_char);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn mgmr_sta_psk_set(psk: *mut ::cty::c_char) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_psk_set(psk: *mut ::cty::c_char);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            wifi_mgmr_sta_psk_set(psk as *mut ::cty::c_char);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn mgmr_sta_connect_ind_stat_get(wifi_mgmr_ind_stat:
                                             *mut wifi_mgmr_sta_connect_ind_stat_info_t)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sta_connect_ind_stat_get(wifi_mgmr_ind_stat:
                                                          *mut wifi_mgmr_sta_connect_ind_stat_info_t);
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            wifi_mgmr_sta_connect_ind_stat_get(wifi_mgmr_ind_stat as
                                                   *mut wifi_mgmr_sta_connect_ind_stat_info_t);
            "----------Result----------";
            Ok(())
        }
    }
    pub fn mgmr_ap_enable() -> BlResult<wifi_interface_t> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_enable()
            -> wifi_interface_t;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_enable();
            "----------Result----------";
            Ok(res)
        }
    }
    pub fn mgmr_ap_mac_set(mac: *mut u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_mac_set(mac: *mut u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_mac_set(mac as *mut u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_mac_get(mac: *mut u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_mac_get(mac: *mut u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_mac_get(mac as *mut u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_ip_get(ip: *mut u32, gw: *mut u32, mask: *mut u32)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_ip_get(ip: *mut u32, gw: *mut u32,
                                       mask: *mut u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_ap_ip_get(ip as *mut u32, gw as *mut u32,
                                    mask as *mut u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_stop(interface: *mut wifi_interface_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_stop(interface: *mut wifi_interface_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_stop(interface as *mut wifi_interface_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_start(interface: *mut wifi_interface_t,
                         ssid: *mut ::cty::c_char, hidden_ssid: ::cty::c_int,
                         passwd: *mut ::cty::c_char, channel: ::cty::c_int)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_start(interface: *mut wifi_interface_t,
                                      ssid: *mut ::cty::c_char,
                                      hidden_ssid: ::cty::c_int,
                                      passwd: *mut ::cty::c_char,
                                      channel: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_ap_start(interface as *mut wifi_interface_t,
                                   ssid as *mut ::cty::c_char,
                                   hidden_ssid as ::cty::c_int,
                                   passwd as *mut ::cty::c_char,
                                   channel as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_sta_cnt_get(sta_cnt: *mut u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_sta_cnt_get(sta_cnt: *mut u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_sta_cnt_get(sta_cnt as *mut u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_sta_info_get(sta_info: *mut wifi_sta_basic_info, idx: u8)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_sta_info_get(sta_info:
                                                 *mut wifi_sta_basic_info,
                                             idx: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_ap_sta_info_get(sta_info as
                                              *mut wifi_sta_basic_info,
                                          idx as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_sta_delete(sta_idx: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_sta_delete(sta_idx: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_sta_delete(sta_idx as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_set_gateway(gateway: *mut ::cty::c_char) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_set_gateway(gateway: *mut ::cty::c_char)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_set_gateway(gateway as *mut ::cty::c_char);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sniffer_enable() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sniffer_enable()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sniffer_enable();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sniffer_disable() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sniffer_disable()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sniffer_disable();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_rate_config(config: u16) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_rate_config(config: u16)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_rate_config(config as u16);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_conf_max_sta(max_sta_supported: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_conf_max_sta(max_sta_supported: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_conf_max_sta(max_sta_supported as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sniffer_register(env: Ptr, cb: sniffer_cb_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sniffer_register(env: *mut ::cty::c_void,
                                              cb: sniffer_cb_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_sniffer_register(env as *mut ::cty::c_void,
                                           cb as sniffer_cb_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_sniffer_unregister(env: Ptr) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_sniffer_unregister(env: *mut ::cty::c_void)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_sniffer_unregister(env as *mut ::cty::c_void);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_state_get(state: *mut ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_state_get(state: *mut ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_state_get(state as *mut ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_status_code_get(s_code: *mut ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_status_code_get(s_code: *mut ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_status_code_get(s_code as *mut ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_rssi_get(rssi: *mut ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_rssi_get(rssi: *mut ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_rssi_get(rssi as *mut ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_channel_get(channel: *mut ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_channel_get(channel: *mut ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_channel_get(channel as *mut ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_channel_set(channel: ::cty::c_int, use_40Mhz: ::cty::c_int)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_channel_set(channel: ::cty::c_int,
                                         use_40Mhz: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_channel_set(channel as ::cty::c_int,
                                      use_40Mhz as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_all_ap_scan(ap_ary: *mut *mut wifi_mgmr_ap_item_t,
                            num: *mut u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_all_ap_scan(ap_ary:
                                             *mut *mut wifi_mgmr_ap_item_t,
                                         num: *mut u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_all_ap_scan(ap_ary as *mut *mut wifi_mgmr_ap_item_t,
                                      num as *mut u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_scan_filter_hidden_ssid(filter: ::cty::c_int)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_scan_filter_hidden_ssid(filter: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_scan_filter_hidden_ssid(filter as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_scan(data: Ptr, cb: scan_complete_cb_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_scan(data: *mut ::cty::c_void,
                                  cb: scan_complete_cb_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_scan(data as *mut ::cty::c_void,
                               cb as scan_complete_cb_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_cfg_req(ops: u32, task: u32, element: u32, type_: u32,
                        length: u32, buf: *mut u32) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_cfg_req(ops: u32, task: u32, element: u32,
                                     type_: u32, length: u32, buf: *mut u32)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_cfg_req(ops as u32, task as u32, element as u32,
                                  type_ as u32, length as u32,
                                  buf as *mut u32);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_scan_complete_callback() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_scan_complete_callback()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_scan_complete_callback();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_cli_scanlist() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_cli_scanlist()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_cli_scanlist();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_cli_init() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_cli_init()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_cli_init();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_scan_ap(ssid: *mut ::cty::c_char,
                        item: *mut wifi_mgmr_ap_item_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_scan_ap(ssid: *mut ::cty::c_char,
                                     item: *mut wifi_mgmr_ap_item_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_scan_ap(ssid as *mut ::cty::c_char,
                                  item as *mut wifi_mgmr_ap_item_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_scan_ap_all(env: *mut wifi_mgmr_ap_item_t, param1: *mut u32,
                            cb: scan_item_cb_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_scan_ap_all(env: *mut wifi_mgmr_ap_item_t,
                                         param1: *mut u32, cb: scan_item_cb_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_scan_ap_all(env as *mut wifi_mgmr_ap_item_t,
                                      param1 as *mut u32,
                                      cb as scan_item_cb_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_raw_80211_send(pkt: *mut u8, len: ::cty::c_int)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_raw_80211_send(pkt: *mut u8, len: ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_raw_80211_send(pkt as *mut u8, len as ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_set_country_code(country_code: *mut ::cty::c_char)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_set_country_code(country_code:
                                                  *mut ::cty::c_char)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_set_country_code(country_code as
                                               *mut ::cty::c_char);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ext_dump_needed() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ext_dump_needed()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ext_dump_needed();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_status_code_str(status_code: u16) -> BlResult<Strn<'static>> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_status_code_str(status_code: u16)
            -> *const ::cty::c_char;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_status_code_str(status_code as u16);
            "----------Result----------";
            Ok(Strn::from_cstr(res as *const u8))
        }
    }
    #[doc = " \\brief Event"]
    #[doc = ""]
    #[doc =
      " Events trigger transitions from a state to another. Event types are defined"]
    #[doc =
      " by the user. Any event may optionally contain a \\ref #event::data"]
    #[doc = " \"payload\"."]
    #[doc = ""]
    #[doc = " \\sa state"]
    #[doc = " \\sa transition"]
    #[repr(C)]
    pub struct event {
        #[doc = " \\brief Type of event. Defined by user."]
        pub type_: ::cty::c_int,
        #[doc = " \\brief Event payload."]
        #[doc = ""]
        #[doc = " How this is used is entirely up to the user. This data"]
        #[doc =
          " is always passed together with #type in order to make it possible to"]
        #[doc = " always cast the data correctly."]
        pub data: *mut ::cty::c_void,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for event { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for event {
        #[inline]
        fn clone(&self) -> event {
            {
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                *self
            }
        }
    }
    impl Default for event {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[doc = " \\brief Transition between a state and another state"]
    #[doc = ""]
    #[doc =
      " All states that are not final must have at least one transition. The"]
    #[doc =
      " transition may be guarded or not. Transitions are triggered by events. If"]
    #[doc =
      " a state has more than one transition with the same type of event (and the"]
    #[doc =
      " same condition), the first transition in the array will be run. An"]
    #[doc =
      " unconditional transition placed last in the transition array of a state can"]
    #[doc =
      " act as a \"catch-all\". A transition may optionally run an #action, which"]
    #[doc =
      " will have the triggering event passed to it as an argument, along with the"]
    #[doc = " current and new states' \\ref state::data \"data\"."]
    #[doc = ""]
    #[doc =
      " It is perfectly valid for a transition to return to the state it belongs"]
    #[doc =
      " to. Such a transition will not call the state's \\ref state::entryAction"]
    #[doc =
      " \"entry action\" or \\ref state::exitAction \"exit action\". If there are no"]
    #[doc =
      " transitions for the current event, the state's parent will be handed the"]
    #[doc = " event."]
    #[doc = ""]
    #[doc = " ### Examples ###"]
    #[doc = " - An ungarded transition to a state with no action performed:"]
    #[doc = " ~~~{.c}"]
    #[doc = " {"]
    #[doc = "    .eventType = Event_timeout,"]
    #[doc = "    .condition = NULL,"]
    #[doc = "    .guard = NULL,"]
    #[doc = "    .action = NULL,"]
    #[doc = "    .nextState = &mainMenuState,"]
    #[doc = " },"]
    #[doc = " ~~~"]
    #[doc = " - A guarded transition executing an action"]
    #[doc = " ~~~{.c}"]
    #[doc = " {"]
    #[doc = "    .eventType = Event_keyboard,"]
    #[doc = "    .condition = NULL,"]
    #[doc = "    .guard = &ensureNumericInput,"]
    #[doc = "    .action = &addToBuffer,"]
    #[doc = "    .nextState = &awaitingInputState,"]
    #[doc = " },"]
    #[doc = " ~~~"]
    #[doc = " - A guarded transition using a condition"]
    #[doc = " ~~~{.c}"]
    #[doc = " {"]
    #[doc = "    .eventType = Event_mouse,"]
    #[doc = "    .condition = boxLimits,"]
    #[doc = "    .guard = &coordinatesWithinLimits,"]
    #[doc = " },"]
    #[doc = " ~~~"]
    #[doc =
      " By using \\ref #condition \"conditions\" a more general guard function can be"]
    #[doc =
      " used, operating on the supplied argument #condition. In this example,"]
    #[doc =
      " `coordinatesWithinLimits` checks whether the coordinates in the mouse event"]
    #[doc = " are within the limits of the \"box\"."]
    #[doc = ""]
    #[doc = " \\sa event"]
    #[doc = " \\sa state"]
    #[repr(C)]
    pub struct transition {
        #[doc = " \\brief The event that will trigger this transition."]
        pub eventType: ::cty::c_int,
        #[doc = " \\brief Condition that event must fulfil"]
        #[doc = ""]
        #[doc =
          " This variable will be passed to the #guard (if #guard is non-NULL) and"]
        #[doc =
          " may be used as a condition that the incoming event's data must fulfil in"]
        #[doc =
          " order for the transition to be performed. By using this variable, the"]
        #[doc =
          " number of #guard functions can be minimised by making them more general."]
        pub condition: *mut ::cty::c_void,
        #[doc =
          " \\brief Check if data passed with event fulfils a condition"]
        #[doc = ""]
        #[doc =
          " A transition may be conditional. If so, this function, if non-NULL, will"]
        #[doc =
          " be called. Its first argument will be supplied with #condition, which"]
        #[doc =
          " can be compared against the \\ref event::data \"payload\" in the #event."]
        #[doc =
          " The user may choose to use this argument or not. Only if the result is"]
        #[doc = " true, the transition will take place."]
        #[doc = ""]
        #[doc =
          " \\param condition event (data) to compare the incoming event against."]
        #[doc = " \\param event the event passed to the state machine."]
        #[doc = ""]
        #[doc =
          " \\returns true if the event's data fulfils the condition, otherwise false."]
        pub guard: ::core::option::Option<unsafe extern "C" fn(condition:
                                                                   *mut ::cty::c_void,
                                                               event:
                                                                   *mut event)
                                              -> bool>,
        #[doc =
          " \\brief Function containing tasks to be performed during the transition"]
        #[doc = ""]
        #[doc =
          " The transition may optionally do some work in this function before"]
        #[doc = " entering the next state. May be NULL."]
        #[doc = ""]
        #[doc =
          " \\param currentStateData the leaving state's \\ref state::data \"data\""]
        #[doc = " \\param event the event passed to the state machine."]
        #[doc =
          " \\param newStateData the new state's (the \\ref state::entryState"]
        #[doc =
          " \"entryState\" of any (chain of) parent states, not the parent state"]
        #[doc = " itself) \\ref state::data \"data\""]
        pub action: ::core::option::Option<unsafe extern "C" fn(currentStateData:
                                                                    *mut ::cty::c_void,
                                                                event:
                                                                    *mut event,
                                                                newStateData:
                                                                    *mut ::cty::c_void)>,
        #[doc = " \\brief The next state"]
        #[doc = ""]
        #[doc =
          " This must point to the next state that will be entered. It cannot be"]
        #[doc =
          " NULL. If it is, the state machine will detect it and enter the \\ref"]
        #[doc = " stateMachine::errorState \"error state\"."]
        pub nextState: *const state,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for transition { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for transition {
        #[inline]
        fn clone(&self) -> transition {
            {
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _:
                        ::core::clone::AssertParamIsClone<::core::option::Option<unsafe extern "C" fn(condition:
                                                                                                          *mut ::cty::c_void,
                                                                                                      event:
                                                                                                          *mut event)
                                                                                     ->
                                                                                         bool>>;
                let _:
                        ::core::clone::AssertParamIsClone<::core::option::Option<unsafe extern "C" fn(currentStateData:
                                                                                                          *mut ::cty::c_void,
                                                                                                      event:
                                                                                                          *mut event,
                                                                                                      newStateData:
                                                                                                          *mut ::cty::c_void)>>;
                let _: ::core::clone::AssertParamIsClone<*const state>;
                *self
            }
        }
    }
    impl Default for transition {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[doc = " \\brief State"]
    #[doc = ""]
    #[doc =
      " The current state in a state machine moves to a new state when one of the"]
    #[doc =
      " #transitions in the current state triggers on an event. An optional \\ref"]
    #[doc =
      " #exitAction \"exit action\" is called when the state is left, and an \\ref"]
    #[doc =
      " #entryAction \"entry action\" is called when the state machine enters a new"]
    #[doc =
      " state. If a state returns to itself, neither #exitAction nor #entryAction"]
    #[doc =
      " will be called. An optional \\ref transition::action \"transition action\" is"]
    #[doc = " called in either case."]
    #[doc = ""]
    #[doc =
      " States may be organised in a hierarchy by setting \\ref #parentState"]
    #[doc =
      " \"parent states\". When a group/parent state is entered, the state machine is"]
    #[doc =
      " redirected to the group state's \\ref #entryState \"entry state\" (if"]
    #[doc =
      " non-NULL). If an event does not trigger a transition in a state and if the"]
    #[doc =
      " state has a parent state, the event will be passed to the parent state."]
    #[doc =
      " This behaviour is repeated for all parents. Thus all children of a state"]
    #[doc =
      " have a set of common #transitions. A parent state's #entryAction will not"]
    #[doc = " be called if an event is passed on to a child state."]
    #[doc = ""]
    #[doc =
      " The following lists the different types of states that may be created, and"]
    #[doc = " how to create them:"]
    #[doc = ""]
    #[doc = " ### Normal state ###"]
    #[doc = " ~~~{.c}"]
    #[doc = " struct state normalState = {"]
    #[doc = "    .parentState = &groupState,"]
    #[doc = "    .entryState = NULL,"]
    #[doc = "    .transition = (struct transition[]){"]
    #[doc =
      "       { Event_keyboard, (void *)(intptr_t)'\\n', &compareKeyboardChar,"]
    #[doc = "          NULL, &msgReceivedState },"]
    #[doc = "    },"]
    #[doc = "    .numTransitions = 1,"]
    #[doc = "    .data = normalStateData,"]
    #[doc = "    .entryAction = &doSomething,"]
    #[doc = "    .exitAction = &cleanUp,"]
    #[doc = " };"]
    #[doc = " ~~~"]
    #[doc =
      " In this example, `normalState` is a child of `groupState`, but the"]
    #[doc =
      " #parentState value may also be NULL to indicate that it is not a child of"]
    #[doc = " any group state."]
    #[doc = ""]
    #[doc = " ### Group/parent state ###"]
    #[doc =
      " A state becomes a group/parent state when it is linked to by child states"]
    #[doc =
      " by using #parentState. No members in the group state need to be set in a"]
    #[doc = " particular way. A parent state may also have a parent."]
    #[doc = " ~~~{.c}"]
    #[doc = " struct state groupState = {"]
    #[doc = "    .entryState = &normalState,"]
    #[doc = "    .entryAction = NULL,"]
    #[doc = " ~~~"]
    #[doc =
      " If there are any transitions in the state machine that lead to a group"]
    #[doc =
      " state, it makes sense to define an entry state in the group. This can be"]
    #[doc =
      " done by using #entryState, but it is not mandatory. If the #entryState"]
    #[doc =
      " state has children, the chain of children will be traversed until a child"]
    #[doc = " with its #entryState set to NULL is found."]
    #[doc = ""]
    #[doc =
      " \\note If #entryState is defined for a group state, the group state's"]
    #[doc =
      " #entryAction will not be called (the state pointed to by #entryState (after"]
    #[doc =
      " following the chain of children), however, will have its #entryAction"]
    #[doc = " called)."]
    #[doc = ""]
    #[doc =
      " \\warning The state machine cannot detect cycles in parent chains and"]
    #[doc =
      " children chains. If such cycles are present, stateM_handleEvent() will"]
    #[doc = " never finish due to never-ending loops."]
    #[doc = ""]
    #[doc = " ### Final state ###"]
    #[doc =
      " A final state is a state that terminates the state machine. A state is"]
    #[doc = " considered as a final state if its #numTransitions is 0:"]
    #[doc = " ~~~{.c}"]
    #[doc = " struct state finalState = {"]
    #[doc = "    .transitions = NULL,"]
    #[doc = "    .numTransitions = 0,"]
    #[doc = " ~~~"]
    #[doc =
      " The error state used by the state machine to indicate errors should be a"]
    #[doc =
      " final state. Any calls to stateM_handleEvent() when the current state is a"]
    #[doc = " final state will return #stateM_noStateChange."]
    #[doc = ""]
    #[doc = " \\sa event"]
    #[doc = " \\sa transition"]
    #[repr(C)]
    pub struct state {
        #[doc =
          " \\brief If the state has a parent state, this pointer must be non-NULL."]
        pub parentState: *const state,
        #[doc =
          " \\brief If this state is a parent state, this pointer may point to a"]
        #[doc = " child state that serves as an entry point."]
        pub entryState: *const state,
        #[doc = " \\brief An array of transitions for the state."]
        pub transitions: *mut transition,
        #[doc = " \\brief Number of transitions in the #transitions array."]
        pub numTransitions: size_t,
        #[doc =
          " \\brief Data that will be available for the state in its #entryAction and"]
        #[doc =
          " #exitAction, and in any \\ref transition::action \"transition action\""]
        pub data: *mut ::cty::c_void,
        #[doc =
          " \\brief This function is called whenever the state is being entered. May"]
        #[doc = " be NULL."]
        #[doc = ""]
        #[doc =
          " \\note If a state returns to itself through a transition (either directly"]
        #[doc =
          " or through a parent/group sate), its #entryAction will not be called."]
        #[doc = ""]
        #[doc =
          " \\note A group/parent state with its #entryState defined will not have"]
        #[doc = " its #entryAction called."]
        #[doc = ""]
        #[doc = " \\param stateData the state's #data will be passed."]
        #[doc =
          " \\param event the event that triggered the transition will be passed."]
        pub entryAction: ::core::option::Option<unsafe extern "C" fn(stateData:
                                                                         *mut ::cty::c_void,
                                                                     event:
                                                                         *mut event)>,
        #[doc =
          " \\brief This function is called whenever the state is being left. May be"]
        #[doc = " NULL."]
        #[doc = ""]
        #[doc =
          " \\note If a state returns to itself through a transition (either directly"]
        #[doc =
          " or through a parent/group sate), its #exitAction will not be called."]
        #[doc = ""]
        #[doc = " \\param stateData the state's #data will be passed."]
        #[doc =
          " \\param event the event that triggered a transition will be passed."]
        pub exitAction: ::core::option::Option<unsafe extern "C" fn(stateData:
                                                                        *mut ::cty::c_void,
                                                                    event:
                                                                        *mut event)>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for state { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for state {
        #[inline]
        fn clone(&self) -> state {
            {
                let _: ::core::clone::AssertParamIsClone<*const state>;
                let _: ::core::clone::AssertParamIsClone<*const state>;
                let _: ::core::clone::AssertParamIsClone<*mut transition>;
                let _: ::core::clone::AssertParamIsClone<size_t>;
                let _: ::core::clone::AssertParamIsClone<*mut ::cty::c_void>;
                let _:
                        ::core::clone::AssertParamIsClone<::core::option::Option<unsafe extern "C" fn(stateData:
                                                                                                          *mut ::cty::c_void,
                                                                                                      event:
                                                                                                          *mut event)>>;
                let _:
                        ::core::clone::AssertParamIsClone<::core::option::Option<unsafe extern "C" fn(stateData:
                                                                                                          *mut ::cty::c_void,
                                                                                                      event:
                                                                                                          *mut event)>>;
                *self
            }
        }
    }
    impl Default for state {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[doc = " \\brief State machine"]
    #[doc = ""]
    #[doc = " There is no need to manipulate the members directly."]
    #[repr(C)]
    pub struct stateMachine {
        #[doc = " \\brief Pointer to the current state"]
        pub currentState: *const state,
        #[doc = " \\brief Pointer to previous state"]
        #[doc = ""]
        #[doc =
          " The previous state is stored for convenience in case the user needs to"]
        #[doc = " keep track of previous states."]
        pub previousState: *const state,
        #[doc =
          " \\brief Pointer to a state that will be entered whenever an error occurs"]
        #[doc = " in the state machine."]
        #[doc = ""]
        #[doc =
          " See #stateM_errorStateReached for when the state machine enters the"]
        #[doc = " error state."]
        pub errorState: *const state,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for stateMachine { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for stateMachine {
        #[inline]
        fn clone(&self) -> stateMachine {
            {
                let _: ::core::clone::AssertParamIsClone<*const state>;
                let _: ::core::clone::AssertParamIsClone<*const state>;
                let _: ::core::clone::AssertParamIsClone<*const state>;
                *self
            }
        }
    }
    impl Default for stateMachine {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type os_messagequeue_t = StaticMessageBuffer_t;
    pub type os_timer_t = StaticTimer_t;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_IDLE: WIFI_MGMR_EVENT = 0;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_CONNECT: WIFI_MGMR_EVENT =
        1;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_SNIFFER: WIFI_MGMR_EVENT =
        2;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_CONNECTED: WIFI_MGMR_EVENT =
        3;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_IP_GOT: WIFI_MGMR_EVENT = 4;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_DISCONNECT: WIFI_MGMR_EVENT
     =
        5;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_RECONNECT: WIFI_MGMR_EVENT =
        6;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_PHY_UP: WIFI_MGMR_EVENT = 7;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_AP_START: WIFI_MGMR_EVENT =
        8;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_AP_STOP: WIFI_MGMR_EVENT =
        9;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_CONF_MAX_STA:
     WIFI_MGMR_EVENT =
        10;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_RC_CONFIG: WIFI_MGMR_EVENT =
        11;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_DENOISE: WIFI_MGMR_EVENT =
        12;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_APP_RELOAD_TSEN: WIFI_MGMR_EVENT
     =
        13;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_MAXAPP_MINIFW: WIFI_MGMR_EVENT =
        14;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_FW_DISCONNECT: WIFI_MGMR_EVENT =
        15;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_FW_POWERSAVING: WIFI_MGMR_EVENT
     =
        16;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_FW_CHANNEL_SET: WIFI_MGMR_EVENT
     =
        17;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_FW_SCAN: WIFI_MGMR_EVENT = 18;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_FW_IND_DISCONNECT:
     WIFI_MGMR_EVENT =
        19;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_FW_IND_CONNECTED:
     WIFI_MGMR_EVENT =
        20;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_FW_DATA_RAW_SEND:
     WIFI_MGMR_EVENT =
        21;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_FW_CFG_REQ: WIFI_MGMR_EVENT =
        22;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_MAXFW_MINI_GLOBAL:
     WIFI_MGMR_EVENT =
        23;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_GLB_SCAN_IND_BEACON:
     WIFI_MGMR_EVENT =
        24;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_GLB_SCAN_IND_PROBE_RESP:
     WIFI_MGMR_EVENT =
        25;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_GLB_AP_IND_STA_NEW:
     WIFI_MGMR_EVENT =
        26;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_GLB_AP_IND_STA_DEL:
     WIFI_MGMR_EVENT =
        27;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_GLB_DISABLE_AUTORECONNECT:
     WIFI_MGMR_EVENT =
        28;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_GLB_ENABLE_AUTORECONNECT:
     WIFI_MGMR_EVENT =
        29;
    pub const WIFI_MGMR_EVENT_WIFI_MGMR_EVENT_GLB_IP_UPDATE: WIFI_MGMR_EVENT =
        30;
    pub type WIFI_MGMR_EVENT = ::cty::c_uint;
    pub use self::WIFI_MGMR_EVENT as WIFI_MGMR_EVENT_T;
    pub const WIFI_MGMR_CONNECTION_STATUS_WIFI_MGMR_CONNECTION_STATUS_IDLE:
     WIFI_MGMR_CONNECTION_STATUS =
        0;
    pub const
     WIFI_MGMR_CONNECTION_STATUS_WIFI_MGMR_CONNECTION_STATUS_CONNECTING:
     WIFI_MGMR_CONNECTION_STATUS =
        1;
    pub const
     WIFI_MGMR_CONNECTION_STATUS_WIFI_MGMR_CONNECTION_STATUS_CONNECTED_IP_YES:
     WIFI_MGMR_CONNECTION_STATUS =
        2;
    pub const
     WIFI_MGMR_CONNECTION_STATUS_WIFI_MGMR_CONNECTION_STATUS_CONNECTED_IP_NO:
     WIFI_MGMR_CONNECTION_STATUS =
        3;
    pub const
     WIFI_MGMR_CONNECTION_STATUS_WIFI_MGMR_CONNECTION_STATUS_DISCONNECTED:
     WIFI_MGMR_CONNECTION_STATUS =
        4;
    pub type WIFI_MGMR_CONNECTION_STATUS = ::cty::c_uint;
    pub use self::WIFI_MGMR_CONNECTION_STATUS as
            WIFI_MGMR_CONNECTION_STATUS_T;
    #[repr(C, packed)]
    pub struct wifi_mgmr_msg {
        pub ev: WIFI_MGMR_EVENT_T,
        pub data1: *mut ::cty::c_void,
        pub data2: *mut ::cty::c_void,
        pub len: u32,
        pub data: __IncompleteArrayField<u8>,
    }
    impl Default for wifi_mgmr_msg {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type wifi_mgmr_msg_t = wifi_mgmr_msg;
    #[repr(C, packed)]
    pub struct wifi_mgmr_cfg_element_msg {
        pub ops: u32,
        pub task: u32,
        pub element: u32,
        pub type_: u32,
        pub length: u32,
        pub buf: __IncompleteArrayField<u32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wifi_mgmr_cfg_element_msg {
        #[inline]
        fn default() -> wifi_mgmr_cfg_element_msg {
            wifi_mgmr_cfg_element_msg{ops:
                                          ::core::default::Default::default(),
                                      task:
                                          ::core::default::Default::default(),
                                      element:
                                          ::core::default::Default::default(),
                                      type_:
                                          ::core::default::Default::default(),
                                      length:
                                          ::core::default::Default::default(),
                                      buf:
                                          ::core::default::Default::default(),}
        }
    }
    pub type wifi_mgmr_cfg_element_msg_t = wifi_mgmr_cfg_element_msg;
    #[repr(C, packed)]
    pub struct wifi_mgmr_profile_msg {
        pub ssid: [::cty::c_char; 32usize],
        pub ssid_tail: [::cty::c_char; 1usize],
        pub ssid_len: u32,
        pub psk: [::cty::c_char; 64usize],
        pub psk_tail: [::cty::c_char; 1usize],
        pub pmk: [::cty::c_char; 64usize],
        pub pmk_tail: [::cty::c_char; 1usize],
        pub psk_len: u32,
        pub pmk_len: u32,
        pub mac: [u8; 6usize],
        pub band: u8,
        pub freq: u16,
        pub dhcp_use: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_profile_msg { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_profile_msg {
        #[inline]
        fn clone(&self) -> wifi_mgmr_profile_msg {
            {
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 32usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 1usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 64usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 1usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 64usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 1usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    impl Default for wifi_mgmr_profile_msg {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type wifi_mgmr_profile_msg_t = wifi_mgmr_profile_msg;
    #[repr(C, packed)]
    pub struct wifi_mgmr_ipgot_msg {
        pub ip: u32,
        pub mask: u32,
        pub gw: u32,
        pub dns1: u32,
        pub dns2: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wifi_mgmr_ipgot_msg {
        #[inline]
        fn default() -> wifi_mgmr_ipgot_msg {
            wifi_mgmr_ipgot_msg{ip: ::core::default::Default::default(),
                                mask: ::core::default::Default::default(),
                                gw: ::core::default::Default::default(),
                                dns1: ::core::default::Default::default(),
                                dns2: ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_ipgot_msg { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_ipgot_msg {
        #[inline]
        fn clone(&self) -> wifi_mgmr_ipgot_msg {
            {
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    pub type wifi_mgmr_ipgot_msg_t = wifi_mgmr_ipgot_msg;
    #[repr(C, packed)]
    pub struct wifi_mgmr_ap_msg {
        pub channel: i32,
        pub ssid: [::cty::c_char; 32usize],
        pub ssid_tail: [::cty::c_char; 1usize],
        pub hidden_ssid: u8,
        pub ssid_len: u32,
        pub psk: [::cty::c_char; 64usize],
        pub psk_tail: [::cty::c_char; 1usize],
        pub psk_len: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_ap_msg { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_ap_msg {
        #[inline]
        fn clone(&self) -> wifi_mgmr_ap_msg {
            {
                let _: ::core::clone::AssertParamIsClone<i32>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 32usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 1usize]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 64usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 1usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    impl Default for wifi_mgmr_ap_msg {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type wifi_mgmr_ap_msg_t = wifi_mgmr_ap_msg;
    #[repr(C)]
    pub struct wifi_mgmr_profile {
        pub ssid: [::cty::c_char; 33usize],
        pub no_autoconnect: u8,
        pub ssid_len: u32,
        pub psk: [::cty::c_char; 65usize],
        pub psk_len: u32,
        pub pmk: [::cty::c_char; 65usize],
        pub pmk_len: u32,
        pub mac: [u8; 6usize],
        pub dhcp_use: u8,
        pub priority: u8,
        pub isActive: u8,
        pub isUsed: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_profile { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_profile {
        #[inline]
        fn clone(&self) -> wifi_mgmr_profile {
            {
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 33usize]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 65usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 65usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    impl Default for wifi_mgmr_profile {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type wifi_mgmr_profile_t = wifi_mgmr_profile;
    #[repr(C, packed)]
    pub struct wifi_mgmr_cipher_t {
        pub _bitfield_align_1: [u8; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wifi_mgmr_cipher_t {
        #[inline]
        fn default() -> wifi_mgmr_cipher_t {
            wifi_mgmr_cipher_t{_bitfield_align_1:
                                   ::core::default::Default::default(),
                               _bitfield_1:
                                   ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_cipher_t { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_cipher_t {
        #[inline]
        fn clone(&self) -> wifi_mgmr_cipher_t {
            {
                let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
                let _:
                        ::core::clone::AssertParamIsClone<__BindgenBitfieldUnit<[u8; 1usize]>>;
                *self
            }
        }
    }
    impl wifi_mgmr_cipher_t {
        #[inline]
        pub fn wep40(&self) -> u8 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as
                                           u8)
            }
        }
        #[inline]
        pub fn set_wep40(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::core::mem::transmute(val);
                self._bitfield_1.set(0usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn wep104(&self) -> u8 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as
                                           u8)
            }
        }
        #[inline]
        pub fn set_wep104(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::core::mem::transmute(val);
                self._bitfield_1.set(1usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn tkip(&self) -> u8 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as
                                           u8)
            }
        }
        #[inline]
        pub fn set_tkip(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::core::mem::transmute(val);
                self._bitfield_1.set(2usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn ccmp(&self) -> u8 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as
                                           u8)
            }
        }
        #[inline]
        pub fn set_ccmp(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::core::mem::transmute(val);
                self._bitfield_1.set(3usize, 1u8, val as u64)
            }
        }
        #[inline]
        pub fn rsvd(&self) -> u8 {
            unsafe {
                ::core::mem::transmute(self._bitfield_1.get(4usize, 4u8) as
                                           u8)
            }
        }
        #[inline]
        pub fn set_rsvd(&mut self, val: u8) {
            unsafe {
                let val: u8 = ::core::mem::transmute(val);
                self._bitfield_1.set(4usize, 4u8, val as u64)
            }
        }
        #[inline]
        pub fn new_bitfield_1(wep40: u8, wep104: u8, tkip: u8, ccmp: u8,
                              rsvd: u8)
         -> __BindgenBitfieldUnit<[u8; 1usize]> {
            let mut __bindgen_bitfield_unit:
                    __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
            __bindgen_bitfield_unit.set(0usize, 1u8,
                                        {
                                            let wep40: u8 =
                                                unsafe {
                                                    ::core::mem::transmute(wep40)
                                                };
                                            wep40 as u64
                                        });
            __bindgen_bitfield_unit.set(1usize, 1u8,
                                        {
                                            let wep104: u8 =
                                                unsafe {
                                                    ::core::mem::transmute(wep104)
                                                };
                                            wep104 as u64
                                        });
            __bindgen_bitfield_unit.set(2usize, 1u8,
                                        {
                                            let tkip: u8 =
                                                unsafe {
                                                    ::core::mem::transmute(tkip)
                                                };
                                            tkip as u64
                                        });
            __bindgen_bitfield_unit.set(3usize, 1u8,
                                        {
                                            let ccmp: u8 =
                                                unsafe {
                                                    ::core::mem::transmute(ccmp)
                                                };
                                            ccmp as u64
                                        });
            __bindgen_bitfield_unit.set(4usize, 4u8,
                                        {
                                            let rsvd: u8 =
                                                unsafe {
                                                    ::core::mem::transmute(rsvd)
                                                };
                                            rsvd as u64
                                        });
            __bindgen_bitfield_unit
        }
    }
    #[repr(C)]
    pub struct wifi_mgmr_scan_item {
        pub ssid: [::cty::c_char; 32usize],
        pub ssid_tail: [::cty::c_char; 1usize],
        pub ssid_len: u32,
        pub bssid: [u8; 6usize],
        pub channel: u8,
        pub rssi: i8,
        pub ppm_abs: i8,
        pub ppm_rel: i8,
        pub auth: u8,
        pub cipher: u8,
        pub is_used: u8,
        pub timestamp_lastseen: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wifi_mgmr_scan_item {
        #[inline]
        fn default() -> wifi_mgmr_scan_item {
            wifi_mgmr_scan_item{ssid: ::core::default::Default::default(),
                                ssid_tail:
                                    ::core::default::Default::default(),
                                ssid_len: ::core::default::Default::default(),
                                bssid: ::core::default::Default::default(),
                                channel: ::core::default::Default::default(),
                                rssi: ::core::default::Default::default(),
                                ppm_abs: ::core::default::Default::default(),
                                ppm_rel: ::core::default::Default::default(),
                                auth: ::core::default::Default::default(),
                                cipher: ::core::default::Default::default(),
                                is_used: ::core::default::Default::default(),
                                timestamp_lastseen:
                                    ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_scan_item { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_scan_item {
        #[inline]
        fn clone(&self) -> wifi_mgmr_scan_item {
            {
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 32usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 1usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<i8>;
                let _: ::core::clone::AssertParamIsClone<i8>;
                let _: ::core::clone::AssertParamIsClone<i8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    pub type wifi_mgmr_scan_item_t = wifi_mgmr_scan_item;
    #[repr(C)]
    pub struct wlan_netif {
        pub mode: ::cty::c_int,
        pub vif_index: u8,
        pub mac: [u8; 6usize],
        pub ipv4: wlan_netif__bindgen_ty_1,
        pub netif: netif,
        pub __bindgen_anon_1: wlan_netif__bindgen_ty_2,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wlan_netif { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wlan_netif {
        #[inline]
        fn clone(&self) -> wlan_netif {
            {
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<wlan_netif__bindgen_ty_1>;
                let _: ::core::clone::AssertParamIsClone<netif>;
                let _:
                        ::core::clone::AssertParamIsClone<wlan_netif__bindgen_ty_2>;
                *self
            }
        }
    }
    #[repr(C)]
    pub struct wlan_netif__bindgen_ty_1 {
        pub ip: u32,
        pub mask: u32,
        pub gw: u32,
        pub dns1: u32,
        pub dns2: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wlan_netif__bindgen_ty_1 {
        #[inline]
        fn default() -> wlan_netif__bindgen_ty_1 {
            wlan_netif__bindgen_ty_1{ip: ::core::default::Default::default(),
                                     mask:
                                         ::core::default::Default::default(),
                                     gw: ::core::default::Default::default(),
                                     dns1:
                                         ::core::default::Default::default(),
                                     dns2:
                                         ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wlan_netif__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wlan_netif__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> wlan_netif__bindgen_ty_1 {
            {
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    #[repr(C)]
    pub union wlan_netif__bindgen_ty_2 {
        pub sta: wlan_netif__bindgen_ty_2__bindgen_ty_1,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wlan_netif__bindgen_ty_2 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wlan_netif__bindgen_ty_2 {
        #[inline]
        fn clone(&self) -> wlan_netif__bindgen_ty_2 {
            { let _: ::core::clone::AssertParamIsCopy<Self>; *self }
        }
    }
    #[repr(C)]
    pub struct wlan_netif__bindgen_ty_2__bindgen_ty_1 {
        pub rssi: i8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wlan_netif__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        fn default() -> wlan_netif__bindgen_ty_2__bindgen_ty_1 {
            wlan_netif__bindgen_ty_2__bindgen_ty_1{rssi:
                                                       ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wlan_netif__bindgen_ty_2__bindgen_ty_1 { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wlan_netif__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        fn clone(&self) -> wlan_netif__bindgen_ty_2__bindgen_ty_1 {
            { let _: ::core::clone::AssertParamIsClone<i8>; *self }
        }
    }
    impl Default for wlan_netif__bindgen_ty_2 {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    impl Default for wlan_netif {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    #[repr(C)]
    pub struct wifi_mgmr_connect_ind_stat_info {
        pub status_code: u16,
        pub type_ind: u8,
        pub ssid: [::cty::c_char; 32usize],
        pub psk: [::cty::c_char; 65usize],
        pub bssid: [u8; 6usize],
        pub chan_freq: u16,
        pub chan_band: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_connect_ind_stat_info { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_connect_ind_stat_info {
        #[inline]
        fn clone(&self) -> wifi_mgmr_connect_ind_stat_info {
            {
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 32usize]>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 65usize]>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    impl Default for wifi_mgmr_connect_ind_stat_info {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type wifi_mgmr_connect_ind_stat_info_t =
     wifi_mgmr_connect_ind_stat_info;
    #[repr(C)]
    pub struct wifi_mgmr_sta_basic_info {
        pub sta_idx: u8,
        pub is_used: u8,
        pub sta_mac: [u8; 6usize],
        pub tsfhi: u32,
        pub tsflo: u32,
        pub rssi: ::cty::c_int,
        pub data_rate: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for wifi_mgmr_sta_basic_info {
        #[inline]
        fn default() -> wifi_mgmr_sta_basic_info {
            wifi_mgmr_sta_basic_info{sta_idx:
                                         ::core::default::Default::default(),
                                     is_used:
                                         ::core::default::Default::default(),
                                     sta_mac:
                                         ::core::default::Default::default(),
                                     tsfhi:
                                         ::core::default::Default::default(),
                                     tsflo:
                                         ::core::default::Default::default(),
                                     rssi:
                                         ::core::default::Default::default(),
                                     data_rate:
                                         ::core::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr_sta_basic_info { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr_sta_basic_info {
        #[inline]
        fn clone(&self) -> wifi_mgmr_sta_basic_info {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<[u8; 6usize]>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    pub type wifi_mgmr_sta_basic_info_t = wifi_mgmr_sta_basic_info;
    #[repr(C)]
    pub struct wifi_mgmr {
        pub ready: u8,
        pub channel: ::cty::c_int,
        pub inf_ap_enabled: ::cty::c_int,
        pub wlan_sta: wlan_netif,
        pub wlan_ap: wlan_netif,
        pub status: WIFI_MGMR_CONNECTION_STATUS_T,
        pub profiles: [wifi_mgmr_profile_t; 2usize],
        pub profile_active_index: ::cty::c_int,
        pub scan_items: [wifi_mgmr_scan_item_t; 50usize],
        pub mq: os_messagequeue_t,
        pub mq_pool: [u8; 2240usize],
        pub m: stateMachine,
        pub timer: os_timer_t,
        pub wifi_mgmr_stat_info: wifi_mgmr_connect_ind_stat_info_t,
        pub country_code: [::cty::c_char; 3usize],
        pub disable_autoreconnect: u8,
        pub channel_nums: ::cty::c_int,
        pub pending_task: u32,
        pub features: u32,
        pub scan_item_timeout: ::cty::c_int,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for wifi_mgmr { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for wifi_mgmr {
        #[inline]
        fn clone(&self) -> wifi_mgmr {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<wlan_netif>;
                let _: ::core::clone::AssertParamIsClone<wlan_netif>;
                let _:
                        ::core::clone::AssertParamIsClone<WIFI_MGMR_CONNECTION_STATUS_T>;
                let _:
                        ::core::clone::AssertParamIsClone<[wifi_mgmr_profile_t; 2usize]>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _:
                        ::core::clone::AssertParamIsClone<[wifi_mgmr_scan_item_t; 50usize]>;
                let _: ::core::clone::AssertParamIsClone<os_messagequeue_t>;
                let _: ::core::clone::AssertParamIsClone<[u8; 2240usize]>;
                let _: ::core::clone::AssertParamIsClone<stateMachine>;
                let _: ::core::clone::AssertParamIsClone<os_timer_t>;
                let _:
                        ::core::clone::AssertParamIsClone<wifi_mgmr_connect_ind_stat_info_t>;
                let _:
                        ::core::clone::AssertParamIsClone<[::cty::c_char; 3usize]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<::cty::c_int>;
                *self
            }
        }
    }
    impl Default for wifi_mgmr {
        fn default() -> Self { unsafe { ::core::mem::zeroed() } }
    }
    pub type wifi_mgmr_t = wifi_mgmr;
    pub fn mgmr_event_notify(msg: *mut wifi_mgmr_msg_t) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_event_notify(msg: *mut wifi_mgmr_msg_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_event_notify(msg as *mut wifi_mgmr_msg_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_state_get_internal(state: *mut ::cty::c_int) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_state_get_internal(state: *mut ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_state_get_internal(state as *mut ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_status_code_clean_internal() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_status_code_clean_internal()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_status_code_clean_internal();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_status_code_get_internal(s_code: *mut ::cty::c_int)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_status_code_get_internal(s_code:
                                                          *mut ::cty::c_int)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_status_code_get_internal(s_code as
                                                       *mut ::cty::c_int);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_set_country_code_internal(country_code: *mut ::cty::c_char)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_set_country_code_internal(country_code:
                                                           *mut ::cty::c_char)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_set_country_code_internal(country_code as
                                                        *mut ::cty::c_char);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_sta_cnt_get_internal(sta_cnt: *mut u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_sta_cnt_get_internal(sta_cnt: *mut u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_sta_cnt_get_internal(sta_cnt as *mut u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_sta_info_get_internal(sta_info_internal:
                                             *mut wifi_mgmr_sta_basic_info_t,
                                         idx: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_sta_info_get_internal(sta_info_internal:
                                                          *mut wifi_mgmr_sta_basic_info_t,
                                                      idx: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_ap_sta_info_get_internal(sta_info_internal as
                                                       *mut wifi_mgmr_sta_basic_info_t,
                                                   idx as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_ap_sta_delete_internal(sta_idx: u8) -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_ap_sta_delete_internal(sta_idx: u8)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_ap_sta_delete_internal(sta_idx as u8);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_scan_complete_notify() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_scan_complete_notify()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_scan_complete_notify();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    extern "C" {
        pub static mut wifiMgmr: wifi_mgmr_t ;
    }
    pub fn mgmr_auth_to_str(auth: u8) -> BlResult<*mut ::cty::c_char> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_auth_to_str(auth: u8)
            -> *mut ::cty::c_char;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_auth_to_str(auth as u8);
            "----------Result----------";
            Ok(res)
        }
    }
    pub fn mgmr_cipher_to_str(cipher: u8) -> BlResult<*mut ::cty::c_char> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_cipher_to_str(cipher: u8)
            -> *mut ::cty::c_char;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_cipher_to_str(cipher as u8);
            "----------Result----------";
            Ok(res)
        }
    }
    pub fn mgmr_api_fw_tsen_reload() -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_api_fw_tsen_reload()
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res = wifi_mgmr_api_fw_tsen_reload();
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
    pub fn mgmr_scan_item_is_timeout(mgmr: *mut wifi_mgmr_t,
                                     item: *mut wifi_mgmr_scan_item_t)
     -> BlResult<()> {
        "----------Extern Decl----------";
        extern "C" {
            pub fn wifi_mgmr_scan_item_is_timeout(mgmr: *mut wifi_mgmr_t,
                                                  item:
                                                      *mut wifi_mgmr_scan_item_t)
            -> ::cty::c_int;
        }
        "----------Validation----------";
        unsafe {
            "----------Call----------";
            let res =
                wifi_mgmr_scan_item_is_timeout(mgmr as *mut wifi_mgmr_t,
                                               item as
                                                   *mut wifi_mgmr_scan_item_t);
            "----------Result----------";
            match res { 0 => Ok(()), _ => Err(BlError::from(res)), }
        }
    }
}
use core::{panic::PanicInfo, str::FromStr};
/// Print a message to the serial console.
/// TODO: Auto-generate this wrapper with `bindgen` from the C declaration
fn puts(s: &str) -> i32 {
    extern "C" {
        /// Print a message to the serial console (from C stdio library)
        fn puts(s: *const u8)
        -> i32;
    }
    let mut s_with_null =
        String::from_str(s).expect("puts conversion failed");
    s_with_null.push('\0').expect("puts overflow");
    let p = s_with_null.as_str().as_ptr();
    unsafe { puts(p) }
}
/// Convert milliseconds to system ticks.
/// TODO: Auto-generate this wrapper with `bindgen` from the C declaration:
/// `ble_npl_time_t ble_npl_time_ms_to_ticks32(uint32_t ms)`
fn time_ms_to_ticks32(ms: u32) -> u32 {
    extern "C" {
        /// Convert milliseconds to system ticks (from NimBLE Porting Layer)
        fn ble_npl_time_ms_to_ticks32(ms: u32)
        -> u32;
    }
    unsafe { ble_npl_time_ms_to_ticks32(ms) }
}
/// Sleep for the specified number of system ticks.
/// TODO: Auto-generate this wrapper with `bindgen` from the C declaration:
/// `void ble_npl_time_delay(ble_npl_time_t ticks)`
fn time_delay(ticks: u32) {
    extern "C" {
        /// Sleep for the specified number of system ticks (from NimBLE Porting Layer)
        fn ble_npl_time_delay(ticks: u32);
    }
    unsafe { ble_npl_time_delay(ticks); }
}
/// Limit Strings to 64 chars, similar to `char[64]` in C
type String = heapless::String<64>;
/// HAL return type and error codes
pub mod result {
    /// Common return type for BL602 HAL.  If no error, returns `Ok(val)` where val has type T.
    /// Upon error, returns `Err(err)` where err is the BlError error code.
    pub type BlResult<T> = ::core::result::Result<T, BlError>;
    /// Error codes for BL602 HAL
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum BlError {

        /// Error code 0 means no error.
        SYS_EOK = 0,
        SYS_UNKNOWN = -1,
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for BlError { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for BlError {
        #[inline]
        fn eq(&self, other: &BlError) -> bool {
            {
                let __self_vi =
                    ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi =
                    ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) { _ => true, }
                } else { false }
            }
        }
    }
    /// Cast `BlError` to `i32`
    impl From<BlError> for i32 {
        /// Cast `BlError` to `i32`
        fn from(err: BlError) -> Self { err as i32 }
    }
    /// Cast `i32` to `BlError`
    impl From<i32> for BlError {
        /// Cast `i32` to `BlError`
        fn from(num: i32) -> Self {
            unsafe { ::core::mem::transmute::<i32, BlError>(num) }
        }
    }
    /// Cast `()` to `BlError`
    impl From<()> for BlError {
        /// Cast `()` to `BlError`
        fn from(_: ()) -> Self { BlError::SYS_UNKNOWN }
    }
    /// Implement formatted output for BlError
    impl core::fmt::Debug for BlError {
        fn fmt(&self, _fmt: &mut ::core::fmt::Formatter<'_>)
         -> ::core::fmt::Result {
            Ok(())
        }
    }
}
/// Represents a null-terminated string, suitable for passing to C APIs as `* const char`.
/// The string can be a null-terminated byte string created in Rust, or a pointer to a null-terminated string returned by C.
/// Pointer may be null.
pub struct Strn<'a> {
    /// Either a byte string terminated with null, or a pointer to a null-terminated string
    pub rep: StrnRep<'a>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <'a> ::core::clone::Clone for Strn<'a> {
    #[inline]
    fn clone(&self) -> Strn<'a> {
        { let _: ::core::clone::AssertParamIsClone<StrnRep<'a>>; *self }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <'a> ::core::marker::Copy for Strn<'a> { }
/// Either a byte string or a string pointer
#[repr(u8)]
pub enum StrnRep<'a> {

    /// Byte string terminated with null
    ByteStr(&'a [u8]),

    /// Pointer to a null-terminated string
    CStr(*const u8),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <'a> ::core::clone::Clone for StrnRep<'a> {
    #[inline]
    fn clone(&self) -> StrnRep<'a> {
        {
            let _: ::core::clone::AssertParamIsClone<&'a [u8]>;
            let _: ::core::clone::AssertParamIsClone<*const u8>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <'a> ::core::marker::Copy for StrnRep<'a> { }
impl <'a> Strn<'a> {
    /// Create a new `Strn` with a byte string. Fail if the last byte is not zero.
    /// ```
    /// Strn::new(b"network\0")
    /// strn!("network")
    /// ```
    pub fn new(bs: &[u8]) -> Strn {
        {
            match (&bs.last(), &Some(&0u8)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(kind, &*left_val,
                                                         &*right_val,
                                                         ::core::option::Option::Some(::core::fmt::Arguments::new_v1(&["no null"],
                                                                                                                     &match ()
                                                                                                                          {
                                                                                                                          ()
                                                                                                                          =>
                                                                                                                          [],
                                                                                                                      })));
                    }
                }
            }
        };
        Strn{rep: StrnRep::ByteStr(bs),}
    }
    /// Create a new `Strn` with a null-terminated string pointer returned by C.
    pub fn from_cstr(cstr: *const u8) -> Strn<'a> {
        Strn{rep: StrnRep::CStr(cstr),}
    }
    /// Return a pointer to the string
    pub fn as_ptr(&self) -> *const u8 {
        match self.rep {
            StrnRep::ByteStr(bs) => { bs.as_ptr() }
            StrnRep::CStr(cstr) => { cstr }
        }
    }
    /// Return the length of the string, excluding the terminating null. For safety, we limit to 128.
    pub fn len(&self) -> usize {
        match self.rep {
            StrnRep::ByteStr(bs) => {
                {
                    match (&bs.last(), &Some(&0u8)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(kind,
                                                                 &*left_val,
                                                                 &*right_val,
                                                                 ::core::option::Option::Some(::core::fmt::Arguments::new_v1(&["no null"],
                                                                                                                             &match ()
                                                                                                                                  {
                                                                                                                                  ()
                                                                                                                                  =>
                                                                                                                                  [],
                                                                                                                              })));
                            }
                        }
                    }
                };
                bs.len() - 1
            }
            StrnRep::CStr(cstr) => {
                if cstr.is_null() { return 0; }
                for len in 0..127 {
                    let ptr: *const u8 = ((cstr as u32) + len) as *const u8;
                    if unsafe { *ptr } == 0 { return len as usize; }
                }
                if !false { ::core::panicking::panic("big strn") };
                return 128 as usize;
            }
        }
    }
    /// Return true if the string is empty
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    /// Return the byte string as a null-terminated `* const char` C-style string.
    /// Fail if the last byte is not zero.
    pub fn as_cstr(&self) -> *const u8 {
        match self.rep {
            StrnRep::ByteStr(bs) => {
                {
                    match (&bs.last(), &Some(&0u8)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(kind,
                                                                 &*left_val,
                                                                 &*right_val,
                                                                 ::core::option::Option::Some(::core::fmt::Arguments::new_v1(&["no null"],
                                                                                                                             &match ()
                                                                                                                                  {
                                                                                                                                  ()
                                                                                                                                  =>
                                                                                                                                  [],
                                                                                                                              })));
                            }
                        }
                    }
                };
                bs.as_ptr() as *const u8
            }
            StrnRep::CStr(cstr) => { cstr }
        }
    }
    /// Return the byte string.
    /// Fail if the last byte is not zero.
    pub fn as_bytestr(&self) -> &'a [u8] {
        match self.rep {
            StrnRep::ByteStr(bs) => {
                {
                    match (&bs.last(), &Some(&0u8)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(kind,
                                                                 &*left_val,
                                                                 &*right_val,
                                                                 ::core::option::Option::Some(::core::fmt::Arguments::new_v1(&["no null"],
                                                                                                                             &match ()
                                                                                                                                  {
                                                                                                                                  ()
                                                                                                                                  =>
                                                                                                                                  [],
                                                                                                                              })));
                            }
                        }
                    }
                };
                &bs
            }
            StrnRep::CStr(_cstr) => {
                if !false { ::core::panicking::panic("strn cstr") };
                b"\0"
            }
        }
    }
    /// Fail if the last byte is not zero.
    pub fn validate(&self) {
        match self.rep {
            StrnRep::ByteStr(bs) => {
                {
                    match (&bs.last(), &Some(&0u8)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(kind,
                                                                 &*left_val,
                                                                 &*right_val,
                                                                 ::core::option::Option::Some(::core::fmt::Arguments::new_v1(&["no null"],
                                                                                                                             &match ()
                                                                                                                                  {
                                                                                                                                  ()
                                                                                                                                  =>
                                                                                                                                  [],
                                                                                                                              })));
                            }
                        }
                    }
                };
            }
            StrnRep::CStr(_cstr) => { }
        }
    }
    /// Fail if the last byte is not zero.
    pub fn validate_bytestr(bs: &'static [u8]) {
        {
            match (&bs.last(), &Some(&0u8)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(kind, &*left_val,
                                                         &*right_val,
                                                         ::core::option::Option::Some(::core::fmt::Arguments::new_v1(&["no null"],
                                                                                                                     &match ()
                                                                                                                          {
                                                                                                                          ()
                                                                                                                          =>
                                                                                                                          [],
                                                                                                                      })));
                    }
                }
            }
        };
    }
}
///  Allow threads to share Strn, since it is static.
unsafe impl <'a> Send for Strn<'a> { }
///  Allow threads to share Strn, since it is static.
unsafe impl <'a> Sync for Strn<'a> { }
///  Declare a `void *` pointer that will be passed to C functions
pub type Ptr = *mut ::cty::c_void;
/// For Testing: This function will be called by the BL602 command-line interface
#[no_mangle]
extern "C" fn test_rust(_result: *mut u8, _len: i32, _argc: i32,
                        _argv: *const *const u8) {
    puts("Hello from Rust!");
    const LED_GPIO: u8 = 11;
    gpio::enable_output(LED_GPIO, 0, 0).expect("GPIO enable output failed");
    for i in 0..10 {
        gpio::output_set(LED_GPIO, i % 2).expect("GPIO output failed");
        time_delay(time_ms_to_ticks32(1000));
    }
}
/// For Testing: This function is called on panic, like an assertion failure
#[panic_handler]
fn test_panic(_info: &PanicInfo) -> ! {
    puts("TODO: Test Rust panic");
    loop { }
}
