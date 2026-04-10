// Automatically generated rust module for 'post.proto' file

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum POST_TYPE {
    NOT_DEFINED = 0,
    SAFE = 1,
    ANONYMOUS = 2,
    NSFW = 4,
    DANGEROUS = 8,
}

impl Default for POST_TYPE {
    fn default() -> Self {
        POST_TYPE::NOT_DEFINED
    }
}

impl From<i32> for POST_TYPE {
    fn from(i: i32) -> Self {
        match i {
            0 => POST_TYPE::NOT_DEFINED,
            1 => POST_TYPE::SAFE,
            2 => POST_TYPE::ANONYMOUS,
            4 => POST_TYPE::NSFW,
            8 => POST_TYPE::DANGEROUS,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for POST_TYPE {
    fn from(s: &'a str) -> Self {
        match s {
            "NOT_DEFINED" => POST_TYPE::NOT_DEFINED,
            "SAFE" => POST_TYPE::SAFE,
            "ANONYMOUS" => POST_TYPE::ANONYMOUS,
            "NSFW" => POST_TYPE::NSFW,
            "DANGEROUS" => POST_TYPE::DANGEROUS,
            _ => Self::default(),
        }
    }
}

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
pub struct PostBody<'a> {
    pub body: lupyd::post::mod_PostBody::OneOfbody<'a>,
}

impl<'a> MessageRead<'a> for PostBody<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.body = lupyd::post::mod_PostBody::OneOfbody::plainText(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.body = lupyd::post::mod_PostBody::OneOfbody::markdown(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.body = lupyd::post::mod_PostBody::OneOfbody::elements(r.read_message::<lupyd::markdown::Elements>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PostBody<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.body {
            lupyd::post::mod_PostBody::OneOfbody::plainText(ref m) => 1 + sizeof_len((m).len()),
            lupyd::post::mod_PostBody::OneOfbody::markdown(ref m) => 1 + sizeof_len((m).len()),
            lupyd::post::mod_PostBody::OneOfbody::elements(ref m) => 1 + sizeof_len((m).get_size()),
            lupyd::post::mod_PostBody::OneOfbody::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.body {            lupyd::post::mod_PostBody::OneOfbody::plainText(ref m) => { w.write_with_tag(10, |w| w.write_string(&**m))? },
            lupyd::post::mod_PostBody::OneOfbody::markdown(ref m) => { w.write_with_tag(18, |w| w.write_string(&**m))? },
            lupyd::post::mod_PostBody::OneOfbody::elements(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            lupyd::post::mod_PostBody::OneOfbody::None => {},
    }        Ok(())
    }
}

pub mod mod_PostBody {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfbody<'a> {
    plainText(Cow<'a, str>),
    markdown(Cow<'a, str>),
    elements(lupyd::markdown::Elements<'a>),
    None,
}

impl<'a> Default for OneOfbody<'a> {
    fn default() -> Self {
        OneOfbody::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostBodies<'a> {
    pub bodies: Vec<lupyd::post::PostBody<'a>>,
}

impl<'a> MessageRead<'a> for PostBodies<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.bodies.push(r.read_message::<lupyd::post::PostBody>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PostBodies<'a> {
    fn get_size(&self) -> usize {
        0
        + self.bodies.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.bodies { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FullPost<'a> {
    pub id: Cow<'a, [u8]>,
    pub title: Cow<'a, str>,
    pub by: Cow<'a, str>,
    pub post_type: u32,
    pub expiry: u64,
    pub replying_to: Cow<'a, [u8]>,
    pub body: Cow<'a, [u8]>,
    pub replies: u64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub is_memory: bool,
    pub vote: Option<lupyd::post::BoolValue>,
}

impl<'a> MessageRead<'a> for FullPost<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.by = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.post_type = r.read_uint32(bytes)?,
                Ok(40) => msg.expiry = r.read_uint64(bytes)?,
                Ok(50) => msg.replying_to = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.body = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(72) => msg.replies = r.read_uint64(bytes)?,
                Ok(80) => msg.upvotes = r.read_int64(bytes)?,
                Ok(88) => msg.downvotes = r.read_int64(bytes)?,
                Ok(96) => msg.is_memory = r.read_bool(bytes)?,
                Ok(106) => msg.vote = Some(r.read_message::<lupyd::post::BoolValue>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FullPost<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.title == "" { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.by == "" { 0 } else { 1 + sizeof_len((&self.by).len()) }
        + if self.post_type == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.post_type) as u64) }
        + if self.expiry == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.expiry) as u64) }
        + if self.replying_to == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.replying_to).len()) }
        + if self.body == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.body).len()) }
        + if self.replies == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.replies) as u64) }
        + if self.upvotes == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.upvotes) as u64) }
        + if self.downvotes == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.downvotes) as u64) }
        + if self.is_memory == false { 0 } else { 1 + sizeof_varint(*(&self.is_memory) as u64) }
        + self.vote.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.id))?; }
        if self.title != "" { w.write_with_tag(18, |w| w.write_string(&**&self.title))?; }
        if self.by != "" { w.write_with_tag(26, |w| w.write_string(&**&self.by))?; }
        if self.post_type != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.post_type))?; }
        if self.expiry != 0u64 { w.write_with_tag(40, |w| w.write_uint64(*&self.expiry))?; }
        if self.replying_to != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.replying_to))?; }
        if self.body != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.body))?; }
        if self.replies != 0u64 { w.write_with_tag(72, |w| w.write_uint64(*&self.replies))?; }
        if self.upvotes != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.upvotes))?; }
        if self.downvotes != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.downvotes))?; }
        if self.is_memory != false { w.write_with_tag(96, |w| w.write_bool(*&self.is_memory))?; }
        if let Some(ref s) = self.vote { w.write_with_tag(106, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreatePostDetails<'a> {
    pub title: Cow<'a, str>,
    pub body: Option<lupyd::post::PostBody<'a>>,
    pub expiry: u64,
    pub post_type: i32,
    pub is_memory: bool,
    pub replying_to: Cow<'a, [u8]>,
    pub files: Vec<Cow<'a, str>>,
    pub editing_from: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for CreatePostDetails<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.body = Some(r.read_message::<lupyd::post::PostBody>(bytes)?),
                Ok(24) => msg.expiry = r.read_uint64(bytes)?,
                Ok(32) => msg.post_type = r.read_int32(bytes)?,
                Ok(40) => msg.is_memory = r.read_bool(bytes)?,
                Ok(50) => msg.replying_to = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.files.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.editing_from = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreatePostDetails<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.title == "" { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + self.body.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.expiry == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.expiry) as u64) }
        + if self.post_type == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.post_type) as u64) }
        + if self.is_memory == false { 0 } else { 1 + sizeof_varint(*(&self.is_memory) as u64) }
        + if self.replying_to == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.replying_to).len()) }
        + self.files.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.editing_from == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.editing_from).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.title != "" { w.write_with_tag(10, |w| w.write_string(&**&self.title))?; }
        if let Some(ref s) = self.body { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.expiry != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.expiry))?; }
        if self.post_type != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.post_type))?; }
        if self.is_memory != false { w.write_with_tag(40, |w| w.write_bool(*&self.is_memory))?; }
        if self.replying_to != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.replying_to))?; }
        for s in &self.files { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if self.editing_from != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.editing_from))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreatePostWithFiles<'a> {
    pub fields: Option<lupyd::post::CreatePostDetails<'a>>,
    pub files: Vec<lupyd::post::File<'a>>,
}

impl<'a> MessageRead<'a> for CreatePostWithFiles<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fields = Some(r.read_message::<lupyd::post::CreatePostDetails>(bytes)?),
                Ok(18) => msg.files.push(r.read_message::<lupyd::post::File>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreatePostWithFiles<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.files.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fields { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.files { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct File<'a> {
    pub name: Cow<'a, str>,
    pub mimeType: Cow<'a, str>,
    pub length: u64,
}

impl<'a> MessageRead<'a> for File<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.mimeType = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.length = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for File<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.mimeType == "" { 0 } else { 1 + sizeof_len((&self.mimeType).len()) }
        + if self.length == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.length) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.mimeType != "" { w.write_with_tag(18, |w| w.write_string(&**&self.mimeType))?; }
        if self.length != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.length))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetPostsData<'a> {
    pub allowed_post_types: u32,
    pub by: Vec<Cow<'a, str>>,
    pub all_posts: bool,
    pub cursor: Cow<'a, [u8]>,
    pub tags: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for GetPostsData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.allowed_post_types = r.read_uint32(bytes)?,
                Ok(18) => msg.by.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.all_posts = r.read_bool(bytes)?,
                Ok(34) => msg.cursor = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.tags = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GetPostsData<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.allowed_post_types == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.allowed_post_types) as u64) }
        + self.by.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.all_posts == false { 0 } else { 1 + sizeof_varint(*(&self.all_posts) as u64) }
        + if self.cursor == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.cursor).len()) }
        + if self.tags == "" { 0 } else { 1 + sizeof_len((&self.tags).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.allowed_post_types != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.allowed_post_types))?; }
        for s in &self.by { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if self.all_posts != false { w.write_with_tag(24, |w| w.write_bool(*&self.all_posts))?; }
        if self.cursor != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.cursor))?; }
        if self.tags != "" { w.write_with_tag(42, |w| w.write_string(&**&self.tags))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FullPosts<'a> {
    pub posts: Vec<lupyd::post::FullPost<'a>>,
}

impl<'a> MessageRead<'a> for FullPosts<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.posts.push(r.read_message::<lupyd::post::FullPost>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FullPosts<'a> {
    fn get_size(&self) -> usize {
        0
        + self.posts.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.posts { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Votes<'a> {
    pub votes: Vec<lupyd::post::Vote<'a>>,
}

impl<'a> MessageRead<'a> for Votes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.votes.push(r.read_message::<lupyd::post::Vote>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Votes<'a> {
    fn get_size(&self) -> usize {
        0
        + self.votes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.votes { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vote<'a> {
    pub id: Cow<'a, [u8]>,
    pub val: Option<lupyd::post::BoolValue>,
}

impl<'a> MessageRead<'a> for Vote<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.val = Some(r.read_message::<lupyd::post::BoolValue>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Vote<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.val.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.id))?; }
        if let Some(ref s) = self.val { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostIds<'a> {
    pub ids: Vec<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for PostIds<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PostIds<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateFileFields<'a> {
    pub expiry: u64,
    pub by: Cow<'a, str>,
    pub files: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for CreateFileFields<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.expiry = r.read_uint64(bytes)?,
                Ok(18) => msg.by = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.files.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreateFileFields<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.expiry == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.expiry) as u64) }
        + if self.by == "" { 0 } else { 1 + sizeof_len((&self.by).len()) }
        + self.files.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.expiry != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.expiry))?; }
        if self.by != "" { w.write_with_tag(18, |w| w.write_string(&**&self.by))?; }
        for s in &self.files { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PollVote<'a> {
    pub post_id: Cow<'a, [u8]>,
    pub option_id: i32,
}

impl<'a> MessageRead<'a> for PollVote<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.post_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.option_id = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PollVote<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.post_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.post_id).len()) }
        + if self.option_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.option_id) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.post_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.post_id))?; }
        if self.option_id != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.option_id))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PollOption {
    pub option_id: i32,
    pub votes: i64,
}

impl<'a> MessageRead<'a> for PollOption {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.option_id = r.read_int32(bytes)?,
                Ok(16) => msg.votes = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PollOption {
    fn get_size(&self) -> usize {
        0
        + if self.option_id == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.option_id) as u64) }
        + if self.votes == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.votes) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.option_id != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.option_id))?; }
        if self.votes != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.votes))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PollResult<'a> {
    pub poll_id: Cow<'a, [u8]>,
    pub values: Vec<i32>,
}

impl<'a> MessageRead<'a> for PollResult<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.poll_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.values = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PollResult<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.poll_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.poll_id).len()) }
        + if self.values.is_empty() { 0 } else { 1 + sizeof_len(self.values.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.poll_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.poll_id))?; }
        w.write_packed_with_tag(18, &self.values, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PollResults<'a> {
    pub results: Vec<lupyd::post::PollResult<'a>>,
}

impl<'a> MessageRead<'a> for PollResults<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.results.push(r.read_message::<lupyd::post::PollResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PollResults<'a> {
    fn get_size(&self) -> usize {
        0
        + self.results.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.results { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreatePollFields<'a> {
    pub post_id: Cow<'a, [u8]>,
    pub num_of_opts: u32,
}

impl<'a> MessageRead<'a> for CreatePollFields<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.post_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.num_of_opts = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreatePollFields<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.post_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.post_id).len()) }
        + if self.num_of_opts == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.num_of_opts) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.post_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.post_id))?; }
        if self.num_of_opts != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.num_of_opts))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PollUserVote<'a> {
    pub poll_id: Cow<'a, [u8]>,
    pub val: u32,
}

impl<'a> MessageRead<'a> for PollUserVote<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.poll_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.val = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PollUserVote<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.poll_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.poll_id).len()) }
        + if self.val == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.val) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.poll_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.poll_id))?; }
        if self.val != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.val))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PollUserVotes<'a> {
    pub votes: Vec<lupyd::post::PollUserVote<'a>>,
}

impl<'a> MessageRead<'a> for PollUserVotes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.votes.push(r.read_message::<lupyd::post::PollUserVote>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PollUserVotes<'a> {
    fn get_size(&self) -> usize {
        0
        + self.votes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.votes { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostReport<'a> {
    pub post_id: Cow<'a, [u8]>,
    pub sevirity: i32,
    pub description: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for PostReport<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.post_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.sevirity = r.read_int32(bytes)?,
                Ok(26) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PostReport<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.post_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.post_id).len()) }
        + if self.sevirity == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.sevirity) as u64) }
        + if self.description == "" { 0 } else { 1 + sizeof_len((&self.description).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.post_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.post_id))?; }
        if self.sevirity != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.sevirity))?; }
        if self.description != "" { w.write_with_tag(26, |w| w.write_string(&**&self.description))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostHashtag<'a> {
    pub name: Cow<'a, str>,
    pub total: i32,
}

impl<'a> MessageRead<'a> for PostHashtag<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.total = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PostHashtag<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.total == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.total) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.total != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.total))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PostHashtags<'a> {
    pub hashtags: Vec<lupyd::post::PostHashtag<'a>>,
}

impl<'a> MessageRead<'a> for PostHashtags<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hashtags.push(r.read_message::<lupyd::post::PostHashtag>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PostHashtags<'a> {
    fn get_size(&self) -> usize {
        0
        + self.hashtags.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.hashtags { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

