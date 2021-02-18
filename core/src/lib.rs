/*
 * Copyright (c) 2021, Pavel Pletenev
 *
 * This file is part of can_bit_timings project
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CanBitTiming {
    pub bs1: u8,	// 1-16
    pub bs2: u8,	// 1-8
    pub sjw: u8,
    pub prescaler: u16
}

impl CanBitTiming {
    pub fn bxcan(&self) -> u32 {
        (self.sjw as u32 - 1 ) << 24
        | (self.bs1 as u32 - 1) << 16
        | (self.bs2 as u32 - 1 ) << 20
        | (self.prescaler as u32 -1 )
    }
}