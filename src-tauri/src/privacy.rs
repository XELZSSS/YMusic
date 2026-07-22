use std::borrow::Cow;

use crate::config;

/// Strip CSP headers and inject CSS into HTML responses so styles are
/// applied before the page renders (prevents text flash).
pub fn on_resource_request(
    request: http::Request<Vec<u8>>,
    response: &mut http::Response<Cow<'static, [u8]>>,
) {
    let is_yt = request.uri().host().is_some_and(|h| {
        h == "music.youtube.com" || h.ends_with(".youtube.com")
    });
    if !is_yt {
        return;
    }

    // Strip CSP so our injected ressources aren't blocked
    response.headers_mut().remove("content-security-policy");
    response.headers_mut().remove("Content-Security-Policy");
    response.headers_mut().remove("content-security-policy-report-only");
    response.headers_mut().remove("Content-Security-Policy-Report-Only");

    // Inject CSS into the main HTML document before it renders
    let is_html = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.contains("text/html"));
    if !is_html {
        return;
    }

    let body = response.body().to_vec();
    if body.is_empty() {
        return;
    }
    let body_str = String::from_utf8_lossy(&body);
    let style_tag = format!("<style>{}</style>", config::INJECTED_CSS);
    if let Some(pos) = body_str.find("</head>") {
        let mut new_body = String::with_capacity(body_str.len() + style_tag.len());
        new_body.push_str(&body_str[..pos]);
        new_body.push_str(&style_tag);
        new_body.push_str(&body_str[pos..]);
        *response.body_mut() = Cow::Owned(new_body.into_bytes());
    }
}
