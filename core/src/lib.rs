/*
 * Copyright (c) 2021, Pavel Pletenev
 *
 * This file is part of can_bit_timings project
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/// This struct holds parts of bit timing for CAN.
/// Field descriptions quote descriptions from [this site](http://www.bittiming.can-wiki.info/).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CanBitTiming {
    /// **bs1** = **PROP_SEG** + **PHASE_SEG1**
    ///
    /// **PROP_SEG** is programmable to be 1, 2,... 8 Time Quanta long. 
    /// It is used to compensate for signal delays across the network.
    ///
    /// **PHASE_SEG1** is programmable to be 1,2, ... 8 Time Quanta long. 
    /// It is used to compensate for edge phase errors and may be lengthened 
    /// during resynchronization.
    pub bs1: u8,

    /// **bs2** = **PHASE_SEG2** (Seg 2) is the maximum of PHASE_SEG1 and 
    /// the Information Processing Time long. It is also used to compensate edge 
    /// phase errors and may be shortened during resynchronization. 
    /// For this the minimum value of PHASE_SEG2 is the value of SJW.
    /// Information Processing Time is less than or equal to 2 Time Quanta long.
    pub bs2: u8,

    /// Synchronization Jump Width
    pub sjw: u8,

    /// Prescaler value
    pub prescaler: u16
}

impl CanBitTiming {
    /// Converts [CanBitTiming] struct to a `u32` register
    /// as used by bxcan module.
    pub fn bxcan(&self) -> u32 {
        (self.sjw as u32 - 1 ) << 24
        | (self.bs1 as u32 - 1) << 16
        | (self.bs2 as u32 - 1 ) << 20
        | (self.prescaler as u32 -1 )
    }

    /// Outputs total time quanta used to clock a CAN bus module
    pub fn total_time_quanta(&self) -> u16 {
        self.bs1 as u16 + self.bs2 as u16 + self.sjw as u16
    }
}