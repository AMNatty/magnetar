use crate::version_1_0::{
    NodeInfo10Protocols, NodeInfo10Services, NodeInfo10Software, NodeInfo10Usage,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NodeInfo11 {
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
    pub software: NodeInfo10Software,
    #[doc = "Usage statistics for this server."]
    pub usage: NodeInfo10Usage,
}
