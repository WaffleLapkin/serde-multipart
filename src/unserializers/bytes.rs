use serde::{Serializer, Serialize};
use serde::ser::{Impossible, SerializeSeq};
use crate::unserializers::UnimplementedError;

#[derive(Default)]
pub(crate) struct BytesUnserializer(Vec<u8>);

impl Serializer for BytesUnserializer {
    type Ok = Vec<u8>;
    type Error = UnimplementedError;
    type SerializeSeq = Self;
    type SerializeTuple = Impossible<Vec<u8>, UnimplementedError>;
    type SerializeTupleStruct = Impossible<Vec<u8>, UnimplementedError>;
    type SerializeTupleVariant = Impossible<Vec<u8>, UnimplementedError>;
    type SerializeMap = Impossible<Vec<u8>, UnimplementedError>;
    type SerializeStruct = Impossible<Vec<u8>, UnimplementedError>;
    type SerializeStructVariant = Impossible<Vec<u8>, UnimplementedError>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_owned())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize
    {
        Err(UnimplementedError)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize
    {
        Err(UnimplementedError)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize
    {
        Err(UnimplementedError)
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            self.0.reserve_exact(len);
        }
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(UnimplementedError)
    }
}

impl SerializeSeq for BytesUnserializer {
    type Ok = Vec<u8>;
    type Error = UnimplementedError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize
    {
        value.serialize(BytesUnserializerInner(&mut self.0))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.0)
    }
}

pub(crate) struct BytesUnserializerInner<'a>(&'a mut Vec<u8>);

impl Serializer for BytesUnserializerInner<'_> {
    type Ok = ();
    type Error = UnimplementedError;
    type SerializeSeq = Impossible<(), UnimplementedError>;
    type SerializeTuple = Impossible<(), UnimplementedError>;
    type SerializeTupleStruct = Impossible<(), UnimplementedError>;
    type SerializeTupleVariant = Impossible<(), UnimplementedError>;
    type SerializeMap = Impossible<(), UnimplementedError>;
    type SerializeStruct = Impossible<(), UnimplementedError>;
    type SerializeStructVariant = Impossible<(), UnimplementedError>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.0.push(v);
        Ok(())
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.extend_from_slice(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize
    {
        Err(UnimplementedError)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize
    {
        Err(UnimplementedError)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize
    {
        Err(UnimplementedError)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(UnimplementedError)
    }

    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(UnimplementedError)
    }
}