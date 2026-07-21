(function() {
    var CHECK_MS = 5 * 60 * 1000;
    var HEAP_HIGH = 300 * 1024 * 1024;
    var warnCount = 0;

    function gc() {
        try {
            if (window.gc) window.gc();
        } catch (_) {}
    }

    function check() {
        try {
            if (!performance || !performance.memory) return;
            var used = performance.memory.usedJSHeapSize;
            if (used > HEAP_HIGH) {
                warnCount++;
                if (warnCount >= 3) { gc(); warnCount = 0; }
            } else {
                warnCount = 0;
            }
        } catch (_) {}
    }

    document.addEventListener('visibilitychange', function() {
        if (document.hidden) gc();
    });

    document.addEventListener('yt-page-data-updated', function() {
        setTimeout(gc, 1000);
    });

    var video = document.querySelector('video');
    if (video) {
        video.addEventListener('play', function() {
            setTimeout(gc, 5000);
        });
        video.addEventListener('ended', function() {
            setTimeout(gc, 1000);
        });
    }

    setInterval(check, CHECK_MS);
})();
