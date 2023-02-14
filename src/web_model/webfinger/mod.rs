use crate::web_model::content_type::{ContentActivityPlusJson, ContentHtml};
use crate::web_model::rel::{RelOStatusSubscribe, RelSelf, RelWebFingerProfilePage};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct WebFinger {
    subject: WebFingerSubject,
    aliases: Vec<WebFingerSubject>,
    links: Vec<WebFingerRel>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum WebFingerSubject {
    Acct(Acct),
    Url(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::enum_variant_names)]
enum WebFingerRel {
    RelWebFingerProfilePage {
        rel: RelWebFingerProfilePage,
        #[serde(rename = "type")]
        content_type: ContentHtml,
        href: String,
    },
    RelSelf {
        rel: RelSelf,
        #[serde(rename = "type")]
        content_type: ContentActivityPlusJson,
        href: String,
    },
    RelOStatusSubscribe {
        rel: RelOStatusSubscribe,
        template: String,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Acct(String);

impl AsRef<str> for Acct {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Serialize for Acct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("acct:{}", self.0))
    }
}

impl<'de> Deserialize<'de> for Acct {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let acct = String::deserialize(deserializer)?;

        if let Some(rem) = acct.strip_prefix("acct:") {
            Ok(Acct(rem.to_owned()))
        } else {
            Err(Error::custom(
                "Missing acct protocol for account!".to_owned(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::web_model::content_type::{ContentActivityPlusJson, ContentHtml};
    use crate::web_model::rel::{RelOStatusSubscribe, RelSelf, RelWebFingerProfilePage};
    use crate::web_model::webfinger::WebFingerSubject::Url;
    use crate::web_model::webfinger::{Acct, WebFinger, WebFingerRel, WebFingerSubject};
    use serde_json::json;

    #[test]
    fn should_remove_acct_prefix() {
        let json = json!("acct:natty@tech.lgbt");

        let acct: Acct = serde_json::from_value(json).unwrap();

        assert_eq!(acct, Acct("natty@tech.lgbt".to_owned()))
    }

    #[test]
    fn should_add_acct_prefix() {
        let acct = Acct("natty@tech.lgbt".to_owned());
        let json = serde_json::to_value(acct).unwrap();

        assert_eq!(json, json!("acct:natty@tech.lgbt"));
    }

    #[test]
    fn should_parse_webfinger() {
        let json = json!({
            "subject": "acct:natty@tech.lgbt",
            "aliases": [
                "https://tech.lgbt/@natty",
                "https://tech.lgbt/users/natty"
            ],
            "links": [
                {
                    "rel": "http://webfinger.net/rel/profile-page",
                    "type": "text/html",
                    "href": "https://tech.lgbt/@natty"
                },
                {
                    "rel": "self",
                    "type": "application/activity+json",
                    "href": "https://tech.lgbt/users/natty"
                },
                {
                    "rel": "http://ostatus.org/schema/1.0/subscribe",
                    "template": "https://tech.lgbt/authorize_interaction?uri={uri}"
                }
            ]
        });

        let webfinger: WebFinger = serde_json::from_value(json).unwrap();

        let real = WebFinger {
            subject: WebFingerSubject::Acct(Acct("natty@tech.lgbt".to_owned())),
            aliases: vec![
                Url("https://tech.lgbt/@natty".to_owned()),
                Url("https://tech.lgbt/users/natty".to_owned()),
            ],
            links: vec![
                WebFingerRel::RelWebFingerProfilePage {
                    rel: RelWebFingerProfilePage,
                    content_type: ContentHtml,
                    href: "https://tech.lgbt/@natty".to_owned(),
                },
                WebFingerRel::RelSelf {
                    rel: RelSelf,
                    content_type: ContentActivityPlusJson,
                    href: "https://tech.lgbt/users/natty".to_owned(),
                },
                WebFingerRel::RelOStatusSubscribe {
                    rel: RelOStatusSubscribe,
                    template: "https://tech.lgbt/authorize_interaction?uri={uri}".to_owned(),
                },
            ],
        };

        assert_eq!(webfinger, real)
    }
}
