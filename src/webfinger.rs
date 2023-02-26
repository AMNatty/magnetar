use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use magnetar_core::web_model::acct::Acct;
use magnetar_core::web_model::content_type::{ContentActivityStreams, ContentHtml};
use magnetar_core::web_model::rel::{RelOStatusSubscribe, RelSelf, RelWebFingerProfilePage};
use magnetar_webfinger::webfinger::{WebFinger, WebFingerRel, WebFingerSubject};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebFingerQuery {
    resource: WebFingerSubject,
    rel: Option<Vec<String>>,
}

// TODO: Filter by rel
pub async fn handle_webfinger(
    Query(WebFingerQuery { resource, rel, .. }): Query<WebFingerQuery>,
) -> Result<Json<WebFinger>, StatusCode> {
    let resource = match resource {
        acct @ WebFingerSubject::Acct(_) => acct,
        // Leniently re-add the acct
        WebFingerSubject::Url(url) if url.contains('@') && !url.starts_with("http") => {
            WebFingerSubject::Acct(Acct::new(url.into()))
        }
        other => other,
    };

    Ok(Json(WebFinger {
        subject: WebFingerSubject::Acct("natty@tech.lgbt".into()),
        aliases: vec![
            WebFingerSubject::Url("https://tech.lgbt/@natty".to_owned()),
            WebFingerSubject::Url("https://tech.lgbt/users/natty".to_owned()),
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
    }))
}
