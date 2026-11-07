//! # WeaBot-Rust
//!
//! A high-performance, modular WhatsApp Bot written in Rust.
//! Powered by https://github.com/KazeDevID/whatsapp-rust
//!
//! Features:
//! - Multi-Device architecture with Signal Protocol E2EE
//! - SQLite persistent session storage
//! - Terminal QR Code and 8-digit Phone Pairing Code support
//! - Fluent BotRouter with Admin Guards & Group Mentions
//! - Interactive WhatsApp polls, location pins, and contact cards

mod commands;

use log::{error, info};
use std::time::Instant;
use whatsapp_rust::pair_code::PairCodeOptions;
use whatsapp_rust::prelude::*;
use whatsapp_rust::prelude::SqliteStore;

fn parse_arg(args: &[String], long: &str, short: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == long || args[i] == short {
            return args.get(i + 1).cloned();
        }
        if let Some(val) = args[i].strip_prefix(&format!("{long}=")) {
            return Some(val.to_string());
        }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load .env if present
    dotenv::dotenv().ok();

    // Setup structured logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "{} [{:<5}] [{}] {}",
                whatsapp_rust::wacore::time::now_utc().format("%H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    let start_time = Instant::now();

    println!(r#"
   __      __             ___         _         ___           _   
   \ \    / /___ __ _    | _ ) ___  _| |_      | _ \ _  _  __| |_ 
    \ \/\/ // -_) _` |   | _ \/ _ \(_-<  _|___ |   /| || |(_-<  _|
     \_/\_/ \___\__,_|   |___/\___//__/\__|___||_|_\ \_,_|/__/\__|
                    Pure Rust WhatsApp Bot Engine                  
    "#);

    let args: Vec<String> = std::env::args().collect();
    let phone_arg = parse_arg(&args, "--phone", "-p")
        .or_else(|| std::env::var("BOT_PHONE").ok())
        .filter(|p| !p.trim().is_empty());

    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "sessions/weabot.db".to_string());
    let prefix = std::env::var("BOT_PREFIX").unwrap_or_else(|_| "!".to_string());

    info!("Initializing session database at: {}", db_path);
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let backend = SqliteStore::open(&db_path).await?;

    // Create & configure high-level command router
    let mut router = BotRouter::new();
    router.prefixes(&[&prefix, "/", "."]);
    router.ignore_self(true);

    // Register all commands from modular folders
    commands::register_all(&mut router, start_time);

    // Fallback when unknown command is invoked
    let prefix_clone = prefix.clone();
    router.default(move |cmd| {
        let p = prefix_clone.clone();
        async move {
            cmd.react("❓").await?;
            cmd.reply(format!(
                "Perintah `{}{}` tidak dikenali.\nKetik `{}help` untuk melihat daftar perintah yang tersedia.",
                cmd.prefix, cmd.command, p
            )).await?;
            Ok(())
        }
    });

    // Build the bot instance
    let mut builder = Bot::builder()
        .with_backend(backend)
        .with_router(router);

    // Pairing configuration
    if let Some(phone) = phone_arg {
        info!("Phone number provided: {}. Using 8-Digit Pairing Code...", phone);
        builder = builder
            .with_pair_code(PairCodeOptions { phone_number: phone, ..Default::default() })
            .on_pair_code(|code, timeout| async move {
                println!("\n╔═════════════════════════════════════════════════╗");
                println!("║   KODE PAIRING WHATSAPP (8-DIGIT):              ║");
                println!("║                  {}                       ║", code);
                println!("║   Berlaku selama: {} detik                    ║", timeout.as_secs());
                println!("╚═════════════════════════════════════════════════╝\n");
                println!("Buka WhatsApp di HP Anda > Perangkat Tertaut > Tautkan dengan Kode Telepon\n");
            });
    } else {
        builder = builder.on_qr_code(|qr_string, timeout| async move {
            println!("\n╔═════════════════════════════════════════════════╗");
            println!("║   SCAN QR CODE WHATSAPP (Berlaku {}s)        ║", timeout.as_secs());
            println!("╚═════════════════════════════════════════════════╝");
            println!("String QR:\n{}\n", qr_string);
            println!("Buka WhatsApp di HP Anda > Perangkat Tertaut > Tautkan Perangkat\n");
        });
    }

    builder = builder.on_connected(|client| async move {
        info!("🎉 Berhasil terhubung dan terotentikasi dengan WhatsApp Multi-Device!");
        if let Some(jid) = client.pn() {
            info!("Nomor Bot Aktif (PN): {}", jid);
        }
    });

    builder = builder.on_logged_out(|info| async move {
        error!("Perangkat telah dikeluarkan (Logged Out) oleh WhatsApp: {:?}", info);
    });

    let bot = builder.build().await?;
    info!("WeaBot-Rust sedang berjalan. Tekan Ctrl+C untuk berhenti.");
    bot.run().await;

    Ok(())
}
