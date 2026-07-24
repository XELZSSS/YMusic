(function() {
  function mergeDeep(t, s) {
    for (var k in s) {
      if (s[k] && typeof s[k] === 'object' && !Array.isArray(s[k])) {
        if (!t[k]) t[k] = {};
        mergeDeep(t[k], s[k]);
      } else { t[k] = s[k]; }
    }
  }
  function setCfg(v) { if (window.ytcfg) { for (var k in v) window.ytcfg.set(k, v[k]); } }
  var RETRIES = 0;
  var MAX_RETRIES = 100;
  function inject() {
    var yt = window.yt;
    if (!yt || !yt.config_) {
      if (++RETRIES < MAX_RETRIES) { setTimeout(inject, 50); }
      return;
    }
    setCfg({ AUDIO_QUALITY: 'AUDIO_QUALITY_HIGH', IS_SUBSCRIBER: true });
    var ef = yt.config_.EXPERIMENT_FLAGS || {};
    mergeDeep(ef, {
      music_web_enable_server_queues: true,
      music_web_is_canary: true,
      music_web_canary_stage: 2,
      ab_sa_ef: true,
      enable_is_extended_monitoring: true,
      music_web_enable_exponential_volume_control: true,
    });
    yt.config_.EXPERIMENT_FLAGS = ef;
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', inject);
  } else { inject(); }
})();
