use anyhow::anyhow;
use magnetar_core::web_model::acct::Acct;
use percent_encoding::percent_decode_str;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct FediverseTag {
    pub name: String,
    pub host: Option<String>,
}

impl<S1: AsRef<str>, S2: AsRef<str>> From<(S1, Option<S2>)> for FediverseTag {
    fn from((name, host): (S1, Option<S2>)) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            host: host.as_ref().map(S2::as_ref).map(str::to_owned),
        }
    }
}

impl From<FediverseTag> for Acct {
    fn from(value: FediverseTag) -> Self {
        value.to_string().into()
    }
}

impl ToString for FediverseTag {
    fn to_string(&self) -> String {
        if let Some(ref host) = self.host {
            format!("{}@{host}", self.name)
        } else {
            self.name.clone()
        }
    }
}

pub fn lenient_parse_acct(acct: &Acct) -> anyhow::Result<FediverseTag> {
    lenient_parse_tag(acct.as_ref())
}

fn split_tag_inner(tag: impl AsRef<str>) -> (String, Option<String>) {
    let tag = tag.as_ref();
    let tag = tag.strip_prefix('@').unwrap_or(tag.as_ref());

    match tag.split_once('@') {
        Some((name, host)) if name.is_empty() => (host.to_owned(), None),
        Some((name, host)) => (name.to_owned(), Some(host.to_owned())),
        None => (tag.to_owned(), None),
    }
}

fn validate_tag_inner((name, host): (&str, Option<&str>)) -> anyhow::Result<()> {
    if name
        .chars()
        .any(|c| !c.is_alphanumeric() && c != '-' && c != '.')
    {
        return Err(anyhow!("Invalid char in tag: {name}"));
    }

    if let Some(host_str) = host {
        if host_str
            .chars()
            .any(|c| c.is_control() || c.is_whitespace() || c == '/' || c == '#')
        {
            return Err(anyhow!("Invalid char in tag: {name}"));
        }
    }

    Ok(())
}

pub fn lenient_parse_tag(tag: impl AsRef<str>) -> anyhow::Result<FediverseTag> {
    let (name, host) = split_tag_inner(tag);

    validate_tag_inner((&name, host.as_ref().map(String::as_ref)))?;

    Ok(FediverseTag { name, host })
}

pub fn lenient_parse_acct_decode(acct: &Acct) -> anyhow::Result<FediverseTag> {
    lenient_parse_tag_decode(acct.as_ref())
}

pub fn lenient_parse_tag_decode(tag: impl AsRef<str>) -> anyhow::Result<FediverseTag> {
    let (name, host) = split_tag_inner(tag);

    let name_decoded = percent_decode_str(&name).decode_utf8()?;
    let host_decoded = host
        .map(|host| percent_decode_str(&host).decode_utf8().map(Cow::into_owned))
        .transpose()?;

    validate_tag_inner((&name_decoded, host_decoded.as_deref()))?;

    Ok(FediverseTag {
        name: name_decoded.into_owned(),
        host: host_decoded,
    })
}
