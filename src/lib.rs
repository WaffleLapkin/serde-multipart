#![feature(type_alias_impl_trait)]

mod unserializers;
mod serializers;






use serde::{Deserialize, Serialize, Serializer};

use std::{borrow::Cow, path::PathBuf, io};
use serde::ser::{SerializeStruct, SerializeTupleVariant};
use std::fmt::Display;
use std::future::Future;
use futures::FutureExt;

/// This object represents the contents of a file to be uploaded.
///
/// [The official docs](https://core.telegram.org/bots/api#inputfile).
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize)]
#[non_exhaustive]
pub enum InputFile {
    File(PathBuf),
    Memory { file_name: String, data: Cow<'static, [u8]> },
    Url(String),
    FileId(String),
}

use reqwest::multipart::Part;
use crate::serializers::MultipartTopLvlSerializer;

impl InputFile {
    async fn into_part(self) -> io::Result<Part> {
        dbg!("into part");
        use tokio_util::codec::{Decoder, FramedRead};
        use bytes::{Bytes, BytesMut};
        use reqwest::Body;
        use bytes::buf::ext::BufExt;

        struct FileDecoder;

        impl Decoder for FileDecoder {
            type Item = Bytes;
            type Error = std::io::Error;

            fn decode(
                &mut self,
                src: &mut BytesMut,
            ) -> Result<Option<Self::Item>, Self::Error> {
                if src.is_empty() {
                    return Ok(None);
                }
                Ok(Some(src.split().freeze()))
            }
        }


        //pub async fn file_to_part(path_to_file: PathBuf) -> std::io::Result<Part> {
        match self {
            Self::File(path_to_file) => {
                dbg!("file into part");
                let file_name = path_to_file.file_name().unwrap().to_string_lossy().into_owned();

                let file = FramedRead::new(tokio::fs::File::open(path_to_file).await?, FileDecoder);

                Ok(Part::stream(Body::wrap_stream(file)).file_name(file_name))
            },
            Self::Memory { file_name, data } => Ok(Part::bytes(data).file_name(file_name)),
            Self::Url(s) | Self::FileId(s) => Ok(Part::text(s)),
        }
    }
}

// impl Serialize for InputFile {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self {
//             InputFile::File(path) => {
//                 // NOTE: file should be actually attached with
//                 // multipart/form-data
//                 serializer.serialize_str(
//                     // TODO: remove unwrap (?)
//                     &format!("attach://{}", path.file_name().unwrap().to_string_lossy()),
//                 )
//             }
//             InputFile::Memory { data, .. } => {
//                 // NOTE: file should be actually attached with
//                 // multipart/form-data
//                 serializer.serialize_str(&format!("attach://{}", String::from_utf8_lossy(data)))
//             }
//             InputFile::Url(url) => serializer.serialize_str(url),
//             InputFile::FileId(id) => serializer.serialize_str(id),
//         }
//     }
// }

// #[tokio::test]
// async fn test() {
//     struct Test {
//         some_field: i64
//     }
// }

use reqwest::multipart::Form;

pub struct Error;

pub async fn to_form<T: ?Sized + Serialize>(val: &T) -> Result<Form, Error> {
    let fut = val.serialize(MultipartTopLvlSerializer {}).map_err(|_| Error)?;
    fut.await.map_err(|_| Error)
}
