// Automatically generated rust module for 'auth.proto' file

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
pub struct UserTokens<'a> {
    pub access_token: Cow<'a, str>,
    pub refresh_token: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for UserTokens<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.access_token = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.refresh_token = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UserTokens<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.access_token == "" { 0 } else { 1 + sizeof_len((&self.access_token).len()) }
        + if self.refresh_token == "" { 0 } else { 1 + sizeof_len((&self.refresh_token).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.access_token != "" { w.write_with_tag(10, |w| w.write_string(&**&self.access_token))?; }
        if self.refresh_token != "" { w.write_with_tag(18, |w| w.write_string(&**&self.refresh_token))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TokenRequest<'a> {
    pub app_id: Cow<'a, str>,
    pub permissions: u32,
    pub refresh_token: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TokenRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.app_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.permissions = r.read_uint32(bytes)?,
                Ok(26) => msg.refresh_token = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TokenRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.app_id == "" { 0 } else { 1 + sizeof_len((&self.app_id).len()) }
        + if self.permissions == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.permissions) as u64) }
        + if self.refresh_token == "" { 0 } else { 1 + sizeof_len((&self.refresh_token).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.app_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.app_id))?; }
        if self.permissions != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.permissions))?; }
        if self.refresh_token != "" { w.write_with_tag(26, |w| w.write_string(&**&self.refresh_token))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NewAppRequest<'a> {
    pub app_id: Cow<'a, str>,
    pub contact_email: Cow<'a, str>,
    pub app_link: Cow<'a, str>,
    pub permisisons: u32,
}

impl<'a> MessageRead<'a> for NewAppRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.app_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.contact_email = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.app_link = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.permisisons = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NewAppRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.app_id == "" { 0 } else { 1 + sizeof_len((&self.app_id).len()) }
        + if self.contact_email == "" { 0 } else { 1 + sizeof_len((&self.contact_email).len()) }
        + if self.app_link == "" { 0 } else { 1 + sizeof_len((&self.app_link).len()) }
        + if self.permisisons == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.permisisons) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.app_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.app_id))?; }
        if self.contact_email != "" { w.write_with_tag(18, |w| w.write_string(&**&self.contact_email))?; }
        if self.app_link != "" { w.write_with_tag(26, |w| w.write_string(&**&self.app_link))?; }
        if self.permisisons != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.permisisons))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NewLoginThirdPartyRequest<'a> {
    pub app: Cow<'a, str>,
    pub tp_app_id: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for NewLoginThirdPartyRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.app = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.tp_app_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NewLoginThirdPartyRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.app == "" { 0 } else { 1 + sizeof_len((&self.app).len()) }
        + if self.tp_app_id == "" { 0 } else { 1 + sizeof_len((&self.tp_app_id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.app != "" { w.write_with_tag(10, |w| w.write_string(&**&self.app))?; }
        if self.tp_app_id != "" { w.write_with_tag(18, |w| w.write_string(&**&self.tp_app_id))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ThirdPartyLoginResponse<'a> {
    pub lupyd_tokens: Option<lupyd::auth::UserTokens<'a>>,
    pub app_tokens: Option<lupyd::auth::UserTokens<'a>>,
}

impl<'a> MessageRead<'a> for ThirdPartyLoginResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.lupyd_tokens = Some(r.read_message::<lupyd::auth::UserTokens>(bytes)?),
                Ok(18) => msg.app_tokens = Some(r.read_message::<lupyd::auth::UserTokens>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ThirdPartyLoginResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + self.lupyd_tokens.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.app_tokens.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.lupyd_tokens { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.app_tokens { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoggedInApp<'a> {
    pub app: Cow<'a, str>,
    pub aud: Cow<'a, str>,
    pub logged_in_since: u64,
    pub last_token_refresh: u64,
}

impl<'a> MessageRead<'a> for LoggedInApp<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.app = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.aud = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.logged_in_since = r.read_uint64(bytes)?,
                Ok(32) => msg.last_token_refresh = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LoggedInApp<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.app == "" { 0 } else { 1 + sizeof_len((&self.app).len()) }
        + if self.aud == "" { 0 } else { 1 + sizeof_len((&self.aud).len()) }
        + if self.logged_in_since == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.logged_in_since) as u64) }
        + if self.last_token_refresh == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.last_token_refresh) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.app != "" { w.write_with_tag(10, |w| w.write_string(&**&self.app))?; }
        if self.aud != "" { w.write_with_tag(18, |w| w.write_string(&**&self.aud))?; }
        if self.logged_in_since != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.logged_in_since))?; }
        if self.last_token_refresh != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.last_token_refresh))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoggedInApps<'a> {
    pub apps: Vec<lupyd::auth::LoggedInApp<'a>>,
}

impl<'a> MessageRead<'a> for LoggedInApps<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.apps.push(r.read_message::<lupyd::auth::LoggedInApp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LoggedInApps<'a> {
    fn get_size(&self) -> usize {
        0
        + self.apps.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.apps { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogoutToken<'a> {
    pub token: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for LogoutToken<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.token = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LogoutToken<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.token == "" { 0 } else { 1 + sizeof_len((&self.token).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.token != "" { w.write_with_tag(10, |w| w.write_string(&**&self.token))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogoutTokens<'a> {
    pub tokens: Vec<lupyd::auth::LogoutToken<'a>>,
}

impl<'a> MessageRead<'a> for LogoutTokens<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tokens.push(r.read_message::<lupyd::auth::LogoutToken>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LogoutTokens<'a> {
    fn get_size(&self) -> usize {
        0
        + self.tokens.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tokens { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

