const TRACK = [
  'fbclid','gclid','gclsrc','dclid','gbraid','wbraid',
  'msclkid','twclid','igshid','yclid','li_fat_id',
  'mc_cid','mc_eid','_hsenc','_hsmi','mkt_tok',
  'utm_source','utm_medium','utm_campaign','utm_term',
  'utm_content','utm_id','utm_reader',
  'utm_referrer','utm_name','utm_social','utm_social-type',
];

function clean(u) {
  if (typeof u !== 'string') return u;
  if (u.startsWith('data:') || u.startsWith('blob:')) return u;
  try {
    const url = new URL(u, location.origin);
    let changed = false;
    for (let i = 0; i < TRACK.length; i++) {
      if (url.searchParams.has(TRACK[i])) { url.searchParams.delete(TRACK[i]); changed = true; }
    }
    return changed ? url.toString() : u;
  } catch(e) { return u; }
}

const origFetch = window.fetch;
window.fetch = function(input, init) {
  if (typeof input === 'string') {
    const c = clean(input);
    if (c !== input) input = c;
  } else if (input instanceof Request && input.url) {
    const c = clean(input.url);
    if (c !== input.url) input = new Request(c, input);
  }
  return origFetch.call(this, input, init);
};

const ps = history.pushState, rs = history.replaceState;
history.pushState = function() {
  ps.apply(this, arguments);
  const c = clean(location.href);
  if (c !== location.href) rs.call(history, null, '', c);
};
history.replaceState = function() {
  rs.apply(this, arguments);
  const c = clean(location.href);
  if (c !== location.href) rs.call(history, null, '', c);
};
const cur = clean(location.href);
if (cur !== location.href) rs.call(history, null, '', cur);
