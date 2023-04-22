use crate::config::MagnetarConfig;
use crate::util::{lenient_parse_acct_decode, FediverseTag};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use hyper::header;
use magnetar_calckey_model::CalckeyModel;
use magnetar_core::web_model::acct::Acct;
use magnetar_core::web_model::content_type::{ContentActivityStreams, ContentHtml, ContentJrdJson};
use magnetar_core::web_model::rel::{RelOStatusSubscribe, RelSelf, RelWebFingerProfilePage};
use magnetar_webfinger::webfinger::{WebFinger, WebFingerRel, WebFingerSubject};
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
pub struct WebFingerQuery {
    resource: WebFingerSubject,
    rel: Option<Vec<String>>,
}

// TODO: Filter by rel
pub async fn handle_webfinger(
    Query(WebFingerQuery { resource, rel, .. }): Query<WebFingerQuery>,
    State((config, ck)): State<(&'static MagnetarConfig, CalckeyModel)>,
) -> Result<impl IntoResponse, StatusCode> {
    let resource = match resource {
        acct @ WebFingerSubject::Acct(_) => acct,
        // Leniently re-add the acct
        WebFingerSubject::Url(url) if !url.starts_with("http:") && !url.starts_with("https:") => {
            WebFingerSubject::Acct(Acct::new(url.into()))
        }
        other => other,
    };

    let user = match resource {
        WebFingerSubject::Acct(acct) => {
            let tag = lenient_parse_acct_decode(&acct).map_err(|e| {
                error!("Failed to parse tag: {e}");
                StatusCode::UNPROCESSABLE_ENTITY
            })?;

            ck.get_user_by_tag(
                &tag.name,
                tag.host
                    .filter(|host| *host != config.networking.host)
                    .as_deref(),
            )
            .await
            .map_err(|e| {
                error!("Data error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
        }
        // Kinda a
        WebFingerSubject::Url(url) => ck.get_user_by_uri(&url).await.map_err(|e| {
            error!("Data error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?,
    };

    if user.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let user = user.unwrap();
    let tag = FediverseTag::from((
        &user.username,
        user.host.as_ref().or(Some(&config.networking.host)),
    ));

    let mut links = Vec::new();
    let mut aliases = Vec::new();

    match tag.host {
        Some(ref host) if host != &config.networking.host => {}
        _ => {
            links.push(WebFingerRel::RelOStatusSubscribe {
                rel: RelOStatusSubscribe,
                template: format!(
                    "{}://{}/authorize-follow?acct={{uri}}",
                    config.networking.protocol, config.networking.host
                ),
            });

            let user_url = format!(
                "{}://{}/@{}",
                config.networking.protocol, config.networking.host, tag.name
            );

            links.push(WebFingerRel::RelWebFingerProfilePage {
                rel: RelWebFingerProfilePage,
                content_type: ContentHtml,
                href: user_url.clone(),
            });

            aliases.push(WebFingerSubject::Url(user_url));
        }
    }

    if let Some(uri) = user.uri {
        links.push(WebFingerRel::RelSelf {
            rel: RelSelf,
            content_type: ContentActivityStreams,
            href: uri,
        });
    }

    Ok((
        [(header::CONTENT_TYPE, ContentJrdJson.as_ref())],
        Json(WebFinger {
            subject: WebFingerSubject::Acct(tag.into()),
            aliases: Some(aliases),
            links,
        }),
    ))
}
