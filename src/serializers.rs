use crate::unserializers::InputFileUnserializer;
use crate::InputFile;
use futures::future::ready;
use futures::future::Either;
use futures::stream::FuturesUnordered;
use futures::{Future, FutureExt, StreamExt, TryStreamExt};
use reqwest::multipart::{Form, Part};
use serde::ser::{Impossible, SerializeStruct, SerializeStructVariant, SerializeSeq};
use serde::{ser, Serialize, Serializer};
use std::fmt::Display;
use std::{fmt, io};
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{Hash, BuildHasher, Hasher};
use std::collections::HashSet;

#[derive(Debug, derive_more::From)]
pub enum Error {
    Custom(String),
    TopLevelNotStruct,
    InputFileUnserializerError(crate::unserializers::UnserializerError),
    Io(std::io::Error),
    Json(serde_json::Error)
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

//type Res = impl Future<Output = io::Result<Form>>;

pub(crate) struct MultipartTopLvlSerializer {}

impl Serializer for MultipartTopLvlSerializer {
    type Ok = <MultipartSerializer as SerializeStruct>::Ok;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = MultipartSerializer;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MultipartSerializer::new())
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }
}

pub(crate) struct MultipartSerializer {
    parts: Vec<(&'static str, Part)>, // TODO: Array vecs
    files: Vec<(String, InputFile)>,
}

impl MultipartSerializer {
    fn new() -> Self {
        Self { parts: Vec::new(), files: vec![] }
    }
}

impl SerializeStruct for MultipartSerializer {
    type Ok = impl Future<Output = io::Result<Form>>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let (part, file) = value.serialize(PartSerializer { n: self.files.len() })?;
        self.parts.push((key, part));
        self.files.extend(file);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let form = self
               .parts
               .into_iter()
               .fold(Form::new(), |acc, (key, value)| {
                   acc.part(key, value)
               });

        if self.files.is_empty() {
            Ok(Either::Left(ready(Ok(form))))
        } else {
            let fut = self
                .files
                .into_iter()
                .map(|(k, f)| f.into_part().map(move |p| (k, p)))
                .collect::<FuturesUnordered<_>>()
                .map(Ok)
                .try_fold(form, |acc, (k, p)| async { Ok(acc.part(k, p?)) });

            Ok(Either::Right(fut))
        }
    }
}


struct PartSerializer {
    n: usize // number of input files serialized
}

impl Serializer for PartSerializer {
    type Ok = (Part, Vec<(String, InputFile)>);
    type Error = Error;
    type SerializeSeq = InnerPartSerializer;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = PartFromFile;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(v.to_string()), Vec::new()))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(v.to_string()), Vec::new()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(v.to_string()), Vec::new()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(v.to_owned()), Vec::new()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let file: InputFile = InputFileUnserializer::NotMem.serialize_newtype_variant(
            name,
            variant_index,
            variant,
            value,
        )?;

        match file {
            f @ InputFile::Memory { .. } | f @ InputFile::File(_) => {
                let mut hasher = RandomState::new().build_hasher();
                // TODO choose hasher
                //      or even do not use hasher at all
                f.hash(&mut hasher);
                self.n.hash(&mut hasher);
                let x = hasher.finish().to_string();
                let part = Part::text(format!("attach://{}", x));

                Ok((part, vec![(x, f)]))
            },
            InputFile::FileId(s) | InputFile::Url(s) => {
                Ok((Part::text(s), Vec::new()))
            }
        }
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Self::SerializeSeq {
            n: self.n,
            array_json_parts: vec![],
            files: vec![]
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(PartFromFile {
            inner: InputFileUnserializer::memory().serialize_struct_variant(
                name,
                variant_index,
                variant,
                len,
            )?,
            n: self.n,
        })
    }
}

struct PartFromFile {
    inner: InputFileUnserializer,
    n: usize,
}

impl SerializeStructVariant for PartFromFile {
    type Ok = (Part, Vec<(String, InputFile)>);
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner
            .serialize_field(key, value)
            .map_err(Error::InputFileUnserializerError)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let file = self.inner.end()?;

        // TODO: to method
        match file {
            f @ InputFile::Memory { .. } | f @ InputFile::File(_) => {
                let mut hasher = RandomState::new().build_hasher();
                // TODO choose hasher
                //      or even do not use hasher at all
                f.hash(&mut hasher);
                self.n.hash(&mut hasher);
                let x = hasher.finish().to_string();
                let part = Part::text(format!("attach://{}", x));

                Ok((part, vec![((x, f))]))
            },
            InputFile::FileId(s) | InputFile::Url(s) => {
                Ok((Part::text(s), vec![]))
            }
        }
    }
}

struct InnerPartSerializer {
    n: usize,
    array_json_parts: Vec<serde_json::Value>, // using value is such a workaround :|
    files: Vec<(String, InputFile)>,
}

impl SerializeSeq for InnerPartSerializer {
    type Ok = (Part, Vec<(String, InputFile)>);
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // NOTE: this is probably highly inefficient (especially for ::Memory),
        //       but at least it works
        let mut value = dbg!(serde_json::to_value(value))?;
        let file: InputFile = serde_json::from_value(value["media"].take())?;

        match file {
            f @ InputFile::Memory { .. } | f @ InputFile::File(_) => {
                let mut hasher = RandomState::new().build_hasher();
                // TODO choose hasher
                //      or even do not use hasher at all
                f.hash(&mut hasher);
                self.n.hash(&mut hasher);
                self.n += 1;
                let x = hasher.finish().to_string();
                value["media"] = serde_json::Value::String(format!("attach://{}", x));
                self.files.push((x, f));
            },
            InputFile::FileId(s) | InputFile::Url(s) => {
                value["media"] = serde_json::Value::String(s);
            }
        }

        self.array_json_parts.push(value);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let s = serde_json::to_string(&self.array_json_parts)?;
        Ok((Part::text(s), self.files))
    }
}
