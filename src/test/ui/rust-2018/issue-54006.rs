// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// edition:2018

#![no_std]
#![crate_type = "lib"]

use alloc::vec;
//~^ ERROR unresolved import `alloc`

pub fn foo() {
    let mut xs = vec![];
    //~^ ERROR cannot determine resolution for the macro `vec`
    xs.push(0);
}
