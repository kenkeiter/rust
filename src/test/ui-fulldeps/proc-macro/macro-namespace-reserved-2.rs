// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// no-prefer-dynamic

#![crate_type = "proc-macro"]

extern crate proc_macro;
use proc_macro::*;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn my_macro_attr(input: TokenStream, _: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(MyTrait)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    input
}

fn check_bang1() {
    my_macro!(); //~ ERROR can't use a procedural macro from the same crate that defines it
}
fn check_bang2() {
    my_macro_attr!(); //~ ERROR can't use a procedural macro from the same crate that defines it
}
fn check_bang3() {
    MyTrait!(); //~ ERROR can't use a procedural macro from the same crate that defines it
}

#[my_macro] //~ ERROR can't use a procedural macro from the same crate that defines it
fn check_attr1() {}
#[my_macro_attr] //~ ERROR can't use a procedural macro from the same crate that defines it
fn check_attr2() {}
#[MyTrait] //~ ERROR can't use a procedural macro from the same crate that defines it
fn check_attr3() {}

#[derive(my_macro)] //~ ERROR can't use a procedural macro from the same crate that defines it
struct CheckDerive1;
#[derive(my_macro_attr)] //~ ERROR can't use a procedural macro from the same crate that defines it
struct CheckDerive2;
#[derive(MyTrait)] //~ ERROR can't use a procedural macro from the same crate that defines it
struct CheckDerive3;
