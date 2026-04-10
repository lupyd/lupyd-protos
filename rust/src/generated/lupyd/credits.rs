// Automatically generated rust module for 'credits.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PurchaseRequest<'a> {
    pub from: Cow<'a, str>,
    pub by: Cow<'a, str>,
    pub credits: f64,
    pub product_id: i64,
}

impl<'a> MessageRead<'a> for PurchaseRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.from = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.by = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(25) => msg.credits = r.read_double(bytes)?,
                Ok(32) => msg.product_id = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PurchaseRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.from == "" { 0 } else { 1 + sizeof_len((&self.from).len()) }
        + if self.by == "" { 0 } else { 1 + sizeof_len((&self.by).len()) }
        + if self.credits == 0f64 { 0 } else { 1 + 8 }
        + if self.product_id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.product_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.from != "" { w.write_with_tag(10, |w| w.write_string(&**&self.from))?; }
        if self.by != "" { w.write_with_tag(18, |w| w.write_string(&**&self.by))?; }
        if self.credits != 0f64 { w.write_with_tag(25, |w| w.write_double(*&self.credits))?; }
        if self.product_id != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.product_id))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AddCredits<'a> {
    pub from: Cow<'a, str>,
    pub to: Cow<'a, str>,
    pub credits: f64,
}

impl<'a> MessageRead<'a> for AddCredits<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.from = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.to = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(25) => msg.credits = r.read_double(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AddCredits<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.from == "" { 0 } else { 1 + sizeof_len((&self.from).len()) }
        + if self.to == "" { 0 } else { 1 + sizeof_len((&self.to).len()) }
        + if self.credits == 0f64 { 0 } else { 1 + 8 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.from != "" { w.write_with_tag(10, |w| w.write_string(&**&self.from))?; }
        if self.to != "" { w.write_with_tag(18, |w| w.write_string(&**&self.to))?; }
        if self.credits != 0f64 { w.write_with_tag(25, |w| w.write_double(*&self.credits))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct WithdrawCredits<'a> {
    pub credits: f64,
    pub by: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for WithdrawCredits<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.credits = r.read_double(bytes)?,
                Ok(18) => msg.by = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for WithdrawCredits<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.credits == 0f64 { 0 } else { 1 + 8 }
        + if self.by == "" { 0 } else { 1 + sizeof_len((&self.by).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.credits != 0f64 { w.write_with_tag(9, |w| w.write_double(*&self.credits))?; }
        if self.by != "" { w.write_with_tag(18, |w| w.write_string(&**&self.by))?; }
        Ok(())
    }
}

