# Strip protocol (MELK-OA10 / Lotus Lamp X "SYMPHONY")

What Glowstrip sends, and what is known about it. Corrections from captures are very welcome.

## Bluetooth

| | UUID | Use |
|---|---|---|
| Service | `0000fff0-0000-1000-8000-00805f9b34fb` | Control service |
| Characteristic | `…fff3…` | Write without response (commands); also readable |
| Characteristic | `…fff4…` | Notify (the MELK-OA10 rarely sends anything) |

Advertised names seen so far: `MELK-OA10 57`, `MELK-OA10 22`. The controller accepts one connection at a time, so the vendor app must be closed.

## Frame format

Every command is 9 bytes:

```
7E  len  cmd  p0  p1  p2  p3  p4  EF
```

Unused parameter bytes are `FF`, except `p4`, which defaults to `00`. `len` is usually `07`.

## Commands

| cmd | Meaning | Parameters as Glowstrip sends them | Example |
|---|---|---|---|
| `05` | Colour | `03 R G B 10` | `7E 07 05 03 FF 00 00 10 EF` (red) — confirmed on MELK-OA10 |
| `04` | Power | on `FF 01 01 02 01`, off `00 01 00 02 01` | `7E 07 04 FF 01 01 02 01 EF` |
| `01` | Brightness | `p0` = 0–100 | `7E 07 01 50 FF FF FF 00 EF` |
| `02` | Speed | `p0` = 0–100 (0 is slowest, not stopped) | `7E 04 02 3C FF FF FF 00 EF` |
| `03` | Built-in effect | `p0` = effect 0–212 | `7E 07 03 6B FF FF FF 00 EF` |
| `31` | Scene | `p0` = scene id | `7E 07 31 05 FF FF FF 00 EF` |
| `07` | Strip microphone on/off | `p0` = 1 / 0 | `7E 07 07 01 FF FF FF 00 EF` |
| `06` | Microphone sensitivity | `p0` = 0–255 | `7E 07 06 7F FF FF FF 00 EF` |
| `76` | Turn off after | `p0 p1` = seconds, little-endian | `7E 07 76 08 07 FF FF 00 EF` (30 min) |
| `87` | Factory reset | — | **Glowstrip never sends this** |

Notes from the decompiled Lotus Lamp X app (via [l0lsec/LotusLampX](https://github.com/l0lsec/LotusLampX)) that differ slightly from what Glowstrip sends:

- Brightness uses `len = 04` and a 0–255 value; speed uses `len = 07` and 0–255.
- Effects send a second parameter, `03`, after the effect id.

The MELK-OA10 responds to Glowstrip's variants. If yours doesn't, try the app's variants in Settings → Lab and open an issue.

## What the controller can't do

There is no command for setting individual pixels. The pixel-count setting in the vendor app only tells the controller how long the strip is. That is why fixed multi-colour segments aren't possible on this controller.

## Keep-alive

Some controllers, and Windows, close a link after about a minute without traffic. After 20 seconds of silence Glowstrip reads `FFF3`, which has no visible effect.
