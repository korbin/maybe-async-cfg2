//!
//! # maybe-async-cfg2
//!
//! **Don't repeat yourself when writing blocking and async code.**
//!
//! [![Build Status](https://github.com/korbin/maybe-async-cfg2/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/korbin/maybe-async-cfg2/actions)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![Latest Version](https://img.shields.io/crates/v/maybe-async-cfg2.svg)](https://crates.io/crates/maybe-async-cfg2)
//! [![maybe-async](https://docs.rs/maybe-async-cfg2/badge.svg)](https://docs.rs/maybe-async-cfg2)
//!
//! When implementing both sync and async variants of an API in a crate, the APIs of the two
//! variants are almost the same except for async/await keywords.
//!
//! `maybe-async-cfg2` helps unify async and sync implementations using a **procedural macro**.
//! - Write async code with normal `async` and `await` keywords, and let `maybe_async_cfg2` handle
//! removing them when blocking code is needed.
//! - Add `maybe` attributes and specify feature conditions in the macro parameters to determine
//! which variant of code should be generated.
//! - Use `only_if` (or `remove_if`) to keep code in a specific variant when necessary.
//!
//! The `maybe` procedural macro can be applied to the following code:
//! - use declarations
//! - trait declarations
//! - trait implementations
//! - function definitions
//! - struct and enum definitions
//! - modules
//!
//! **RECOMMENDATION**: Use resolver version 2 in `Cargo.toml`, which was introduced in Rust 1.51.
//! Without it, two crates in a dependency with conflicting versions (one async and another
//! blocking) can fail compilation.
//!
//! ```toml
//! [package]
//! resolver = "2"
//! # or when using workspaces
//! [workspace]
//! resolver = "2"
//! ```
//!
//!
//! ## Motivation
//!
//! The async/await language feature transformed the async world of Rust. Compared with the
//! map/and_then style, async code now more closely resembles sync code.
//!
//! In many crates, the async and sync variants share the same API, but the minor
//! difference that all async code must be awaited prevents the unification of async and sync code.
//! In other words, it is necessary to write an async and a sync implementation respectively.
//!
//!
//! ## Macros in Detail
//!
//! To use `maybe-async-cfg2`, it is necessary to distinguish which code is used exclusively in the
//! sync vs. async variants. These two variants of the implementation should share the same function
//! signatures except for async/await keywords.
//!
//! Use the `maybe` macro for code that is the *same* in both async and sync variants.
//! Specify in the macro parameters the conditions (based on features) under which async and/or sync
//! variants of the code should appear.
//!
//! - attribute macro **`maybe`**
//!
//!     Offers a unified way to provide sync and async conversion on demand depending on enabled
//! feature flags, with an **async first** policy.
//!
//!     ```toml
//!     [dependencies]
//!     maybe_async_cfg2 = "0.3"
//!
//!     [features]
//!     use_sync = []
//!     use_async = []
//!     ```
//!
//!     In this and all the following examples, two features are used. Any conditions
//! can be used, for example, replacing `feature="use_sync"` with
//! `not(feature="use_async")` everywhere. `maybe-async-cfg2` does not analyze the
//! conditions in any way, just substituting them as is.
//!
//!     Add the `maybe` attribute before all items that must be different in sync vs. async code.
//!
//!     To keep async code, specify the `async` parameter with the condition (based on
//! features) for when the code should be async.
//!
//!     To convert async code to sync, specify the `sync` parameter with the condition when
//! sync code should be generated.
//!
//!     ```rust
//!     #[maybe_async_cfg2::maybe(
//!         idents(Foo),
//!         sync(feature="use_sync"),
//!         async(feature="use_async")
//!     )]
//!     struct Struct {
//!         f: Foo,
//!     }
//!     ```
//!     After conversion:
//!     ```rust
//!     #[cfg(feature="use_sync")]
//!     struct StructSync {
//!         f: FooSync,
//!     }
//!     #[cfg(feature="use_async")]
//!     struct StructAsync {
//!         f: FooAsync,
//!     }
//!     ```
//!
//! - procedural macro **`content`**
//!
//!     The `content` macro allows specifying common parameters for many `maybe` macros. Use the
//! internal `default` attribute with the required parameters inside the `content` macro.
//!
//!     ```rust
//!     maybe_async_cfg2::content!{
//!     #![maybe_async_cfg2::default(
//!         idents(Foo, Bar),
//!     )]
//!
//!     #[maybe_async_cfg2::maybe(
//!         sync(feature="use_sync"),
//!         async(feature="use_async")
//!     )]
//!     struct Struct {
//!         f: Foo,
//!     }
//!
//!     #[maybe_async_cfg2::maybe(
//!         sync(feature="use_sync"),
//!         async(feature="use_async")
//!     )]
//!     async fn func(b: Bar) {
//!         todo!()
//!     }
//!     } // content!
//!     ```
//!     After conversion:
//!     ```rust
//!     #[cfg(feature="use_sync")]
//!     struct StructSync {
//!         f: FooSync,
//!     }
//!     #[cfg(feature="use_async")]
//!     struct StructAsync {
//!         f: FooAsync,
//!     }
//!
//!     #[cfg(feature="use_sync")]
//!     fn func_sync(b: BarSync) {
//!         todo!()
//!     }
//!     #[cfg(feature="use_async")]
//!     async fn func_async(b: BarAsync) {
//!         todo!()
//!     }
//!     ```
//!
//! ## Doctests
//!     
//! When writing doctests, they can be marked as applicable only in the corresponding code variant.
//! To do this, specify `only_if(`_VARIANT_KEY_`)` in the doctest attributes. Then in all other
//! variants, this doctest will be replaced with an empty string.
//!
//! ```rust
//! #[maybe_async_cfg2::maybe(
//!     idents(Foo),
//!     sync(feature = "use_sync"),
//!     async(feature = "use_async")
//! )]
//! /// This is a structure.
//! /// ```rust, only_if(sync)
//! /// let s = StructSync{ f: FooSync::new() };
//! /// ```
//! /// ```rust, only_if(async)
//! /// let s = StructAsync{ f: FooAsync::new().await };
//! /// ```
//! struct Struct {
//!     f: Foo,
//! }
//! ```
//! After conversion:
//! ```rust
//! #[cfg(feature = "use_sync")]
//! /// This is a structure.
//! /// ```rust, only_if(sync)
//! /// let s = StructSync{ f: FooSync::new() };
//! /// ```
//! struct StructSync {
//!     f: FooSync,
//! }
//! #[cfg(feature = "use_async")]
//! /// This is a structure.
//! ///
//! /// ```rust, only_if(async)
//! /// let s = StructAsync{ f: FooAsync::new().await };
//! /// ```
//! struct StructAsync {
//!     f: FooAsync,
//! }
//! ```
//!
//! ## Examples
//!
//! ### Rust client for services
//!
//! When implementing a Rust client for any service, like awz3, the higher-level API of async and
//! sync variants is almost the same, such as creating or deleting a bucket, retrieving an object,
//! etc.
//!
//! The example `service_client` is a proof of concept that `maybe_async_cfg2` can actually free us
//! from writing duplicate code for sync and async variants. The `is_sync` feature gate
//! allows toggling between sync and async AWZ3 client implementations.
//!
//!
//! ## Acknowledgements
//!
//! This crate is a redesigned fork of these wonderful crates:
//!
//! - [fMeow/maybe-async-rs](https://github.com/fMeow/maybe-async-rs)
//!
//! - [marioortizmanero/maybe-async-rs](https://github.com/marioortizmanero/maybe-async-rs)
//!
//! Thanks!
//!
//!
//! # License
//! MIT
#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

// note: the `rustdoc::missing_doc_code_examples` lint is unstable
//#![deny(rustdoc::missing_doc_code_examples)]

use manyhow::manyhow;
use proc_macro::TokenStream;

mod macros;
mod params;
mod utils;
mod visit_ext;
mod visitor_async;
mod visitor_content;

#[cfg(feature = "doctests")]
mod doctests;

mod debug;

const DEFAULT_CRATE_NAME: &'static str = "maybe_async_cfg2";
const MACRO_MAYBE_NAME: &'static str = "maybe";
const MACRO_ONLY_IF_NAME: &'static str = "only_if";
const MACRO_REMOVE_IF_NAME: &'static str = "remove_if";
const MACRO_NOOP_NAME: &'static str = "noop";
const MACRO_REMOVE_NAME: &'static str = "remove";
const MACRO_DEFAULT_NAME: &'static str = "default";

const STANDARD_MACROS: &'static [&'static str] = &[
    "dbg",
    "print",
    "println",
    "assert",
    "assert_eq",
    "assert_ne",
];

/// Marks code that can be presented in several variants.
///
/// ### The `maybe` macro has the following parameters:
///
/// - `disable`
///
///     The macro with `disable` parameter will do nothing, like `noop`. Use it to write and debug
/// initial async code.
///
/// - `prefix`
///
///     The name of `maybe-async-cfg2` crate. If not set, `"maybe_async_cfg2"` will be used.
///
/// - `sync`, `async`
///
///     Defines variants of code: the item to which the attribute `maybe` refers will be
/// replaced with multiple copies (one for each variant), which will be modified according to
/// the variant kind and its parameters.
///
///     For the `sync` variant, the item will be converted from async to sync code by deleting
/// the `async` and `await` keywords. Types `Future<Output=XXX>` will also be replaced with just
/// `XXX`. For the `async` variant, the item will be left async.
///
///     In any case, the item will be converted according to all parameters described below. For
/// functions, structs/enums and traits, the name will be changed as if it is mentioned in the
/// `idents` list (if it is not explicitly specified there and if `keep_self` is not present).
///
/// - All other parameters will be passed to all variants (with merging).
///
///     Therefore, those parts of the variant parameters that match in all variants can be specified
/// here. For example, this is the expected behavior for the `idents` list.
///
/// ### Every variant has the following parameters:
///
/// - `disable`
///
///     Ignore this variant entirely.
///
/// - `key`
///
///     Defines unique name of the variant to use it in `only_if`/`remove_if` conditions. If
/// omitted, `sync`/`async` will be used.
///
///     ```rust
///     #[maybe_async_cfg2::maybe(
///         sync(key="foo", feature="use_sync"),
///         async(key="bar", feature="use_async"),
///     )]
///     struct Struct {
///         f: usize,
///         
///         // This field will only be present in sync variant
///         #[maybe_async_cfg2::only_if(key="foo")]
///         sync_only_field: bool,
///     }
///     ```
///     After conversion:
///     ```rust
///     #[cfg(feature="use_sync")]
///     struct StructSync {
///         f: usize,
///         sync_only_field: bool,
///     }
///     #[cfg(feature="use_async")]
///     struct StructAsync {
///         f: usize,
///     }
///     ```
///
/// - `cfg`
///
///     Defines the condition (based on features), under which the current variant should appear.
///
///     Note: conditions like `feature = "..."`, `not(...)`, `all(...)`, `any(...)` will be
/// processed correctly, even if the `cfg(...)` was omitted.
///
///     ```rust
///     #[maybe_async_cfg2::maybe(
///         sync(cfg(feature="use_sync")),
///         async(feature="use_async")
///     )]
///     struct Struct {
///         f: Foo,
///     }
///     ```
///     After conversion:
///     ```rust
///     #[cfg(feature="use_sync")]
///     struct StructSync {
///         f: Foo,
///     }
///     #[cfg(feature="use_async")]
///     struct StructAsync {
///         f: Foo,
///     }
///     ```
///  
/// - `idents`
///
///     Defines a list of identifiers that should be renamed depending on the variant of code.
///
///     Each identifier can have the following clarifying parameters:
///
///     - `snake`, `fn`, `mod`
///
///         means that this is the snake-case name of the function or module and it should be
/// converted by adding the suffixes `"_sync"`/`"_async"` (otherwise, the suffixes
/// `"Sync"`/`"Async"` will be used).
///
///     - `use`
///
///         in `use` lists, using this identifier will result in renaming via the `as` expression,
/// rather than a simple replacement as is. In other cases, a simple replacement will be used.
///
///     - `keep`
///
///         this identifier will not be converted anywhere
///
///     - `sync`, `async`
///
///         specifies the name that will be used in the corresponding variant of code. Overrides
/// the standard scheme of suffixes used by default. If the parameter value is omitted,
/// the identifier will not be renamed in this case.
///
///     ```rust
///     #[maybe_async_cfg2::maybe(
///         idents(
///             Foo,
///             Bar,
///             baz(fn),
///             Qux(use),
///             waldo(sync, async="async_waldo"),
///             xyzzy(fn, use, sync="xizzy_the_sync_func"),
///         ),
///         sync(feature="use_sync"),
///         async(feature="use_async"),
///     )]
///     async fn func() {
///         struct Foo {}
///         use extcrate::{
///             Bar,
///             baz,
///             Qux,
///             waldo::{
///                 plugh,
///                 xyzzy
///             }
///         };
///         let _ = baz( Foo {}, Bar::new() ).await;
///         let _ = xizzy( Qux::flob(b).await );
///     }
///     ```
///     After conversion:
///     ```rust
///     #[cfg(feature="use_sync")]
///     fn func_sync() {
///         struct FooSync {}
///         use extcrate::{
///             BarSync,
///             baz_sync,
///             Qux as QuxSync,
///             waldo::{
///                 plugh,
///                 xyzzy as xizzy_the_sync_func
///             }
///         };         
///         let _ = baz_sync( FooSync {}, BarSync::new() );
///         let _ = xizzy_the_sync_func( QuxSync::flob() );
///     }
///     #[cfg(feature="use_async")]
///     async fn func_async() {
///         struct FooAsync {}
///         use extcrate::{
///             BarAsync,
///             baz_async,
///             Qux as QuxAsync,
///             async_waldo::{
///                 plugh,
///                 xyzzy as xyzzy_async
///             }
///         };
///         let _ = baz_async( FooAsync {}, BarAsync::new() ).await;
///         let _ = xyzzy_async( QuxAsync::flob().await );     
///     }
///     ```
///
/// - `keep_self`
///
///     Do not change name of item to which attribute `maybe` refers.
///
/// - `self`
///
///     Defines the name that will be assigned to the item in this variant.
///
/// - `send`
///
///     If `send = "Send"` or `send = "true"` is present, the attribute
/// `#[async_trait::async_trait]` will be added before the async code. If `send = "?Send"` or
/// `send = "false"` then `#[async_trait::async_trait(?Send)]` will be added.  
///
/// - `drop_attrs`
///
///     Remove any attributes with specified names.
///
///     ```rust
///     #[maybe_async_cfg2::maybe(
///         sync(feature="use_sync", drop_attrs(attr)),
///         async(feature="use_async"),
///     )]
///     struct Struct {
///         f: usize,
///
///         // This attribute will be removed in sync variant
///         #[attr(param)]
///         field1: bool,
///     }
///     ```
///     After conversion:
///     ```rust
///     #[cfg(feature="use_sync")]
///     struct StructSync {
///         f: usize,
///         field1: bool,
///     }
///     #[cfg(feature="use_async")]
///     struct StructAsync {
///         f: usize,
///         #[attr(param)]
///         field1: bool,
///     }
///     ```
///
/// - `replace_features`
///
///     Replace one feature name with another.
///
///     ```rust
///     #[maybe_async_cfg2::maybe(
///         sync(feature="use_sync", replace_feature("secure", "secure_sync")),
///         async(feature="use_async"),
///     )]
///     struct Struct {
///         f: usize,
///         // In sync variant "secure" feature will be replaced with "secure_sync" feature
///         #[cfg(feature="secure")]
///         field: bool,
///     }
///     ```
///     After conversion:
///     ```rust
///     #[cfg(feature="use_sync")]
///     struct StructSync {
///         f: usize,
///         #[cfg(feature="secure_sync")]
///         field: bool,
///     }
///     #[cfg(feature="use_async")]
///     struct StructAsync {
///         f: usize,
///         #[cfg(feature="secure")]
///         field: bool,
///     }
///     ```
///
/// - `inner`, `outer`
///
///     Adds some attributes to the generated code. Inner attributes will appear below attribute
/// `#[cfg(...)]`, outer attributes will appear above it.
///
///     Note: if the variant parameter is not parsed as a parameter of some other type, it will be
/// interpreted as an inner attribute.
///
///     Useful for testing: just write `test` in variant parameters.
///
///     ```rust
///     #[maybe_async_cfg2::maybe(
///         sync(feature="secure_sync", test, "resource(path = \"/foo/bar\")", outer(xizzy)),
///         async(feature="secure_sync", inner(baz(qux), async_attributes::test)),
///     )]
///     async fn test_func() {
///         todo!()
///     }
///     ```
///     After conversion:
///     ```rust
///     #[xizzy]
///     #[cfg(feature="use_sync")]
///     #[test]
///     #[resource(path = "/foo/bar")]
///     fn test_func_sync() {
///         todo!()
///     }
///     #[cfg(feature="use_async")]
///     #[baz(qux)]
///     #[async_attributes::test]
///     async fn test_func_async() {
///         todo!()
///     }
///     ```
///
/// - In other cases, the following rules apply:
///     - name-value pairs (`xxx = "yyy"`) with a name other than `key`, `prefix`, `send` and
/// `feature` will produce an error.
///     
///     - `feature = "..."`, `not(...)`, `all(...)`, `any(...)` will be interpreted as condition for
/// current variant (as wrapped in `cfg(...)`).
///
///     - all another parameters will be interpreted as inner attribute for current variant (as
/// wrapped in `inner(...)`).
///
/// ### Formal syntax
///
/// > _ParametersList_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;_Parameter_ (`,` _Parameter_)<sup>\*</sup>
/// >
/// > _Parameter_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;`disable`\
/// > &nbsp;&nbsp;|&nbsp;`keep_self`\
/// > &nbsp;&nbsp;|&nbsp;`prefix` `=` _STRING_LITERAL_\
/// > &nbsp;&nbsp;|&nbsp;(`sync` | `async`) `(` _VersionParametersList_ `)`\
/// > &nbsp;&nbsp;|&nbsp;`idents` `(` _IdentsList_ `)`\
/// >
/// > _VersionParametersList_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;_VersionParameter_ (`,` _VersionParameter_)<sup>\*</sup>
/// >
/// > _VersionParameter_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;`disable`\
/// > &nbsp;&nbsp;|&nbsp;`keep_self`\
/// > &nbsp;&nbsp;|&nbsp;`key` `=` _STRING_LITERAL_\
/// > &nbsp;&nbsp;|&nbsp;`feature` `=` _STRING_LITERAL_\
/// > &nbsp;&nbsp;|&nbsp;`self` `=` _STRING_LITERAL_\
/// > &nbsp;&nbsp;|&nbsp;`send` `=` (`""` | `"Send"` | `"true"` | `"?Send"` | `"false"`)\
/// > &nbsp;&nbsp;|&nbsp;(`cfg` | `any` | `all` | `not`) `(` _ANY_CFG_CONDITION_ `)`\
/// > &nbsp;&nbsp;|&nbsp;`idents` `(` _IdentsList_ `)`\
/// > &nbsp;&nbsp;|&nbsp;(`outer` | `inner`) `(` _AttributesList_ `)`\
/// > &nbsp;&nbsp;|&nbsp;`replace_feature` `(` _STRING_LITERAL_ `,` _STRING_LITERAL_ `)`\
/// > &nbsp;&nbsp;|&nbsp;`drop_attrs` `(` _IdentifiersList_ `)`\
/// > &nbsp;&nbsp;|&nbsp;_Attribute_
/// >
/// > _Path_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;_IDENTIFIER_ (`::` _IDENTIFIER_)<sup>\+</sup>
/// >
/// > _IdentifiersList_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;_IDENTIFIER_ (`,` _IDENTIFIER_)<sup>\*</sup>
/// >
/// > _IdentsList_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;_Ident_ (`,` _Ident_)<sup>\*</sup>
/// >
/// > _Ident_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;_IDENTIFIER_ (`(` _IdentParametersList_ `)`)<sup>\?</sup>
/// >
/// > _IdentParametersList_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;_IdentParameter_ (`,` _IdentParameter_)<sup>\*</sup>
/// >
/// > _IdentParameter_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;`keep`\
/// > &nbsp;&nbsp;|&nbsp;`use`\
/// > &nbsp;&nbsp;|&nbsp;(`snake` | `fn` | `mod` )\
/// > &nbsp;&nbsp;|&nbsp;`use`\
/// > &nbsp;&nbsp;|&nbsp;(`sync` | `async` | _IDENTIFIER_) (`=` _STRING_LITERAL_)<sup>\?</sup>
/// >
/// > _AttributesList_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;_Attribute_ (`,` _Attribute_)<sup>\*</sup>
/// >
/// > _Attribute_ :\
/// > &nbsp;&nbsp;&nbsp;&nbsp;(_IDENTIFIER_ | _Path_) (`(` _ANY_VALID_ARGS_ `)`)<sup>\?</sup>\
/// > &nbsp;&nbsp;|&nbsp;_STRING_LITERAL_
#[manyhow]
#[proc_macro_attribute]
pub fn maybe(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    macros::maybe(args, input)
}

/// Marks conditional content that should only be used in the specified variant of code.
#[manyhow]
#[proc_macro_attribute]
pub fn only_if(_: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    Ok(body)
}

/// Marks conditional content that should be used in all variants of code except the specified
/// one.
#[manyhow]
#[proc_macro_attribute]
pub fn remove_if(_: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    Ok(body)
}

/// Does nothing (leaves content intact).
#[manyhow]
#[proc_macro_attribute]
pub fn noop(_: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    Ok(body)
}

/// Removes marked content.
#[manyhow]
#[proc_macro_attribute]
pub fn remove(_: TokenStream, _: TokenStream) -> syn::Result<TokenStream> {
    Ok(TokenStream::new())
}

/// A wrapper for code with common `maybe` parameters
///
/// The `content` macro allows you to specify common parameters for many `maybe` macros. Use the
/// internal `default` attribute with the required parameters inside the `content` macro.
///
/// ```rust
/// maybe_async_cfg2::content! {
/// #![maybe_async_cfg2::default(
///     idents(Foo, Bar),
/// )]
///
/// #[maybe_async_cfg2::maybe(sync(feature="use_sync"), async(feature="use_async"))]
/// struct Struct {
///     f: Foo,
/// }
///
/// #[maybe_async_cfg2::maybe(sync(feature="use_sync"), async(feature="use_async"))]
/// async fn func(b: Bar) {
///     todo!()
/// }
/// } // content!
/// ```
/// After conversion:
/// ```rust
/// #[cfg(feature = "use_sync")]
/// struct StructSync {
///     f: FooSync,
/// }
/// #[cfg(feature = "use_async")]
/// struct StructAsync {
///     f: FooAsync,
/// }
///
/// #[cfg(feature = "use_sync")]
/// fn func_sync(b: BarSync) {
///     todo!()
/// }
/// #[cfg(feature = "use_async")]
/// async fn func_async(b: BarAsync) {
///     todo!()
/// }
/// ```
#[manyhow]
#[proc_macro]
pub fn content(body: TokenStream) -> syn::Result<TokenStream> {
    macros::content(body)
}
