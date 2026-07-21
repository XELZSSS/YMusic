function pruner(o) {
  if (o && typeof o === 'object') {
    delete o.playerAds;
    delete o.adPlacements;
    delete o.adSlots;
    if (o.playerResponse) {
      delete o.playerResponse.playerAds;
      delete o.playerResponse.adPlacements;
      delete o.playerResponse.adSlots;
    }
    if (o.ytInitialPlayerResponse) {
      delete o.ytInitialPlayerResponse.playerAds;
      delete o.ytInitialPlayerResponse.adPlacements;
      delete o.ytInitialPlayerResponse.adSlots;
    }
  }
  return o;
}

var _prunerFn = pruner;

JSON.parse = new Proxy(JSON.parse, {
  apply: function(target, thisArg, args) { return _prunerFn(Reflect.apply(target, thisArg, args)); },
});

Response.prototype.json = new Proxy(Response.prototype.json, {
  apply: function(target, thisArg, args) {
    return Reflect.apply(target, thisArg, args).then(function(o) { return _prunerFn(o); });
  },
});

var trapProp = function(owner, prop, configurable, handler) {
  if (!handler.init(owner[prop])) return;
  var odesc = Object.getOwnPropertyDescriptor(owner, prop);
  var pg, ps;
  if (odesc instanceof Object) {
    if (odesc.configurable === false) return;
    if (odesc.get instanceof Function) pg = odesc.get;
    if (odesc.set instanceof Function) ps = odesc.set;
  }
  Object.defineProperty(owner, prop, {
    configurable: configurable,
    get: function() {
      if (pg) pg();
      return handler.getter();
    },
    set: function(a) {
      if (ps) ps(a);
      handler.setter(a);
    },
  });
};

var trapChain = function(owner, chain) {
  var pos = chain.indexOf('.');
  if (pos === -1) {
    trapProp(owner, chain, false, {
      v: undefined,
      getter: function() { return undefined; },
      setter: function(a) {},
      init: function(v) { this.v = v; return true; },
    });
    return;
  }
  var prop = chain.slice(0, pos);
  var v = owner[prop];
  var remaining = chain.slice(pos + 1);
  if (v instanceof Object || (typeof v === 'object' && v !== null)) {
    trapChain(v, remaining);
    return;
  }
  trapProp(owner, prop, true, {
    v: undefined,
    getter: function() { return this.v; },
    setter: function(a) { this.v = a; if (a instanceof Object) trapChain(a, remaining); },
    init: function(v) { this.v = v; return true; },
  });
};

[
  'playerResponse.adPlacements',
  'playerResponse.playerAds',
  'playerResponse.adSlots',
  'ytInitialPlayerResponse.playerAds',
  'ytInitialPlayerResponse.adPlacements',
  'ytInitialPlayerResponse.adSlots',
].forEach(function(chain) { trapChain(window, chain); });
