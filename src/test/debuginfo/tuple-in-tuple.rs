// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-android: FIXME(#10381)
// min-lldb-version: 310

// compile-flags:-g

// === GDB TESTS ===================================================================================

// gdb-command:run

// gdb-command:print no_padding1
// gdb-check:$1 = {{0, 1}, 2, 3}
// gdb-command:print no_padding2
// gdb-check:$2 = {4, {5, 6}, 7}
// gdb-command:print no_padding3
// gdb-check:$3 = {8, 9, {10, 11}}

// gdb-command:print internal_padding1
// gdb-check:$4 = {12, {13, 14}}
// gdb-command:print internal_padding2
// gdb-check:$5 = {15, {16, 17}}

// gdb-command:print padding_at_end1
// gdb-check:$6 = {18, {19, 20}}
// gdb-command:print padding_at_end2
// gdb-check:$7 = {{21, 22}, 23}


// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:print no_padding1
// lldb-check:[...]$0 = ((0, 1), 2, 3)
// lldb-command:print no_padding2
// lldb-check:[...]$1 = (4, (5, 6), 7)
// lldb-command:print no_padding3
// lldb-check:[...]$2 = (8, 9, (10, 11))

// lldb-command:print internal_padding1
// lldb-check:[...]$3 = (12, (13, 14))
// lldb-command:print internal_padding2
// lldb-check:[...]$4 = (15, (16, 17))

// lldb-command:print padding_at_end1
// lldb-check:[...]$5 = (18, (19, 20))
// lldb-command:print padding_at_end2
// lldb-check:[...]$6 = ((21, 22), 23)

#![allow(unused_variables)]
#![omit_gdb_pretty_printer_section]

fn main() {
    let no_padding1: ((u32, u32), u32, u32) = ((0, 1), 2, 3);
    let no_padding2: (u32, (u32, u32), u32) = (4, (5, 6), 7);
    let no_padding3: (u32, u32, (u32, u32)) = (8, 9, (10, 11));

    let internal_padding1: (i16, (i32, i32)) = (12, (13, 14));
    let internal_padding2: (i16, (i16, i32)) = (15, (16, 17));

    let padding_at_end1: (i32, (i32, i16)) = (18, (19, 20));
    let padding_at_end2: ((i32, i16), i32) = ((21, 22), 23);

    zzz(); // #break
}

fn zzz() {()}
