use whatsapp_rust::prelude::*;

pub fn register(router: &mut BotRouter) {
    // !poll <pertanyaan> - Buat polling interaktif
    router.command("poll", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        let question = if cmd.raw_args.is_empty() {
            "Bahasa pemrograman apa yang paling Anda sukai untuk bot?".to_string()
        } else {
            cmd.raw_args.clone()
        };

        let options = vec![
            "Rust 🦀 (WeaBot-Rust - Blazing Fast)".to_string(),
            "Go 🐹 (WhatsMeow)".to_string(),
            "TypeScript / Node.js 🟩 (Baileys)".to_string(),
            "Python 🐍".to_string(),
        ];

        cmd.react("📊").await?;
        cmd.send_poll(&question, &options, 1).await?;
        Ok(())
    });

    // !location - Kirim sampel pin lokasi GPS
    router.command("location", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        cmd.react("📍").await?;
        cmd.reply_location(
            -6.1753924,
            106.8271528,
            Some("Monas (Monumen Nasional)".into()),
            Some("Gambir, Kecamatan Gambir, Kota Jakarta Pusat, DKI Jakarta".into()),
        ).await?;
        Ok(())
    });

    // !contact - Kirim kartu nama kontak resmi vCard
    router.command("contact", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        cmd.react("👤").await?;
        let vcard = "BEGIN:VCARD\n\
VERSION:3.0\n\
FN:KazeDevID (WeaBot Creator)\n\
ORG:whatsapp-rust Developer\n\
TEL;type=CELL;type=VOICE;waid=628123456789:+62 812-3456-789\n\
NOTE:Blazingly fast WhatsApp Bot in Rust\n\
URL:https://github.com/KazeDevID/weabot-rust\n\
END:VCARD";
        cmd.reply_contact("KazeDevID (Bot Developer)", vcard).await?;
        Ok(())
    });
}
