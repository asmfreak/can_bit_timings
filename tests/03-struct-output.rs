/*
 * Copyright (c) 2021, Pavel Pletenev
 *
 * This file is part of can_bit_timings project
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use can_bit_timings::{CanBitTiming as CanBits,can_timings};

fn main(){
    assert_eq!(
        can_timings!(10_000_000, 1000_000), 
        CanBits{sjw: 1, bs1: 8, bs2: 1, prescaler: 1});
    //	0.0000	1	10	8	1	90.0 
}