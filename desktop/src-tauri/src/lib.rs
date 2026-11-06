//! Native Bluetooth for the Glowstrip desktop app.
//!
//! The web UI calls these commands through a small shim that mimics the Web
//! Bluetooth API, so the same index.html runs in the browser and here.

use std::sync::Arc;
use std::time::{Duration, Instant};

use btleplug::api::{Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::StreamExt;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager as _, State};
use tokio::sync::Mutex;
use uuid::Uuid;

#[cfg(windows)]
mod win;

/// Whether the frosted-glass window effect took (Windows only).
struct Glass(bool);

const SERVICE: Uuid = Uuid::from_u128(0x0000fff0_0000_1000_8000_00805f9b34fb);
const WRITE: Uuid = Uuid::from_u128(0x0000fff3_0000_1000_8000_00805f9b34fb);
const NOTIFY: Uuid = Uuid::from_u128(0x0000fff4_0000_1000_8000_00805f9b34fb);

struct Conn {
    p: Peripheral,
    w: Characteristic,
}

#[derive(Default)]
struct Ble {
    adapter: Option<Adapter>,
    conn: Option<Conn>,
    scanning: bool,
}

type St = Arc<Mutex<Ble>>;

#[derive(Clone, Serialize)]
struct Found {
    id: String,
    name: String,
    rssi: Option<i16>,
}

fn err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

fn key(p: &Peripheral) -> String {
    format!("{:?}", p.id())
}

/// First Bluetooth adapter, created once. Also starts a watcher that tells the
/// UI when the connected strip drops.
async fn adapter(st: &St, app: &AppHandle) -> Result<Adapter, String> {
    let mut g = st.lock().await;
    if let Some(a) = &g.adapter {
        return Ok(a.clone());
    }
    let m = Manager::new().await.map_err(err)?;
    let a = m
        .adapters()
        .await
        .map_err(err)?
        .into_iter()
        .next()
        .ok_or("No Bluetooth adapter found. Turn Bluetooth on in Windows settings.")?;

    let mut events = a.events().await.map_err(err)?;
    let (app2, st2) = (app.clone(), st.clone());
    tauri::async_runtime::spawn(async move {
        while let Some(ev) = events.next().await {
            if let CentralEvent::DeviceDisconnected(id) = ev {
                let mut g = st2.lock().await;
                if g.conn.as_ref().map(|c| c.p.id() == id).unwrap_or(false) {
                    g.conn = None;
                    drop(g);
                    let _ = app2.emit("ble-disconnected", ());
                }
            }
        }
    });

    g.adapter = Some(a.clone());
    Ok(a)
}

async fn is_strip(p: &Peripheral) -> Option<Found> {
    let pr = p.properties().await.ok()??;
    let name = pr.local_name.clone().unwrap_or_default();
    let ok = name.starts_with("MELK") || name.starts_with("ELK") || pr.services.contains(&SERVICE);
    ok.then(|| Found {
        id: key(p),
        name: if name.is_empty() { "LED strip".into() } else { name },
        rssi: pr.rssi,
    })
}

/// Scan for strips and emit `ble-found` for each one (repeatedly, so signal
/// strength stays current). Stops after `seconds` or on `ble_stop_scan`.
#[tauri::command]
async fn ble_scan(app: AppHandle, st: State<'_, St>, seconds: u64) -> Result<(), String> {
    let st = st.inner().clone();
    let a = adapter(&st, &app).await?;
    a.start_scan(ScanFilter::default()).await.map_err(err)?;
    st.lock().await.scanning = true;
    let end = Instant::now() + Duration::from_secs(seconds.clamp(2, 60));
    while Instant::now() < end && st.lock().await.scanning {
        for p in a.peripherals().await.map_err(err)? {
            if let Some(f) = is_strip(&p).await {
                let _ = app.emit("ble-found", f);
            }
        }
        tokio::time::sleep(Duration::from_millis(600)).await;
    }
    st.lock().await.scanning = false;
    let _ = a.stop_scan().await;
    Ok(())
}

#[tauri::command]
async fn ble_stop_scan(st: State<'_, St>) -> Result<(), String> {
    st.lock().await.scanning = false;
    Ok(())
}

async fn find(a: &Adapter, id: &str) -> Result<Option<Peripheral>, String> {
    Ok(a.peripherals().await.map_err(err)?.into_iter().find(|p| key(p) == id))
}

/// Connect to a strip by id. If Windows hasn't seen it yet this session,
/// scan for it first (up to 8 s).
#[tauri::command]
async fn ble_connect(app: AppHandle, st: State<'_, St>, id: String) -> Result<String, String> {
    let st = st.inner().clone();
    let a = adapter(&st, &app).await?;

    if let Some(old) = st.lock().await.conn.take() {
        let _ = old.p.disconnect().await;
    }

    let mut p = find(&a, &id).await?;
    if p.is_none() {
        a.start_scan(ScanFilter::default()).await.map_err(err)?;
        let end = Instant::now() + Duration::from_secs(8);
        while p.is_none() && Instant::now() < end {
            tokio::time::sleep(Duration::from_millis(400)).await;
            p = find(&a, &id).await?;
        }
        let _ = a.stop_scan().await;
    }
    let p = p.ok_or("Strip not found. Is it switched on and in range?")?;

    tokio::time::timeout(Duration::from_secs(15), p.connect())
        .await
        .map_err(|_| "The strip didn't answer. Close the Lotus app on your phone and try again.".to_string())?
        .map_err(err)?;
    p.discover_services().await.map_err(err)?;

    let chars = p.characteristics();
    let w = chars
        .iter()
        .find(|c| c.uuid == WRITE)
        .cloned()
        .ok_or("This device doesn't have the strip control channel (FFF3).")?;

    if let Some(n) = chars.iter().find(|c| c.uuid == NOTIFY) {
        if p.subscribe(n).await.is_ok() {
            if let Ok(mut stream) = p.notifications().await {
                let app2 = app.clone();
                tauri::async_runtime::spawn(async move {
                    while let Some(v) = stream.next().await {
                        let _ = app2.emit("ble-notify", v.value);
                    }
                });
            }
        }
    }

    let name = p
        .properties()
        .await
        .ok()
        .flatten()
        .and_then(|x| x.local_name)
        .unwrap_or_default();
    st.lock().await.conn = Some(Conn { p, w });
    Ok(name)
}

#[tauri::command]
async fn ble_write(st: State<'_, St>, data: Vec<u8>) -> Result<(), String> {
    let (p, w) = {
        let g = st.lock().await;
        let c = g.conn.as_ref().ok_or("Not connected")?;
        (c.p.clone(), c.w.clone())
    };
    p.write(&w, &data, WriteType::WithoutResponse).await.map_err(err)
}

#[tauri::command]
async fn ble_read(st: State<'_, St>) -> Result<Vec<u8>, String> {
    let (p, w) = {
        let g = st.lock().await;
        let c = g.conn.as_ref().ok_or("Not connected")?;
        (c.p.clone(), c.w.clone())
    };
    p.read(&w).await.map_err(err)
}

#[tauri::command]
fn glass_enabled(g: State<'_, Glass>) -> bool {
    g.0
}

#[tauri::command]
async fn ble_disconnect(app: AppHandle, st: State<'_, St>) -> Result<(), String> {
    let old = st.lock().await.conn.take();
    if let Some(c) = old {
        let _ = c.p.disconnect().await;
    }
    let _ = app.emit("ble-disconnected", ());
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .manage(St::default())
        .setup(|app| {
            #[allow(unused_mut)]
            let mut glass = false;
            if let Some(w) = app.get_webview_window("main") {
                #[cfg(windows)]
                {
                    if let Ok(h) = w.hwnd() {
                        glass = win::glass(windows::Win32::Foundation::HWND(h.0));
                    }
                    win::allow_microphone(&w);
                }
                let _ = &w;
            }
            app.manage(Glass(glass));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            glass_enabled,
            ble_scan,
            ble_stop_scan,
            ble_connect,
            ble_write,
            ble_read,
            ble_disconnect
        ])
        .run(tauri::generate_context!())
        .expect("failed to start Glowstrip");
}
