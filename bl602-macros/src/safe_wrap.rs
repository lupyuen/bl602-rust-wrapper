/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
//! Macro that creates a safe wrapper for C APIs
extern crate proc_macro;
use std::{
    collections::HashMap,
    fs::File, 
    io::{BufReader, BufRead}
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input,
    Expr,
    FnArg,
    ForeignItem,
    ForeignItemFn,
    Ident,
    ItemForeignMod,
    LitStr,
    PatType,
    ReturnType,
    Type,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma},
};

/// Given a function name like `bl_gpio_output_set`, return true if we should create the wrapper
fn function_is_allowlisted(fname: &str) -> bool {
    //  Match allowlisted (true) and blocklisted (false) functions by name
    match fname {
        "hal_spi_init" => false,
        "spi_init" => true,
        _ => { 
            //  Functions starting with `bl_`, `hal_`, `i2c_`, `pwm_` and `wifi_` are allowlisted.
            if fname.starts_with("bl_")
                || fname.starts_with("hal_")
                || fname.starts_with("i2c_")    
                || fname.starts_with("pwm_")    
                || fname.starts_with("wifi_")    
                { true }
            else { false }
        }
    }
}

/// Given a function name like `bl_gpio_output_set`, return the namespace (`bl_gpio`)
fn get_namespace(fname: &str) -> String {
    //  Handle `bl_` and `hal_` prefixes
    if fname.starts_with("bl_") || fname.starts_with("hal_") { return get_namespace_hal(fname); }
    //  Get the first part before `_`.
    let fname_split: Vec<&str> = fname.splitn(2, "_").collect();
    let namespace = fname_split[0];
    //  TODO: Match the namespace and ignore if it's not a known namespace.
    match namespace {
        "do"     => { "".to_string() }   //  `do` is not a valid namespace e.g. `do_server_post()`
        "sensor" => {
            //  TODO: If it matches `sensor_network`, return `sensor_network`.
            if fname.starts_with("sensor_network_") { "sensor_network".to_string() }
            else { "sensor".to_string() }
        }
        _ => { namespace.to_string() }  //  Else it's a valid namspace
    }
}

/// Given a HAL function name like `bl_gpio_output_set`, return the namespace (`bl_gpio`). Used to strip namespace from function names, leaving `create` as the stripped name.
fn get_namespace_hal(fname: &str) -> String {
    //  println!("get_namespace {}", fname);
    //  Get the first 2 parts between `_`.
    let fname_split: Vec<&str> = fname.splitn(3, "_").collect();
    if fname_split.len() < 3 { return "".to_string(); }  //  Not a valid namespace if doesn't follow pattern like `bl_gpio`
    let namespace1 = fname_split[0];
    let namespace2 = fname_split[1];
    //  Match the namespace and ignore if it's not a known namespace.
    match namespace1 {
        "bl"  => format!("{}_{}", namespace1, namespace2),  //  If function is `bl_namespace2_...`, return namespace `bl_namespace2`
        "hal" => format!("{}_{}", namespace1, namespace2),  //  If function is `hal_namespace2_...`, return namespace `hal_namespace2`
        _     => "".to_string()                             //  Otherwise not a valid namspace
    }
}

/// Given an `extern "C"` block of function declarations, generate the safe wrapper for the function.
pub fn safe_wrap_internal(_attr: TokenStream, item: TokenStream) -> TokenStream {    
    //  Read the links that will be embedded into the documentation
    read_doclinks();

    //  println!("attr: {:#?}", attr);
    //  println!("item: {:#?}", item);
    //  Parse the macro input as an extern "C" function declaration.
    let input = parse_macro_input!(item as ItemForeignMod);
    //  println!("input: {:#?}", input);
    //  For each `ForeignItem` function or variable declaration...
    for foreign_item in input.items {  //  `foreign_item` has type `ForeignItem`
        match foreign_item {
            ForeignItem::Fn(foreign_fn) => {
                //  Generate the safe wrapper tokens for the extern function.
                let expanded = wrap_function(&foreign_fn);
                //  Return the expanded tokens back to the compiler.
                //  println!("expanded: {:#?}", expanded);
                return TokenStream::from(expanded)  //  TODO: Handle multiple functions.
            }
            ForeignItem::Static(foreign_static) => {
                return quote! { extern "C" { #foreign_static } }.into()
            }
            _ => {
                println!("safe_wrap_internal: Unknown extern {:#?}", quote! { #foreign_item }.to_string());
                assert!(false, "Unknown extern");  //  TODO: Handle unknown extern.
            }
        }        
        break;
    }
    println!("safe_wrap_internal: Loop should not terminate");
    assert!(false, "Non-function extern");  //  TODO: Handle non-function externs.
    "// Should not come here".parse().unwrap()
}

/// Return the safe wrapper tokens for the extern function
pub fn wrap_function(foreign_fn: &ForeignItemFn) -> proc_macro2::TokenStream {
    //  println!("foreign_fn: {:#?}", foreign_fn);
    let sig = &foreign_fn.sig;  //  Get the function signature
    //  Contains `#[doc] ... pub fn ...`
    let foreign_item_tokens = quote! { #foreign_fn };
    //  println!("foreign_item_tokens: {:#?}", foreign_item_tokens.to_string());

    //  Get the function name, with and without namespace (`os_task_init` vs `task_init`)
    let transformed_fname = transform_function_name(&sig.ident);
    let TransformedFunctionName{ 
        ident: fname,
        token: fname_token, 
        without_namespace_token: fname_without_namespace_token, 
        doc_tokens,
        .. 
    } = transformed_fname.clone();

    //  If function name is not allowlisted, return the extern tokens without wrapping.
    if !function_is_allowlisted(&fname) { 
        return quote! { extern "C" { #foreign_item_tokens } }
    }

    //  Move the `#[doc]` attributes out from the extern and into the top level.
    //  TODO: Accumulate doc in attrs and rename args.
    let attrs = &foreign_fn.attrs;
    let mut doc_tokens = doc_tokens;
    for attr in attrs {
        //  println!("attr: {:#?}", quote! { #attr }.to_string());
        let attr_span = attr.span();
        let tokens = quote_spanned!(attr_span => #attr);
        doc_tokens.extend(tokens);
    }

    //  Get the extern declaration without the doc attributes.
    //  let extern_decl = foreign_fn.decl;
    //  let extern_decl_span = &foreign_fn.span();
    //  let extern_decl_tokens = quote_spanned!(extern_decl_span => #extern_decl);

    //  Transform the return type.
    let transformed_ret = transform_return_type(&transformed_fname, &sig.output);
    let TransformedReturnType{ 
        declare_result_tokens, get_result_tokens, return_result_tokens, .. 
    } = transformed_ret;

    //  Get the function args and transform each arg into 3 forms:
    //  (1) Wrap Declaration: How the arg type is exposed via the wrapper
    //  (2) Validation Stmt: To validate each arg if needed, e.g. check strings are null-terminated
    //  (3) Call Expr: Inside the wrapper, call the C API with type casting
    let args = &sig.inputs;
    let transformed_args = transform_arg_list(args);

    //  For all args, collect the tokens for the Wrap, Validation and Call forms.
    let wrap_tokens = collect_wrap(&transformed_args);
    let validation_tokens = collect_validation(&transformed_args);
    let call_tokens = collect_call(&transformed_args);

    //  Compose the wrapper code as tokens.
    let expanded = quote! {
        //  "----------Doc----------";
        #doc_tokens
        //  "----------Func Decl----------";
        pub fn #fname_without_namespace_token(
            //  "----------Wrapped Decl----------";
            #wrap_tokens
            /* Like this:
                t: Out<os_task>,  //  Previously: *mut os_task
                name: &Strn,      //  Previously: *const ::cty::c_char
                func: os_task_func_t,
                arg: Ptr,         //  Previously: *mut ::cty::c_void
                prio: u8,
                sanity_itvl: os_time_t,
                stack_bottom: Out<[os_stack_t]>,  //  Previously: *mut os_stack_t
                stack_size: usize,                //  Previously: u16 */
        ) -> #declare_result_tokens {             //  e.g. BlResult<()> or BlResult<* mut os_eventq>
            "----------Extern Decl----------";
            extern "C" { #foreign_item_tokens }
            "----------Validation----------";
            #validation_tokens
            unsafe {
                "----------Call----------";
                #get_result_tokens #fname_token(
                    //  "----------Call Expr----------";
                    #call_tokens
                    /* Like this:
                        t,
                        name.bytestr.as_ptr() as *const ::cty::c_char,  //  Converted to pointer
                        func,
                        arg,
                        prio,
                        sanity_itvl,
                        stack_bottom.as_ptr() as *mut os_stack_t,  //  Converted to pointer
                        stack_size as u16  */
                );
                "----------Result----------";
                #return_result_tokens
            }
        }
    };
    expanded
}

/// Collect the Wrapped Declarations for all args. Return a TokenStream of the declarations for the wrapper function:
/// `t: Out<os_task>, name: &Strn, func: os_task_func_t, ...`
/// Preserve the span info for error display.
fn collect_wrap(args: &Vec<TransformedArg>) -> proc_macro2::TokenStream {
    //  Construct a new `TokenStream` to accumulate the expanded code.
    //  We use `TokenStream` instead of string because `TokenStream` remembers the source location (span) in case of errors.
    //  `quote!` returns `proc_macro2::TokenStream` instead of `proc_macro::TokenStream`, so we use `proc_macro2::TokenStream`.
    let mut expanded = proc_macro2::TokenStream::new();
    for arg in args {
        //  Construct the wrap identifier and type tokens, preserving the span: `t: Out<os_task>`
        let TransformedArg{ ident, wrap_type, ident_span, type_span, .. } = arg;
        let tokens = expand_decl(ident, wrap_type, ident_span, type_span, Separator::Colon);
        if !expanded.is_empty() { expanded.extend(quote!{ , }) }  //  Add a comma if not first arg.
        expanded.extend(tokens);
    }
    //  Return the expanded tokens.
    //  println!("expanded: {:#?}", expanded);
    expanded
}

/// Collect the Call Expressions for all args. Return a TokenStream of the call expressions for the wrapper function:
/// `t, name.bytestr.as_ptr() as *const ::cty::c_char, ...`
/// Preserve the span info for error display.
fn collect_call(args: &Vec<TransformedArg>) -> proc_macro2::TokenStream {
    //  Construct a new `TokenStream` to accumulate the expanded code.
    //  We use `TokenStream` instead of string because `TokenStream` remembers the source location (span) in case of errors.
    //  `quote!` returns `proc_macro2::TokenStream` instead of `proc_macro::TokenStream`, so we use `proc_macro2::TokenStream`.
    let mut expanded = proc_macro2::TokenStream::new();
    for arg in args {
        //  Construct the call expr and type tokens, preserving the span: `t as *mut os_task`
        let TransformedArg{ call_expr, extern_type, ident_span, type_span, .. } = arg;
        let tokens = expand_decl(call_expr, extern_type, ident_span, type_span, Separator::As);
        if !expanded.is_empty() { expanded.extend(quote!{ , }) }  //  Add a comma if not first arg.
        expanded.extend(tokens);
    }
    //  Return the expanded tokens.
    //  println!("expanded: {:#?}", expanded);
    expanded
}

/// Collect the Validate Statements for all args. Return a TokenStream of the validation statements for the wrapper function:
/// `Strn::validate_bytestr(name.bytestr); ...`
/// Preserve the span info for error display.
fn collect_validation(args: &Vec<TransformedArg>) -> proc_macro2::TokenStream {
    //  Construct a new `TokenStream` to accumulate the expanded code.
    //  We use `TokenStream` instead of string because `TokenStream` remembers the source location (span) in case of errors.
    //  `quote!` returns `proc_macro2::TokenStream` instead of `proc_macro::TokenStream`, so we use `proc_macro2::TokenStream`.
    let mut expanded = proc_macro2::TokenStream::new();
    for arg in args {
        //  Construct the call validation tokens, preserving the span.
        let TransformedArg{ validation_stmt, ident_span, .. } = arg;
        if validation_stmt.as_str() == "" { continue; }
        let stmt_token = syn::parse_str::<Expr>(validation_stmt).unwrap();
        let tokens = quote_spanned!(**ident_span=> #stmt_token);
        expanded.extend(quote!{ #tokens ; });  //  Add a semicolon.
    }
    //  Return the expanded tokens.
    //  println!("expanded: {:#?}", expanded);
    expanded
}

/// Given identifier `ident` and type `ty`, return the tokens for the declaration `ident: ty`.
/// Preserve the identifier and type spans specified in `ident_span` and `type_span`.
fn expand_decl(ident: &str, ty: &str, ident_span: &Span, type_span: &Span, separator: Separator) -> proc_macro2::TokenStream {
    //  Parse the ident and type strings as syn::Pat and syn::Type.
    let ident_token = syn::parse_str::<Expr>(ident).unwrap();
    let type_token = syn::parse_str::<Type>(ty).unwrap();
    //  Wrap the parsed tokens with the spans (locations) of the ident and type.
    let ident_token_spanned = quote_spanned!(*ident_span=> #ident_token);
    let type_token_spanned  = quote_spanned!(*type_span => #type_token);
    //  Return the tokens.
    match separator {
        Separator::Colon => { quote!{ #ident_token_spanned : #type_token_spanned } }
        Separator::As    => { quote!{ #ident_token_spanned as #type_token_spanned } }
    }
}

/// Given a list of extern function arg declarations, return the transformed args.
fn transform_arg_list(args: &Punctuated<FnArg, Comma>) -> Vec<TransformedArg>{
    // println!("args: {:#?}", args);
    let mut res = Vec::new();
    for arg in args {
        //  println!("arg: {:#?}", arg);
        if let FnArg::Typed(arg) = arg {
            //  `arg` contains `pat : ty`
            //  println!("arg: {:#?}", arg);
            let arg_transformed = transform_arg(&arg);
            res.push(arg_transformed);
        }
        else { assert!(false, "Unknown arg"); }
    }
    res
}

/// Transform the extern arg for Wrap declaration, Validation statement and Call expression.
fn transform_arg(arg: &PatType) -> TransformedArg {
    //  `arg` contains `pat : ty` e.g. `t : * mut os_task`
    //  println!("arg: {:#?}", arg);
    let syn::PatType { pat, ty, .. } = arg;
    //  println!("pat: {}, ty: {}", quote!{ #pat }, quote!{ #ty });
    let pat_span = pat.span();  //  syn::Pat
    let ty_span = ty.span();    //  syn::Type
    let ident = quote!{ #pat }.to_string();
    let extern_type = quote!{ #ty }.to_string();
    let mut wrap_type = quote!{ #ty }.to_string();
    let mut validation_stmt = "".to_string();
    let mut call_expr = quote!{ #pat }.to_string();
    //  Match the type and transform accordingly.
    match extern_type.as_str() {
        //  * const :: cty :: c_char => &Strn
        "* const :: cty :: c_char" => {
            wrap_type = "&Strn".to_string();
            //  e.g. `name.validate()`
            validation_stmt = format!("{}.validate()", ident);
            //  e.g. `name.as_ptr()`
            call_expr = format!("{}.as_ptr()", ident);
        }
        //  * mut :: cty :: c_void => Ptr
        "* mut :: cty :: c_void" => {
            wrap_type = "Ptr".to_string();
        }
        //  * mut os_task => Out<os_task>
        "* mut os_task" => {
            //  TODO: Use regex to match any `os_*`: https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
            wrap_type = "Out<os_task>".to_string();
        }
        //  * mut os_stack_t => Out<[os_stack_t]>
        "* mut os_stack_t" => {
            wrap_type = "Out<[os_stack_t]>".to_string();
            //  e.g. stack_bottom.as_ptr()
            call_expr = format!("{}.as_ptr()", ident);
        }
        _ => {}
    }
    //  Return the transformed arg.
    TransformedArg {
        ident:           Box::new(ident),
        extern_type:     Box::new(extern_type),
        wrap_type:       Box::new(wrap_type),
        validation_stmt: Box::new(validation_stmt),
        call_expr:       Box::new(call_expr),
        ident_span:      Box::new(pat_span),
        type_span:       Box::new(ty_span),
    }
}

/// Transform the extern return type e.g. `:: cty :: c_int` becomes `BlResult< () >`
fn transform_return_type(fname: &TransformedFunctionName, output: &ReturnType) -> TransformedReturnType {
    let extern_type = quote! { output }.to_string();
    let type_span = output.span();
    let wrap_type =
        if let ReturnType::Type (_, return_type) = output {
            //  e.g. `:: cty :: c_int` for error code, or `* mut os_eventq`
            (quote! { #return_type }).to_string()
        }
        else { "".to_string() };  //  No return type
    //  println!("wrap_type: {:#?}", wrap_type);

    //  If Function Name (without namespace) starts with `get_`, we return as Int instead of Error Code (e.g. `bl_adc_get_channel_by_gpio`)
    let returns_int = fname.without_namespace.starts_with("get_");  

    //  TODO: #[cfg(feature = "mynewt_os")]  //  If building for BL602 IoT SDK...
    let result_token = quote! { BlResult };  //  Result type is BlResult

    //  TODO: #[cfg(feature = "riot_os")]    //  If building for RIOT OS...
    //  let result_token = quote! { LvglResult };    //  Result type is LvglResult

    //  Declare the result type.
    let declare_result_tokens =
        match wrap_type.as_str() {
            //  No return type (void)
            ""                          => { quote! { #result_token< () > } }
            //  Int or Error code
            ":: cty :: c_int"           => { 
                //  Int
                if returns_int { quote! { #result_token< ::cty::c_int > } }
                //  Error code
                else           { quote! { #result_token< () > } }
            }
            //  String becomes `Strn<'static>`
            "* const :: cty :: c_char"  => { quote! { #result_token< Strn<'static> > } }
            //  Specified return type e.g. `* mut os_eventq`
            _ => {
                let return_type_tokens = syn::parse_str::<Type>(&wrap_type).unwrap();
                quote! { #result_token< #return_type_tokens > }  
            }
        };        
    //  Assign the result.
    let get_result_tokens =
        match wrap_type.as_str() {
            "" => { quote! {} }  //  No return type, so no result.
            _  => { quote! { let res = } }
        };
    //  Return the result or error.
    let return_result_tokens = 
        match wrap_type.as_str() {
            //  If no return type, return nothing
            "" => { quote! { Ok( () ) } }
            //  Return Int or Error code
            ":: cty :: c_int" => {
                //  Return Int
                if returns_int { quote! { Ok(res) } }
                //  Return Error Code
                else {
                    quote! {
                        match res {
                            0 => Ok(()),
                            _ => Err(BlError::from(res))
                        }
                    }    
                }
            }
            //  Return string wrapped as `Strn`
            "* const :: cty :: c_char" => { 
                quote! { 
                    Ok(Strn::from_cstr(res as *const u8))
                } 
            }
            //  Return `void *` pointers with null checking
            "* mut :: cty :: c_void" => {
                quote! {
                    if res.is_null() { Err(BlError::SYS_NULLPOINTER) }
                    else { Ok(res) }
                }
            }
            //  Return specified type e.g. `* mut os_eventq`
            _ => { quote! { Ok(res) } }
        };
    //  Return the transformed return type.
    TransformedReturnType {
        extern_type:                Box::new(extern_type),
        wrap_type:                  Box::new(wrap_type),
        declare_result_tokens:      Box::new(quote_spanned!(type_span=> #declare_result_tokens)),
        get_result_tokens:          Box::new(quote_spanned!(type_span=> #get_result_tokens)),
        return_result_tokens:       Box::new(quote_spanned!(type_span=> #return_result_tokens)),
        type_span:                  Box::new(type_span),
    }
}

/// Transform the extern function name e.g. `bl_gpio_enable_output`
fn transform_function_name(ident: &Ident) -> TransformedFunctionName {
    //  Extern function name e.g `bl_gpio_enable_output`
    let fname = ident.to_string();
    //  Get namespace e.g. `gpio`
    let namespace = get_namespace(&fname);
    //  println!("fname: {:#?}, namespace: {:#?}", fname, namespace);
    //  Get namespace prefix e.g. `gpio_`
    let namespace_prefix = 
        if namespace.len() > 0 { 
            format!("{}_", namespace).to_string()  //  e.g. `gpio_`
        } else {
            "".to_string()
        };
    // Transform the function name based on namespace.
    assert!(fname.starts_with(&namespace_prefix));
    let fname_without_namespace = &fname[namespace_prefix.len()..];
    let fname_token = Ident::new(&fname, ident.span());
    let fname_without_namespace_token = Ident::new(&fname_without_namespace, ident.span());

    //  Compose the function doc by looking up the doclink:
    //  #[doc = "Configure a GPIO Pin for Output Mode. See `bl_gpio_enable_output` in \"Read and Write GPIO\" <https://lupyuen.github.io/articles/led#read-and-write-gpio>"]
    let mut doc_tokens = proc_macro2::TokenStream::new();
    //  If the doclink exists for the function name...
    if let Some(doclink) = DOCLINKS.get(&fname) {
        //  Compose "Configure a GPIO Pin for Output Mode. See `bl_gpio_enable_output` in \"Read and Write GPIO\" <https://lupyuen.github.io/articles/led#read-and-write-gpio>"
        let desc = &doclink[0];
        let doc =
            if doclink.len() < 3 { desc.to_string() }  //  Description only
            else {  //  Description, section and URL
                let section = &doclink[1];
                let url = &doclink[2];        
                format!("{} See `{}` in \"{}\" <{}>", desc, fname, section, url)
            };
        //  Convert to a literal token.
        let lit = LitStr::new(&doc, ident.span());        
        //  Append to the doc tokens.
        doc_tokens.extend(
            quote! {
                #[doc = #lit]
            }
        );
    }

    //  Return the transformed function name.
    TransformedFunctionName {
        ident:                      Box::new(fname.clone()),
        namespace:                  Box::new(namespace.to_string()),
        without_namespace:          Box::new(fname_without_namespace.to_string()),
        token:                      Box::new(fname_token),
        without_namespace_token:    Box::new(fname_without_namespace_token),
        ident_span:                 Box::new(ident.span()),
        doc_tokens:                 Box::new(doc_tokens),
    }
}

/// Read the links that will be embedded into the documentation
fn read_doclinks() -> DocLinkType {
    //  Read the doclinks into a HashMap
    let input = File::open("doclinks.md")
        .expect("doclinks.md is missing");
    let buffered = BufReader::new(input);

    //  Split each line into columns and add to the HashMap
    let mut map = DocLinkType::new();
    for line in buffered.lines() {
        //  Split the line by "|"
        let split: Vec<String> = line
            .unwrap()
            .split("|")
            .map(|s| s.trim().to_string())
            .collect();

        //  Every row must have function name and description
        if split.len() < 3 { continue; }
        
        //  Map a function name to its doclink:
        //  `bl_gpio_enable_output` -> [ `Configure a GPIO Pin for Output Mode`, `Enable GPIO`, `https://lupyuen.github.io/articles/led#enable-gpio` ]
        map.insert(
            split[1].trim().to_string(),  //  `bl_gpio_enable_output`
            split[2..].to_vec()           //  [ `Configure a GPIO Pin for Output Mode`, `Enable GPIO`, `https://lupyuen.github.io/articles/led#enable-gpio` ]
        );
    }

    //  How we fetch the doclink by function name:
    //  println!("map.get={:#?}", map.get("bl_gpio_enable_output"));
    map
}

//  Init the globals lazily because Rust doesn't allow `::new()` to be called during init
lazy_static::lazy_static! {
    /// Map a function name to its doclink:
    /// `bl_gpio_enable_output` -> [ `Configure a GPIO Pin for Output Mode`, `Enable GPIO`, `https://lupyuen.github.io/articles/led#enable-gpio` ]
    static ref DOCLINKS: DocLinkType = read_doclinks();
}

/// Map a function name to its doclink
type DocLinkType = HashMap<String, Vec<String>>;

/// Extern arg declaration transformed into the Wrap, Validation and Call forms 
struct TransformedArg {
    /// Identifier e.g. `name`
    ident: Box<String>,
    /// Original extern type e.g. `*const ::cty::c_char`
    extern_type: Box<String>,
    /// Wrapped type to be exposed e.g. `&Strn`
    wrap_type: Box<String>,
    /// Validation statement e.g. `Strn::validate_bytestr(name.bytestr)`
    validation_stmt: Box<String>,
    /// Call expression e.g. `name.bytestr.as_ptr() as *const ::cty::c_char`
    call_expr: Box<String>,
    /// Span of the identifier (file location)
    ident_span: Box<Span>,
    /// Span of the type (file location)
    type_span: Box<Span>,
}

/// Extern return type declaration transformed
#[allow(dead_code)]  //  TODO
struct TransformedReturnType {
    /// Original extern type e.g. `:: cty :: c_int` or `* mut os_eventq`
    extern_type: Box<String>,
    /// Wrapped type to be exposed e.g. `:: cty :: c_int` or `* mut os_eventq`
    wrap_type: Box<String>,
    /// Declare the result type e.g. `BlResult< * mut os_eventq >`
    declare_result_tokens: Box<proc_macro2::TokenStream>,
    /// Assign the result e.g. `let res = `
    get_result_tokens: Box<proc_macro2::TokenStream>,
    /// Return the result or error e.g. `Ok( res )`
    return_result_tokens: Box<proc_macro2::TokenStream>,
    /// Span of the type (file location)
    type_span: Box<Span>,
}

/// Extern function name transformed
#[allow(dead_code)]  //  TODO
#[derive(Clone)]
struct TransformedFunctionName {
    /// Identifier e.g. `bl_gpio_enable_output`
    ident: Box<String>,
    /// Namespace e.g. `bl_gpio`. Or empty if no namespace.
    namespace: Box<String>,
    /// Function name without namespace e.g. `enable_output`
    without_namespace: Box<String>,
    /// Token of function name e.g. `bl_gpio_enable_output`
    token: Box<Ident>,
    /// Token of function name without namespace e.g. `enable_output`
    without_namespace_token: Box<Ident>,
    /// Span of the identifier (file location)
    ident_span: Box<Span>,
    /// Tokens for the function documentation e.g. #[doc = "Configure a GPIO Pin for Output Mode. See `bl_gpio_enable_output` in \"Read and Write GPIO\" <https://lupyuen.github.io/articles/led#read-and-write-gpio>"]
    doc_tokens: Box<proc_macro2::TokenStream>,
}

/// Separator for composing declarations and call expressions
enum Separator {
    /// `ident: ty`
    Colon,
    /// `ident as ty`
    As,
}
