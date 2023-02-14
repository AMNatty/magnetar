use serde::Serialize;

pub mod jsonld;
pub mod webfinger;

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

    content_type!(pub ContentActivityPlusJson, "application/activity+json");
    content_type!(pub ContentHtml, "text/html");
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
