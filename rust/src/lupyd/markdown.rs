// Automatically generated rust module for 'lupyd-md.proto' file

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
pub enum ElementType {
    Normal = 0,
    Bold = 1,
    Italic = 2,
    Header = 4,
    UnderLine = 8,
    Code = 16,
    Quote = 32,
    Spoiler = 64,
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::Normal
    }
}

impl From<i32> for ElementType {
    fn from(i: i32) -> Self {
        match i {
            0 => ElementType::Normal,
            1 => ElementType::Bold,
            2 => ElementType::Italic,
            4 => ElementType::Header,
            8 => ElementType::UnderLine,
            16 => ElementType::Code,
            32 => ElementType::Quote,
            64 => ElementType::Spoiler,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ElementType {
    fn from(s: &'a str) -> Self {
        match s {
            "Normal" => ElementType::Normal,
            "Bold" => ElementType::Bold,
            "Italic" => ElementType::Italic,
            "Header" => ElementType::Header,
            "UnderLine" => ElementType::UnderLine,
            "Code" => ElementType::Code,
            "Quote" => ElementType::Quote,
            "Spoiler" => ElementType::Spoiler,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HyperElementType {
    Mention = 0,
    HashTag = 1,
    Post = 2,
    Group = 3,
    Link = 4,
}

impl Default for HyperElementType {
    fn default() -> Self {
        HyperElementType::Mention
    }
}

impl From<i32> for HyperElementType {
    fn from(i: i32) -> Self {
        match i {
            0 => HyperElementType::Mention,
            1 => HyperElementType::HashTag,
            2 => HyperElementType::Post,
            3 => HyperElementType::Group,
            4 => HyperElementType::Link,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for HyperElementType {
    fn from(s: &'a str) -> Self {
        match s {
            "Mention" => HyperElementType::Mention,
            "HashTag" => HyperElementType::HashTag,
            "Post" => HyperElementType::Post,
            "Group" => HyperElementType::Group,
            "Link" => HyperElementType::Link,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct HyperCustomElement<'a> {
    pub tag: Cow<'a, str>,
    pub body: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for HyperCustomElement<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tag = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.body = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HyperCustomElement<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.tag == "" { 0 } else { 1 + sizeof_len((&self.tag).len()) }
        + if self.body == "" { 0 } else { 1 + sizeof_len((&self.body).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.tag != "" { w.write_with_tag(10, |w| w.write_string(&**&self.tag))?; }
        if self.body != "" { w.write_with_tag(18, |w| w.write_string(&**&self.body))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct HyperElement<'a> {
    pub tag: lupyd::markdown::HyperElementType,
    pub body: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for HyperElement<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.tag = r.read_enum(bytes)?,
                Ok(18) => msg.body = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HyperElement<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.tag == lupyd::markdown::HyperElementType::Mention { 0 } else { 1 + sizeof_varint(*(&self.tag) as u64) }
        + if self.body == "" { 0 } else { 1 + sizeof_len((&self.body).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.tag != lupyd::markdown::HyperElementType::Mention { w.write_with_tag(8, |w| w.write_enum(*&self.tag as i32))?; }
        if self.body != "" { w.write_with_tag(18, |w| w.write_string(&**&self.body))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileElement<'a> {
    pub filename: Cow<'a, str>,
    pub url: Cow<'a, str>,
    pub mimeType: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for FileElement<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.filename = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.mimeType = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileElement<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.filename == "" { 0 } else { 1 + sizeof_len((&self.filename).len()) }
        + if self.url == "" { 0 } else { 1 + sizeof_len((&self.url).len()) }
        + if self.mimeType == "" { 0 } else { 1 + sizeof_len((&self.mimeType).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.filename != "" { w.write_with_tag(10, |w| w.write_string(&**&self.filename))?; }
        if self.url != "" { w.write_with_tag(18, |w| w.write_string(&**&self.url))?; }
        if self.mimeType != "" { w.write_with_tag(26, |w| w.write_string(&**&self.mimeType))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PrimitiveElement<'a> {
    pub elementType: u32,
    pub text: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for PrimitiveElement<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.elementType = r.read_uint32(bytes)?,
                Ok(18) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PrimitiveElement<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.elementType == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.elementType) as u64) }
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.elementType != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.elementType))?; }
        if self.text != "" { w.write_with_tag(18, |w| w.write_string(&**&self.text))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Element<'a> {
    pub element: lupyd::markdown::mod_Element::OneOfelement<'a>,
}

impl<'a> MessageRead<'a> for Element<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.element = lupyd::markdown::mod_Element::OneOfelement::primitiveElement(r.read_message::<lupyd::markdown::PrimitiveElement>(bytes)?),
                Ok(18) => msg.element = lupyd::markdown::mod_Element::OneOfelement::hyperElement(r.read_message::<lupyd::markdown::HyperElement>(bytes)?),
                Ok(26) => msg.element = lupyd::markdown::mod_Element::OneOfelement::hyperCustomElement(r.read_message::<lupyd::markdown::HyperCustomElement>(bytes)?),
                Ok(34) => msg.element = lupyd::markdown::mod_Element::OneOfelement::fileElement(r.read_message::<lupyd::markdown::FileElement>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Element<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.element {
            lupyd::markdown::mod_Element::OneOfelement::primitiveElement(ref m) => 1 + sizeof_len((m).get_size()),
            lupyd::markdown::mod_Element::OneOfelement::hyperElement(ref m) => 1 + sizeof_len((m).get_size()),
            lupyd::markdown::mod_Element::OneOfelement::hyperCustomElement(ref m) => 1 + sizeof_len((m).get_size()),
            lupyd::markdown::mod_Element::OneOfelement::fileElement(ref m) => 1 + sizeof_len((m).get_size()),
            lupyd::markdown::mod_Element::OneOfelement::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.element {            lupyd::markdown::mod_Element::OneOfelement::primitiveElement(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            lupyd::markdown::mod_Element::OneOfelement::hyperElement(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            lupyd::markdown::mod_Element::OneOfelement::hyperCustomElement(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            lupyd::markdown::mod_Element::OneOfelement::fileElement(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            lupyd::markdown::mod_Element::OneOfelement::None => {},
    }        Ok(())
    }
}

pub mod mod_Element {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfelement<'a> {
    primitiveElement(lupyd::markdown::PrimitiveElement<'a>),
    hyperElement(lupyd::markdown::HyperElement<'a>),
    hyperCustomElement(lupyd::markdown::HyperCustomElement<'a>),
    fileElement(lupyd::markdown::FileElement<'a>),
    None,
}

impl<'a> Default for OneOfelement<'a> {
    fn default() -> Self {
        OneOfelement::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Elements<'a> {
    pub elements: Vec<lupyd::markdown::Element<'a>>,
}

impl<'a> MessageRead<'a> for Elements<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.elements.push(r.read_message::<lupyd::markdown::Element>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Elements<'a> {
    fn get_size(&self) -> usize {
        0
        + self.elements.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.elements { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Markdown<'a> {
    pub elements: Option<lupyd::markdown::Elements<'a>>,
}

impl<'a> MessageRead<'a> for Markdown<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.elements = Some(r.read_message::<lupyd::markdown::Elements>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Markdown<'a> {
    fn get_size(&self) -> usize {
        0
        + self.elements.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.elements { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

