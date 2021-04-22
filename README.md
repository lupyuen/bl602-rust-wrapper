# Rust Wrapper for BL602 IoT SDK

Read the article...

-   ["Run Rust RISC-V Firmware with BL602 IoT SDK"](https://lupyuen.github.io/articles/rust)

This script reads the BL602 IoT SDK Header File [`bl_gpio.h`](https://github.com/lupyuen/bl_iot_sdk/blob/master/components/hal_drv/bl602_hal/bl_gpio.h)...

-   [`scripts/gen-bindings.sh`](scripts/gen-bindings.sh)

And auto-generates the Rust Bindings...

-   [`bl602-sdk/src/gpio.rs`](bl602-sdk/src/gpio.rs)

Which call the `safe_wrap` Procedural Macro...

-   [`bl602-macros/src/safe_wrap.rs`](bl602-macros/src/safe_wrap.rs)

To produce the Rust Wrappers for BL602 IoT SDK...

-   [`logs/sdk-expanded.rs` (Expansion of `safe_wrap` macro)](logs/sdk-expanded.rs)

Here's how we call the Rust Wrappers for BL602 GPIO HAL in our BL602 Rust Firmware...

-   [`bl602-sdk/src/lib.rs`](bl602-sdk/src/lib.rs)

Build the project with this script...

-   [`scripts/build.sh`](scripts/build.sh)
