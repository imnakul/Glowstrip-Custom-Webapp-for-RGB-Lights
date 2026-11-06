# Glowstrip

A phone web app for controlling MELK-OA10 (Lotus Lamp X family) Bluetooth RGB strips from Chrome on Android.

## Use it
1. Open the deployed site in **Chrome on Android** (Web Bluetooth needs https and Chrome).
2. Force-stop the Lotus Lamp X app — the strip accepts one connection at a time.
3. Tap **Connect strip** and pick `MELK-OA10`.
4. Settings (slider icon) → **Install app**, or Chrome menu → **Install app**. It then opens full screen from your app drawer with its own icon.

## What it does
- Solid colours, hue/saturation, saved colours
- All 213 built-in effects + 29 scenes (run on the strip itself)
- 42 studio effects, a colour-combo builder and shuffle (streamed from the page; whole strip, page must stay open)
- Phone-mic music sync, built-in mic control, auto-off timer
- Lab: send raw bytes and read replies

## Limits
- This controller has no per-pixel command, so the Segments tab is a designer/preview only.
- iPhone browsers don't support Web Bluetooth.

## Protocol
Service `FFF0`, write `FFF3`, notify `FFF4`, 9-byte frames `7E len cmd … EF`.
Command IDs from the community reverse engineering at https://github.com/l0lsec/LotusLampX.

## Versions
The current version shows under the app name and in Settings → About. With every change, bump `VERSION`, `RELEASED` and `NOTES` at the top of the script in `index.html`, and `CACHE` in `sw.js`.

- **1.6.0** (6 Nov 2026): connection sheet (tap the pill) with Disconnect, Reconnect to last strip without the picker, and Connect a different strip; quiet auto-retry when the strip drops
- **1.5.0** (6 Nov 2026): effects and scenes show a spinner until their first frame reaches the strip, then turn active; rapid taps only activate the last one; skip duplicate stream frames and pace writes to the controller's speed; Lab shows average write time
- **1.4.2** (6 Nov 2026): fix content jumping on tab switch; each tab remembers its scroll position
- **1.4.1** (6 Nov 2026): fix bottom bar jumping on tab switch (horizontal overflow made Chrome rescale the page); splash now continues from Android's launch icon; non-blocking fonts; service worker falls back to cache after 1.2 s on slow networks
- **1.4.0** (6 Nov 2026): animated splash with the wave logo; power button off until connected (tap to connect); sliding tab indicator, panel transitions, press feedback, colour fades, light haptics
- **1.3.0** (6 Nov 2026): installable PWA (manifest, icons, offline service worker, Install button)
- **1.2.0** (6 Nov 2026): version display; 42 studio effects, colour combo builder, shuffle; Segments marked preview-only

## Files
- `index.html`: the whole app
- `manifest.webmanifest`, `icons/`: install metadata and app icons
- `sw.js`: service worker (network first, offline fallback)
- `vercel.json`: keeps `sw.js` uncached so updates arrive straight away
