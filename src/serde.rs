use std::collections::HashMap;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::Visitor,
    ser::{SerializeMap, SerializeSeq},
};

use crate::{Any, Bytes, Number, Str};

macro_rules! impl_visitor {
    ($method:ident, $type:ty, $variant:ident) => {
        fn $method<E>(self, v: $type) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok($variant::from(v))
        }
    };
}

struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = Str<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting string value")
    }

    impl_visitor!(visit_borrowed_str, &'de str, Str);
    impl_visitor!(visit_string, String, Str);
}

struct AnyVisitor;

impl<'de> Deserialize<'de> for Str<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Str<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrVisitor)
    }
}

impl<'de> Visitor<'de> for AnyVisitor {
    type Value = Any<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expected value")
    }

    impl_visitor!(visit_bool, bool, Any);
    impl_visitor!(visit_i8, i8, Any);
    impl_visitor!(visit_i16, i16, Any);
    impl_visitor!(visit_i32, i32, Any);
    impl_visitor!(visit_i64, i64, Any);
    impl_visitor!(visit_u8, u8, Any);
    impl_visitor!(visit_u16, u16, Any);
    impl_visitor!(visit_u32, u32, Any);
    impl_visitor!(visit_u64, u64, Any);
    impl_visitor!(visit_f32, f32, Any);
    impl_visitor!(visit_f64, f64, Any);
    impl_visitor!(visit_borrowed_bytes, &'de [u8], Any);
    impl_visitor!(visit_byte_buf, Vec<u8>, Any);
    impl_visitor!(visit_borrowed_str, &'de str, Any);
    impl_visitor!(visit_string, String, Any);

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any::Null)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut list: Vec<Any<'de>> = Vec::new();

        while let Some(v) = seq.next_element()? {
            list.push(v);
        }

        Ok(Any::from(list))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut m: HashMap<Str<'de>, Any<'de>> = HashMap::new();

        while let Some((k, v)) = map.next_entry()? {
            m.insert(k, v);
        }

        Ok(Any::from(m))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        // we don't care about the variant, we are all powerfull
        // and can handle ANY TYPE!!!! BOW DOWN BEFORE ME AND TREMBLE
        let (bookmark, _variant) = data.variant()?;
        Ok(bookmark)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Deserialize<'de> for Any<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Any<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyVisitor)
    }
}

impl Serialize for Any<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Any::Null => serializer.serialize_none(),
            Any::Str(str) => str.serialize(serializer),
            Any::Bytes(bts) => bts.serialize(serializer),
            Any::Number(num) => num.serialize(serializer),
            Any::Bool(bool) => serializer.serialize_bool(*bool),
            Any::List(list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for item in list {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Any::Map(map) => {
                let mut seq = serializer.serialize_map(Some(map.len()))?;
                for (key, item) in map {
                    seq.serialize_entry(key, item)?;
                }
                seq.end()
            }
        }
    }
}

impl Serialize for Str<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for Bytes<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_ref())
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Number::Float(f) => serializer.serialize_f64(*f),
            Number::Integer(i) => serializer.serialize_i64(*i),
            Number::UInteger(u) => serializer.serialize_u64(*u),
        }
    }
}
