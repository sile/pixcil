const CACHE_NAME = "pixcil-__UUID__";

// @ts-ignore
self.addEventListener("install", (e: InstalLEvent) => {
  console.log("[Service Worker] Install");
  // @ts-ignore
  e.waitUntil(self.skipWaiting());
});

self.addEventListener("fetch", (e) => {
  // @ts-ignore
  if (!e.request.url.startsWith("https://")) {
    // @ts-ignore
    e.respondWith(fetch(e.request));
    return;
  }

  // @ts-ignore
  e.respondWith(
    // @ts-ignore
    caches.match(e.request).then((r) => {
      // @ts-ignore
      console.log("[Service Worker] Fetching resource: " + e.request.url);
      // @ts-ignore
      const promise = r || fetch(e.request);
      // @ts-ignore
      return promise.then((response) => {
        return caches.open(CACHE_NAME).then((cache) => {
          // @ts-ignore
          console.log("[Service Worker] Caching new resource: " + e.request.url);
          // @ts-ignore
          cache.put(e.request, response.clone());
          return response;
        });
      });
    })
  );
});

self.addEventListener("activate", (e) => {
  console.log("[Service Worker] Activate");
  // @ts-ignore
  e.waitUntil(
    caches.keys().then((keyList) => {
      return Promise.all(
        keyList.map((key) => {
          if (key !== CACHE_NAME) {
            console.log("[Service Worker] Delete old cache: " + key);
            return caches.delete(key);
          }
        })
      );
    })
  );
});
