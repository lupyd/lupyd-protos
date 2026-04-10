// Automatically generated rust module for 'ads.proto' file

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
pub struct Ad<'a> {
    pub id: i64,
    pub by: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub body: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Ad<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(18) => msg.by = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.body = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Ad<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.by == "" { 0 } else { 1 + sizeof_len((&self.by).len()) }
        + if self.title == "" { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.body == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.body).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.id))?; }
        if self.by != "" { w.write_with_tag(18, |w| w.write_string(&**&self.by))?; }
        if self.title != "" { w.write_with_tag(26, |w| w.write_string(&**&self.title))?; }
        if self.body != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.body))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AdResponse<'a> {
    pub ads: Vec<lupyd::ads::Ad<'a>>,
}

impl<'a> MessageRead<'a> for AdResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ads.push(r.read_message::<lupyd::ads::Ad>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AdResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ads.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ads { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AdRequest<'a> {
    pub tags: Vec<Cow<'a, str>>,
    pub metadata: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for AdRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tags.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.metadata = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AdRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.tags.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.metadata == "" { 0 } else { 1 + sizeof_len((&self.metadata).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tags { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if self.metadata != "" { w.write_with_tag(18, |w| w.write_string(&**&self.metadata))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateAdRequest<'a> {
    pub title: Cow<'a, str>,
    pub body: Cow<'a, [u8]>,
    pub expiry_ts: u64,
    pub max_views: u64,
    pub max_clicks: u64,
}

impl<'a> MessageRead<'a> for CreateAdRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.body = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.expiry_ts = r.read_uint64(bytes)?,
                Ok(40) => msg.max_views = r.read_uint64(bytes)?,
                Ok(48) => msg.max_clicks = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreateAdRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.title == "" { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.body == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.body).len()) }
        + if self.expiry_ts == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.expiry_ts) as u64) }
        + if self.max_views == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.max_views) as u64) }
        + if self.max_clicks == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.max_clicks) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.title != "" { w.write_with_tag(10, |w| w.write_string(&**&self.title))?; }
        if self.body != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.body))?; }
        if self.expiry_ts != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.expiry_ts))?; }
        if self.max_views != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.max_views))?; }
        if self.max_clicks != 0u64 { w.write_with_tag(48, |w| w.write_uint64(*&self.max_clicks))?; }
        Ok(())
    }
}

