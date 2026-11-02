//! The opencad-ui gallery in its own window:
//!
//! ```sh
//! cargo run -p opencad-ui --example gallery
//! # screenshots of every page in both languages, then quit:
//! OPENCAD_UI_SHOT_DIR=/tmp/shots cargo run -p opencad-ui --example gallery
//! ```
//!
//! `OPENCAD_UI_PAGE=<key>` and `OPENCAD_UI_LANG=en|de` limit the
//! screenshots; `OPENCAD_UI_SIZE=<w>x<h>` sets the window size.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eframe::egui;
use opencad_ui::gallery::{Gallery, Page};
use opencad_ui::i18n::Lang;

struct Shots {
    dir: PathBuf,
    queue: Vec<(Page, Lang)>,
    frame: u32,
    /// The part of a long page (scrolled by a viewport each).
    part: u32,
    pending: Arc<Mutex<Option<bool>>>,
}

struct App {
    gallery: Gallery,
    shots: Option<Shots>,
    /// The requested window size; applied in the first frames too,
    /// because macOS may restore the previous window frame.
    size: egui::Vec2,
    frames: u32,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        if self.frames < 3 {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(self.size));
        }
        self.frames += 1;
        let c = opencad_ui::colors(ui);
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(c.bg).inner_margin(16))
            .show(ui, |ui| self.gallery.ui(ui));
        let Some(shots) = &mut self.shots else {
            return;
        };
        ctx.request_repaint();
        let Some(&(page, lang)) = shots.queue.first() else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        };
        self.gallery.page = page;
        self.gallery.lang = Some(lang);
        if shots.frame < 20 {
            let step = (self.gallery.view_height - 60.0).max(100.0);
            self.gallery.scroll_to = Some(shots.part as f32 * step);
        }
        shots.frame += 1;
        let done = shots.pending.lock().map(|p| *p).unwrap_or(None);
        match done {
            Some(true) => {
                if let Ok(mut p) = shots.pending.lock() {
                    *p = None;
                }
                shots.frame = 0;
                let step = (self.gallery.view_height - 60.0).max(100.0);
                let next = (shots.part + 1) as f32 * step;
                if next < self.gallery.content_height - 60.0 {
                    shots.part += 1;
                } else {
                    shots.queue.remove(0);
                    shots.part = 0;
                }
            }
            Some(false) => {}
            None if shots.frame == 25 => {
                if let Ok(mut p) = shots.pending.lock() {
                    *p = Some(false);
                }
                let path = shots.dir.join(format!(
                    "gallery-{}-{}-{}.png",
                    page.key(),
                    lang.key(),
                    shots.part + 1
                ));
                let pending = shots.pending.clone();
                if self.gallery.overflow > 0.5 {
                    eprintln!(
                        "overflow {:.0} pt on page {} ({})",
                        self.gallery.overflow,
                        page.key(),
                        lang.key()
                    );
                }
                ctx.request_screenshot(move |img| {
                    std::fs::write(&path, png(&img)).expect("write png");
                    eprintln!("screenshot {}", path.display());
                    if let Ok(mut p) = pending.lock() {
                        *p = Some(true);
                    }
                });
            }
            None => {}
        }
    }
}

/// A PNG writer (stored deflate blocks; no dependency).
fn png(img: &egui::ColorImage) -> Vec<u8> {
    let [w, h] = img.size;
    let mut raw = Vec::with_capacity((w * 4 + 1) * h);
    for y in 0..h {
        raw.push(0);
        for p in &img.pixels[y * w..(y + 1) * w] {
            raw.extend_from_slice(&p.to_srgba_unmultiplied());
        }
    }
    let mut z = vec![0x78, 0x01];
    let chunks: Vec<&[u8]> = raw.chunks(65535).collect();
    for (i, chunk) in chunks.iter().enumerate() {
        z.push(u8::from(i + 1 == chunks.len()));
        let n = chunk.len() as u16;
        z.extend_from_slice(&n.to_le_bytes());
        z.extend_from_slice(&(!n).to_le_bytes());
        z.extend_from_slice(chunk);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &x in &raw {
        a = (a + u32::from(x)) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let crc = |data: &[u8]| {
        let mut c = 0xFFFF_FFFFu32;
        for &x in data {
            c ^= u32::from(x);
            for _ in 0..8 {
                c = if c & 1 != 0 {
                    0xEDB8_8320 ^ (c >> 1)
                } else {
                    c >> 1
                };
            }
        }
        !c
    };
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut chunk = |kind: &[u8], data: &[u8]| {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        let mut body = kind.to_vec();
        body.extend_from_slice(data);
        out.extend_from_slice(&body);
        out.extend_from_slice(&crc(&body).to_be_bytes());
    };
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&(w as u32).to_be_bytes());
    ihdr.extend_from_slice(&(h as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(b"IHDR", &ihdr);
    chunk(b"IDAT", &z);
    chunk(b"IEND", &[]);
    out
}

fn main() -> eframe::Result {
    let shots = std::env::var_os("OPENCAD_UI_SHOT_DIR").map(|dir| {
        let page = std::env::var("OPENCAD_UI_PAGE").ok();
        let lang = std::env::var("OPENCAD_UI_LANG")
            .ok()
            .and_then(|l| Lang::from_key(&l));
        let mut queue = Vec::new();
        for p in Page::ALL {
            if page.as_deref().is_some_and(|k| k != p.key()) {
                continue;
            }
            for l in Lang::ALL {
                if lang.is_some_and(|x| x != l) {
                    continue;
                }
                queue.push((p, l));
            }
        }
        let dir = PathBuf::from(dir);
        std::fs::create_dir_all(&dir).expect("create the screenshot directory");
        Shots {
            dir,
            queue,
            frame: 0,
            part: 0,
            pending: Arc::new(Mutex::new(None)),
        }
    });
    let size = std::env::var("OPENCAD_UI_SIZE")
        .ok()
        .and_then(|s| {
            let (w, h) = s.split_once('x')?;
            Some([w.parse().ok()?, h.parse().ok()?])
        })
        .unwrap_or([1600.0, 1000.0]);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_title("OpenCAD UI Gallery"),
        // The size comes from OPENCAD_UI_SIZE, not from the last run.
        persist_window: false,
        ..Default::default()
    };
    eframe::run_native(
        "opencad-ui-gallery",
        options,
        Box::new(move |cc| {
            opencad_ui::install(&cc.egui_ctx, |_| {});
            Ok(Box::new(App {
                gallery: Gallery::default(),
                shots,
                size: size.into(),
                frames: 0,
            }))
        }),
    )
}
