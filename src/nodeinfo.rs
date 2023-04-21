use crate::config::MagnetarConfig;
use axum::extract::State;
use axum::Json;
use magnetar_core::web_model::rel::{RelNodeInfo20, RelNodeInfo21};
use magnetar_core::web_model::Rel;
use magnetar_nodeinfo::version_1_0::{
    NodeInfo10Services, NodeInfo10Software, NodeInfo10Usage, NodeInfo10UsageUsers,
};
use magnetar_nodeinfo::version_2_0::NodeInfo20;
use magnetar_nodeinfo::version_2_1::{NodeInfo21, NodeInfo21Software};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

const NODEINFO_PATH: &str = "/nodeinfo";

#[derive(Clone, Debug, Serialize)]
pub struct NodeInfoLink {
    rel: &'static str,
    href: &'static str,
}

pub async fn handle_nodeinfo(
    State(config): State<&'static MagnetarConfig>,
) -> Json<Vec<NodeInfoLink>> {
    Json(vec![
        NodeInfoLink {
            href: "/nodeinfo/2.0",
            rel: RelNodeInfo20.rel(),
        },
        NodeInfoLink {
            href: "/nodeinfo/2.1",
            rel: RelNodeInfo21.rel(),
        },
    ])
}

pub async fn handle_nodeinfo_21(State(config): State<&'static MagnetarConfig>) -> Json<NodeInfo21> {
    Json(NodeInfo21 {
        software: NodeInfo21Software {
            name: config.branding.name.clone(),
            version: config.branding.version.clone(),
            homepage: Some(config.branding.homepage.clone()),
            repository: Some(config.branding.repository.clone()),
        },
        protocols: HashSet::from(["activitypub".to_owned()]),
        services: NodeInfo10Services {
            inbound: HashSet::new(),
            outbound: HashSet::new(),
        },
        open_registrations: false,
        usage: NodeInfo10Usage {
            users: NodeInfo10UsageUsers {
                total: Some(0),
                active_halfyear: Some(0),
                active_month: Some(0),
            },
            local_posts: Some(0),
            local_comments: Some(0),
        },
        metadata: HashMap::new(),
    })
}

pub async fn handle_nodeinfo_20(State(config): State<&'static MagnetarConfig>) -> Json<NodeInfo20> {
    Json(NodeInfo20 {
        software: NodeInfo10Software {
            name: config.branding.name.clone(),
            version: config.branding.version.clone(),
        },
        protocols: HashSet::from(["activitypub".to_owned()]),
        services: NodeInfo10Services {
            inbound: HashSet::new(),
            outbound: HashSet::new(),
        },
        open_registrations: false,
        usage: NodeInfo10Usage {
            users: NodeInfo10UsageUsers {
                total: Some(0),
                active_halfyear: Some(0),
                active_month: Some(0),
            },
            local_posts: Some(0),
            local_comments: None,
        },
        metadata: HashMap::new(),
    })
}
