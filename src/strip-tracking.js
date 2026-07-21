(function() {
  var params = new Set([
    'fbclid', 'gclid', 'gclsrc', 'dclid', 'gbraid', 'wbraid',
    'msclkid', 'twclid', 'igshid', 'yclid', 'li_fat_id',
    'mc_cid', 'mc_eid', '_hsenc', '_hsmi', 'mkt_tok',
    'utm_source', 'utm_medium', 'utm_campaign', 'utm_term',
    'utm_content', 'utm_id', 'utm_cid', 'utm_reader',
    'utm_referrer', 'utm_name', 'utm_social', 'utm_social-type',
  ]);

  function clean(urlStr) {
    try {
      var url = new URL(urlStr, location.origin);
      var removed = 0;
      params.forEach(function(p) { if (url.searchParams.has(p)) { url.searchParams.delete(p); removed++; } });
      if (removed > 0) return url.toString();
    } catch(e) {}
    return urlStr;
  }

  function cleanLocation() {
    var cleaned = clean(location.href);
    if (cleaned !== location.href) history.replaceState(null, '', cleaned);
  }

  var origFetch = window.fetch;
  window.fetch = function(input, init) {
    if (typeof input === 'string') input = clean(input);
    else if (input instanceof Request && input.url) {
      var cleaned = clean(input.url);
      if (cleaned !== input.url) input = new Request(cleaned, input);
    }
    return origFetch.call(this, input, init);
  };

  var origPush = history.pushState;
  history.pushState = function() { origPush.apply(this, arguments); cleanLocation(); };
  var origReplace = history.replaceState;
  history.replaceState = function() { origReplace.apply(this, arguments); cleanLocation(); };

  cleanLocation();
})();
