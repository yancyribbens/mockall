// vim: tw=80
#![cfg_attr(feature = "nightly-docs", feature(doc_cfg))]

//! Examples of mock objects and their generated methods.
//!
//! This crate only exists to document the autogenerated methods of the
//! [`Mockall`] crate.  You should never use this crate directly.
//!
//! [`Mockall`]: ../mockall/index.html
//

#[cfg(all(feature = "nightly-docs", rustdoc))]
use mockall::*;

#[cfg(all(feature = "nightly-docs", rustdoc))]
#[automock]
pub trait Foo {
    /// A method with a `'static` return type
    fn foo(&self, x: i32, y: i16) -> i32;

    /// A method returning a reference
    fn bar(&self, x: i32) -> &i32;

    /// A method returning a mutable reference
    fn baz(&mut self, x: i32) -> &mut i32;

    /// A method returning a `'static` reference
    fn bean(&self) -> &'static i32;

    /// A static method
    fn bang(x: i32) -> i32;
}

#[cfg(all(feature = "nightly-docs", rustdoc))]
#[automock(mod mock_ffi;)]
extern "C" {
    /// A foreign "C" function
    pub fn ffi_func();
}