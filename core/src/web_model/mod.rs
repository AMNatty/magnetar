use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Debug;
use std::str::FromStr;

pub mod acct;
pub mod activity_streams;

trait ContentType: Serialize {
    fn mime_type(&self) -> &'static str;
}

macro_rules! content_type {
    ($visib:vis $typ:ident, $expression:expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
        $visib struct $typ;

        impl AsRef<str> for $typ {
            fn as_ref(&self) -> &'static str {
                $expression
            }
        }

        impl Serialize for $typ {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str($expression)
            }
        }

        impl ContentType for $typ {
            fn mime_type(&self) -> &'static str {
                $expression
            }
        }

        impl<'de> Deserialize<'de> for $typ {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let content_type = String::deserialize(deserializer)?;

                if matches!(content_type.as_ref(), $expression) {
                    Ok(Self)
                } else {
                    Err(Error::custom(format!(
                        "Invalid content type: {content_type}"
                    )))
                }
            }
        }
    };
}

pub mod content_type {
    use crate::web_model::ContentType;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    content_type!(pub ContentActivityStreams, "application/activity+json");
    content_type!(pub ContentHtml, "text/html");
    content_type!(pub ContentJson, "application/json");
    content_type!(pub ContentMultipartFormData, "multipart/form-data");
    content_type!(pub ContentUrlEncoded, "application/x-www-form-urlencoded");
}

macro_rules! link_rel {
    ($visib:vis $typ:ident, $expression:expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
        $visib struct $typ;

        impl AsRef<str> for $typ {
            fn as_ref(&self) -> &'static str {
                $expression
            }
        }

        impl Serialize for $typ {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str($expression)
            }
        }

        impl Rel for $typ {
            fn rel(&self) -> &'static str {
                $expression
            }
        }

        impl<'de> Deserialize<'de> for $typ {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let rel = String::deserialize(deserializer)?;

                if matches!(rel.as_ref(), $expression) {
                    Ok(Self)
                } else {
                    Err(Error::custom(format!(
                        "Invalid rel: {rel}"
                    )))
                }
            }
        }
    };
}

trait Rel: Serialize {
    fn rel(&self) -> &'static str;
}

pub mod rel {
    use crate::web_model::Rel;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    link_rel!(pub RelWebFingerProfilePage, "http://webfinger.net/rel/profile-page");
    link_rel!(pub RelSelf, "self");
    link_rel!(pub RelOStatusSubscribe, "http://ostatus.org/schema/1.0/subscribe");
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ListContaining<T: Clone + Eq + PartialEq + Debug + AsRef<str> + FromStr>(pub T);

impl<T> Serialize for ListContaining<T>
where
    T: Clone + Eq + PartialEq + Debug + AsRef<str> + FromStr,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        vec![AsRef::<str>::as_ref(&self.0)].serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for ListContaining<T>
where
    T: Clone + Eq + PartialEq + Debug + AsRef<str> + FromStr,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = Vec::<Value>::deserialize(deserializer)?;

        let dt = data
            .iter()
            .filter_map(Value::as_str)
            .filter_map(|val| T::from_str(val).ok())
            .next();

        if let Some(value) = dt {
            Ok(ListContaining(value))
        } else {
            Err(Error::custom("Count not find item in list.".to_string()))
        }
    }
}
