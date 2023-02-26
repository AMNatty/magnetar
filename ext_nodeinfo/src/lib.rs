use crate::version_1_0::NodeInfo10;
use crate::version_1_1::NodeInfo11;
use crate::version_2_0::NodeInfo20;
use crate::version_2_1::NodeInfo21;
use serde::{Deserialize, Serialize};

pub mod version_1_0;
pub mod version_1_1;
pub mod version_2_0;
pub mod version_2_1;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum NodeInfo {
    #[serde(rename = "1.0")]
    V1_0(NodeInfo10),
    #[serde(rename = "1.1")]
    V1_1(NodeInfo11),
    #[serde(rename = "2.0")]
    V2_0(NodeInfo20),
    #[serde(rename = "2.1")]
    V2_1(NodeInfo21),
}

#[cfg(test)]
mod test {
    use crate::version_1_0::{
        NodeInfo10Services, NodeInfo10Software, NodeInfo10Usage, NodeInfo10UsageUsers,
    };
    use crate::version_2_0::NodeInfo20;
    use crate::version_2_1::{NodeInfo21, NodeInfo21Software};
    use crate::NodeInfo;
    use serde_json::json;
    use std::collections::{HashMap, HashSet};

    fn json_mastodon() -> serde_json::Value {
        json!({
            "version": "2.0",
            "software": {
                "name": "mastodon",
                "version": "4.1.0"
            },
            "protocols": [
                "activitypub"
            ],
            "services": {
                "outbound": [],
                "inbound": []
            },
            "usage": {
                "users": {
                    "total": 10360,
                    "activeMonth": 4627,
                    "activeHalfyear": 10089
                },
                "localPosts": 1033206
            },
            "openRegistrations": true,
            "metadata": {}
        })
    }

    fn data_mastodon() -> NodeInfo20 {
        NodeInfo20 {
            software: NodeInfo10Software {
                name: "mastodon".to_owned(),
                version: "4.1.0".to_owned(),
            },
            protocols: HashSet::from(["activitypub".to_owned()]),
            services: NodeInfo10Services {
                inbound: HashSet::new(),
                outbound: HashSet::new(),
            },
            open_registrations: true,
            usage: NodeInfo10Usage {
                users: NodeInfo10UsageUsers {
                    total: Some(10360),
                    active_halfyear: Some(10089),
                    active_month: Some(4627),
                },
                local_posts: Some(1033206),
                local_comments: None,
            },
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn should_serialize_nodeinfo_20() {
        let json = json_mastodon();

        let node_info = serde_json::to_value(NodeInfo::V2_0(data_mastodon())).unwrap();

        assert_eq!(node_info, json);
    }

    #[test]
    fn should_parse_nodeinfo_20() {
        let json = json_mastodon();

        let node_info: NodeInfo = serde_json::from_value(json).unwrap();

        assert_eq!(node_info, NodeInfo::V2_0(data_mastodon()));
    }

    #[test]
    fn should_parse_nodeinfo_21() {
        let json = json!({
            "version": "2.1",
            "software": {
              "name": "calckey",
              "version": "13.1.2",
              "repository": "https://github.com/misskey-dev/misskey",
              "homepage": "https://calckey.cloud"
            },
            "protocols": [
              "activitypub"
            ],
            "services": {
              "inbound": [],
              "outbound": [
                "atom1.0",
                "rss2.0"
              ]
            },
            "openRegistrations": false,
            "usage": {
              "users": {
                "total": 2,
                "activeHalfyear": 1,
                "activeMonth": 1
              },
              "localPosts": 1936,
              "localComments": 0
            },
            "metadata": {}
        });

        let node_info: NodeInfo = serde_json::from_value(json).unwrap();

        assert_eq!(
            node_info,
            NodeInfo::V2_1(NodeInfo21 {
                software: NodeInfo21Software {
                    name: "calckey".to_owned(),
                    version: "13.1.2".to_owned(),
                    homepage: Some("https://calckey.cloud".to_owned()),
                    repository: Some("https://github.com/misskey-dev/misskey".to_owned())
                },
                protocols: HashSet::from(["activitypub".to_owned()]),
                services: NodeInfo10Services {
                    inbound: HashSet::new(),
                    outbound: HashSet::from(["atom1.0".to_owned(), "rss2.0".to_owned()])
                },
                open_registrations: false,
                usage: NodeInfo10Usage {
                    users: NodeInfo10UsageUsers {
                        total: Some(2),
                        active_halfyear: Some(1),
                        active_month: Some(1)
                    },
                    local_posts: Some(1936),
                    local_comments: Some(0)
                },
                metadata: HashMap::new()
            })
        );
    }
}
