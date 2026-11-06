# Contributing to Glowstrip

Thanks for helping. The project is small on purpose: one `index.html` for the app, and a thin Tauri wrapper in `desktop/`.

## Report a strip that doesn't work

Open a [controller report](https://github.com/imnakul/Glowstrip-Custom-Webapp-for-RGB-Lights/issues/new?template=controller.yml). Include:

- the Bluetooth name (from nRF Connect)
- the service UUIDs the strip shows
- the app named on the box or QR code
- what happened when you tried Glowstrip

### Capture the vendor app's commands (most useful contribution)

This shows exactly what bytes the vendor app sends, so support can be added without guesswork.

1. On Android, open Settings → About phone and tap **Build number** seven times to enable Developer options.
2. In Developer options, turn on **Bluetooth HCI snoop log**, then turn Bluetooth off and on.
3. Open the vendor app and do a few things slowly, about three seconds apart: power off, power on, red, green, blue, one effect, brightness low then high. Write the order down.
4. In Developer options, tap **Take bug report**. When it's ready, share the zip.
5. Inside the zip, find `FS/data/misc/bluetooth/logs/btsnoop_hci.log`. Attach only that file to the issue, with your list of actions. The full bug report contains personal data; don't attach it.

## Work on the code

```sh
python3 -m http.server 8000   # open http://localhost:8000 in Chrome
```

- No build step. Keep the app in one file unless there's a clear reason not to.
- Test at phone width (about 390 px) and check that nothing scrolls sideways.
- Respect `prefers-reduced-motion` for any new animation.
- Commands go through `link.send(key, bytes)`; the newest frame per key wins, so streams don't pile up.

### Desktop app

```sh
cd desktop
npm install
npx tauri dev
```

Bluetooth commands live in `desktop/src-tauri/src/lib.rs`. The shim at the top of the script in `index.html` maps Web Bluetooth calls onto them.

## Versions and releases

Run `node scripts/bump.mjs X.Y.Z` to set the version in the web app, service worker and desktop app together, then update `RELEASED` and `NOTES` in `index.html` and add a line to `CHANGELOG.md`.

Pushing a new version to `main` deploys the web app (Vercel) and builds and publishes the Windows installer (GitHub Actions). A version that already has a release is skipped.

## Pull requests

Keep them focused, describe what you tested and on which strip, and add a screenshot for visible changes.
