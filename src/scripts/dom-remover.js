(function() {
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

  function remove() {
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

  var debounceId = null;
  function onMutation() {
    if (debounceId) clearTimeout(debounceId);
    debounceId = setTimeout(remove, 150);
  }

  if (document.documentElement) {
    var obs = new MutationObserver(onMutation);
    obs.observe(document.documentElement, { childList: true, subtree: true });
  }
  remove();
  setInterval(remove, 5000);
})();
