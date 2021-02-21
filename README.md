# can_bit_timings

[![CI](https://github.com/ASMfreaK/can_bit_timings/actions/workflows/ci.yml/badge.svg)](https://github.com/ASMfreaK/can_bit_timings/actions/workflows/ci.yml)

This is a procedural macro (originally a `constexpr` function (`const fn`)) to calculate CAN bus timings for different STM32 MCUs bxcan module. It can be useful to calculate timings for different MCUs, but one should write a function formatting the calculated values into appropriate register value.

This project is based on a similar piece of code from [modm](https://github.com/modm/modm) project. You can find out more on bit timings [here](http://www.bittiming.can-wiki.info/)

## Example:

```
#[no-std]
use can_bus_timings::can_timings_bxcan;

const fn can0_timings() -> u32{
    can_timings_bxcan!(10.mhz(), 1.mhz())
}

fn main(){
    // ... CAN hardware initialization
    can0_timings();
}
```
