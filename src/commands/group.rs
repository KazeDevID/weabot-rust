use whatsapp_rust::prelude::*;

pub fn register(router: &mut BotRouter) {
    // !tagall / !hidetag - Tag semua member grup (Hanya Admin)
    router.admin_command("tagall", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        cmd.react("📢").await?;
        let header = if cmd.raw_args.is_empty() {
            "📢 *PENGUMUMAN UNTUK SEMUA ANGGOTA:*"
        } else {
            &cmd.raw_args
        };
        cmd.mention_all(Some(header)).await?;
        Ok(())
    });

    // !admins - Daftar admin grup
    router.group_command("admins", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        cmd.react("🛡️").await?;
        let meta = cmd.group_metadata().await?;
        let admins: Vec<_> = meta.participants.iter().filter(|p| p.is_admin()).collect();
        let mut msg = format!("🛡️ *Daftar Administrator Grup ({})*:\n\n", admins.len());
        for (i, adm) in admins.iter().enumerate() {
            msg.push_str(&format!("{}. @{}\n", i + 1, adm.jid.user));
        }
        let admin_jids: Vec<_> = admins.iter().map(|p| p.jid.clone()).collect();
        cmd.reply_with_mentions(msg, &admin_jids).await?;
        Ok(())
    });

    // !link - Dapatkan link undangan grup (Hanya Admin)
    router.admin_command("link", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        cmd.react("🔗").await?;
        match cmd.group_invite_code().await {
            Ok(code) => {
                cmd.reply(format!("🔗 *Tautan Undangan Grup WhatsApp:*\nhttps://chat.whatsapp.com/{code}")).await?;
            }
            Err(e) => {
                cmd.reply(format!("❌ Gagal mengambil tautan grup: {e}")).await?;
            }
        }
        Ok(())
    });

    // !kick <nomor> - Keluarkan member dari grup (Hanya Admin)
    router.admin_command("kick", |cmd| async move {
        let _ = cmd.mark_as_read().await;
        if cmd.args.is_empty() {
            cmd.reply("⚠️ Mohon sebutkan nomor target. Contoh: `!kick 6281234567890`").await?;
            return Ok(());
        }
        let target_raw = cmd.args[0].replace(['+', '-', ' ', '@', 's', '.', 'w', 'h', 'a', 't', 's', 'p', 'n', 'e'], "");
        let target_jid = match whatsapp_rust::Jid::new(&target_raw, whatsapp_rust::Server::Pn) {
            target => target,
        };

        match cmd.group_kick(&[target_jid]).await {
            Ok(_) => {
                cmd.react("👢").await?;
                cmd.reply(format!("✅ Anggota @{target_raw} berhasil dikeluarkan dari grup.")).await?;
            }
            Err(e) => {
                cmd.reply(format!("❌ Gagal mengeluarkan anggota: {e} (Pastikan bot memiliki status Admin Grup)")).await?;
            }
        }
        Ok(())
    });
}
