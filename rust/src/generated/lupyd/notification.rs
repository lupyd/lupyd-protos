// Automatically generated rust module for 'notification.proto' file

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
pub struct NotificationType<'a> {
    pub notification: lupyd::notification::mod_NotificationType::OneOfnotification<'a>,
}

impl<'a> MessageRead<'a> for NotificationType<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.notification = lupyd::notification::mod_NotificationType::OneOfnotification::follow(r.read_message::<lupyd::notification::FollowType>(bytes)?),
                Ok(18) => msg.notification = lupyd::notification::mod_NotificationType::OneOfnotification::comment(r.read_message::<lupyd::notification::CommentType>(bytes)?),
                Ok(26) => msg.notification = lupyd::notification::mod_NotificationType::OneOfnotification::like(r.read_message::<lupyd::notification::LikeType>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NotificationType<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.notification {
            lupyd::notification::mod_NotificationType::OneOfnotification::follow(ref m) => 1 + sizeof_len((m).get_size()),
            lupyd::notification::mod_NotificationType::OneOfnotification::comment(ref m) => 1 + sizeof_len((m).get_size()),
            lupyd::notification::mod_NotificationType::OneOfnotification::like(ref m) => 1 + sizeof_len((m).get_size()),
            lupyd::notification::mod_NotificationType::OneOfnotification::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.notification {            lupyd::notification::mod_NotificationType::OneOfnotification::follow(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            lupyd::notification::mod_NotificationType::OneOfnotification::comment(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            lupyd::notification::mod_NotificationType::OneOfnotification::like(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            lupyd::notification::mod_NotificationType::OneOfnotification::None => {},
    }        Ok(())
    }
}

pub mod mod_NotificationType {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfnotification<'a> {
    follow(lupyd::notification::FollowType<'a>),
    comment(lupyd::notification::CommentType<'a>),
    like(lupyd::notification::LikeType<'a>),
    None,
}

impl<'a> Default for OneOfnotification<'a> {
    fn default() -> Self {
        OneOfnotification::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FollowType<'a> {
    pub uname: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for FollowType<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.uname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FollowType<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.uname == "" { 0 } else { 1 + sizeof_len((&self.uname).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.uname != "" { w.write_with_tag(10, |w| w.write_string(&**&self.uname))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommentType<'a> {
    pub postId: Cow<'a, str>,
    pub commentedBy: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for CommentType<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.postId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.commentedBy = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CommentType<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.postId == "" { 0 } else { 1 + sizeof_len((&self.postId).len()) }
        + if self.commentedBy == "" { 0 } else { 1 + sizeof_len((&self.commentedBy).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.postId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.postId))?; }
        if self.commentedBy != "" { w.write_with_tag(18, |w| w.write_string(&**&self.commentedBy))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LikeType<'a> {
    pub postId: Cow<'a, str>,
    pub likedBy: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for LikeType<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.postId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.likedBy = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LikeType<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.postId == "" { 0 } else { 1 + sizeof_len((&self.postId).len()) }
        + if self.likedBy == "" { 0 } else { 1 + sizeof_len((&self.likedBy).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.postId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.postId))?; }
        if self.likedBy != "" { w.write_with_tag(18, |w| w.write_string(&**&self.likedBy))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Notification<'a> {
    pub id: Cow<'a, [u8]>,
    pub seen: bool,
    pub notificationType: Option<lupyd::notification::NotificationType<'a>>,
}

impl<'a> MessageRead<'a> for Notification<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.seen = r.read_bool(bytes)?,
                Ok(26) => msg.notificationType = Some(r.read_message::<lupyd::notification::NotificationType>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Notification<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.seen == false { 0 } else { 1 + sizeof_varint(*(&self.seen) as u64) }
        + self.notificationType.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.id))?; }
        if self.seen != false { w.write_with_tag(16, |w| w.write_bool(*&self.seen))?; }
        if let Some(ref s) = self.notificationType { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Notifications<'a> {
    pub notifications: Vec<lupyd::notification::Notification<'a>>,
}

impl<'a> MessageRead<'a> for Notifications<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.notifications.push(r.read_message::<lupyd::notification::Notification>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Notifications<'a> {
    fn get_size(&self) -> usize {
        0
        + self.notifications.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.notifications { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

