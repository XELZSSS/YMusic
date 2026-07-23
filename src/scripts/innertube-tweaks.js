const _prevFetch = window.fetch;
window.fetch = function(input, init) {
  if (init && typeof init.body === 'string') {
    const url = typeof input === 'string' ? input : (input instanceof Request ? input.url : null);
    if (url && url.indexOf('/youtubei/v1/') !== -1) {
      try {
        const body = JSON.parse(init.body);
        body.contentCheckOk = true;
        body.racyCheckOk = true;
        if (body.context && body.context.client) {
          body.context.client.clientName = 'ANDROID';
          body.context.client.clientVersion = '6.30.56';
          body.context.client.androidSdkVersion = 33;
          body.context.client.osVersion = '13';
          body.context.client.platform = 'MOBILE';
          body.context.client.userAgent = 'com.google.android.apps.youtube.music/6.30.56 (Linux; U; Android 13) gzip';
        }
        init = Object.assign({}, init, { body: JSON.stringify(body) });
      } catch(e) {}
    }
  }
  return _prevFetch.call(this, input, init);
};
