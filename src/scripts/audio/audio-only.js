(function() {
  var styleId = '__ym_audio_only';
  var enabled = false;
  var origFetch = null;

  function getUrlString(url) {
    if (typeof url === 'string') return url;
    if (url && typeof url === 'object' && url.url) return url.url;
    return null;
  }

  function shouldIntercept(urlStr) {
    return urlStr &&
      urlStr.indexOf('/youtubei/v1/player') !== -1 &&
      urlStr.indexOf('music.youtube.com') !== -1;
  }

  function stripVideoFormats(data) {
    var modified = false;
    if (data.streamingData) {
      if (data.streamingData.formats) {
        var a = data.streamingData.formats.filter(function(f) {
          return f.mimeType && f.mimeType.indexOf('audio/') === 0;
        });
        if (a.length !== data.streamingData.formats.length) {
          data.streamingData.formats = a;
          modified = true;
        }
      }
      if (data.streamingData.adaptiveFormats) {
        var a = data.streamingData.adaptiveFormats.filter(function(f) {
          return f.mimeType && f.mimeType.indexOf('audio/') === 0;
        });
        if (a.length !== data.streamingData.adaptiveFormats.length) {
          data.streamingData.adaptiveFormats = a;
          modified = true;
        }
      }
    }
    return modified;
  }

  function setupFetchHook() {
    if (origFetch) return;
    origFetch = window.fetch;
    window.fetch = function(url, options) {
      var urlStr = getUrlString(url);
      if (enabled && shouldIntercept(urlStr)) {
        return origFetch.call(window, url, options).then(function(response) {
          if (!response.ok) return response;
          var clone = response.clone();
          return clone.json().then(function(data) {
            if (stripVideoFormats(data)) {
              return new Response(JSON.stringify(data), {
                status: response.status,
                statusText: response.statusText,
                headers: response.headers
              });
            }
            return response;
          }).catch(function() {
            return response;
          });
        });
      }
      return origFetch.call(window, url, options);
    };
  }

  function removeFetchHook() {
    if (origFetch) {
      window.fetch = origFetch;
      origFetch = null;
    }
  }

  var xhrOpen = XMLHttpRequest.prototype.open;
  XMLHttpRequest.prototype.open = function(method, url) {
    var urlStr = typeof url === 'string' ? url : (url ? url.toString() : null);
    this._ymIntercept = shouldIntercept(urlStr);
    return xhrOpen.apply(this, arguments);
  };

  function cleanupHandler(self, handler) {
    self.removeEventListener('readystatechange', handler);
    self.removeEventListener('abort', handler);
    self.removeEventListener('error', handler);
  }

  var xhrSend = XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.send = function(body) {
    if (enabled && this._ymIntercept) {
      var self = this;
      var handler = function() {
        if (self.readyState !== 4) {
          if (self.readyState === 0) cleanupHandler(self, handler);
          return;
        }
        cleanupHandler(self, handler);
        try {
          var data = JSON.parse(self.responseText);
          if (data && stripVideoFormats(data)) {
            Object.defineProperty(self, 'responseText', {
              configurable: true,
              get: function() { return JSON.stringify(data); }
            });
          }
        } catch(e) {}
      };
      this.addEventListener('readystatechange', handler);
      this.addEventListener('abort', handler);
      this.addEventListener('error', handler);
    }
    return xhrSend.apply(this, arguments);
  };

  function apply() {
    var el = document.getElementById(styleId);
    if (enabled) {
      if (!el) {
        el = document.createElement('style');
        el.id = styleId;
        el.textContent = 'video{display:none!important}ytmusic-video-preview-renderer{display:none!important}';
        document.head.appendChild(el);
      }
      setupFetchHook();
    } else {
      if (el) el.remove();
      removeFetchHook();
      window.location.reload();
    }
  }

  window.__ym = window.__ym || {};
  window.__ym.audioOnly = {
    toggle: function(on) {
      enabled = on !== undefined ? on : !enabled;
      apply();
      return enabled;
    },
    isEnabled: function() { return enabled; },
  };
})();