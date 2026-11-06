// Glowstrip service worker: network first so new deploys show up immediately,
// cached copy when offline.
const CACHE = "glowstrip-v1.10.0";
const CORE = ["/", "/index.html", "/manifest.webmanifest", "/icons/icon-192.png", "/icons/icon-512.png"];

self.addEventListener("install", e => {
  e.waitUntil(caches.open(CACHE).then(c => c.addAll(CORE)).then(() => self.skipWaiting()));
});

self.addEventListener("activate", e => {
  e.waitUntil(
    caches.keys().then(keys => Promise.all(keys.filter(k => k !== CACHE).map(k => caches.delete(k))))
      .then(() => self.clients.claim())
  );
});

self.addEventListener("fetch", e => {
  const req = e.request;
  if (req.method !== "GET") return;
  const url = new URL(req.url);
  const isFont = url.hostname.endsWith("fonts.googleapis.com") || url.hostname.endsWith("fonts.gstatic.com");
  if (url.origin !== location.origin && !isFont) return;

  if (isFont) {
    // fonts rarely change: cache first
    e.respondWith(caches.match(req).then(hit => hit || fetch(req).then(res => {
      const copy = res.clone(); caches.open(CACHE).then(c => c.put(req, copy)); return res;
    })));
    return;
  }
  // app files: network first, but if the network is slow (>1.2 s) use the cached
  // copy so the launch screen doesn't hang; the fresh copy still lands in the cache.
  const net = fetch(req).then(res => {
    if (res.ok) { const copy = res.clone(); caches.open(CACHE).then(c => c.put(req, copy)); }
    return res;
  });
  const cached = () => caches.match(req).then(hit => hit || (req.mode === "navigate" ? caches.match("/index.html") : undefined));
  const slow = new Promise(r => setTimeout(r, 1200)).then(cached);
  e.respondWith(
    Promise.race([net, slow.then(hit => hit || net)]).catch(() => cached())
  );
  e.waitUntil(net.catch(() => {}));
});
