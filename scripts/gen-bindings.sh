#!/usr/bin/env bash
#  Generate Rust bindings for BL602 IoT SDK. Install "bindgen" before running:
#  cargo install bindgen
#  Also install rustfmt when prompted
#  Ignore any "unused option" errors
#    unused option: --whitelist-function (?i)...
#    unused option: --whitelist-var (?i)...
#    unused option: --whitelist-type (?i)...
#  TODO: Remove derive[Debug]

set -e  #  Exit when any command fails.
set -x  #  Echo all commands.
export RUST_BACKTRACE=1  #  Show Rust errors.

#  Define the library name
libname=bl602
headerprefix=HEADERPREFIX 
# Previously pinetime_lvgl_mynewt

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
    local whitelist="$@" # Whitelist Options: --raw-line, --blacklist-item, --whitelist-function, --whitelist-type, --whitelist-var
    echo "whitelist=$whitelist"

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
        $whitelist \
        -o $tmpexpandpath \
        $headerfile \
        -- \
        -Ibaselibc/include/ \
        $CCFLAGS \
        $CCFLAGS_BINDGEN

    # Change extern "C"
    # to     #[safe_wrap(_)] extern "C"
    # Change #[doc = " @param dev The device to open"]
    # to     #[doc = " - __`dev`__: The device to open"]
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
        | sed 's/@param \([^ ][^ ]*\) /- __`\1`__: /' \
        | sed 's/@return /Return: /' \
        | sed 's/@code{.c}/```c/' \
        | sed 's/@code/```/' \
        | sed 's/@endcode/```/' \
        | sed 's/@note/__Note:__/' \
        | sed 's/\(pub const LV_ALIGN_[^ ][^ ]*\): _[^ ]* /\1: lv_align_t /' \
        >$expandpath
    ####rm $tmpexpandpath
}

function generate_bindings_core() {
    #  Add whitelist and blacklist
    local modname=MODNAME
    local submodname=$1  # Submodule name e.g. gpio
    if [ "$submodname" == 'style' ]; then
        #  TODO: Combine lv_style.h, lv_obj.h and lv_obj_style_dec.h for processing, because lv_obj_style_dec.h contains macros that define functions like "lv_style_set_text_font"
        local headerfile=$headerprefix/src/lv_$modname/combined.h
        cat \
            $headerprefix/src/lv_$modname/lv_style.h \
            $headerprefix/src/lv_$modname/lv_obj.h \
            $headerprefix/src/lv_$modname/lv_obj_style_dec.h \
            >$headerfile
    else
        # components/hal_drv/bl602_hal/bl_gpio.h
        # TODO: local headerfile=$headerprefix/src/lv_$modname/lv_$submodname.h
        local headerfile=../bl_iot_sdk/components/hal_drv/bl602_hal/bl_$submodname.h
    fi
    local whitelistname=bl_$submodname # e.g. bl_gpio
    # Previously lv_$submodname
    if [ "$submodname" == 'obj' ]; then
        # For lv_obj.c, include the core constants and the core types
        local whitelisttypes=`cat << EOF
        --whitelist-var      LV_ALIGN_.* \
        --whitelist-type     lv_.*
EOF
`
        local blacklist=
    else
        # For files other than lv_obj.c, exclude the core types.
        # lv_indev_drv_* functions should be defined under lv_hal. 
        local whitelisttypes=
        local blacklist=`cat << EOF
            --blacklist-item     _lv_.* \
            --blacklist-item     lv_.*_t \
            --blacklist-item     lv_indev_drv_init \
            --blacklist-item     lv_indev_get_next
EOF
`
    fi
    #  TODO: Fix returned string lifetime for lv_obj_get_style_value_str.
    #  This function is probably not essential because our Rust app should already have the string.
    local whitelist=`cat << EOF
        --raw-line use \
        --raw-line super::*; \
        --whitelist-function (?i)${whitelistname}.* \
        --whitelist-type     (?i)${whitelistname}.* \
        --whitelist-var      (?i)${whitelistname}.* \
        ${whitelisttypes} \
        --blacklist-item     lv_obj_get_style_value_str \
        ${blacklist}
EOF
`    
    #  Generate the bindings for lv_core/lv_obj: libname, modname, submodname, headerfile, whitelist
    generate_bindings $libname $modname $submodname $headerfile $whitelist

    #  Delete the combined lv_style.h and lv_obj_style_dec.h
    if [ "$submodname" == 'style' ]; then
        rm $headerfile
    fi
}

#  Generate bindings for
#  components/hal_drv/bl602_hal/bl_gpio.h
generate_bindings_core gpio

#  Generate bindings for
#  components/hal_drv/bl602_hal/bl_pwm.h
generate_bindings_core pwm

#  Expand the safe wrapper macros
##cargo rustc -- -Z unstable-options --pretty expanded >logs/expanded.rs

#  Build the bindings
##cargo build

#  Generate the doc for inspection
##cargo doc

exit

→ bindgen --help
bindgen 0.58.1
Generates Rust bindings from C/C++ headers.

USAGE:
    bindgen [FLAGS] [OPTIONS] <header> -- <clang-args>...

FLAGS:
        --block-extern-crate                     Use extern crate instead of use for block.
        --builtins                               Output bindings for builtin definitions, e.g. __builtin_va_list.
        --conservative-inline-namespaces         Conservatively generate inline namespaces to avoid name conflicts.
        --disable-header-comment
            Suppress insertion of bindgen's version identifier into generated bindings.

        --disable-name-namespacing
            Disable namespacing via mangling, causing bindgen to generate names like "Baz" instead of "foo_bar_Baz" for
            an input name "foo::bar::Baz".
        --disable-nested-struct-naming
            Disable nested struct naming, causing bindgen to generate names like "bar" instead of "foo_bar" for a nested
            definition "struct foo { struct bar { } b; };".
        --disable-untagged-union                 Disable support for native Rust unions.
        --distrust-clang-mangling                Do not trust the libclang-provided mangling
        --dump-preprocessed-input
            Preprocess and dump the input header files to disk. Useful when debugging bindgen, using C-Reduce, or when
            filing issues. The resulting file will be named something like `__bindgen.i` or `__bindgen.ii`.
        --dynamic-link-require-all               Require successful linkage to all functions in the library.
        --emit-clang-ast                         Output the Clang AST for debugging purposes.
        --emit-ir                                Output our internal IR for debugging purposes.
        --enable-cxx-namespaces                  Enable support for C++ namespaces.
        --enable-function-attribute-detection
            Enables detecting unexposed attributes in functions (slow).
                                   Used to generate #[must_use] annotations.
        --fit-macro-constant-types               Try to fit macro constants into types smaller than u32/i32
        --generate-block                         Generate block signatures instead of void pointers.
        --generate-inline-functions              Generate inline functions.
    -h, --help                                   Prints help information
        --ignore-functions
            Do not generate bindings for functions or methods. This is useful when you only care about struct layouts.

        --ignore-methods                         Do not generate bindings for methods.
        --impl-debug                             Create Debug implementation, if it can not be derived automatically.
        --impl-partialeq
            Create PartialEq implementation, if it can not be derived automatically.

        --no-convert-floats                      Do not automatically convert floats to f32/f64.
        --no-derive-copy                         Avoid deriving Copy on any type.
        --no-derive-debug                        Avoid deriving Debug on any type.
        --no-doc-comments
            Avoid including doc comments in the output, see: https://github.com/rust-lang/rust-bindgen/issues/426

        --no-include-path-detection              Do not try to detect default include paths
        --no-layout-tests                        Avoid generating layout tests for any type.
        --no-prepend-enum-name                   Do not prepend the enum name to constant or newtype variants.
        --no-record-matches
            Do not record matching items in the regex sets. This disables reporting of unused items.

        --no-recursive-allowlist
            Disable allowlisting types recursively. This will cause bindgen to emit Rust code that won't compile! See
            the `bindgen::Builder::allowlist_recursively` method's documentation for details.
        --no-rustfmt-bindings                    Do not format the generated bindings with rustfmt.
        --objc-extern-crate                      Use extern crate instead of use for objc.
        --respect-cxx-access-specs
            Makes generated bindings `pub` only for items if the items are publically accessible in C++.

        --rustfmt-bindings
            Format the generated bindings with rustfmt. DEPRECATED: --rustfmt-bindings is now enabled by default.
            Disable with --no-rustfmt-bindings.
        --size_t-is-usize                        Translate size_t to usize.
        --time-phases                            Time the different bindgen phases and print to stderr
        --translate-enum-integer-types           Always translate enum integer types to native Rust integer types.
        --unstable-rust                          Generate unstable Rust code (deprecated; use --rust-target instead).
        --use-array-pointers-in-arguments        Use `*const [T; size]` instead of `*const T` for C arrays
        --use-core                               Use types from Rust core instead of std.
        --use-msvc-mangling                      MSVC C++ ABI mangling. DEPRECATED: Has no effect.
    -V, --version                                Prints version information
        --verbose                                Print verbose error messages.
        --with-derive-default                    Derive Default on any type.
        --with-derive-eq
            Derive eq on any type. Enable this option also enables --with-derive-partialeq

        --with-derive-hash                       Derive hash on any type.
        --with-derive-ord
            Derive ord on any type. Enable this option also enables --with-derive-partialord

        --with-derive-partialeq                  Derive partialeq on any type.
        --with-derive-partialord                 Derive partialord on any type.

OPTIONS:
        --allowlist-function <regex>...
            Allowlist all the free-standing functions matching <regex>. Other non-allowlisted functions will not be
            generated.
        --allowlist-type <regex>...
            Only generate types matching <regex>. Other non-allowlisted types will not be generated.

        --allowlist-var <regex>...
            Allowlist all the free-standing variables matching <regex>. Other non-allowlisted variables will not be
            generated.
        --anon-fields-prefix <prefix>
            Use the given prefix for the anon fields. [default: __bindgen_anon_]

        --bitfield-enum <regex>...
            Mark any enum whose name matches <regex> as a set of bitfield flags.

        --blocklist-function <function>...            Mark <function> as hidden.
        --blocklist-item <item>...                    Mark <item> as hidden.
        --blocklist-type <type>...                    Mark <type> as hidden.
        --constified-enum <regex>...                  Mark any enum whose name matches <regex> as a series of constants.
        --constified-enum-module <regex>...           Mark any enum whose name matches <regex> as a module of constants.
        --ctypes-prefix <prefix>                      Use the given prefix before raw types instead of ::std::os::raw.
        --default-alias-style <variant>
            The default style of code used to generate typedefs. [default: type_alias]  [possible values: type_alias,
            new_type, new_type_deref]
        --default-enum-style <variant>
            The default style of code used to generate enums. [default: consts]  [possible values: consts, moduleconsts,
            bitfield, newtype, rust, rust_non_exhaustive]
        --default-macro-constant-type <variant>
            The default signed/unsigned type for C macro constants. [default: unsigned]  [possible values: signed,
            unsigned]
        --dynamic-loading <dynamic-loading>           Use dynamic loading mode with the given library name.
        --emit-ir-graphviz <path>                     Dump graphviz dot file.
        --generate <generate>
            Generate only given items, split by commas. Valid values are "functions","types", "vars", "methods",
            "constructors" and "destructors".
        --module-raw-line <module-name> <raw-line>    Add a raw line of Rust code to a given module.
        --new-type-alias <regex>...
            Mark any typedef alias whose name matches <regex> to have a new type generated for it.

        --new-type-alias-deref <regex>...
            Mark any typedef alias whose name matches <regex> to have a new type with Deref and DerefMut to the inner
            type.
        --newtype-enum <regex>...                     Mark any enum whose name matches <regex> as a newtype.
        --no-copy <regex>...                          Avoid deriving Copy for types matching <regex>.
        --no-debug <regex>...                         Avoid deriving Debug for types matching <regex>.
        --no-default <regex>...                       Avoid deriving/implement Default for types matching <regex>.
        --no-hash <regex>...                          Avoid deriving Hash for types matching <regex>.
        --no-partialeq <regex>...                     Avoid deriving PartialEq for types matching <regex>.
        --normal-alias <regex>...
            Mark any typedef alias whose name matches <regex> to use normal type aliasing.

        --opaque-type <type>...                       Mark <type> as opaque.
    -o, --output <output>                             Write Rust bindings to <output>.
        --raw-line <raw-line>...                      Add a raw line of Rust code at the beginning of output.
        --rust-target <rust-target>
            Version of the Rust compiler to target. Valid options are: ["1.0", "1.1", "1.19", "1.20", "1.21", "1.25",
            "1.26", "1.27", "1.28", "1.30", "1.33", "1.36", "1.40", "nightly"]. Defaults to "1.40".
        --rustfmt-configuration-file <path>
            The absolute path to the rustfmt configuration file. The configuration file will be used for formatting the
            bindings. This parameter is incompatible with --no-rustfmt-bindings.
        --rustified-enum <regex>...                   Mark any enum whose name matches <regex> as a Rust enum.
        --wasm-import-module-name <name>
            The name to be used in a #[link(wasm_import_module = ...)] statement


ARGS:
    <header>           C or C++ header file
    <clang-args>...    
