# Changelog

All dates are release dates. The version shows under the app name and in Settings → About.

- **1.10.0** (6 Nov 2026): strip-mic mode no longer loops reconnecting (drops in mic mode are left alone; three drops in 30 s pause auto-reconnect; drops logged with the command that preceded them); clear microphone errors; desktop app grants itself microphone access, has a frosted-glass window like Venu, and opens at 400×720, centred and resizable
- **1.9.1** (6 Nov 2026): studio effects tint the app with their own colour; desktop installer is published automatically when the version changes; README, protocol notes and contribution guide
- **1.9.0** (6 Nov 2026): keep-alive read every 20 s of silence (fixes idle drops on PC browsers); drops logged with idle time; Windows desktop app with native Bluetooth
- **1.8.1** (6 Nov 2026): strip-mic mode is a proper mode (stops app streams, keeps Bluetooth alive in background, restored after reconnect); Music tab lists it first
- **1.8.0** (6 Nov 2026): auto-connect to the last strip on launch and when returning from the background (uses getDevices() and watchAdvertisements() where Chrome supports them); toggle in Settings
- **1.7.1** (6 Nov 2026): keep-alive track lengthened past Chrome's 5 s media-notification threshold; music starts keep-alive inside the tap; background check log in Settings
- **1.7.0** (6 Nov 2026): background running for studio effects, combos, shuffle and music (worker timers, media notification with stop, optional keep-screen-on); scenes, effects and combos show a spinner while switching and only one item is ever active
- **1.6.0** (6 Nov 2026): connection sheet (tap the pill) with Disconnect, Reconnect to last strip without the picker, and Connect a different strip; quiet auto-retry when the strip drops
- **1.5.0** (6 Nov 2026): effects and scenes show a spinner until their first frame reaches the strip, then turn active; rapid taps only activate the last one; skip duplicate stream frames and pace writes to the controller's speed; Lab shows average write time
- **1.4.2** (6 Nov 2026): fix content jumping on tab switch; each tab remembers its scroll position
- **1.4.1** (6 Nov 2026): fix bottom bar jumping on tab switch (horizontal overflow made Chrome rescale the page); splash now continues from Android's launch icon; non-blocking fonts; service worker falls back to cache after 1.2 s on slow networks
- **1.4.0** (6 Nov 2026): animated splash with the wave logo; power button off until connected (tap to connect); sliding tab indicator, panel transitions, press feedback, colour fades, light haptics
- **1.3.0** (6 Nov 2026): installable PWA (manifest, icons, offline service worker, Install button)
- **1.2.0** (6 Nov 2026): version display; 42 studio effects, colour combo builder, shuffle; Segments marked preview-only
- **1.1.0** (6 Nov 2026): Freeze button removed (the controller treats speed 0 as slowest, not stopped)
- **1.0.0** (5 Nov 2026): first version: colour, 213 built-in effects, scenes, studio effects, segment designer, phone-mic music, Lab
