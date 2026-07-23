(function() {
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
            body.context.client.clientName = 'ANDROID';
            body.context.client.clientVersion = '6.30.56';
            body.context.client.androidSdkVersion = 33;
            body.context.client.osVersion = '13';
            body.context.client.platform = 'MOBILE';
            body.context.client.userAgent = 'com.google.android.apps.youtube.music/6.30.56 (Linux; U; Android 13) gzip';
          }
          init = Object.assign({}, init, { body: JSON.stringify(body) });
        } catch(e) {}
      }
    }
    return init;
  }

  var _origFetch = window.fetch;
  window.fetch = function(input, init) {
    var url = typeof input === 'string' ? input : (input instanceof Request ? input.url : '');
    url = cleanTrackingParams(url);
    if (url !== input && typeof input === 'string') input = url;
    init = tweakInnertubeRequest(input, init);
    return _origFetch.call(this, input, init);
  };

  var ps = history.pushState, rs = history.replaceState;
  history.pushState = function() {
    ps.apply(this, arguments);
    var c = cleanTrackingParams(location.href);
    if (c !== location.href) rs.call(history, null, '', c);
  };
  history.replaceState = function() {
    rs.apply(this, arguments);
    var c = cleanTrackingParams(location.href);
    if (c !== location.href) rs.call(history, null, '', c);
  };
  var cur = cleanTrackingParams(location.href);
  if (cur !== location.href) rs.call(history, null, '', cur);
})();
