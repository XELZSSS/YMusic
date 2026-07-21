use std::borrow::Cow;

/// Strip CSP headers from YouTube responses so the webview can load
/// custom fonts, styles, and scripts without restriction.
pub fn on_resource_request(
    request: http::Request<Vec<u8>>,
    response: &mut http::Response<Cow<'static, [u8]>>,
) {
    if let Some(host) = request.uri().host() {
        if host.ends_with(".youtube.com") || host == "music.youtube.com" {
            response.headers_mut().remove("content-security-policy");
            response.headers_mut().remove("Content-Security-Policy");
            response.headers_mut().remove("content-security-policy-report-only");
            response.headers_mut().remove("Content-Security-Policy-Report-Only");
        }
    }
}

/*
  Tracking parameter stripping (utm_*, fbclid, gclid, etc.) is handled
  by src/strip-tracking.js, injected via webview.eval in lib.rs.
  It patches fetch() and history.pushState/replaceState to clean URLs.
*/
