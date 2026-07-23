let wasAd = false, userMuted = false, lastVideo = null;

function handleAudioAd() {
  if (document.hidden) return;

  const video = document.querySelector('video');
  if (video !== lastVideo) {
    if (wasAd && lastVideo && !userMuted) lastVideo.muted = false;
    wasAd = false;
    userMuted = false;
    lastVideo = video;
  }

  const isAd = document.querySelector('ytmusic-player-bar[ad-active], .ad-showing');
  if (isAd) {
    if (!wasAd && video) { userMuted = video.muted; video.muted = true; }
    const skip = document.querySelector('ytmusic-skip-ad-button');
    if (skip) skip.click();
    wasAd = true;
  } else {
    if (wasAd && video && !userMuted) video.muted = false;
    wasAd = false;
  }
}

setInterval(handleAudioAd, 1000);
