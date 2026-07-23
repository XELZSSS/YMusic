var EQ_PRESETS = [
  ['Flat',         [0,0,0,0,0,0,0,0,0,0], 0],
  ['Pop',          [-1,2,3,4,5,3,2,1,0,-1], 0],
  ['Rock',         [4,3,2,1,0,0,1,2,3,4], 0],
  ['Jazz',         [3,2,1,2,3,3,2,1,2,3], 0],
  ['Classical',    [4,3,2,1,0,0,1,2,3,4], -2],
  ['Bass Booster', [6,5,4,2,0,-1,-2,-3,-4,-5], -3],
  ['Treble Boost', [-5,-4,-3,-2,0,2,4,5,6,6], -3],
];
window.__ym_eq_presets = EQ_PRESETS;

window.__ym_eq_toggle_panel = function() {};

document.addEventListener('keydown', function(e) {
  if (e.ctrlKey && e.shiftKey && e.key === 'E') {
    e.preventDefault();
    if (window.__ym_eq_toggle) window.__ym_eq_toggle(!window.__ym_eq_enabled);
  }
});
