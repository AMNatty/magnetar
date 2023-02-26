use magnetar_core::web_model::acct::Acct;
use magnetar_core::web_model::content_type::{ContentActivityStreams, ContentHtml};
use magnetar_core::web_model::rel::{RelOStatusSubscribe, RelSelf, RelWebFingerProfilePage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WebFinger {
    pub subject: WebFingerSubject,
    pub aliases: Vec<WebFingerSubject>,
    pub links: Vec<WebFingerRel>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebFingerSubject {
    Acct(Acct),
    Url(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::enum_variant_names)]
pub enum WebFingerRel {
    RelWebFingerProfilePage {
        rel: RelWebFingerProfilePage,
        #[serde(rename = "type")]
        content_type: ContentHtml,
        href: String,
    },
    RelSelf {
        rel: RelSelf,
        #[serde(rename = "type")]
        content_type: ContentActivityStreams,
        href: String,
    },
    RelOStatusSubscribe {
        rel: RelOStatusSubscribe,
        template: String,
    },
}

#[cfg(test)]
mod test {
    use crate::webfinger::WebFingerSubject::Url;
    use crate::webfinger::{WebFinger, WebFingerRel, WebFingerSubject};
    use magnetar_core::web_model::acct::Acct;
    use magnetar_core::web_model::content_type::{ContentActivityStreams, ContentHtml};
    use magnetar_core::web_model::rel::{RelOStatusSubscribe, RelSelf, RelWebFingerProfilePage};
    use serde_json::json;

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
            subject: WebFingerSubject::Acct(Acct::new("natty@tech.lgbt".into())),
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
                    content_type: ContentActivityStreams,
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
