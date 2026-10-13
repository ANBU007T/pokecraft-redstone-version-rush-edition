// ============================================================================
// PokéCRAFT - Pure Rust Native Edition (High Performance Engine)
// Multi-threaded In-Memory Caching Server + GPU Accelerated WebView2 Host
// ============================================================================

#![windows_subsystem = "windows"]

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tiny_http::{Header, Response, Server, StatusCode};
use wry::{WebContext, WebViewBuilder};

#[cfg(target_os = "windows")]
use wry::WebViewBuilderExtWindows;

type MemoryCache = Arc<RwLock<HashMap<String, (Arc<Vec<u8>>, String)>>>;

fn resolve_app_dir() -> PathBuf {
    // 1. Candidate paths in current working directory
    let candidates = [
        PathBuf::from("app"),
        PathBuf::from("PokeCraft-Game").join("resources").join("app"),
        PathBuf::from("resources").join("app"),
        PathBuf::from("PC-Rust").join("app"),
        PathBuf::from("PC-GitHub-Repo").join("PokeCraft-Game").join("resources").join("app"),
        PathBuf::from("PC").join("PokeCraft-Game").join("resources").join("app"),
    ];

    for c in &candidates {
        if c.is_dir() && c.join("index.html").exists() {
            return c.clone();
        }
    }

    // 2. Candidate paths relative to the executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_candidates = [
                exe_dir.join("app"),
                exe_dir.join("PokeCraft-Game").join("resources").join("app"),
                exe_dir.join("resources").join("app"),
                exe_dir.join("PC-Rust").join("app"),
            ];
            for c in &exe_candidates {
                if c.is_dir() && c.join("index.html").exists() {
                    return c.clone();
                }
            }
            if let Some(proj_dir) = exe_dir.parent() {
                let proj_candidates = [
                    proj_dir.join("app"),
                    proj_dir.join("PokeCraft-Game").join("resources").join("app"),
                    proj_dir.join("resources").join("app"),
                    proj_dir.join("PC-Rust").join("app"),
                ];
                for c in &proj_candidates {
                    if c.is_dir() && c.join("index.html").exists() {
                        return c.clone();
                    }
                }
            }
        }
    }

    PathBuf::from("app")
}

fn start_local_server(app_dir: PathBuf, running: Arc<AtomicBool>) -> u16 {
    let server = Arc::new(Server::http("127.0.0.1:0").expect("Failed to bind local HTTP server"));
    let port = server.server_addr().to_ip().expect("Failed to resolve server IP").port();
    let worker_count = 12;

    // Synchronously pre-warm critical game files into RAM cache (<15ms)
    // so every single boot asset is served instantly from memory with zero lock contention
    let mut initial_cache = HashMap::new();
    let critical_files = [
        "index.html",
        "controls_manager.js",
        "save_manager.js",
        "settings.js",
        "settings.css",
        "sponsor.css",
        "src/main.js",
        "src/game.js",
        "src/assets.js",
        "src/world.js",
        "src/battle.js",
        "assets/data/pokemon.json",
        "assets/world/world.json",
        "assets/world/grid.bin",
        "assets/world/ground.bin",
        "assets/world/col.bin",
        "assets/world/beh.bin",
        "assets/world/zone.bin",
        "assets/world/area.bin",
        "assets/world/prot.bin",
        "assets/world/autotile.json",
        "assets/world/atlas.png",
        "assets/world/atlas_top.png",
        "assets/ui/title_logo.png",
        "assets/ui/favicon.png",
        "assets/ui/font.json",
        "assets/ui/font.png",
        "assets/ui/font_small.json",
        "assets/ui/font_small.png",
        "assets/ui/window.png",
        "assets/mc/title_steve.png",
        "assets/mc/items.json",
        "assets/mc/font.json",
        "assets/mc/font.png",
        "assets/mc/blocks.png",
        "assets/mc/icons.png",
        "assets/sfx/sfx.json",
        "assets/battle/colors.json",
        "assets/ui/party_colors.json",
    ];
    for rel in &critical_files {
        let path = if app_dir.join(rel).is_file() {
            app_dir.join(rel)
        } else if let Some(parent) = app_dir.parent() {
            if parent.join(rel).is_file() {
                parent.join(rel)
            } else if let Some(grandparent) = parent.parent() {
                if grandparent.join(rel).is_file() {
                    grandparent.join(rel)
                } else {
                    app_dir.join(rel)
                }
            } else {
                app_dir.join(rel)
            }
        } else {
            app_dir.join(rel)
        };
        if let Ok(bytes) = std::fs::read(&path) {
            let mime = mime_guess::from_path(&path).first_or_octet_stream().to_string();
            initial_cache.insert(rel.to_string(), (Arc::new(bytes), mime));
        }
    }
    let cache: MemoryCache = Arc::new(RwLock::new(initial_cache));

    // Spawn 12 concurrent worker threads for parallel asset streaming
    for _ in 0..worker_count {
        let server = server.clone();
        let app_dir = app_dir.clone();
        let running = running.clone();
        let cache = cache.clone();

        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let request = match server.recv() {
                    Ok(req) => req,
                    Err(_) => break,
                };

                let url_raw = request.url();
                let url_clean = url_raw.split('?').next().unwrap_or(url_raw);
                let mut rel_path = url_clean.trim_start_matches('/');
                if rel_path.is_empty() {
                    rel_path = "index.html";
                }

                let decoded_path = percent_encoding::percent_decode_str(rel_path)
                    .decode_utf8_lossy()
                    .to_string();

                // 1. Fast path: Memory Cache hit (serves in <0.01ms)
                if let Ok(c) = cache.read() {
                    if let Some((data, mime)) = c.get(&decoded_path) {
                        let content_type = match Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()) {
                            Ok(h) => h,
                            Err(_) => Header::from_bytes(&b"Content-Type"[..], &b"application/octet-stream"[..]).unwrap(),
                        };
                        let cors = Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap();
                        let cache_hdr = Header::from_bytes(&b"Cache-Control"[..], &b"public, max-age=31536000, immutable"[..]).unwrap();
                        let keep_alive = Header::from_bytes(&b"Connection"[..], &b"keep-alive"[..]).unwrap();

                        let mut response = Response::from_data((**data).clone());
                        response.add_header(content_type);
                        response.add_header(cors);
                        response.add_header(cache_hdr);
                        response.add_header(keep_alive);
                        let _ = request.respond(response);
                        continue;
                    }
                }

                // 2. Slow path: Read disk once, cache in RAM, stream instantly
                let mut target_file = app_dir.join(&decoded_path);
                if !target_file.is_file() {
                    if let Some(parent) = app_dir.parent() {
                        let parent_file = parent.join(&decoded_path);
                        if parent_file.is_file() {
                            target_file = parent_file;
                        } else if let Some(grandparent) = parent.parent() {
                            let gp_file = grandparent.join(&decoded_path);
                            if gp_file.is_file() {
                                target_file = gp_file;
                            }
                        }
                    }
                }

                if target_file.is_file() {
                    if let Ok(bytes) = std::fs::read(&target_file) {
                        let mime = mime_guess::from_path(&target_file)
                            .first_or_octet_stream()
                            .to_string();

                        let data_arc = Arc::new(bytes);

                        if let Ok(mut c) = cache.write() {
                            c.insert(decoded_path, (data_arc.clone(), mime.clone()));
                        }

                        let content_type = match Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()) {
                            Ok(h) => h,
                            Err(_) => Header::from_bytes(&b"Content-Type"[..], &b"application/octet-stream"[..]).unwrap(),
                        };
                        let cors = Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap();
                        let cache_hdr = Header::from_bytes(&b"Cache-Control"[..], &b"public, max-age=31536000, immutable"[..]).unwrap();
                        let keep_alive = Header::from_bytes(&b"Connection"[..], &b"keep-alive"[..]).unwrap();

                        let mut response = Response::from_data((*data_arc).clone());
                        response.add_header(content_type);
                        response.add_header(cors);
                        response.add_header(cache_hdr);
                        response.add_header(keep_alive);

                        let _ = request.respond(response);
                        continue;
                    }
                }

                let not_found = Response::from_string("404 Not Found")
                    .with_status_code(StatusCode(404));
                let _ = request.respond(not_found);
            }
        });
    }

    port
}

fn prune_temp_caches(data_dir: &std::path::Path) {
    let ebwebview = data_dir.join("EBWebView");
    let dirs_to_remove = [
        ebwebview.join("Crashpad"),
        ebwebview.join("ShaderCache"),
        ebwebview.join("GrShaderCache"),
        ebwebview.join("GPUPersistentCache"),
        ebwebview.join("BrowserMetrics"),
        ebwebview.join("Default").join("Cache"),
        ebwebview.join("Default").join("Code Cache"),
        ebwebview.join("Default").join("GPUCache"),
        ebwebview.join("Default").join("DawnGraphiteCache"),
        ebwebview.join("Default").join("DawnWebGPUCache"),
        ebwebview.join("Default").join("blob_storage"),
        ebwebview.join("Default").join("AutofillAiModelCache"),
    ];
    for d in &dirs_to_remove {
        if d.is_dir() {
            let _ = std::fs::remove_dir_all(d);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_dir = resolve_app_dir();
    let running = Arc::new(AtomicBool::new(true));

    let port = start_local_server(app_dir.clone(), running.clone());
    let game_url = format!("http://127.0.0.1:{}/index.html", port);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("PokéCRAFT - Pokemon Redstone PC Edition")
        .with_inner_size(LogicalSize::new(1024.0, 720.0))
        .with_min_inner_size(LogicalSize::new(480.0, 320.0))
        .build(&event_loop)?;

    // Isolated save data storage
    let data_dir = app_dir.parent().unwrap_or(&app_dir).join("save_data");
    std::fs::create_dir_all(&data_dir).ok();
    prune_temp_caches(&data_dir);
    let mut web_context = WebContext::new(Some(data_dir.clone()));

    let builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url(&game_url)
        .with_devtools(cfg!(debug_assertions));

    // Force dedicated hardware GPU acceleration, zero throttling & minimal disk cache
    #[cfg(target_os = "windows")]
    let builder = builder
        .with_additional_browser_args(
            "--ignore-gpu-blocklist --enable-gpu-rasterization --enable-zero-copy --disable-background-timer-throttling --disable-renderer-backgrounding --disable-features=CalculateNativeWinOcclusion,OptimizationHints,Translate,CertificateRevocation --autoplay-policy=no-user-gesture-required --disk-cache-size=1 --media-cache-size=1 --disable-component-update"
        )
        .with_browser_accelerator_keys(true);

    let _webview = builder.build(&window)?;

    let exit_data_dir = data_dir.clone();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            running.store(false, Ordering::Relaxed);
            prune_temp_caches(&exit_data_dir);
            *control_flow = ControlFlow::Exit;
        }
    });
}
