use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Acct(String);

impl Acct {
    pub fn new(uri_without_acct: Cow<'_, str>) -> Self {
        Acct(uri_without_acct.to_string())
    }
}

impl From<&str> for Acct {
    fn from(value: &str) -> Self {
        Acct(value.strip_prefix("acct:").unwrap_or(value).to_string())
    }
}

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
            Err(de::Error::custom(
                "Missing acct protocol for account!".to_owned(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::web_model::acct::Acct;
    use serde_json::json;

    #[test]
    fn should_remove_acct_prefix() {
        let json = json!("acct:natty@tech.lgbt");

        let acct: Acct = serde_json::from_value(json).unwrap();

        assert_eq!(acct, Acct::from("natty@tech.lgbt"))
    }

    #[test]
    fn should_add_acct_prefix() {
        let acct = Acct::from("natty@tech.lgbt");
        let json = serde_json::to_value(acct).unwrap();

        assert_eq!(json, json!("acct:natty@tech.lgbt"));
    }
}
