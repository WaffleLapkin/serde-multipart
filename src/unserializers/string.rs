use serde::{Serializer, Serialize};
use serde::ser::Impossible;
use crate::unserializers::UnimplementedError;

pub(crate) struct StringUnserializer;

impl Serializer for StringUnserializer {
    type Ok = String;
    type Error = UnimplementedError;
    type SerializeSeq = Impossible<String, UnimplementedError>;
    type SerializeTuple = Impossible<String, UnimplementedError>;
    type SerializeTupleStruct = Impossible<String, UnimplementedError>;
    type SerializeTupleVariant = Impossible<String, UnimplementedError>;
    type SerializeMap = Impossible<String, UnimplementedError>;
    type SerializeStruct = Impossible<String, UnimplementedError>;
    type SerializeStructVariant = Impossible<String, UnimplementedError>;

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
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_owned())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(UnimplementedError)
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
