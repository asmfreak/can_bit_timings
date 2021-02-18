/*
 * Copyright (c) 2021, Pavel Pletenev
 *
 * This file is part of can_bit_timings project
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-simple-bit-timings.rs");
    t.pass("tests/02-frequency-with-mhz.rs");
    t.pass("tests/03-struct-output.rs");
    t.pass("tests/04-optargs.rs");
    //t.compile_fail("tests/04-multiple-of-8bits.rs");
    //t.pass("tests/05-accessor-signatures.rs");
    //t.pass("tests/06-enums.rs");
    //t.pass("tests/07-optional-discriminant.rs");
    //t.compile_fail("tests/08-non-power-of-two.rs");
    //t.compile_fail("tests/09-variant-out-of-range.rs");
    //t.pass("tests/10-bits-attribute.rs");
    //t.compile_fail("tests/11-bits-attribute-wrong.rs");
    //t.pass("tests/12-accessors-edge.rs");
}