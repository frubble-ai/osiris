//! The id of something GitHub keeps (a comment, a review, a release, a run), as an action takes
//! it: a whole number, or the text of one, since every answer gives its ids as text so a caller
//! can pass one straight back.

use serde::{Deserialize, Deserializer, de};
use std::{borrow::Cow, fmt};
use utoipa::{
    PartialSchema, ToSchema,
    openapi::{
        KnownFormat, ObjectBuilder, OneOfBuilder, RefOr, SchemaFormat,
        schema::{Schema, Type},
    },
};

/// An id GitHub gave something.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GithubId(pub u64);

impl fmt::Display for GithubId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for GithubId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(IdVisitor)
    }
}

/// Reads a whole number that isn't negative, or text of only digits.
struct IdVisitor;

impl de::Visitor<'_> for IdVisitor {
    type Value = GithubId;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an id: a whole number, or the text of one")
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<GithubId, E> {
        Ok(GithubId(value))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<GithubId, E> {
        u64::try_from(value)
            .map(GithubId)
            .map_err(|_| E::invalid_value(de::Unexpected::Signed(value), &self))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<GithubId, E> {
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(E::invalid_value(de::Unexpected::Str(value), &self));
        }
        value
            .parse()
            .map(GithubId)
            .map_err(|_| E::invalid_value(de::Unexpected::Str(value), &self))
    }
}

impl PartialSchema for GithubId {
    /// A whole number, or text of only digits.
    fn schema() -> RefOr<Schema> {
        OneOfBuilder::new()
            .item(
                ObjectBuilder::new()
                    .schema_type(Type::Integer)
                    .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
                    .minimum(Some(0)),
            )
            .item(
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .pattern(Some("^[0-9]+$")),
            )
            .description(Some("An id GitHub gave something: a whole number, or the text of one, as answers give it."))
            .into()
    }
}

impl ToSchema for GithubId {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GithubId")
    }
}

#[cfg(test)]
mod tests;
