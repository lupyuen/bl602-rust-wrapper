#![feature(prelude_import)]
//!  Main Rust Application for BL602 Firmware
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
pub mod gpio {

    //  Import the Rust Core Library
    //  For `PanicInfo` type used by `panic` function
    //  For converting `str` to `String`

    //  Don't mangle the name `rust_main`
    //  Declare `extern "C"` because it will be called by BL602 firmware
    //  Command line (char *)
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

    ///////////////////////////////////////////////////////////////////////////////
    //  Safe Wrappers for BL602 C Functions imported from
    //  BL602 IoT SDK and NimBLE Porting Layer
    //  (Will be auto-generated by `bindgen` and Rust Procedural Macro)

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

    //  GPIO pin number (uint8_t)
    //  0 for no pullup, 1 for pullup (uint8_t) 
    //  0 for no pulldown, 1 for pulldown (uint8_t)
    //  Returns an error code (int)

    //  Import C Function

    //  Call the C function
    //  Flag this code as unsafe because we're calling a C function

    //  Check the result code
    //  If no error, return OK
    //  Else return the result code as an error

    //  GPIO pin number (uint8_t)
    //  0 for low, 1 to high
    //  Returns an error code (int)

    //  Import C Function

    //  Call the C function
    //  Flag this code as unsafe because we're calling a C function

    //  Check the result code
    //  If no error, return OK
    //  Else return the result code as an error

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
    //  BL602 Types



    //  Allow type names to have non-camel case




    //  TODO
    use super::*;
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
    pub fn gpio_enable_output(pin: u8, pullup: u8, pulldown: u8)
     -> BlResult<()> {
        "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
        extern "C" {
            pub fn bl_gpio_enable_output(pin: u8, pullup: u8, pulldown: u8)
            -> ::cty::c_int;
        }
        "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
        unsafe {
            "----------Insert Call: `let result_value = os_task_init(`----------";
            let result_value =
                bl_gpio_enable_output(pin as u8, pullup as u8,
                                      pulldown as u8);
            "----------Insert Result: `Ok(Strn::from_cstr(result_value))`----------";
            if result_value == 0 {
                Ok(())
            } else { Err(BlError::from(result_value)) }
        }
    }
    pub fn gpio_enable_input(pin: u8, pullup: u8, pulldown: u8)
     -> BlResult<()> {
        "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
        extern "C" {
            pub fn bl_gpio_enable_input(pin: u8, pullup: u8, pulldown: u8)
            -> ::cty::c_int;
        }
        "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
        unsafe {
            "----------Insert Call: `let result_value = os_task_init(`----------";
            let result_value =
                bl_gpio_enable_input(pin as u8, pullup as u8, pulldown as u8);
            "----------Insert Result: `Ok(Strn::from_cstr(result_value))`----------";
            if result_value == 0 {
                Ok(())
            } else { Err(BlError::from(result_value)) }
        }
    }
    pub fn gpio_output_set(pin: u8, value: u8) -> BlResult<()> {
        "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
        extern "C" {
            pub fn bl_gpio_output_set(pin: u8, value: u8)
            -> ::cty::c_int;
        }
        "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
        unsafe {
            "----------Insert Call: `let result_value = os_task_init(`----------";
            let result_value = bl_gpio_output_set(pin as u8, value as u8);
            "----------Insert Result: `Ok(Strn::from_cstr(result_value))`----------";
            if result_value == 0 {
                Ok(())
            } else { Err(BlError::from(result_value)) }
        }
    }
    pub fn gpio_input_get(pin: u8, value: *mut u8) -> BlResult<()> {
        "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
        extern "C" {
            pub fn bl_gpio_input_get(pin: u8, value: *mut u8)
            -> ::cty::c_int;
        }
        "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
        unsafe {
            "----------Insert Call: `let result_value = os_task_init(`----------";
            let result_value = bl_gpio_input_get(pin as u8, value as *mut u8);
            "----------Insert Result: `Ok(Strn::from_cstr(result_value))`----------";
            if result_value == 0 {
                Ok(())
            } else { Err(BlError::from(result_value)) }
        }
    }
    pub fn gpio_input_get_value(pin: u8) -> BlResult<()> {
        "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
        extern "C" {
            pub fn bl_gpio_input_get_value(pin: u8)
            -> ::cty::c_int;
        }
        "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
        unsafe {
            "----------Insert Call: `let result_value = os_task_init(`----------";
            let result_value = bl_gpio_input_get_value(pin as u8);
            "----------Insert Result: `Ok(Strn::from_cstr(result_value))`----------";
            if result_value == 0 {
                Ok(())
            } else { Err(BlError::from(result_value)) }
        }
    }
    pub fn gpio_int_clear(gpioPin: u8, intClear: u8) -> BlResult<()> {
        "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
        extern "C" {
            pub fn bl_gpio_int_clear(gpioPin: u8, intClear: u8)
            -> ::cty::c_int;
        }
        "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
        unsafe {
            "----------Insert Call: `let result_value = os_task_init(`----------";
            let result_value =
                bl_gpio_int_clear(gpioPin as u8, intClear as u8);
            "----------Insert Result: `Ok(Strn::from_cstr(result_value))`----------";
            if result_value == 0 {
                Ok(())
            } else { Err(BlError::from(result_value)) }
        }
    }
    pub fn gpio_intmask(gpiopin: u8, mask: u8) -> BlResult<()> {
        "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
        extern "C" {
            pub fn bl_gpio_intmask(gpiopin: u8, mask: u8);
        }
        "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
        unsafe {
            "----------Insert Call: `let result_value = os_task_init(`----------";
            bl_gpio_intmask(gpiopin as u8, mask as u8);
            "----------Insert Result: `Ok(Strn::from_cstr(result_value))`----------";
            Ok(())
        }
    }
    pub fn gpio_register(pstnode: *mut gpio_ctx_t) -> BlResult<()> {
        "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
        extern "C" {
            pub fn bl_gpio_register(pstnode: *mut gpio_ctx_t);
        }
        "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
        unsafe {
            "----------Insert Call: `let result_value = os_task_init(`----------";
            bl_gpio_register(pstnode as *mut gpio_ctx_t);
            "----------Insert Result: `Ok(Strn::from_cstr(result_value))`----------";
            Ok(())
        }
    }
}
use core::{panic::PanicInfo, str::FromStr};
/// `rust_main` will be called by the BL602 command-line interface
#[no_mangle]
extern "C" fn rust_main(_buf: *const u8, _len: i32, _argc: i32,
                        _argv: *const *const u8) {
    puts("Hello from Rust!");
    const LED_GPIO: u8 = 11;
    bl_gpio_enable_output(LED_GPIO, 0, 0).expect("GPIO enable output failed");
    for i in 0..10 {
        bl_gpio_output_set(LED_GPIO, i % 2).expect("GPIO output failed");
        time_delay(time_ms_to_ticks32(1000));
    }
}
/// This function is called on panic, like an assertion failure
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { puts("TODO: Rust panic"); loop  { } }
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
/// Configure a GPIO pin for output (instead of input).
/// TODO: Auto-generate this wrapper with `bindgen` from the C declaration:
/// `int bl_gpio_enable_output(uint8_t pin, uint8_t pullup, uint8_t pulldown)`
fn bl_gpio_enable_output(pin: u8, pullup: u8, pulldown: u8)
 -> Result<(), i32> {
    extern "C" {
        /// Configure a GPIO pin for output (from BL602 GPIO HAL)
        fn bl_gpio_enable_output(pin: u8, pullup: u8, pulldown: u8)
        -> i32;
    }
    let res = unsafe { bl_gpio_enable_output(pin, pullup, pulldown) };
    match res { 0 => Ok(()), _ => Err(res), }
}
/// Set the GPIO pin output to high or low.
/// TODO: Auto-generate this wrapper with `bindgen` from the C declaration:
/// `int bl_gpio_output_set(uint8_t pin, uint8_t value)`
fn bl_gpio_output_set(pin: u8, value: u8) -> Result<(), i32> {
    extern "C" {
        /// Set the GPIO pin output to high or low (from BL602 GPIO HAL)
        fn bl_gpio_output_set(pin: u8, value: u8)
        -> i32;
    }
    let res = unsafe { bl_gpio_output_set(pin, value) };
    match res { 0 => Ok(()), _ => Err(res), }
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
type String = heapless::String<heapless::consts::U64>;
/// Return type and error codes for BL602 API
pub mod result {
    /// Common return type for BL602 API.  If no error, returns `Ok(val)` where val has type T.
    /// Upon error, returns `Err(err)` where err is the BlError error code.
    pub type BlResult<T> = ::core::result::Result<T, BlError>;
    /// Error codes for BL602 API
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
