/*
 * Copyright (c) 2021, Pavel Pletenev
 *
 * This file is part of can_bit_timings project
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use assert_hex::assert_eq_hex as assert_eq;

use can_bit_timings::can_timings_bxcan;

fn main(){
    assert_eq!(
        can_timings_bxcan!(10_000_000, 1000_000), 
        0x00070000);//	0.0000	1	10	8	1	90.0	 
    assert_eq!(
        can_timings_bxcan!(10_000_000, 500_000), 
        0x2f0000);//	0.0000	2	10	8	1	90.0	 
    assert_eq!(
        can_timings_bxcan!(10_000_000, 250_000), 
        0x2f0001);//	0.0000	4	10	8	1	90.0	 
    assert_eq!(
        can_timings_bxcan!(10_000_000, 125_000), 
        0x2f0003);//	0.0000	5	16	13	2	87.5	 
}