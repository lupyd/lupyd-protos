// Automatically generated rust module for 'chats.proto' file

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
pub struct ChatSession<'a> {
    pub ts: u64,
    pub version: u32,
    pub user1: Cow<'a, str>,
    pub user2: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ChatSession<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ts = r.read_uint64(bytes)?,
                Ok(16) => msg.version = r.read_uint32(bytes)?,
                Ok(26) => msg.user1 = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.user2 = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChatSession<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.ts == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.ts) as u64) }
        + if self.version == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.user1 == "" { 0 } else { 1 + sizeof_len((&self.user1).len()) }
        + if self.user2 == "" { 0 } else { 1 + sizeof_len((&self.user2).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ts != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.ts))?; }
        if self.version != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.version))?; }
        if self.user1 != "" { w.write_with_tag(26, |w| w.write_string(&**&self.user1))?; }
        if self.user2 != "" { w.write_with_tag(34, |w| w.write_string(&**&self.user2))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatSessions<'a> {
    pub sessions: Vec<lupyd::chat::ChatSession<'a>>,
}

impl<'a> MessageRead<'a> for ChatSessions<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.sessions.push(r.read_message::<lupyd::chat::ChatSession>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChatSessions<'a> {
    fn get_size(&self) -> usize {
        0
        + self.sessions.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.sessions { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatKeys<'a> {
    pub keys: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ChatKeys<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.keys.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChatKeys<'a> {
    fn get_size(&self) -> usize {
        0
        + self.keys.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.keys { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatKey<'a> {
    pub key: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ChatKey<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChatKey<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.key == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.key).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.key))?; }
        Ok(())
    }
}

