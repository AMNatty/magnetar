use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NodeInfo10Protocols {
    #[doc = "The protocols this server can receive traffic for."]
    pub inbound: HashSet<String>,
    #[doc = "The protocols this server can generate traffic for."]
    pub outbound: HashSet<String>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NodeInfo10Services {
    #[doc = "The third party sites this server can retrieve messages from for combined display with "]
    #[doc = "regular traffic."]
    pub inbound: HashSet<String>,
    #[doc = "The third party sites this server can publish messages to on the behalf of a user."]
    pub outbound: HashSet<String>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NodeInfo10Software {
    #[doc = "The canonical name of this server software."]
    pub name: String,
    #[doc = "The version of this server software."]
    pub version: String,
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct NodeInfo10UsageUsers {
    #[doc = "The amount of users that signed in at least once in the last 180 days."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeHalfyear")]
    pub active_halfyear: Option<i64>,
    #[doc = "The amount of users that signed in at least once in the last 30 days."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeMonth")]
    pub active_month: Option<i64>,
    #[doc = "The total amount of on this server registered users."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NodeInfo10Usage {
    #[doc = "The amount of comments that were made by users that are registered on this server."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "localComments")]
    pub local_comments: Option<i64>,
    #[doc = "The amount of posts that were made by users that are registered on this server."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "localPosts")]
    pub local_posts: Option<i64>,
    #[doc = "statistics about the users of this server."]
    pub users: NodeInfo10UsageUsers,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NodeInfo10 {
    #[doc = "The schema version, must be 1.0."]
    pub version: String,
    #[doc = "Free form key value pairs for software specific values. Clients should not rely on any "]
    #[doc = "specific key present."]
    pub metadata: HashMap<String, serde_json::Value>,
    #[doc = "Whether this server allows open self-registration."]
    #[serde(rename = "openRegistrations")]
    pub open_registrations: bool,
    #[doc = "The protocols supported on this server."]
    pub protocols: NodeInfo10Protocols,
    #[doc = "The third party sites this server can connect to via their application API."]
    pub services: NodeInfo10Services,
    #[doc = "Metadata about server software in use."]
    pub software: NodeInfo10Protocols,
    #[doc = "Usage statistics for this server."]
    pub usage: NodeInfo10Usage,
}
