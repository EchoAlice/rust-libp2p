// Automatically generated rust module for 'payload.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NoiseHandshakePayload {
    pub identity_key: Vec<u8>,
    pub identity_sig: Vec<u8>,
    pub data: Vec<u8>,
}

impl<'a> MessageRead<'a> for NoiseHandshakePayload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.identity_key = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.identity_sig = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for NoiseHandshakePayload {
    fn get_size(&self) -> usize {
        0
        + if self.identity_key.is_empty() { 0 } else { 1 + sizeof_len((&self.identity_key).len()) }
        + if self.identity_sig.is_empty() { 0 } else { 1 + sizeof_len((&self.identity_sig).len()) }
        + if self.data.is_empty() { 0 } else { 1 + sizeof_len((&self.data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.identity_key.is_empty() { w.write_with_tag(10, |w| w.write_bytes(&**&self.identity_key))?; }
        if !self.identity_sig.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.identity_sig))?; }
        if !self.data.is_empty() { w.write_with_tag(26, |w| w.write_bytes(&**&self.data))?; }
        Ok(())
    }
}

