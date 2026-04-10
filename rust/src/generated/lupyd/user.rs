// Automatically generated rust module for 'user.proto' file

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
pub struct BoolValue {
    pub val: bool,
}

impl<'a> MessageRead<'a> for BoolValue {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.val = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BoolValue {
    fn get_size(&self) -> usize {
        0
        + if self.val == false { 0 } else { 1 + sizeof_varint(*(&self.val) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.val != false { w.write_with_tag(8, |w| w.write_bool(*&self.val))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FullUser<'a> {
    pub uname: Cow<'a, str>,
    pub bio: Cow<'a, [u8]>,
    pub followers: i32,
    pub settings: i32,
    pub uid: Cow<'a, str>,
    pub credits: f32,
}

impl<'a> MessageRead<'a> for FullUser<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.uname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.bio = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.followers = r.read_int32(bytes)?,
                Ok(32) => msg.settings = r.read_int32(bytes)?,
                Ok(42) => msg.uid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(53) => msg.credits = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FullUser<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uname == "" { 0 } else { 1 + sizeof_len((&self.uname).len()) }
        + if self.bio == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.bio).len()) }
        + if self.followers == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.followers) as u64) }
        + if self.settings == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.settings) as u64) }
        + if self.uid == "" { 0 } else { 1 + sizeof_len((&self.uid).len()) }
        + if self.credits == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uname != "" { w.write_with_tag(10, |w| w.write_string(&**&self.uname))?; }
        if self.bio != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.bio))?; }
        if self.followers != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.followers))?; }
        if self.settings != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.settings))?; }
        if self.uid != "" { w.write_with_tag(42, |w| w.write_string(&**&self.uid))?; }
        if self.credits != 0f32 { w.write_with_tag(53, |w| w.write_float(*&self.credits))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FullUserWithProfile<'a> {
    pub user: Option<lupyd::user::FullUser<'a>>,
    pub pfp: Option<lupyd::post::File<'a>>,
}

impl<'a> MessageRead<'a> for FullUserWithProfile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.user = Some(r.read_message::<lupyd::user::FullUser>(bytes)?),
                Ok(18) => msg.pfp = Some(r.read_message::<lupyd::post::File>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FullUserWithProfile<'a> {
    fn get_size(&self) -> usize {
        0
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.pfp.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.user { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pfp { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FullUsers<'a> {
    pub users: Vec<lupyd::user::FullUser<'a>>,
}

impl<'a> MessageRead<'a> for FullUsers<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.users.push(r.read_message::<lupyd::user::FullUser>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FullUsers<'a> {
    fn get_size(&self) -> usize {
        0
        + self.users.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.users { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Users<'a> {
    pub users: Vec<lupyd::user::User<'a>>,
}

impl<'a> MessageRead<'a> for Users<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.users.push(r.read_message::<lupyd::user::User>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Users<'a> {
    fn get_size(&self) -> usize {
        0
        + self.users.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.users { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateUserInfo<'a> {
    pub bio: Option<lupyd::post::PostBody<'a>>,
    pub settings: i32,
}

impl<'a> MessageRead<'a> for UpdateUserInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.bio = Some(r.read_message::<lupyd::post::PostBody>(bytes)?),
                Ok(32) => msg.settings = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdateUserInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.bio.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.settings == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.settings) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.bio { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.settings != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.settings))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct User<'a> {
    pub uname: Cow<'a, str>,
    pub bio: Cow<'a, [u8]>,
    pub settings: i32,
    pub followers: i32,
}

impl<'a> MessageRead<'a> for User<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.uname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.bio = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.settings = r.read_int32(bytes)?,
                Ok(32) => msg.followers = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for User<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uname == "" { 0 } else { 1 + sizeof_len((&self.uname).len()) }
        + if self.bio == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.bio).len()) }
        + if self.settings == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.settings) as u64) }
        + if self.followers == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.followers) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uname != "" { w.write_with_tag(10, |w| w.write_string(&**&self.uname))?; }
        if self.bio != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.bio))?; }
        if self.settings != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.settings))?; }
        if self.followers != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.followers))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relation<'a> {
    pub uname: Cow<'a, str>,
    pub relation: bool,
}

impl<'a> MessageRead<'a> for Relation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.uname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.relation = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Relation<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uname == "" { 0 } else { 1 + sizeof_len((&self.uname).len()) }
        + if self.relation == false { 0 } else { 1 + sizeof_varint(*(&self.relation) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uname != "" { w.write_with_tag(10, |w| w.write_string(&**&self.uname))?; }
        if self.relation != false { w.write_with_tag(16, |w| w.write_bool(*&self.relation))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relations<'a> {
    pub relations: Vec<lupyd::user::Relation<'a>>,
}

impl<'a> MessageRead<'a> for Relations<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.relations.push(r.read_message::<lupyd::user::Relation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Relations<'a> {
    fn get_size(&self) -> usize {
        0
        + self.relations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.relations { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

