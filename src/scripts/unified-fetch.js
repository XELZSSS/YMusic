(function() {
  "use strict";
  var TRACK = [
    'fbclid','gclid','gclsrc','dclid','gbraid','wbraid',
    'msclkid','twclid','igshid','yclid','li_fat_id',
    'mc_cid','mc_eid','_hsenc','_hsmi','mkt_tok',
    'utm_source','utm_medium','utm_campaign','utm_term',
    'utm_content','utm_id','utm_reader',
    'utm_referrer','utm_name','utm_social','utm_social-type',
  ];

  function cleanTrackingParams(u) {
    if (typeof u !== 'string') return u;
    if (u.startsWith('data:') || u.startsWith('blob:')) return u;
    try {
      var url = new URL(u, location.origin);
      var changed = false;
      for (var i = 0; i < TRACK.length; i++) {
        if (url.searchParams.has(TRACK[i])) { url.searchParams.delete(TRACK[i]); changed = true; }
      }
      return changed ? url.toString() : u;
    } catch(e) { return u; }
  }

  function tweakInnertubeRequest(input, init) {
    if (init && typeof init.body === 'string') {
      var url = typeof input === 'string' ? input : (input instanceof Request ? input.url : null);
      if (url && url.indexOf('/youtubei/v1/') !== -1) {
        try {
          var body = JSON.parse(init.body);
          body.contentCheckOk = true;
          body.racyCheckOk = true;
          if (body.context && body.context.client) {
            body.context.client.clientName = 'WEB_REMIX';
            body.context.client.clientVersion = '1.20250310.01.00';
            body.context.client.platform = 'DESKTOP';
            body.context.client.osName = 'Windows';
            body.context.client.osVersion = '10';
          }
          init = Object.assign({}, init, { body: JSON.stringify(body) });
        } catch(e) {}
      }
    }
    return init;
  }

  var _origFetch = window.fetch;
  window.fetch = function(input, init) {
    if (typeof input === 'string') {
      input = cleanTrackingParams(input);
    } else if (input instanceof Request) {
      var cleaned = cleanTrackingParams(input.url);
      if (cleaned !== input.url) {
        input = new Request(cleaned, input);
      }
    }
    init = tweakInnertubeRequest(input, init);
    return _origFetch.call(this, input, init);
  };

  var ps = history.pushState, rs = history.replaceState;
  history.pushState = function(data, title, url) {
    var cleaned = typeof url === 'string' ? cleanTrackingParams(url) : url;
    return ps.call(this, data, title, cleaned || url);
  };
  history.replaceState = function(data, title, url) {
    var cleaned = typeof url === 'string' ? cleanTrackingParams(url) : url;
    return rs.call(this, data, title, cleaned || url);
  };
  var cur = cleanTrackingParams(location.href);
  if (cur !== location.href) rs.call(history, null, '', cur);
})();
