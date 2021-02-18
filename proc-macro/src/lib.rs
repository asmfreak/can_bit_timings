/*
 * Copyright (c) 2021, Pavel Pletenev
 *
 * This file is part of can_bit_timings project
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate proc_macro;
use can_bit_timings_core::CanBitTiming;
use proc_macro::TokenStream;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use syn::{
    Expr, ExprLit, ExprMethodCall, ExprAssign, ExprPath,
    Lit,
    Ident, Token, parse_macro_input};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use quote::quote;
use std::convert::TryFrom;



fn round_uint32(f: f64) -> u32 {
    let f_int = f as u32;
    if f - (f_int as f64) > 0.5 {
        f_int + 1
    }else{
        f_int
    }
}

struct CanBits(CanBitTiming, Percents);

#[derive(Clone, Copy)]
struct Frequency(u32);

#[derive(Clone, Copy)]
struct Ratio(f64);

#[derive(Clone, Copy)]
struct Percents(f64);

impl From<Percents> for Ratio{
    fn from(p: Percents) -> Self{
        Self(p.0 as f64 / 100.0)
    }
}

impl From<Ratio> for Percents{
    fn from(r: Ratio) -> Self{
        Self(r.0 as f64 * 100.0)
    }
}

struct CanBitsArgs{
    clk:Frequency, 
    bitrate : Frequency, 
    midpoint: Ratio, 
    tolerance: Percents
}

impl CanBits {
/*
 * Copyright (c) 2016, Antal Szabó
 * Copyright (c) 2016, Kevin Läufer
 * Copyright (c) 2016-2018, Niklas Hauser
 * Copyright (c) 2018, Christopher Durand
 *
 * The following algorithm was taken from the modm project and
 * translated into Rust by Pavel Pletenev
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
/**
 * CAN Bit Timing
 *
 * Example for CAN bit timing:
 *   CLK on APB1 = 36 MHz
 *   BaudRate = 125 kBPs = 1 / NominalBitTime
 *   NominalBitTime = 8uS = tq + tBS1 + tBS2
 * with:
 *   tBS1 = tq * (TS1[3:0] + 1) = 12 * tq
 *   tBS2 = tq * (TS2[2:0] + 1) = 5 * tq
 *   tq = (BRP[9:0] + 1) * tPCLK
 * where tq refers to the Time quantum
 *   tPCLK = time period of the APB clock = 1 / 36 MHz
 *
 * STM32F1xx   tPCLK = 1 / 36 MHz
 * STM32F20x   tPCLK = 1 / 30 MHz
 * STM32F40x   tPCLK = 1 / 42 MHz
 *
 *
 */
    fn best(clk:Frequency, bitrate : Frequency, midpoint:Ratio) -> (CanBitTiming, Percents){
        const MIN_BS1_BS2 : u8 = 8;
        const MAX_BS1_BS2 : u8 = 25;

        let mut min_error: f64 = 10_000.0;
        let mut best_prescaler: u16 = 0;
        let mut best_bs1_bs2:u8 = 0;

        for bs1_bs2 in MIN_BS1_BS2..(MAX_BS1_BS2+1) {
            let ideal_prescaler = (clk.0 as f64) / (
                (bitrate.0 as f64) * ((1 + bs1_bs2) as f64));
            let int_prescaler = round_uint32(ideal_prescaler);
            let error = (1. - (int_prescaler as f64)/ideal_prescaler).abs();
            if error <= min_error {
                // eprintln!("tqs: {}  psc: {} err: {}", bs1_bs2+1, int_prescaler, min_error);
                best_prescaler = int_prescaler as u16;
                min_error = error;
                best_bs1_bs2 = bs1_bs2;
            }
        }

        let bs2 = (midpoint.0 * (best_bs1_bs2 as f64 + 1.)).floor() as u8;
        let bs1 = best_bs1_bs2 - bs2;
        eprintln!(
            "Selecting for {} Hz {} Hz  tqs: {} {},  psc: {} err: {}", 
            clk.0, bitrate.0, bs1, bs2, best_prescaler, min_error);
        (
            CanBitTiming{bs1, bs2, sjw: 1, prescaler: best_prescaler}, 
            Percents(min_error * 100.0))
    }
    fn new(clk:Frequency, bitrate : Frequency, midpoint: Ratio, tolerance: Percents) -> Self{
        let (best, min_error) = CanBits::best(clk, bitrate, midpoint);
        // check assertions
        if (best.prescaler as i8) <= 0 {
            abort_call_site!("CAN bitrate is too high for standard bit timings!");
        }
        if best.prescaler > ((1 << 10) -1) {
            abort_call_site!("Prescaler value too large!");
        }
        if min_error.0 > tolerance.0 {
            abort_call_site!("Error is too high for this configuration ({} %)!", min_error.0);
        }
        CanBits(best, min_error)
    }
}

impl Frequency {
    fn mul_from_ident(i: &Ident) -> u32{
        let name = i.to_string();
        if name == "mhz" { 1_000_000 }
        else if name == "khz" { 1_000 }
        else if name == "hz" { 1 }
        else if name == "bps" { 1 }
        else { abort!(i, "Unknown multiplier") }
    }
}

impl TryFrom<&Expr> for Frequency{
    type Error = syn::Error;
    fn try_from(value: &Expr) -> Result<Self>{
        Ok(match value {
            Expr::Lit(ExprLit{lit: Lit::Int(lit),..}) => {
                Self(lit.base10_parse()?)
            }
            Expr::MethodCall(ExprMethodCall{
                receiver, method, args ,..
            }) => {
                let mut ret = Frequency::try_from(receiver.as_ref())?;
                if args.len() != 0 { abort!(args, "Expected no arguments!");}
                ret.0 *= Frequency::mul_from_ident(method);
                ret
            }
            _ => {abort!(value, "Expected int literal or `int.unit()` expression");}
        })
    }
}

impl TryFrom<&Expr> for Ratio{
    type Error = syn::Error;
    fn try_from(value: &Expr) -> Result<Self>{
        Ok(match value {
            Expr::Lit(ExprLit{lit,..}) => {
                match lit {
                    Lit::Int(lit) => Self(lit.base10_parse::<u32>()? as f64),
                    Lit::Float(lit) => Self(lit.base10_parse()?),
                    _ => abort!(lit, "Expected int or float literal")
                }
            }
            _ => {abort!(value, "Expected int or float literal");}
        })
    }
}

impl TryFrom<&Expr> for Percents{
    type Error = syn::Error;
    fn try_from(value: &Expr) -> Result<Self>{
        if let Ok(r) = Ratio::try_from(value){
            return Ok(r.into())
        }
        Ok(match value {
            Expr::MethodCall(ExprMethodCall{
                receiver, method, args ,..
            }) => {
                let ret = Ratio::try_from(receiver.as_ref())?;
                if args.len() != 0 { abort!(args, "Expected no arguments!");}
                if method.to_string() != "pct"{ abort!(method, "Expected pct!")}
                Percents(ret.0)
            }
            _ => {abort!(value, "Expected float literal");}
        })
    }
}


impl Parse for CanBitsArgs{
    fn parse(input: ParseStream) -> Result<Self> {
        let mut ret = Self{
            clk: Frequency(0), 
            bitrate: Frequency(0),
            midpoint: Ratio(0.175),
            tolerance: Percents(0.5),
        };
        let input= 
            Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        for (i, expr) in (0..(input.len())).zip(input.iter()){
            if let Expr::Assign(ExprAssign{left, right, ..}) = expr {
                if let Expr::Path(ExprPath{path,..}) = left.as_ref(){
                    let var = path.get_ident().unwrap_or_else(
                        || abort!(path, "This should be an indentifier"));
                    if var == "clk" {
                        ret.clk = Frequency::try_from(right.as_ref())?;
                    } else if var == "bitrate" {
                        ret.bitrate = Frequency::try_from(right.as_ref())?;
                    } else if var == "midpoint" {
                        ret.midpoint = Ratio::try_from(right.as_ref())?;
                    } else if var == "tolerance" {
                        ret.tolerance = Percents::try_from(right.as_ref())?;
                    }
                }else {
                    abort!(left, "Unknown type of expression!")
                }
            }else if let Ok(f) = Frequency::try_from(expr){
                if i == 0{
                    ret.clk = f;
                }else if i == 1{
                    ret.bitrate = f;
                };
            }
        }
        Ok(ret)
    }
}

impl Parse for CanBits {
    fn parse(input: ParseStream) -> Result<Self> {
        let CanBitsArgs { 
            clk, bitrate, midpoint, tolerance 
        } = input.parse::<CanBitsArgs>()?;
        
        Ok(CanBits::new(clk,bitrate,midpoint, tolerance))
    }
}


#[proc_macro_error]
#[proc_macro]
pub fn can_timings(item: TokenStream) -> TokenStream {
    let CanBits(CanBitTiming { 
        bs1, bs2, sjw, prescaler, 
    }, _) = parse_macro_input!(item as CanBits);
    TokenStream::from(quote!(::can_bit_timings_core::CanBitTiming{
        sjw: #sjw,
        bs1: #bs1,
        bs2: #bs2,
        prescaler: #prescaler,
    }))
}

#[proc_macro_error]
#[proc_macro]
pub fn can_timings_bxcan(item: TokenStream) -> TokenStream {
    let rf = parse_macro_input!(item as CanBits);
    let result = rf.0.bxcan();
    TokenStream::from(quote!(#result))
}