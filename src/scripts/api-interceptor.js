function prune(o) {
  if (o && typeof o === 'object' && !Array.isArray(o)) {
    delete o.playerAds;
    delete o.adPlacements;
    delete o.adSlots;
    if (o.playerResponse && typeof o.playerResponse === 'object' && !Array.isArray(o.playerResponse)) {
      delete o.playerResponse.playerAds;
      delete o.playerResponse.adPlacements;
      delete o.playerResponse.adSlots;
    }
  }
  return o;
}
JSON.parse = new Proxy(JSON.parse, {
  apply: function(t, _, a) { return prune(Reflect.apply(t, _, a)); }
});
Response.prototype.json = new Proxy(Response.prototype.json, {
  apply: function(t, s, a) {
    return Reflect.apply(t, s, a).then(function(v) { return prune(v); });
  }
});
