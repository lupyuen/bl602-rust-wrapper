//!  Main Rust Application for BL602 Firmware
#![no_std]  //  Use the Rust Core Library instead of the Rust Standard Library, which is not compatible with embedded systems

use bl602_macros::safe_wrap;
use result::*;

//  Include the BL602 Wrappers generated by `bindgen` and `safe_wrap`
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod i2c;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod gpio;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod pwm;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod spi;

//  Import the Rust Core Library
use core::{
    panic::PanicInfo,  //  For `PanicInfo` type used by `panic` function
    str::FromStr,      //  For converting `str` to `String`
};

/// `rust_main` will be called by the BL602 command-line interface
#[no_mangle]              //  Don't mangle the name `rust_main`
extern "C" fn rust_main(  //  Declare `extern "C"` because it will be called by BL602 firmware
    _buf:  *const u8,        //  Command line (char *)
    _len:  i32,              //  Length of command line (int)
    _argc: i32,              //  Number of command line args (int)
    _argv: *const *const u8  //  Array of command line args (char **)
) {
    //  Show a message on the serial console
    puts("Hello from Rust!");

    //  PineCone Blue LED is connected on BL602 GPIO 11
    const LED_GPIO: u8 = 11;  //  `u8` is 8-bit unsigned integer

    //  Configure the LED GPIO for output (instead of input)
    gpio::gpio_enable_output(LED_GPIO, 0, 0)   //  No pullup, no pulldown
        .expect("GPIO enable output failed");  //  Halt on error

    //  Blink the LED 5 times
    for i in 0..10 {  //  Iterates 10 times from 0 to 9 (`..` excludes 10)

        //  Toggle the LED GPIO between 0 (on) and 1 (off)
        gpio::gpio_output_set(  //  Set the GPIO output (from BL602 GPIO HAL)
            LED_GPIO,           //  GPIO pin number
            i % 2               //  0 for low, 1 for high
        ).expect("GPIO output failed");  //  Halt on error

        //  Sleep 1 second
        time_delay(                   //  Sleep by number of ticks (from NimBLE Porting Layer)
            time_ms_to_ticks32(1000)  //  Convert 1,000 milliseconds to ticks (from NimBLE Porting Layer)
        );
    }

    //  Return to the BL602 command-line interface
}

/// This function is called on panic, like an assertion failure
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  //  `!` means that panic handler will never return
    //  TODO: Implement the complete panic handler like this:
    //  https://github.com/lupyuen/pinetime-rust-BL602/blob/master/rust/app/src/lib.rs#L115-L146

    //  For now we display a message
    puts("TODO: Rust panic"); 

	//  Loop forever, do not pass go, do not collect $200
    loop {}
}

///////////////////////////////////////////////////////////////////////////////
//  Safe Wrappers for BL602 C Functions imported from
//  BL602 IoT SDK and NimBLE Porting Layer
//  (Will be auto-generated by `bindgen` and Rust Procedural Macro)

/// Print a message to the serial console.
/// TODO: Auto-generate this wrapper with `bindgen` from the C declaration
fn puts(s: &str) -> i32 {  //  `&str` is a reference to a string slice, similar to `const char *` in C

    extern "C" {  //  Import C Function
        /// Print a message to the serial console (from C stdio library)
        fn puts(s: *const u8) -> i32;
    }

    //  Convert `str` to `String`, which similar to `char [64]` in C
    let mut s_with_null = String::from_str(s)  //  `mut` because we will modify it
        .expect("puts conversion failed");     //  If it exceeds 64 chars, halt with an error
    
    //  Terminate the string with null, since we will be passing to C
    s_with_null.push('\0')
        .expect("puts overflow");  //  If we exceed 64 chars, halt with an error

    //  Convert the null-terminated string to a pointer
    let p = s_with_null.as_str().as_ptr();

    //  Call the C function
    unsafe {  //  Flag this code as unsafe because we're calling a C function
        puts(p)
    }

    //  No semicolon `;` here, so the value returned by the C function will be passed to our caller
}

/// Convert milliseconds to system ticks.
/// TODO: Auto-generate this wrapper with `bindgen` from the C declaration:
/// `ble_npl_time_t ble_npl_time_ms_to_ticks32(uint32_t ms)`
fn time_ms_to_ticks32(
    ms: u32  //  Number of milliseconds
) -> u32 {   //  Returns the number of ticks (uint32_t)
    extern "C" {        //  Import C Function
        /// Convert milliseconds to system ticks (from NimBLE Porting Layer)
        fn ble_npl_time_ms_to_ticks32(ms: u32) -> u32;
    }

    //  Call the C function
    unsafe {  //  Flag this code as unsafe because we're calling a C function
        ble_npl_time_ms_to_ticks32(ms)
    }

    //  No semicolon `;` here, so the value returned by the C function will be passed to our caller
}

/// Sleep for the specified number of system ticks.
/// TODO: Auto-generate this wrapper with `bindgen` from the C declaration:
/// `void ble_npl_time_delay(ble_npl_time_t ticks)`
fn time_delay(
    ticks: u32  //  Number of ticks to sleep
) {
    extern "C" {  //  Import C Function
        /// Sleep for the specified number of system ticks (from NimBLE Porting Layer)
        fn ble_npl_time_delay(ticks: u32);
    }

    //  Call the C function
    unsafe {  //  Flag this code as unsafe because we're calling a C function
        ble_npl_time_delay(ticks);
    }
}

/// Limit Strings to 64 chars, similar to `char[64]` in C
type String = heapless::String::<heapless::consts::U64>;

///////////////////////////////////////////////////////////////////////////////
//  BL602 Types

/// Return type and error codes for BL602 API
pub mod result {

    /// Common return type for BL602 API.  If no error, returns `Ok(val)` where val has type T.
    /// Upon error, returns `Err(err)` where err is the BlError error code.
    pub type BlResult<T> = ::core::result::Result<T, BlError>;

    /// Error codes for BL602 API
    #[repr(i32)]
    #[derive(PartialEq)]
    #[allow(non_camel_case_types)]    //  Allow type names to have non-camel case
    pub enum BlError {
        /// Error code 0 means no error.
        SYS_EOK         = 0,
        SYS_UNKNOWN     = -1,
    }

    /// Cast `BlError` to `i32`
    impl From<BlError> for i32 {
        /// Cast `BlError` to `i32`
        fn from(err: BlError) -> Self {
            err as i32
        }
    }

    /// Cast `i32` to `BlError`
    impl From<i32> for BlError {
        /// Cast `i32` to `BlError`
        fn from(num: i32) -> Self {
            unsafe { 
                ::core::mem::transmute::
                    <i32, BlError>
                    (num)
            }  
        }
    }

    /// Cast `()` to `BlError`
    impl From<()> for BlError {
        /// Cast `()` to `BlError`
        fn from(_: ()) -> Self {
            BlError::SYS_UNKNOWN
        }
    }

    /// Implement formatted output for BlError
    impl core::fmt::Debug for BlError {
        fn fmt(&self, _fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            //  TODO
            Ok(())
        }
    }
}

///  Declare a `void *` pointer that will be passed to C functions
pub type Ptr = *mut ::cty::c_void;
