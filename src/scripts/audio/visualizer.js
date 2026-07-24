(function() {
  var canvas = null;
  var ctx2d = null;
  var animId = null;
  var active = false;
  var style = 'bars';
  var barColor = '#ff0000';
  var sensitivity = 1.0;

  function getAnalyser() {
    var a = window.__ym && window.__ym.audio && window.__ym.audio.getAnalyser();
    return a || null;
  }

  function render() {
    if (!active) return;
    var a = getAnalyser();
    if (!a || !canvas || !ctx2d) { stop(); return; }
    var buf = new Uint8Array(a.frequencyBinCount);
    a.getByteFrequencyData(buf);
    var w = canvas.width, h = canvas.height;
    ctx2d.clearRect(0, 0, w, h);
    if (style === 'bars') {
      var bars = 64;
      var bw = w / bars;
      for (var i = 0; i < bars; i++) {
        var val = buf[Math.floor(i * buf.length / bars)] / 255 * h * sensitivity;
        ctx2d.fillStyle = barColor;
        ctx2d.fillRect(i * bw, h - val, bw - 1, val);
      }
    }
    animId = requestAnimationFrame(render);
  }

  function start(c, s) {
    if (active) return;
    canvas = c;
    style = s || 'bars';
    ctx2d = canvas.getContext('2d');
    active = true;
    render();
  }

  function stop() {
    active = false;
    if (animId) { cancelAnimationFrame(animId); animId = null; }
  }

  window.__ym = window.__ym || {};
  window.__ym.viz = { start: start, stop: stop };
})();
