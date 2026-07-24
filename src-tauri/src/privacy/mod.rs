use std::borrow::Cow;

use crate::app::config;

fn is_youtube_request(request: &http::Request<Vec<u8>>) -> bool {
    request.uri().host().is_some_and(|h| {
        h == "music.youtube.com" || h.ends_with(".youtube.com")
    })
}

fn strip_csp_headers(response: &mut http::Response<Cow<'static, [u8]>>) {
    response.headers_mut().remove("content-security-policy");
    response.headers_mut().remove("content-security-policy-report-only");
}

fn is_html_response(response: &http::Response<Cow<'static, [u8]>>) -> bool {
    response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.contains("text/html"))
}

fn inject_styles(response: &mut http::Response<Cow<'static, [u8]>>) {
    if response.body().is_empty() {
        return;
    }
    let body_str = String::from_utf8_lossy(response.body());
    let Some(pos) = body_str.find("</head>") else {
        return;
    };
    let style_tag = format!("<style>{}</style>", config::INJECTED_CSS);
    let mut new_body = String::with_capacity(body_str.len() + style_tag.len());
    new_body.push_str(&body_str[..pos]);
    new_body.push_str(&style_tag);
    new_body.push_str(&body_str[pos..]);
    *response.body_mut() = Cow::Owned(new_body.into_bytes());
}

pub fn on_resource_request(
    request: http::Request<Vec<u8>>,
    response: &mut http::Response<Cow<'static, [u8]>>,
) {
    if !is_youtube_request(&request) {
        return;
    }

    strip_csp_headers(response);

    if is_html_response(response) {
        inject_styles(response);
    }
}
