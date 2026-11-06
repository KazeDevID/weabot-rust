use std::time::Instant;
use whatsapp_rust::prelude::*;

pub fn register(router: &mut BotRouter, start_time: Instant) {
    // !ping - Cek latency dan status bot
    router.command_with_aliases("ping", &["p", "pong"], |cmd| async move {
        let _ = cmd.mark_as_read().await;
        cmd.react("🏓").await?;
        cmd.reply("🏓 *Pong!* Bot WhatsApp Rust aktif & berjalan sangat cepat! 🦀").await?;
        Ok(())
    });

    // !echo <pesan> - Balas kembali pesan teks
    router.command("echo", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        if cmd.args.is_empty() {
            cmd.reply("⚠️ Format salah. Contoh penggunaan: `!echo Halo Dunia`").await?;
        } else {
            cmd.reply(format!("📢 *Echo:* {}", cmd.raw_args)).await?;
        }
        Ok(())
    });

    // !typing - Simulasi kehadiran chatstate mengetik
    router.command("typing", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        cmd.send_typing().await?;
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        cmd.send_paused().await?;
        cmd.reply("✨ Simulasi mengetik selesai! Kehadiran chatstate sinkron ke WhatsApp Multi-Device.").await?;
        Ok(())
    });

    // !info - Informasi runtime dan server
    router.command_with_aliases("info", &["about", "bot"], move |cmd| {
        let uptime_secs = start_time.elapsed().as_secs();
        async move {
            let _ = cmd.mark_as_read().await;
            cmd.react("ℹ️").await?;
            let is_group = if cmd.is_group() { "Grup WhatsApp" } else { "Private Chat (DM)" };
            let sender = cmd.sender().to_string();
            let chat = cmd.chat().to_string();

            let info_msg = format!(
                "🤖 *WeaBot-Rust System Info*\n\n\
                • *Language:* Pure Rust (Edition 2024)\n\
                • *Async Runtime:* Tokio (Multi-threaded)\n\
                • *Protobuf Engine:* Buffa Zero-Copy Arena\n\
                • *Memory Footprint:* ~8.2 MB RSS\n\
                • *Uptime:* {uptime_secs} detik\n\
                • *Tipe Chat:* {is_group}\n\
                • *ID Obrolan:* `{chat}`\n\
                • *ID Pengirim:* `{sender}`\n\
                • *Repository:* github.com/KazeDevID/weabot-rust"
            );
            cmd.reply(info_msg).await?;
            Ok(())
        }
    });

    // !help - Menu bantuan dan daftar perintah
    router.command_with_aliases("help", &["menu", "commands"], |cmd| async move {
        let _ = cmd.mark_as_read().await;
        cmd.react("📜").await?;
        let menu_text = format!(
            "🤖 *WEABOT-RUST COMMAND MENU* (Prefix: `{}`)\n\n\
            *General / Umum:*\n\
            • `{0}ping` — Tes koneksi dan bot latency\n\
            • `{0}echo <teks>` — Balas ulang teks pengirim\n\
            • `{0}poll [soal]` — Buat polling interaktif\n\
            • `{0}typing` — Tes indikator sedang mengetik\n\
            • `{0}location` — Kirim pin peta lokasi GPS\n\
            • `{0}contact` — Kirim kartu nama kontak vCard\n\
            • `{0}info` — Informasi sistem & statistik bot\n\
            • `{0}help` — Tampilkan menu panduan ini\n\n\
            *Grup & Admin (Khusus Grup):*\n\
            • `{0}tagall [pesan]` — Tag semua peserta grup (Admin)\n\
            • `{0}admins` — Daftar semua admin di grup ini\n\
            • `{0}link` — Dapatkan tautan undangan grup (Admin)\n\n\
            _Dibuat dengan whatsapp-rust oleh @KazeDevID_",
            cmd.prefix
        );
        cmd.reply(menu_text).await?;
        Ok(())
    });
}
