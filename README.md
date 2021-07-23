# Rust Wrapper for BL602 IoT SDK

Read the article and docs...

-   ["Run Rust RISC-V Firmware with BL602 IoT SDK"](https://lupyuen.github.io/articles/rust)

-   [Rust Docs](https://lupyuen.github.io/bl602-rust-wrapper/)

-   [Rust Crate `bl602-sdk`](https://crates.io/crates/bl602-sdk)

-   [Rust Crate `bl602-macros`](https://crates.io/crates/bl602-macros)

-   [Check this Twitter Thread for updates](https://twitter.com/MisterTechBlog/status/1416608940876435462)

## Build Instructions

To generate the Rust Wrapper and build the project...

```bash
#  Install bindgen and clang: https://rust-lang.github.io/rust-bindgen/requirements.html 
cargo install bindgen
sudo apt install llvm-dev libclang-dev clang

#  Download the source code
git clone --recursive https://github.com/lupyuen/bl602-rust-wrapper
git clone --recursive https://github.com/lupyuen/bl_iot_sdk

#  Generate the wrapper
cd bl602-rust-wrapper
scripts/gen-bindings.sh

#  Build the docs and the test project
scripts/build.sh
```

## How It Works

This script...

-   [`scripts/gen-bindings.sh`](scripts/gen-bindings.sh)

Reads the BL602 IoT SDK Header Files...

```c
//  Function Declarations from BL602 IoT SDK (GPIO HAL)
//  https://github.com/lupyuen/bl_iot_sdk/blob/master/components/hal_drv/bl602_hal/bl_gpio.h
int bl_gpio_enable_output(uint8_t pin, uint8_t pullup, uint8_t pulldown);
int bl_gpio_output_set(uint8_t pin, uint8_t value);
```

And auto-generates the Rust Bindings...

-   [`bl602-sdk/src/gpio.rs`](bl602-sdk/src/gpio.rs)
-   [`bl602-sdk/src/i2c.rs`](bl602-sdk/src/i2c.rs)
-   [`bl602-sdk/src/pwm.rs`](bl602-sdk/src/pwm.rs)
-   [`bl602-sdk/src/spi.rs`](bl602-sdk/src/spi.rs)
-   [More Bindings](bl602-sdk/src)

```rust
//  Rust Bindings for BL602 GPIO generated by gen-bindings.sh
#[safe_wrap(_)] extern "C" {
    pub fn bl_gpio_enable_output(pin: u8, pullup: u8, pulldown: u8) -> ::cty::c_int;
}

#[safe_wrap(_)] extern "C" {
    pub fn bl_gpio_output_set(pin: u8, value: u8) -> ::cty::c_int;
}
```

Which call the `safe_wrap` Procedural Macro...

-   [`bl602-macros/src/safe_wrap.rs`](bl602-macros/src/safe_wrap.rs)

To produce the Rust Wrappers for BL602 IoT SDK...

-   [Expanded `safe_wrap` macros: `logs/sdk-expanded.rs`](logs/sdk-expanded.rs)

```rust
//  Expanded version of `safe_wrap` macros for the GPIO Rust Bindings
#[doc = "Configure a GPIO Pin for Output Mode. See `bl_gpio_enable_output` in \"Enable GPIO\" <https://lupyuen.github.io/articles/led#enable-gpio>"]
pub fn enable_output(pin: u8, pullup: u8, pulldown: u8)
    -> BlResult<()> {
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

#[doc = "Set output value of GPIO Pin. See `bl_gpio_output_set` in \"Read and Write GPIO\" <https://lupyuen.github.io/articles/led#read-and-write-gpio>"]
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
```

Here's how we call the Rust Wrappers for BL602 GPIO HAL in our BL602 Rust Firmware...

-   [`bl602-sdk/src/lib.rs`](bl602-sdk/src/lib.rs)

```rust
//  Configure the LED GPIO for output (instead of input)
gpio::enable_output(LED_GPIO, 0, 0)   //  No pullup, no pulldown
    .expect("GPIO enable output failed");  //  Halt on error

//  Blink the LED 5 times
for i in 0..10 {  //  Iterates 10 times from 0 to 9 (`..` excludes 10)

    //  Toggle the LED GPIO between 0 (on) and 1 (off)
    gpio::output_set(  //  Set the GPIO output (from BL602 GPIO HAL)
        LED_GPIO,           //  GPIO pin number
        i % 2               //  0 for low, 1 for high
    ).expect("GPIO output failed");  //  Halt on error
```

Links to ["The RISC-V BL602 Book"](https://lupyuen.github.io/articles/book) are defined here...

-   [`doclinks.md`](doclinks.md)

Build the project with this script...

-   [`scripts/build.sh`](scripts/build.sh)

```text
+ export RUST_BACKTRACE=1
+ RUST_BACKTRACE=1
+ rust_build_options='     --target riscv32imacf-unknown-none-elf.json     -Z build-std=core '
+ pushd bl602-macros
/mnt/c/pinecone/bl602-rust-wrapper/bl602-macros /mnt/c/pinecone/bl602-rust-wrapper
+ cargo build
   Compiling proc-macro2 v1.0.27
   Compiling memchr v2.4.0
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.73
   Compiling cty v0.2.1
   Compiling rustc-serialize v0.3.24
   Compiling lazy_static v1.4.0
   Compiling cstr_core v0.2.4
   Compiling quote v1.0.9
   Compiling bl602-macros v2.0.0 (/mnt/c/pinecone/bl602-rust-wrapper/bl602-macros)
    Finished dev [unoptimized + debuginfo] target(s) in 52.87s
+ popd
/mnt/c/pinecone/bl602-rust-wrapper
+ pushd bl602-sdk
/mnt/c/pinecone/bl602-rust-wrapper/bl602-sdk /mnt/c/pinecone/bl602-rust-wrapper+ cargo rustc --target riscv32imacf-unknown-none-elf.json -Z build-std=core -- -Z unstable-options --pretty expanded
   Compiling compiler_builtins v0.1.47
   Compiling core v0.0.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling typenum v1.13.0
   Compiling version_check v0.9.3
   Compiling heapless v0.6.1
   Compiling generic-array v0.14.4
   Compiling rustc-std-workspace-core v1.99.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling byteorder v1.4.3
   Compiling stable_deref_trait v1.2.0
   Compiling cty v0.2.1
   Compiling hash32 v0.1.1
   Compiling generic-array v0.13.3
   Compiling generic-array v0.12.4
   Compiling as-slice v0.1.5
   Compiling app v0.0.1 (/mnt/c/pinecone/bl602-rust-wrapper/bl602-sdk)
    Finished dev [unoptimized + debuginfo] target(s) in 35.17s
+ cargo build --target riscv32imacf-unknown-none-elf.json -Z build-std=core
   Compiling app v0.0.1 (/mnt/c/pinecone/bl602-rust-wrapper/bl602-sdk)
    Finished dev [unoptimized + debuginfo] target(s) in 3.36s
+ cargo doc --target riscv32imacf-unknown-none-elf.json -Z build-std=core
    Checking unicode-xid v0.2.2
 Documenting unicode-xid v0.2.2
    Checking cty v0.2.1
 Documenting cty v0.2.1
    Checking lazy_static v1.4.0
    Checking rustc-serialize v0.3.24
 Documenting lazy_static v1.4.0
 Documenting rustc-serialize v0.3.24
    Checking memchr v2.4.0
 Documenting memchr v2.4.0
    Checking typenum v1.13.0
    Checking stable_deref_trait v1.2.0
    Checking byteorder v1.4.3
 Documenting typenum v1.13.0
 Documenting stable_deref_trait v1.2.0
 Documenting byteorder v1.4.3
    Checking proc-macro2 v1.0.27
    Checking cstr_core v0.2.4
 Documenting proc-macro2 v1.0.27
    Checking hash32 v0.1.1
 Documenting cstr_core v0.2.4
    Checking generic-array v0.13.3
    Checking generic-array v0.12.4
    Checking generic-array v0.14.4
 Documenting hash32 v0.1.1
    Checking quote v1.0.9
    Checking syn v1.0.73
    Checking as-slice v0.1.5
 Documenting quote v1.0.9
    Checking heapless v0.6.1
 Documenting syn v1.0.73
 Documenting bl602-macros v2.0.0 (/mnt/c/pinecone/bl602-rust-wrapper/bl602-macros)
 Documenting generic-array v0.14.4
 Documenting as-slice v0.1.5
 Documenting heapless v0.6.1
 Documenting app v0.0.1 (/mnt/c/pinecone/bl602-rust-wrapper/bl602-sdk)
    Finished dev [unoptimized + debuginfo] target(s) in 2m 07s
+ popd
/mnt/c/pinecone/bl602-rust-wrapper
```
