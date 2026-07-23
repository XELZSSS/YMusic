window.__ym_eq_bands = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
window.__ym_eq_preamp = 0;
window.__ym_eq_enabled = true;

var EQ_FREQ = [31, 62, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];

function __eq_setup(video) {
  if (!video || video.__ym_eq_done) return;

  try {
    var ctx = new (window.AudioContext || window.webkitAudioContext)();
    if (ctx.state === 'suspended') ctx.resume();

    var source = ctx.createMediaElementSource(video);

    var preGain = ctx.createGain();
    preGain.gain.value = 1;
    var filters = [];
    var prev = preGain;

    for (var i = 0; i < EQ_FREQ.length; i++) {
      var f = ctx.createBiquadFilter();
      f.type = 'peaking';
      f.frequency.value = EQ_FREQ[i];
      f.Q.value = Math.SQRT1_2;
      f.gain.value = 0;
      prev.connect(f);
      prev = f;
      filters.push(f);
    }

    source.connect(preGain);
    prev.connect(ctx.destination);

    window.__ym_eq_ctx = ctx;
    window.__ym_eq_filters = filters;
    window.__ym_eq_pregain = preGain;

    video.__ym_eq_done = true;

    __eq_apply();
  } catch(e) {
    console.warn('[YMusic] EQ setup failed:', e);
  }
}

function __eq_apply() {
  if (!window.__ym_eq_filters) return;
  var on = window.__ym_eq_enabled;
  var fs = window.__ym_eq_filters;
  for (var i = 0; i < fs.length; i++) {
    fs[i].gain.value = on ? window.__ym_eq_bands[i] : 0;
  }
  if (window.__ym_eq_pregain) {
    window.__ym_eq_pregain.gain.value = on ? Math.pow(10, window.__ym_eq_preamp / 20) : 1;
  }
}

window.__ym_eq_set_band = function(idx, gainDb) {
  if (idx >= 0 && idx < window.__ym_eq_bands.length) {
    window.__ym_eq_bands[idx] = gainDb;
    __eq_apply();
  }
};

window.__ym_eq_set_preamp = function(gainDb) {
  window.__ym_eq_preamp = gainDb;
  __eq_apply();
};

window.__ym_eq_toggle = function(enabled) {
  window.__ym_eq_enabled = enabled;
  __eq_apply();
};

window.__ym_eq_apply_preset = function(bands, preamp) {
  if (bands && bands.length === window.__ym_eq_bands.length) {
    for (var i = 0; i < bands.length; i++) window.__ym_eq_bands[i] = bands[i];
  }
  if (preamp !== undefined) window.__ym_eq_preamp = preamp;
  __eq_apply();
};

(function() {
  function tryBind() {
    var video = document.querySelector('video');
    if (video && video.readyState >= 2) {
      __eq_setup(video);
    }
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', tryBind);
  } else {
    tryBind();
  }

  var obs = new MutationObserver(function() {
    var video = document.querySelector('video');
    if (video && !video.__ym_eq_done && video.readyState >= 2) {
      __eq_setup(video);
    }
  });
  if (document.body) {
    obs.observe(document.body, { childList: true, subtree: true });
  } else {
    document.addEventListener('DOMContentLoaded', function() {
      obs.observe(document.body, { childList: true, subtree: true });
    });
  }
})();
