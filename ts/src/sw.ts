const CACHE_NAME = "pixcil-";
const CACHE_VERSION = "__UUID__";
const CACHE_KEY = CACHE_NAME + CACHE_VERSION;

// @ts-ignore
self.addEventListener("install", (e: InstalLEvent) => {
  console.log("[Service Worker] Install");
  // @ts-ignore
  e.waitUntil(self.skipWaiting());
});

self.addEventListener("fetch", (e) => {
  // @ts-ignore
  if (!e.request.url.startsWith("https://")|| e.request.cache === "no-store") {
    // @ts-ignore
    e.respondWith(fetch(e.request));
    return;
  }

  // @ts-ignore
  e.respondWith(
    // @ts-ignore
    caches.match(e.request).then((r) => {
      if (r) {
        return Promise.resolve(r);
      }

      // @ts-ignore
      console.log("[Service Worker] Fetching resource: " + e.request.url);
      // @ts-ignore
      return fetch(e.request).then((response) => {
        return caches.open(CACHE_KEY).then((cache) => {
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
          if (key.startsWith(CACHE_NAME) && key !== CACHE_KEY) {
            console.log("[Service Worker] Delete old cache: " + key);
            return caches.delete(key);
          }
        })
      );
    })
  );
});
