use crate::web_model::ListContaining;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct ContextActivityStreams;

impl AsRef<str> for ContextActivityStreams {
    fn as_ref(&self) -> &'static str {
        "https://www.w3.org/ns/activitystreams"
    }
}

impl Serialize for ContextActivityStreams {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_ref())
    }
}

impl FromStr for ContextActivityStreams {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if matches!(
            s,
            "https://www.w3.org/ns/activitystreams" | "http://www.w3.org/ns/activitystreams"
        ) {
            Ok(Self)
        } else {
            Err(format!("Invalid context: {s}"))
        }
    }
}

impl<'de> Deserialize<'de> for ContextActivityStreams {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let context = String::deserialize(deserializer)?;

        ContextActivityStreams::from_str(&context).map_err(Error::custom)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
    String(ContextActivityStreams),
    Object {
        #[serde(rename = "@vocab")]
        ld_vocab: ContextActivityStreams,
    },
    List(ListContaining<ContextActivityStreams>),
}

impl Default for Context {
    fn default() -> Self {
        Context::String(ContextActivityStreams)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
struct ActivityStreamsDocument<T> {
    #[serde(rename = "@context", default)]
    ld_context: Context,
    #[serde(flatten)]
    data: T,
}

#[cfg(test)]
mod test {
    use crate::web_model::activity_streams::{
        ActivityStreamsDocument, Context, ContextActivityStreams,
    };
    use crate::web_model::ListContaining;
    use serde_json::json;

    #[test]
    fn should_parse_context() {
        let json = json!({
            "@context": "https://www.w3.org/ns/activitystreams",
            "some": "stuff"
        });

        let doc: ActivityStreamsDocument<()> = serde_json::from_value(json).unwrap();

        assert_eq!(doc.ld_context, Context::String(ContextActivityStreams));
    }

    #[test]
    fn should_parse_missing_context() {
        let json = json!({
            "some": "stuff"
        });

        let doc: ActivityStreamsDocument<()> = serde_json::from_value(json).unwrap();

        assert_eq!(doc.ld_context, Context::String(ContextActivityStreams));
    }

    #[test]
    fn should_parse_context_http() {
        let json = json!({
            "@context": "http://www.w3.org/ns/activitystreams",
            "some": "stuff"
        });

        let doc: ActivityStreamsDocument<()> = serde_json::from_value(json).unwrap();

        assert_eq!(doc.ld_context, Context::String(ContextActivityStreams));
    }

    #[test]
    fn should_parse_context_vocab() {
        let json = json!({
            "@context": {
                "@vocab": "https://www.w3.org/ns/activitystreams",
                "foo": "bar"
            },
            "some": "stuff"
        });

        let doc: ActivityStreamsDocument<()> = serde_json::from_value(json).unwrap();

        assert_eq!(
            doc.ld_context,
            Context::Object {
                ld_vocab: ContextActivityStreams
            }
        );
    }

    #[test]
    fn should_parse_context_array() {
        let json = json!({
            "@context": [
                {
                    "foo": "bar"
                },
                "https://www.w3.org/ns/activitystreams",
            ],
            "some": "stuff"
        });

        let doc: ActivityStreamsDocument<()> = serde_json::from_value(json).unwrap();

        assert_eq!(
            doc.ld_context,
            Context::List(ListContaining(ContextActivityStreams))
        );
    }
}
