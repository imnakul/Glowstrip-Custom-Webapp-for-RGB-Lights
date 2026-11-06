//! Windows-only window setup: frosted glass behind the window (the same
//! technique as Venu's settings window) and microphone permission for the
//! web view, which otherwise refuses getUserMedia inside the app.

use std::ffi::c_void;

use windows::core::{s, w};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::{
    DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
    DWM_WINDOW_CORNER_PREFERENCE,
};
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};

/// `WCA_ACCENT_POLICY`.
const ACCENT_POLICY: u32 = 19;
/// `ACCENT_ENABLE_ACRYLICBLURBEHIND`.
const ACCENT_ACRYLIC: u32 = 4;

#[repr(C)]
struct AccentPolicy {
    state: u32,
    flags: u32,
    /// `0xAABBGGRR`, nearly clear: the real tint is painted by the page.
    gradient: u32,
    animation: u32,
}

#[repr(C)]
struct CompositionData {
    attribute: u32,
    data: *mut c_void,
    size: usize,
}

type SetWindowCompositionAttribute = unsafe extern "system" fn(HWND, *mut CompositionData) -> i32;

/// Blur what is behind the window, round its corners and use a dark frame.
/// Uses the accent policy rather than the documented backdrop because the
/// documented acrylic turns flat whenever the window loses focus.
/// Returns whether the blur took; if not, the page paints itself solid.
pub fn glass(hwnd: HWND) -> bool {
    unsafe {
        let dark: i32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &dark as *const _ as *const c_void,
            std::mem::size_of::<i32>() as u32,
        );
        let round: DWM_WINDOW_CORNER_PREFERENCE = DWMWCP_ROUND;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &round as *const _ as *const c_void,
            std::mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
        );

        let Ok(user32) = GetModuleHandleW(w!("user32.dll")) else {
            return false;
        };
        let Some(proc) = GetProcAddress(user32, s!("SetWindowCompositionAttribute")) else {
            return false;
        };
        let set: SetWindowCompositionAttribute = std::mem::transmute(proc);

        let mut policy = AccentPolicy {
            state: ACCENT_ACRYLIC,
            flags: 0,
            gradient: 0x10_16_10_10,
            animation: 0,
        };
        let mut data = CompositionData {
            attribute: ACCENT_POLICY,
            data: &mut policy as *mut _ as *mut c_void,
            size: std::mem::size_of::<AccentPolicy>(),
        };
        set(hwnd, &mut data) != 0
    }
}

/// Grant the microphone to the app's own page. WebView2 otherwise answers
/// getUserMedia with "blocked", and a refusal is remembered.
pub fn allow_microphone(window: &tauri::WebviewWindow) {
    let _ = window.with_webview(|wv| unsafe {
        use webview2_com::Microsoft::Web::WebView2::Win32::{
            COREWEBVIEW2_PERMISSION_KIND, COREWEBVIEW2_PERMISSION_KIND_MICROPHONE,
            COREWEBVIEW2_PERMISSION_STATE_ALLOW,
        };
        use webview2_com::PermissionRequestedEventHandler;

        let Ok(core) = wv.controller().CoreWebView2() else {
            return;
        };
        let handler = PermissionRequestedEventHandler::create(Box::new(|_sender, args| {
            if let Some(args) = args {
                let mut kind = COREWEBVIEW2_PERMISSION_KIND::default();
                args.PermissionKind(&mut kind)?;
                if kind == COREWEBVIEW2_PERMISSION_KIND_MICROPHONE {
                    args.SetState(COREWEBVIEW2_PERMISSION_STATE_ALLOW)?;
                }
            }
            Ok(())
        }));
        let mut token: i64 = 0;
        let _ = core.add_PermissionRequested(&handler, &mut token);
    });
}
