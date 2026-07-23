window.__ym = window.__ym || {};

(function(ym) {
  document.addEventListener('keydown', function(e) {
    if (e.ctrlKey && e.shiftKey && e.key === 'E') {
      e.preventDefault();
      if (ym.eq) {
        var newState = !ym.eq.isEnabled();
        ym.eq.toggle(newState);
        try {
          window.__TAURI__.invoke('save_eq_state', {
            enabled: newState,
            presetIndex: null,
            bands: ym.eq.getBands(),
            preamp: ym.eq.getPreamp(),
          });
        } catch(ex) {}
      }
    }
  });
})(window.__ym);
