# Rust Wrapper for BL602 IoT SDK

Read the article...

-   ["Run Rust RISC-V Firmware with BL602 IoT SDK"](https://lupyuen.github.io/articles/rust)

This script reads the BL602 IoT SDK Header File [`bl_gpio.h`](https://github.com/lupyuen/bl_iot_sdk/blob/master/components/hal_drv/bl602_hal/bl_gpio.h)...

-   [`scripts/gen-bindings.sh`](scripts/gen-bindings.sh)

```c
int bl_gpio_enable_output(uint8_t pin, uint8_t pullup, uint8_t pulldown);
int bl_gpio_enable_input(uint8_t pin, uint8_t pullup, uint8_t pulldown);
int bl_gpio_output_set(uint8_t pin, uint8_t value);
```

And auto-generates the Rust Bindings...

-   [`bl602-sdk/src/gpio.rs`](bl602-sdk/src/gpio.rs)

```rust
#[safe_wrap(attr)] extern "C" {
    pub fn bl_gpio_enable_output(pin: u8, pullup: u8, pulldown: u8) -> ::cty::c_int;
}
#[safe_wrap(attr)] extern "C" {
    pub fn bl_gpio_output_set(pin: u8, value: u8) -> ::cty::c_int;
}
```

Which call the `safe_wrap` Procedural Macro...

-   [`bl602-macros/src/safe_wrap.rs`](bl602-macros/src/safe_wrap.rs)

To produce the Rust Wrappers for BL602 IoT SDK...

-   [`logs/sdk-expanded.rs` (Shows expansion of `safe_wrap` macro)](logs/sdk-expanded.rs)

```rust
pub fn gpio_enable_output(pin: u8, pullup: u8, pulldown: u8)
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
pub fn gpio_output_set(pin: u8, value: u8) -> BlResult<()> {
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
gpio::gpio_enable_output(LED_GPIO, 0, 0)   //  No pullup, no pulldown
    .expect("GPIO enable output failed");  //  Halt on error

//  Blink the LED 5 times
for i in 0..10 {  //  Iterates 10 times from 0 to 9 (`..` excludes 10)

    //  Toggle the LED GPIO between 0 (on) and 1 (off)
    gpio::gpio_output_set(  //  Set the GPIO output (from BL602 GPIO HAL)
        LED_GPIO,           //  GPIO pin number
        i % 2               //  0 for low, 1 for high
    ).expect("GPIO output failed");  //  Halt on error
```

Build the project with this script...

-   [`scripts/build.sh`](scripts/build.sh)

```text
+ export RUST_BACKTRACE=1
+ RUST_BACKTRACE=1
+ rust_build_options='     --target riscv32imacf-unknown-none-elf.json     -Z build-std=core '
+ pushd bl602-macros
~/pinecone/bl602-rust-wrapper/bl602-macros ~/pinecone/bl602-rust-wrapper
+ cargo build
   Compiling proc-macro2 v1.0.26
   Compiling memchr v2.3.4
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.69
   Compiling cty v0.2.1
   Compiling lazy_static v1.4.0
   Compiling rustc-serialize v0.3.24
   Compiling cstr_core v0.2.3
   Compiling quote v1.0.9
   Compiling bl602-macros v2.0.0 (/Users/Luppy/pinecone/bl602-rust-wrapper/bl602-macros)
    Finished dev [unoptimized + debuginfo] target(s) in 21.92s
+ popd
~/pinecone/bl602-rust-wrapper
+ pushd bl602-sdk
~/pinecone/bl602-rust-wrapper/bl602-sdk ~/pinecone/bl602-rust-wrapper
+ cargo rustc --target riscv32imacf-unknown-none-elf.json -Z build-std=core -- -Z unstable-options --pretty expanded
    Updating crates.io index
   Compiling compiler_builtins v0.1.39
   Compiling core v0.0.0 (/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core)
   Compiling typenum v1.13.0
   Compiling version_check v0.9.3
   Compiling heapless v0.6.1
   Compiling generic-array v0.14.4
   Compiling rustc-std-workspace-core v1.99.0 (/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling stable_deref_trait v1.2.0
   Compiling byteorder v1.4.3
   Compiling cty v0.2.1
   Compiling hash32 v0.1.1
   Compiling generic-array v0.12.4
   Compiling generic-array v0.13.3
   Compiling as-slice v0.1.5
   Compiling app v0.0.1 (/Users/Luppy/pinecone/bl602-rust-wrapper/bl602-sdk)
warning: type `gpio_ctx_t` should have an upper camel case name
  --> bl602-sdk/src/gpio.rs:21:10
   |
21 | pub type gpio_ctx_t = _gpio_ctx_desc;
   |          ^^^^^^^^^^ help: convert the identifier to upper camel case: `GpioCtxT`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 25.47s
+ cargo build --target riscv32imacf-unknown-none-elf.json -Z build-std=core
    Updating crates.io index
   Compiling app v0.0.1 (/Users/Luppy/pinecone/bl602-rust-wrapper/bl602-sdk)
warning: type `gpio_ctx_t` should have an upper camel case name
  --> bl602-sdk/src/gpio.rs:21:10
   |
21 | pub type gpio_ctx_t = _gpio_ctx_desc;
   |          ^^^^^^^^^^ help: convert the identifier to upper camel case: `GpioCtxT`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: structure field `gpioPin` should have a snake case name
  --> bl602-sdk/src/gpio.rs:12:9
   |
12 |     pub gpioPin: u8,
   |         ^^^^^^^ help: convert the identifier to snake case: `gpio_pin`
   |
   = note: `#[warn(non_snake_case)]` on by default

warning: structure field `intCtrlMod` should have a snake case name
  --> bl602-sdk/src/gpio.rs:13:9
   |
13 |     pub intCtrlMod: u8,
   |         ^^^^^^^^^^ help: convert the identifier to snake case: `int_ctrl_mod`

warning: structure field `intTrgMod` should have a snake case name
  --> bl602-sdk/src/gpio.rs:14:9
   |
14 |     pub intTrgMod: u8,
   |         ^^^^^^^^^ help: convert the identifier to snake case: `int_trg_mod`

warning: 4 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 3.80s
+ popd
~/pinecone/bl602-rust-wrapper
```