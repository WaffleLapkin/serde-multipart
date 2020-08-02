//! ## serde-multiart
//!
//! Crate for serializing into `multipart/form-data` ([`reqwest::multipart::Form`])
//!
//! [`reqwest::multipart::Form`]: reqwest::multipart::Form
//!
//! You may declare a struct like this:
//!
//! ```
//! use std::{path::PathBuf, borrow::Cow};
//! #[derive(serde::Serialize)]
//! enum InputFile {
//!     File(PathBuf),
//!     Memory {
//!         file_name: String,
//!         data: Cow<'static, [u8]>,
//!     },
//!     Url(String),
//!     FileId(String),
//! }
//! ```
//!
//! To serialize files.
//!
//! ## Current Limitations
//!
//! - works only with `reqwest`
//! - the `InputFile` structure is highly hard-coded (todo: make it less hard-coded or expose it or
//!   something)
//!
//! ## How it works
//!
//! You better not know...
#![feature(type_alias_impl_trait)] // TODO: should be possible to make nightly opt-in/out

#[macro_use]
mod local_macros;

mod serializers;
mod unserializers;

use std::{borrow::Cow, io, path::PathBuf};

use reqwest::multipart::{Form, Part};
use serde::Serialize;

use crate::serializers::MultipartTopLvlSerializer;

pub use serializers::Error;

/// Serializes given value into [`Form`]
///
/// [`Form`]:  reqwest::multipart::Form
pub async fn to_form<T: ?Sized + Serialize>(val: &T) -> Result<Form, Error> {
    let fut = val.serialize(MultipartTopLvlSerializer {})?;
    let res = fut.await?;
    Ok(res)
}

/// This object represents the contents of a file to be uploaded.
///
/// [The official docs](https://core.telegram.org/bots/api#inputfile).
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize)]
enum InputFile {
    File(PathBuf),
    Memory {
        file_name: String,
        data: Cow<'static, [u8]>,
    },
    Url(String),
    FileId(String),
}

impl InputFile {
    async fn into_part(self) -> io::Result<Part> {
        use bytes::{Bytes, BytesMut};
        use reqwest::Body;
        use tokio_util::codec::{Decoder, FramedRead};

        struct FileDecoder;

        impl Decoder for FileDecoder {
            type Item = Bytes;
            type Error = std::io::Error;

            fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
                if src.is_empty() {
                    return Ok(None);
                }
                Ok(Some(src.split().freeze()))
            }
        }

        match self {
            Self::File(path_to_file) => {
                let file_name = path_to_file
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();

                let file = FramedRead::new(tokio::fs::File::open(path_to_file).await?, FileDecoder);

                Ok(Part::stream(Body::wrap_stream(file)).file_name(file_name))
            }
            Self::Memory { file_name, data } => Ok(Part::bytes(data).file_name(file_name)),
            Self::Url(s) | Self::FileId(s) => Ok(Part::text(s)),
        }
    }
}
