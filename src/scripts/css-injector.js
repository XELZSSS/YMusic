(function() {
  function inject() {
    if (!window.__YM_CSS || document.getElementById('__ym-css') || !document.head) return;
    var s = document.createElement('style');
    s.id = '__ym-css';
    s.textContent = window.__YM_CSS;
    document.head.appendChild(s);
  }
  if (document.readyState === 'loading')
    document.addEventListener('DOMContentLoaded', inject);
  else inject();
})();
