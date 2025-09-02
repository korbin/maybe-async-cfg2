<!-- cargo-sync-readme start -->


# maybe-async-cfg2

**Don't repeat yourself when writing blocking and async code.**

[![Build Status](https://github.com/korbin/maybe-async-cfg2/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/korbin/maybe-async-cfg2/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Latest Version](https://img.shields.io/crates/v/maybe-async-cfg2.svg)](https://crates.io/crates/maybe-async-cfg2)
[![maybe-async](https://docs.rs/maybe-async-cfg2/badge.svg)](https://docs.rs/maybe-async-cfg2)

When implementing both sync and async variants of an API in a crate, the APIs of the two variants
are almost the same except for async/await keywords.

`maybe-async-cfg2` helps unify async and sync implementations using a **procedural macro**.
- Write async code with normal `async` and `await` keywords, and let `maybe_async_cfg2` handle
removing them when blocking code is needed.
- Add `maybe` attributes and specify feature conditions in the macro parameters to determine
which variant of code should be generated.
- Use `only_if` (or `remove_if`) to keep code in a specific variant when necessary.

The `maybe` procedural macro can be applied to the following code:
- use declarations
- trait declarations
- trait implementations
- function definitions
- struct and enum definitions
- modules

**RECOMMENDATION**: Use resolver version 2 in `Cargo.toml`, which was introduced in Rust 1.51. Without
it, two crates in a dependency with conflicting versions (one async and another blocking) can fail
compilation.

```toml
[package]
resolver = "2"
# or when using workspaces
[workspace]
resolver = "2"
```


## Motivation

The async/await language feature transformed the async world of Rust. Compared with the map/and_then
style, async code now more closely resembles sync code.

In many crates, the async and sync variants share the same API, but the minor
difference that all async code must be awaited prevents the unification of async and sync code.
In other words, it is necessary to write an async and a sync implementation respectively.


## Macros in Detail

To use `maybe-async-cfg2`, it is necessary to distinguish which code is used exclusively in the sync vs. async variants. These two variants of the implementation should share the same function signatures except for async/await keywords.

Use the `maybe` macro for code that is the *same* in both async and sync variants.
Specify in the macro parameters the conditions (based on features) under which async and/or sync variants of the code should appear.

- attribute macro **`maybe`**

    Offers a unified way to provide sync and async conversion on demand depending on enabled feature flags, with an **async first** policy.

    ```toml
    [dependencies]
    maybe_async_cfg2 = "0.3"

    [features]
    use_sync = []
    use_async = []
    ```

    In this and all the following examples, two features are used. Any conditions
can be used, for example, replacing `feature="use_sync"` with
`not(feature="use_async")` everywhere. `maybe-async-cfg2` does not analyze the
conditions in any way, just substituting them as is.

    Add the `maybe` attribute before all items that must be different in sync vs. async code.

    To keep async code, specify the `async` parameter with the condition (based on
features) for when the code should be async.

    To convert async code to sync, specify the `sync` parameter with the condition when
sync code should be generated.

    ```rust
    #[maybe_async_cfg2::maybe(
        idents(Foo),
        sync(feature="use_sync"),
        async(feature="use_async")
    )]
    struct Struct {
        f: Foo,
    }
    ```
    After conversion:
    ```rust
    #[cfg(feature="use_sync")]
    struct StructSync {
        f: FooSync,
    }
    #[cfg(feature="use_async")]
    struct StructAsync {
        f: FooAsync,
    }
    ```

- procedural macro **`content`**

    The `content` macro allows specifying common parameters for many `maybe` macros. Use the
internal `default` attribute with the required parameters inside the `content` macro.

    ```rust
    maybe_async_cfg2::content!{
    #![maybe_async_cfg2::default(
        idents(Foo, Bar),
    )]

    #[maybe_async_cfg2::maybe(
        sync(feature="use_sync"), 
        async(feature="use_async")
    )]
    struct Struct {
        f: Foo,
    }

    #[maybe_async_cfg2::maybe(
        sync(feature="use_sync"), 
        async(feature="use_async")
    )]
    async fn func(b: Bar) {
        todo!()
    }
    } // content!
    ```
    After conversion:
    ```rust
    #[cfg(feature="use_sync")]
    struct StructSync {
        f: FooSync,
    }
    #[cfg(feature="use_async")]
    struct StructAsync {
        f: FooAsync,
    }

    #[cfg(feature="use_sync")]
    fn func_sync(b: BarSync) {
        todo!()
    }
    #[cfg(feature="use_async")]
    async fn func_async(b: BarAsync) {
        todo!()
    }
    ```

## Doctests
    
When writing doctests, they can be marked as applicable only in the corresponding code variant. 
To do this, specify `only_if(`_VARIANT_KEY_`)` in the doctest attributes. Then in all other
variants, this doctest will be replaced with an empty string.

```rust
#[maybe_async_cfg2::maybe(
    idents(Foo),
    sync(feature="use_sync"),
    async(feature="use_async")
)]
/// This is a structure. 
/// ```rust, only_if(sync)
/// let s = StructSync{ f: FooSync::new() };
/// ```
/// ```rust, only_if(async)
/// let s = StructAsync{ f: FooAsync::new().await };
/// ```
struct Struct {
    f: Foo,
}
```
After conversion:
```rust
#[cfg(feature="use_sync")]
/// This is a structure. 
/// ```rust, only_if(sync)
/// let s = StructSync{ f: FooSync::new() };
/// ```
///
struct StructSync {
f: FooSync,
}
#[cfg(feature="use_async")]
/// This is a structure. 
///
/// ```rust, only_if(async)
/// let s = StructAsync{ f: FooAsync::new().await };
/// ```
struct StructAsync {
    f: FooAsync,
}
```

## Examples

### Rust client for services

When implementing a Rust client for any service, like AWS S3, the higher-level API of async and
sync variants is almost the same, such as creating or deleting a bucket, retrieving an object,
etc.

The example `service_client` is a proof of concept that `maybe_async_cfg2` can eliminate
the need to write duplicate code for sync and async variants. The `is_sync` feature gate
allows toggling between sync and async AWZ3 client implementations.


## Acknowledgements

This crate is a maintained fork of:

- [nvksv/maybe-async-cfg](https://github.com/nvksv/maybe-async-cfg/)

which is a redesigned fork of these wonderful crates:

- [fMeow/maybe-async-rs](https://github.com/fMeow/maybe-async-rs)

- [marioortizmanero/maybe-async-rs](https://github.com/marioortizmanero/maybe-async-rs)

Thanks!


# License
MIT

<!-- cargo-sync-readme end -->
