use crate::config::{MagnetarConfig, MagnetarNetworking};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde_json::json;

pub async fn handle_actor_get(
    State(MagnetarConfig {
        networking: MagnetarNetworking { host, .. },
        ..
    }): State<MagnetarConfig>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let public_key_pkcs1_pem = pub_key
        .to_pkcs1_pem(LineEnding::LF)
        .expect("TODO: panic message");

    Ok(Json(json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1"
        ],

        "id": format!("https://{host}/actor"),
        "type": "Person",
        "preferredUsername": "alice",
        "inbox": format!("https://{host}/inbox"),

        "publicKey": {
            "id": "https://my-example.com/actor#main-key",
            "owner": "https://my-example.com/actor",
            "publicKeyPem": public_key_pkcs1_pem
        }
    })))
}

pub async fn handle_outbox_get() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
       "@context" : [
          "https://www.w3.org/ns/activitystreams",
          "https://w3id.org/security/v1",
          {
             "Emoji" : "toot:Emoji",
             "Hashtag" : "as:Hashtag",
             "PropertyValue" : "schema:PropertyValue",
             "_misskey_content" : "misskey:_misskey_content",
             "_misskey_quote" : "misskey:_misskey_quote",
             "_misskey_reaction" : "misskey:_misskey_reaction",
             "_misskey_talk" : "misskey:_misskey_talk",
             "_misskey_votes" : "misskey:_misskey_votes",
             "discoverable" : "toot:discoverable",
             "featured" : "toot:featured",
             "fedibird" : "http://fedibird.com/ns#",
             "isCat" : "misskey:isCat",
             "manuallyApprovesFollowers" : "as:manuallyApprovesFollowers",
             "misskey" : "https://misskey-hub.net/ns#",
             "movedToUri" : "as:movedTo",
             "quoteUri" : "fedibird:quoteUri",
             "quoteUrl" : "as:quoteUrl",
             "schema" : "http://schema.org#",
             "sensitive" : "as:sensitive",
             "toot" : "http://joinmastodon.org/ns#",
             "value" : "schema:value",
             "vcard" : "http://www.w3.org/2006/vcard/ns#"
          }
       ],
       "first" : "https://astolfo.social/users/9awy7u3l76/outbox?page=true",
       "id" : "https://astolfo.social/users/9awy7u3l76/outbox",
       "last" : "https://astolfo.social/users/9awy7u3l76/outbox?page=true&since_id=000000000000000000000000",
       "totalItems" : 1413,
       "type" : "OrderedCollection"
    })))
}
