window.__ym = window.__ym || {};

(function(ym) {
  "use strict";
  var bands = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  var preamp = 0;
  var enabled = true;
  var FREQ = [31, 62, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];

  var ctx = null;
  var filters = null;
  var preGain = null;

  function teardown() {
    if (ctx && ctx.state !== 'closed') {
      ctx.close();
    }
    ctx = null;
    filters = null;
    preGain = null;
  }

  function setup(video) {
    if (!video || video.__ym_eq_done) return;
    teardown();
    try {
      var c = new (window.AudioContext || window.webkitAudioContext)();
      if (c.state === 'suspended') c.resume();
      var s = c.createMediaElementSource(video);
      var pg = c.createGain();
      pg.gain.value = 1;
      var fs = [];
      var prev = pg;
      for (var i = 0; i < FREQ.length; i++) {
        var f = c.createBiquadFilter();
        f.type = 'peaking';
        f.frequency.value = FREQ[i];
        f.Q.value = Math.SQRT1_2;
        f.gain.value = 0;
        prev.connect(f);
        prev = f;
        fs.push(f);
      }
      s.connect(pg);
      prev.connect(c.destination);
      ctx = c; filters = fs; preGain = pg;
      video.__ym_eq_done = true;
      apply();
    } catch(e) {
      console.warn('[YMusic] EQ setup failed:', e);
    }
  }

  function apply() {
    if (!filters) return;
    var on = enabled;
    for (var i = 0; i < filters.length; i++) {
      filters[i].gain.value = on ? bands[i] : 0;
    }
    if (preGain) {
      preGain.gain.value = on ? Math.pow(10, preamp / 20) : 1;
    }
  }

  ym.eq = {
    setBand: function(idx, gainDb) {
      if (idx >= 0 && idx < bands.length) { bands[idx] = gainDb; apply(); }
    },
    setPreamp: function(gainDb) { preamp = gainDb; apply(); },
    toggle: function(on) { enabled = on; apply(); },
    applyPreset: function(b, p) {
      if (b && b.length === bands.length) { for (var i = 0; i < b.length; i++) bands[i] = b[i]; }
      if (p !== undefined) preamp = p;
      apply();
    },
    getBands: function() { return bands.slice(); },
    getPreamp: function() { return preamp; },
    isEnabled: function() { return enabled; },
  };

  function tryBind() {
    var video = document.querySelector('video');
    if (video && video.readyState >= 2) setup(video);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', tryBind);
  } else {
    tryBind();
  }

  var obsDebounce = null;
  var obs = new MutationObserver(function() {
    if (obsDebounce) clearTimeout(obsDebounce);
    obsDebounce = setTimeout(function() {
      var video = document.querySelector('video');
      if (video && !video.__ym_eq_done && video.readyState >= 2) setup(video);
    }, 300);
  });
  if (document.body) {
    obs.observe(document.body, { childList: true, subtree: true });
  } else {
    document.addEventListener('DOMContentLoaded', function() {
      obs.observe(document.body, { childList: true, subtree: true });
    });
  }
})(window.__ym);
