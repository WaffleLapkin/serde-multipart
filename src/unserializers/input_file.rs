use reqwest::multipart::Part;
use serde::ser::{SerializeTupleVariant, SerializeStructVariant, Impossible};
use crate::InputFile;
use serde::{Serialize, ser, Serializer};
use std::path::PathBuf;
use std::borrow::Cow;
use crate::unserializers::string::StringUnserializer;
use crate::unserializers::bytes::BytesUnserializer;
use crate::unserializers::UnimplementedError;
use std::fmt::Display;
use serde::export::Formatter;
use std::fmt;

// pub enum InputFile {
//     File(PathBuf),
//     Memory { file_name: String, data: Cow<'static, [u8]> },
//     Url(String),
//     FileId(String),
// }

pub(crate) enum InputFileUnserializer {
    Memory {
        file_name: String,
        data: Cow<'static, [u8]>
    },
    NotMem,
}

impl InputFileUnserializer {
    pub(crate) fn file() -> Self {
        Self::NotMem
    }

    pub(crate) fn memory() -> Self {
        Self::Memory {
            file_name: String::new(),
            data: Cow::Borrowed(&[]),
        }
    }

    pub(crate) fn url() -> Self {
        Self::NotMem
    }

    pub(crate) fn file_id() -> Self {
        Self::NotMem
    }
}

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub(crate) enum Error {
    StringUnimplemented(UnimplementedError),
    WrongUsage,
    UnknownField,
}


impl SerializeStructVariant for InputFileUnserializer {
    type Ok = InputFile;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize
    {
        match self {
            Self::Memory { file_name, data } => match key {
                "file_name" => *file_name = value.serialize(StringUnserializer)?,
                "data" => *data = Cow::Owned(value.serialize(BytesUnserializer::default())?),
                _ => return Err(Error::UnknownField),
            },
            _ => return Err(Error::WrongUsage)
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::Memory { file_name, data } => Ok(InputFile::Memory { file_name, data }),
            Self::NotMem => Err(Error::WrongUsage),
        }
    }
}

impl Serializer for InputFileUnserializer {
    type Ok = InputFile;
    type Error = Error;
    type SerializeSeq = Impossible<InputFile, Error>;
    type SerializeTuple = Impossible<InputFile, Error>;
    type SerializeTupleStruct = Impossible<InputFile, Error>;
    type SerializeTupleVariant = Impossible<InputFile, Error>;
    type SerializeMap = Impossible<InputFile, Error>;
    type SerializeStruct = Impossible<InputFile, Error>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize
    {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize
    {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, _: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize
    {
        if name != "InputFile" {
            return Err(Error::WrongUsage);
        }

        match variant {
            "File" => Ok(InputFile::File(value.serialize(StringUnserializer)?.into())),
            "Url" => Ok(InputFile::Url(value.serialize(StringUnserializer)?)),
            "FileId" => Ok(InputFile::FileId(value.serialize(StringUnserializer)?)),
            _ => Err(Error::WrongUsage)
        }
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_tuple_variant(self, name: &'static str, _: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::StringUnimplemented(UnimplementedError))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        if name == "InputFile" && variant == "Memory" && len == 2 {
            Ok(self)
        } else {
            Err(Error::WrongUsage)
        }
    }
}


impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        unimplemented!()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::StringUnimplemented(err) => Some(err),
            Self::WrongUsage | Self::UnknownField => None
        }
    }
}
