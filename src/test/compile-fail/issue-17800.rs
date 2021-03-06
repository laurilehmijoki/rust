// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum MyOption<T> {
    MySome(T),
    MyNone,
}

fn main() {
    match MyOption::MySome(42is) {
        MyOption::MySome { x: 42is } => (),
        //~^ ERROR `MyOption::MySome` does not name a struct or a struct variant
        _ => (),
    }
}
