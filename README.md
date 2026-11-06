# Glowstrip

A phone web app for controlling MELK-OA10 (Lotus Lamp X family) Bluetooth RGB strips from Chrome on Android.

## Use it
1. Open the deployed site in **Chrome on Android** (Web Bluetooth needs https and Chrome).
2. Force-stop the Lotus Lamp X app — the strip accepts one connection at a time.
3. Tap **Connect strip** and pick `MELK-OA10`.
4. Chrome menu → **Add to Home screen** to use it like an app.

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
