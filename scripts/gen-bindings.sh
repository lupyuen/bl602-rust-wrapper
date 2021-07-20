#!/usr/bin/env bash
#  Generate Rust bindings for BL602 IoT SDK. Install "bindgen" before running:
#    cargo install bindgen
#    sudo apt install llvm-dev libclang-dev clang
#  Also install rustfmt when prompted
#  Ignore any "unused option" errors
#    unused option: --allowlist-function (?i)...
#    unused option: --allowlist-var (?i)...
#    unused option: --allowlist-type (?i)...
#  TODO: Remove derive[Debug]

set -e  #  Exit when any command fails.
set -x  #  Echo all commands.
export RUST_BACKTRACE=1  #  Show Rust errors.

#  Define the library name
libname=bl602
headerprefix=HEADERPREFIX 

#  TODO: Sync gcc options with make.log
CCFLAGS=" -g \
    -DconfigUSE_TICKLESS_IDLE=0 \
    -DFEATURE_WIFI_DISABLE=1 \
    -D BL_SDK_VER=\"0\" \
    -D BL_SDK_PHY_VER=\"0\" \
    -D BL_SDK_RF_VER=\"0\" \
    -D BL_CHIP_NAME=\"BL602\" \
    -MMD \
    -MP \
    -D BL_SDK_VER=\"0\" \
    -D BL_SDK_PHY_VER=\"0\" \
    -D BL_SDK_RF_VER=\"0\"  \
    -DARCH_RISCV \
    -DCONFIG_PSM_EASYFLASH_SIZE=16384 \
    -DconfigUSE_TICKLESS_IDLE=0 \
    -DFEATURE_WIFI_DISABLE=1 \
    -DARCH_RISCV \
    -DBFLB_CRYPT_HARDWARE \
    -DBFLB_PKA_HARDWARE \
    -DSTDDRV_VERSION=096d971a96c12b5857abc7606bfd5ac1bf371a41 \
    -DBL602_USE_HAL_DRIVER \
    -DCFG_COMPONENT_BLOG_ENABLE=0 \
    -D __FILENAME__=\"bl602_dma.c\" \
    -D __FILENAME_WO_SUFFIX__=\"bl602_dma\" \
    -D __FILENAME_WO_SUFFIX_DEQUOTED__=bl602_dma \
    -D __COMPONENT_NAME__=\"bl602_std\" \
    -D __COMPONENT_NAME_DEQUOTED__=bl602_std \
    -D __COMPONENT_FILE_NAME__=\"bl602_stdbl602_dma\" \
    -D__COMPONENT_FILE_NAMED__=bl602_std.bl602_dma \
    -D__COMPONENT_FILE_NAME_DEQUOTED__=bl602_stdbl602_dma \
    -I ../bl_iot_sdk/components/bl602/bl602_std \
    -I ../bl_iot_sdk/components/bl602/bl602_std/include \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/StdDriver/Inc \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/Device/Bouffalo/BL602/Peripherals \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/RISCV/Device/Bouffalo/BL602/Startup \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/RISCV/Core/Include \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/Include \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/Common/platform_print \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/Common/soft_crc \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/Common/partition \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/Common/xz \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/Common/cipher_suite/inc \
    -I ../bl_iot_sdk/components/bl602/bl602_std/bl602_std/Common/ring_buffer \
    -I ../bl_iot_sdk/components/bl602/bl602 \
    -I ../bl_iot_sdk/components/bl602/bl602/include \
    -I ../bl_iot_sdk/components/stage/blfdt \
    -I ../bl_iot_sdk/components/stage/blfdt/include \
    -I ../bl_iot_sdk/components/stage/blfdt/inc \
    -I ../bl_iot_sdk/components/sys/blmtd \
    -I ../bl_iot_sdk/components/sys/blmtd/include \
    -I ../bl_iot_sdk/components/sys/blmtd/include \
    -I ../bl_iot_sdk/components/stage/blog \
    -I ../bl_iot_sdk/components/stage/blog/include \
    -I ../bl_iot_sdk/components/stage/blog \
    -I ../bl_iot_sdk/components/stage/blog_testc \
    -I ../bl_iot_sdk/components/stage/blog_testc/include \
    -I ../bl_iot_sdk/components/stage/blog_testc \
    -I ../bl_iot_sdk/components/sys/bloop/bloop \
    -I ../bl_iot_sdk/components/sys/bloop/bloop/include \
    -I ../bl_iot_sdk/components/sys/bloop/bloop/include \
    -I ../bl_iot_sdk/components/sys/bltime \
    -I ../bl_iot_sdk/components/sys/bltime/include \
    -I ../bl_iot_sdk/components/sys/bltime/include \
    -I ../bl_iot_sdk/components/stage/cli \
    -I ../bl_iot_sdk/components/stage/cli/include \
    -I ../bl_iot_sdk/components/stage/cli/cli/include \
    -I ../bl_iot_sdk/components/stage/easyflash4 \
    -I ../bl_iot_sdk/components/stage/easyflash4/include \
    -I ../bl_iot_sdk/components/stage/easyflash4/inc \
    -I ../bl_iot_sdk/components/bl602/freertos_riscv_ram \
    -I ../bl_iot_sdk/components/bl602/freertos_riscv_ram/include \
    -I ../bl_iot_sdk/components/bl602/freertos_riscv_ram/config \
    -I ../bl_iot_sdk/components/bl602/freertos_riscv_ram/portable/GCC/RISC-V \
    -I ../bl_iot_sdk/components/bl602/freertos_riscv_ram/portable/GCC/RISC-V/chip_specific_extensions/RV32F_float_abi_single \
    -I ../bl_iot_sdk/components/bl602/freertos_riscv_ram/panic \
    -I ../bl_iot_sdk/components/hal_drv \
    -I ../bl_iot_sdk/components/hal_drv/include \
    -I ../bl_iot_sdk/components/hal_drv/bl602_hal \
    -I ../bl_iot_sdk/components/sys/bloop/looprt \
    -I ../bl_iot_sdk/components/sys/bloop/looprt/include \
    -I ../bl_iot_sdk/components/sys/bloop/loopset \
    -I ../bl_iot_sdk/components/sys/bloop/loopset/include \
    -I ../bl_iot_sdk/components/3rdparty/lora-sx1276/include \
    -I ../bl_iot_sdk/components/3rdparty/lorawan/include \
    -I ../bl_iot_sdk/components/network/lwip \
    -I ../bl_iot_sdk/components/network/lwip/include \
    -I ../bl_iot_sdk/components/network/lwip/src/include \
    -I ../bl_iot_sdk/components/network/lwip/lwip-port \
    -I ../bl_iot_sdk/components/network/lwip/lwip-port/config \
    -I ../bl_iot_sdk/components/network/lwip/lwip-port/FreeRTOS \
    -I ../bl_iot_sdk/components/network/lwip/lwip-port/arch \
    -I ../bl_iot_sdk/components/3rdparty/nimble-porting-layer/include \
    -I ../bl_iot_sdk/components/fs/romfs \
    -I ../bl_iot_sdk/components/fs/romfs/include \
    -I ../bl_iot_sdk/components/utils \
    -I ../bl_iot_sdk/components/utils/include \
    -I ../bl_iot_sdk/components/fs/vfs \
    -I ../bl_iot_sdk/components/fs/vfs/include \
    -I ../bl_iot_sdk/components/fs/vfs/posix/include \
    -I ../bl_iot_sdk/components/stage/yloop \
    -I ../bl_iot_sdk/components/stage/yloop/include \
    -I ../bl_iot_sdk/components/stage/yloop/include \
    "

#  gcc options for bindgen only. We disable "static" and "inline" so that wrappers will be generated for static inline functions like "lv_style_set_text_font"
CCFLAGS_BINDGEN=" -D static= -D inline= "

# bindgen \
#     --verbose \
#     --use-core \
#     --ctypes-prefix "::cty" \
#     --with-derive-default \
#     --no-derive-debug \
#     --no-layout-tests \
#     -o a.rs \
#     ../bl_iot_sdk/components/hal_drv/bl602_hal/bl_gpio.h \
#     -- \
#     -Ibaselibc/include/ \
#     $CCFLAGS \
#     $CCFLAGS_BINDGEN
# exit

function generate_bindings() {
    #  Generate bindings for the module.
    local libname=$1     # Library name e.g. lvgl
    local modname=$2     # Module name e.g. core
    local submodname=$3  # Submodule name e.g. obj
    local headerfile=$4  # Header file e.g. components/hal_drv/bl602_hal/bl_gpio.h
    # Previously pinetime_lvgl_mynewt/src/lv_core/lv_obj.h
    shift 4
    local allowlist="$@" # Allowlist Options: --raw-line, --blocklist-item, --allowlist-function, --allowlist-type, --allowlist-var
    echo "allowlist=$allowlist"

    local expandpath=bl602-sdk/src/$modname_$submodname.rs
    # Previously src/$modname/$submodname.rs
    local tmpexpandpath=bl602-sdk/src/$modname_$submodname.tmp
    # Previously src/$modname/$submodname.tmp

    #  Generate Rust bindings
    #  TODO: Ensure that output folder has been created
    bindgen \
        --verbose \
        --use-core \
        --ctypes-prefix "::cty" \
        --with-derive-default \
        --no-derive-debug \
        --no-layout-tests \
        $allowlist \
        -o $tmpexpandpath \
        $headerfile \
        -- \
        -Ibaselibc/include/ \
        $CCFLAGS \
        $CCFLAGS_BINDGEN

    # Change extern "C"
    # to     #[safe_wrap(_)] extern "C"
    # Change #[doc = " @param[in]  spi      the spi device"]
    # to     #[doc = " - __`spi`__: (in) the spi device"]
    # Change @return to Return
    # Change @code{.c} to ```c
    # Change @code{...} to ```
    # Change @endcode to ```
    # Change @note to __Note:__
    # Change pub const LV_ALIGN_CENTER: _bindgen_ty_3 = 0;
    # to     pub const LV_ALIGN_CENTER: lv_align_t = 0;
    # Change pub const LV_LABEL_ALIGN_CENTER: _bindgen_ty_33 = 1;
    # to     pub const LV_LABEL_ALIGN_CENTER: lv_label_align_t = 1;
    # Change pub const LV_LABEL_LONG_BREAK: _bindgen_ty_32 = 1;
    # to     pub const LV_LABEL_LONG_BREAK: lv_label_long_mode_t = 1;
    cat $tmpexpandpath \
        | sed 's/^extern "C" /#[safe_wrap(_)] extern "C" /' \
        | sed 's/@param\[\(.*\)\][ ][ ]*\([^ ][^ ]*\) /- __`\2`__: \(\1\) /' \
        | sed 's/@return /Return: /' \
        | sed 's/@code{.c}/```c/' \
        | sed 's/@code/```/' \
        | sed 's/@endcode/```/' \
        | sed 's/@note/__Note:__/' \
        | sed 's/\(pub const LV_ALIGN_[^ ][^ ]*\): _[^ ]* /\1: lv_align_t /' \
        >$expandpath
    rm $tmpexpandpath
}

#  Generate bindings for BL602 HAL e.g.
#  components/hal_drv/bl602_hal/hal_spi.h
#  components/hal_drv/bl602_hal/bl_gpio.h
function generate_bindings_core() {
    #  Add allowlist and blocklist
    local modname=$1     # Module name: bl or hal
    local submodname=$2  # Submodule name e.g. gpio

    #  Header file to be processed e.g. components/hal_drv/bl602_hal/bl_gpio.h
    local headerfile=../bl_iot_sdk/components/hal_drv/bl602_hal/${modname}_${submodname}.h

    #  For SPI, combine the header files for processing
    if [ "$submodname" == 'spi' ]; then
        local headerfile=/tmp/gen-bindings-${submodname}.h
        echo "#include <stdint.h>" >$headerfile
        cat \
            ../bl_iot_sdk/components/fs/vfs/include/hal/soc/spi.h \
            ../bl_iot_sdk/components/hal_drv/bl602_hal/hal_spi.h \
            >>$headerfile
    fi

    #  For ADC, insert stdint into the header file
    if [ "$submodname" == 'adc' ]; then
        local headerfile=/tmp/gen-bindings-${submodname}.h
        echo "#include <stdint.h>" >$headerfile
        cat \
            ../bl_iot_sdk/components/hal_drv/bl602_hal/bl_adc.h \
            >>$headerfile
    fi

    #  Define allowlist by submodule e.g. gpio, i2c
    local allowlistname=$submodname
    if [ "$submodname" == 'obj' ]; then
        # TODO: For lv_obj.c, include the core constants and the core types
        local allowlisttypes=`cat << EOF
        --allowlist-var      LV_ALIGN_.* \
        --allowlist-type     lv_.*
EOF
`
        local blocklist=
    else
        # TODO: For files other than lv_obj.c, exclude the core types.
        # lv_indev_drv_* functions should be defined under lv_hal. 
        local allowlisttypes=
        local blocklist=`cat << EOF
            --blocklist-item     _lv_.* \
            --blocklist-item     lv_.*_t \
            --blocklist-item     lv_indev_drv_init \
            --blocklist-item     lv_indev_get_next
EOF
`
    fi
    # For submodname=i2c or gpio or spi, allowlist i2c*, bl_gpio* and hal_spi*
    local allowlist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --allowlist-function (?i)${allowlistname}.* \
        --allowlist-type     (?i)${allowlistname}.* \
        --allowlist-var      (?i)${allowlistname}.* \
        --allowlist-function (?i)bl_${allowlistname}.* \
        --allowlist-type     (?i)bl_${allowlistname}.* \
        --allowlist-var      (?i)bl_${allowlistname}.* \
        --allowlist-function (?i)hal_${allowlistname}.* \
        --allowlist-type     (?i)hal_${allowlistname}.* \
        --allowlist-var      (?i)hal_${allowlistname}.* \
        ${allowlisttypes} \
        --blocklist-item     lv_obj_get_style_value_str \
        ${blocklist}
EOF
`    
    #  TODO: Generate the bindings for lv_core/lv_obj: libname, modname, submodname, headerfile, allowlist
    generate_bindings $libname $modname $submodname $headerfile $allowlist

    #  TODO: Delete the combined lv_style.h and lv_obj_style_dec.h
    if [ "$submodname" == 'style' ]; then
        rm $headerfile
    fi
}

#  Generate bindings for ADC
#  components/hal_drv/bl602_hal/bl_adc.h
generate_bindings_core bl adc

#  Generate bindings for GPIO
#  components/hal_drv/bl602_hal/bl_gpio.h
generate_bindings_core bl gpio

#  Generate bindings for I2C
#  components/hal_drv/bl602_hal/bl_i2c.h
generate_bindings_core bl i2c

#  Generate bindings for PWM
#  components/hal_drv/bl602_hal/bl_pwm.h
generate_bindings_core bl pwm

#  Generate bindings for SPI
#  components/hal_drv/bl602_hal/hal_spi.h
#  components/fs/vfs/include/hal/soc/spi.h
generate_bindings_core hal spi

#  Generate bindings for UART
#  components/hal_drv/bl602_hal/bl_uart.h
generate_bindings_core bl uart

#  Generate bindings for WiFi
#  components/hal_drv/bl602_hal/hal_wifi.h
#  components/bl602/bl602_wifidrv/bl60x_wifi_driver/wifi_mgmr.h
#  components/bl602/bl602_wifidrv/bl60x_wifi_driver/wifi_mgmr_ext.h
generate_bindings_core hal wifi
