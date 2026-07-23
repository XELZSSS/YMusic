(function() {
  "use strict";
  var wasAd = false, userMuted = false, lastVideo = null;

  function handle() {
    if (document.hidden) return;
    var video = document.querySelector('video');
    if (video !== lastVideo) {
      if (wasAd && lastVideo && !userMuted) lastVideo.muted = false;
      wasAd = false; userMuted = false; lastVideo = video;
    }
    if (!video) return;
    var isAd = document.querySelector('ytmusic-player-bar[ad-active], .ad-showing');
    if (isAd) {
      if (!wasAd) { userMuted = video.muted; video.muted = true; }
      var skip = document.querySelector('ytmusic-skip-ad-button');
      if (skip) skip.click();
      wasAd = true;
    } else {
      if (wasAd && video && !userMuted) video.muted = false;
      wasAd = false;
    }
  }

  var adInterval = setInterval(handle, 1000);
  window.__ym_cleanup = window.__ym_cleanup || [];
  window.__ym_cleanup.push(function() { clearInterval(adInterval); });
})();
