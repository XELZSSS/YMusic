(function() {
  "use strict";
  var ALL_SELECTORS = [
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
  ].join(',');

  function remove() {
    var els = document.querySelectorAll(ALL_SELECTORS);
    for (var i = 0; i < els.length; i++) {
      var el = els[i];
      if (el && el.parentNode) el.remove();
    }
  }

  remove();

  var debounceId = null;
  var obs = new MutationObserver(function() {
    if (debounceId) clearTimeout(debounceId);
    debounceId = setTimeout(remove, 150);
  });
  if (document.documentElement) {
    obs.observe(document.documentElement, { childList: true, subtree: true });
  }
  window.__ym_cleanup = window.__ym_cleanup || [];
  window.__ym_cleanup.push(function() { obs.disconnect(); if (debounceId) clearTimeout(debounceId); });
})();
