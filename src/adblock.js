(function() {
  if (window.__ym_adblock) return;
  window.__ym_adblock = true;

  /* ── 1. Inject CSS ── */
  function injectCSS() {
    if (!window.__YM_CSS || document.getElementById('__ym-css')) return;
    var s = document.createElement('style');
    s.id = '__ym-css';
    s.textContent = window.__YM_CSS;
    document.head.appendChild(s);
  }
  if (document.readyState === 'loading')
    document.addEventListener('DOMContentLoaded', injectCSS);
  else injectCSS();

  /* ── 2. API-level ad interception ── */
  (function() {
    function prune(o) {
      if (o && typeof o === 'object') {
        delete o.playerAds;
        delete o.adPlacements;
        delete o.adSlots;
        if (o.playerResponse) {
          delete o.playerResponse.playerAds;
          delete o.playerResponse.adPlacements;
          delete o.playerResponse.adSlots;
        }
      }
      return o;
    }
    JSON.parse = new Proxy(JSON.parse, {
      apply: function(t, _, a) { return prune(Reflect.apply(t, _, a)); }
    });
    Response.prototype.json = new Proxy(Response.prototype.json, {
      apply: function(t, s, a) {
        return Reflect.apply(t, s, a).then(function(v) { return prune(v); });
      }
    });
  })();

  /* ── 3. DOM ad removal (static selectors only, no remote fetching) ── */
  var SELECTORS = [
    'ytmusic-mealbar-promo-renderer',
    'ytmusic-ad-placeholder',
    'ytmusic-ad-in-collection',
    'ytmusic-ad-banner',
    'ytd-action-companion-ad-renderer',
    'ytd-display-ad-renderer',
    'ytd-video-masthead-ad-v3-renderer',
    'ytd-ad-slot-renderer',
    'ytd-companion-slot-renderer',
    'ytd-rich-item[rendered-with-ad]',
    'ytd-promoted-sparkles-text-search-renderer',
    'ytd-promoted-video-renderer',
    'ytd-in-feed-ad-layout-renderer',
    'ytd-banner-promo-renderer',
    'ytd-unlimited-offer-renderer',
    '#masthead-ad',
    '#player-ads',
    '.ytp-ad-module',
    '.video-ads',
    '.ytp-ad-image-overlay',
    '.ytp-ad-overlay-container',
    'ytmusic-cast-button',
  ];

  function removeAds() {
    var all = SELECTORS.join(',');
    var els = document.querySelectorAll(all);
    for (var i = 0; i < els.length; i++) {
      var el = els[i];
      if (el && el.parentNode) el.remove();
    }
    var mealbar = document.querySelector('ytmusic-mealbar-promo-renderer');
    if (mealbar) {
      var btn = mealbar.querySelector('#dismiss-button, [aria-label="Close"], tp-yt-paper-icon-button');
      if (btn) btn.click();
      mealbar.remove();
    }
  }

  if (document.documentElement) {
    var obs = new MutationObserver(removeAds);
    obs.observe(document.documentElement, { childList: true, subtree: true });
  }
  removeAds();
  setInterval(removeAds, 2000);

  /* ── 4. Audio ad handling ── */
  (function() {
    var wasAd = false, userMuted = false;
    setInterval(function() {
      var video = document.querySelector('video');
      var isAd = document.querySelector('ytmusic-player-bar[ad-active], .ad-showing');
      if (isAd) {
        if (!wasAd && video) { userMuted = video.muted; video.muted = true; }
        var skip = document.querySelector('ytmusic-skip-ad-button');
        if (skip) skip.click();
        wasAd = true;
      } else {
        if (wasAd && video && !userMuted) video.muted = false;
        wasAd = false;
      }
    }, 1000);
  })();

  /* ── 5. Strip tracking params ── */
  (function() {
    var TRACK = [
      'fbclid','gclid','gclsrc','dclid','gbraid','wbraid',
      'msclkid','twclid','igshid','yclid','li_fat_id',
      'mc_cid','mc_eid','_hsenc','_hsmi','mkt_tok',
      'utm_source','utm_medium','utm_campaign','utm_term',
      'utm_content','utm_id','utm_reader',
      'utm_referrer','utm_name','utm_social','utm_social-type',
    ];
    function clean(u) {
      try {
        var url = new URL(u, location.origin);
        var changed = false;
        for (var i = 0; i < TRACK.length; i++) {
          if (url.searchParams.has(TRACK[i])) { url.searchParams.delete(TRACK[i]); changed = true; }
        }
        return changed ? url.toString() : u;
      } catch(e) { return u; }
    }
    var origFetch = window.fetch;
    window.fetch = function(input, init) {
      if (typeof input === 'string') input = clean(input);
      else if (input instanceof Request && input.url) {
        var c = clean(input.url);
        if (c !== input.url) input = new Request(c, input);
      }
      return origFetch.call(this, input, init);
    };
    var ps = history.pushState, rs = history.replaceState;
    history.pushState = function() {
      ps.apply(this, arguments);
      var c = clean(location.href);
      if (c !== location.href) rs.call(history, null, '', c);
    };
    history.replaceState = function() {
      rs.apply(this, arguments);
      var c = clean(location.href);
      if (c !== location.href) rs.call(history, null, '', c);
    };
    var cur = clean(location.href);
    if (cur !== location.href) rs.call(history, null, '', cur);
  })();
})();
