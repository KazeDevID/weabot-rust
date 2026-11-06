# 🦀 WeaBot-Rust

> **Bot WhatsApp Multi-Device Berkecepatan Tinggi (Blazingly Fast) Berbasis Bahasa Pemrograman Rust.**

[![Rust](https://img.shields.io/badge/Language-Rust_2024-orange.svg)](https://www.rust-lang.org)
[![Engine](https://img.shields.io/badge/Engine-whatsapp--rust-brightgreen.svg)](https://github.com/KazeDevID/whatsapp-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Multi-Device](https://img.shields.io/badge/WhatsApp-Multi--Device_E2EE-25D366.svg)](https://whatsapp.com)

**WeaBot-Rust** adalah template dan framework WhatsApp Bot siap pakai yang ditenagai oleh pustaka [`whatsapp-rust`](https://github.com/KazeDevID/whatsapp-rust). Dibuat dengan performa tinggi, aman secara memori (*memory-safe*), dan sangat hemat RAM (~8 MB RSS) dibandingkan bot berbasis Node.js/Baileys (~150 MB) atau Go/WhatsMeow (~45 MB).

---

## ⚡ Fitur Utama

- 🔒 **End-to-End Encryption (E2EE):** Menggunakan Signal Protocol (Curve25519, Double Ratchet, SenderKey) & Noise Protocol handshake.
- 📱 **Multi-Device Pairing Fleksibel:** Mendukung scan **QR Code terminal** maupun **8-Digit Pairing Code telepon**.
- 🚦 **Sistem Command Router Cerdas:**
  - Dukungan multi-prefix (`!`, `/`, `.`)
  - Auto-ignore pesan bot sendiri (*self-suppress*)
  - **Admin Guards:** Pembatasan otomatis perintah khusus Admin Grup (`router.admin_command`)
  - **Group Guards:** Pembatasan perintah hanya untuk grup (`router.group_command`)
- 📢 **Tag All / Hidetag (`!tagall`):** Mention semua peserta grup dalam 1 pesan pengumuman.
- 📊 **Polling Interaktif:** Buat polling pilihan multi-device langsung dari chat.
- 📍 **Rich Location & Contact:** Kirim pin lokasi GPS Google Maps dan kartu kontak vCard.
- 💾 **Sesi Persisten:** Otomatis tersimpan di database lokal SQLite (`sessions/weabot.db`).
- ⚡ **Zero-Copy Protobuf:** Parsing frame WhatsApp menggunakan mesin `buffa`.

---

## 📋 Daftar Perintah Bawaan

| Perintah | Deskripsi | Hak Akses |
| :--- | :--- | :--- |
| `!ping` | Tes kecepatan respons bot (*pong!*) | Publik |
| `!echo <teks>` | Membalas kembali teks yang dikirimkan | Publik |
| `!poll [pertanyaan]` | Membuat polling WhatsApp interaktif | Publik |
| `!typing` | Demonstrasi indikator sedang mengetik | Publik |
| `!location` | Mengirimkan pin koordinat GPS Google Maps | Publik |
| `!contact` | Mengirimkan kartu nama kontak (vCard) developer | Publik |
| `!info` | Menampilkan statistik RAM, uptime, dan session JID | Publik |
| `!help` | Menampilkan menu bantuan perintah | Publik |
| `!tagall [pesan]` | Menandai / mention seluruh anggota grup | **Admin Grup** |
| `!admins` | Menampilkan daftar seluruh admin di grup ini | Anggota Grup |
| `!link` | Mengambil tautan undangan grup WhatsApp | **Admin Grup** |
| `!kick <nomor>` | Mengeluarkan peserta dari grup | **Admin Grup** |

---

## 🛠️ Tutorial Instalasi & Menjalankan Bot

### 1. Prasyarat Sistem

Pastikan komputer atau VPS Linux Anda telah terpasang compiler Rust & dependensi build:

#### Linux (Ubuntu / Debian):
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev git clang sqlite3 libsqlite3-dev curl

# Install Rust toolchain (jika belum ada)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### macOS:
```bash
brew install openssl pkg-config sqlite
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows:
Pastikan telah menginstal [Rustup](https://rustup.rs/) dan C++ Build Tools (Visual Studio).

---

### 2. Clone Repositori

```bash
git clone https://github.com/KazeDevID/weabot-rust.git
cd weabot-rust
```

---

### 3. Konfigurasi Lingkungan (`.env`)

Salin file contoh konfigurasi:

```bash
cp .env.example .env
```

Isi file `.env` sesuai kebutuhan Anda:
```env
# Prefix perintah bot (contoh: !)
BOT_PREFIX="!"

# (Opsional) Nomor telepon bot untuk Pairing Code 8-Digit
# Format internasional tanpa tanda (+) atau spasi. Contoh: 628123456789
BOT_PHONE=""

# Lokasi penyimpanan sesi SQLite
DATABASE_PATH="sessions/weabot.db"

# Level logging
RUST_LOG="info"
```

---

### 4. Menjalankan Bot

#### Opsi A: Menghubungkan Menggunakan Scan QR Code

Jika `BOT_PHONE` di `.env` dikosongkan:

```bash
cargo run --release
```

Bot akan menampilkan string/QR code di terminal. Buka **WhatsApp di HP Anda** > **Perangkat Tertaut** > **Tautkan Perangkat** dan scan kode yang muncul.

---

#### Opsi B: Menghubungkan Menggunakan 8-Digit Phone Pairing Code

Tanpa perlu scan kamera! Cukup masukkan nomor HP Anda (format: `628xxxxxxxxxx`):

```bash
cargo run --release -- --phone 628123456789
```

Bot akan menampilkan kode 8 karakter, contoh:
```text
╔═════════════════════════════════════════════════╗
║   KODE PAIRING WHATSAPP (8-DIGIT):              ║
║                  K4ZE-9X8R                      ║
║   Berlaku selama: 60 detik                      ║
╚═════════════════════════════════════════════════╝
```

Buka **WhatsApp di HP** > **Perangkat Tertaut** > **Tautkan dengan nomor telepon**, lalu masukkan kode 8-digit tersebut. Selesai! Sesi akan tersimpan di `sessions/weabot.db` secara permanen.

---

### 5. Menjalankan Menggunakan Docker (Opsional)

Jika Anda ingin menjalankan bot secara kontainer di server:

```bash
# Build dan jalankan dengan docker compose
docker compose up -d

# Cek log bot (untuk melihat QR atau pairing code)
docker compose logs -f
```

---

### 6. Menjalankan Sebagai Systemd Service (VPS Linux)

Agar bot otomatis menyala kembali saat server restart:

1. Buat file service:
```bash
sudo nano /etc/systemd/system/weabot.service
```

2. Tempel konfigurasi berikut (sesuaikan path):
```ini
[Unit]
Description=WeaBot-Rust WhatsApp Multi-Device Bot
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root/weabot-rust
ExecStart=/root/.cargo/bin/cargo run --release
Restart=always
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

3. Aktifkan dan jalankan:
```bash
sudo systemctl daemon-reload
sudo systemctl enable weabot
sudo systemctl start weabot

# Cek status bot
sudo systemctl status weabot
```

---

## 🧩 Cara Menambahkan Perintah Baru

Menambahkan perintah baru sangat mudah dan bersih:

Buka file `src/commands/general.rs` (atau buat file baru di `src/commands/`):

```rust
// Contoh: Perintah !waktu
router.command("waktu", |cmd| async move {
    let _ = cmd.mark_as_read().await;
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    cmd.reply(format!("🕒 Waktu server saat ini: {now}")).await?;
    Ok(())
});

// Contoh: Perintah khusus Admin Grup
router.admin_command("peringatan", |cmd| async move {
    cmd.react("⚠️").await?;
    cmd.reply("⚠️ Harap patuhi tata tertib grup ini!").await?;
    Ok(())
});
```

---

## 📦 Struktur Direktori Proyek

```text
weabot-rust/
├── Cargo.toml               # Dependensi & konfigurasi Rust
├── Dockerfile               # Multi-stage image build
├── docker-compose.yml       # Docker orchestrator
├── .env.example             # Template variabel lingkungan
├── .gitignore               # Daftar pengabaian file sesi/target
├── README.md                # Dokumentasi & panduan instalasi
├── sessions/                # Direktori penyimpanan database SQLite (auto-created)
└── src/
    ├── main.rs              # Titik masuk utama program bot
    └── commands/
        ├── mod.rs           # Registry pemanggil semua modul perintah
        ├── general.rs       # Perintah umum (!ping, !echo, !info, !help)
        ├── group.rs         # Perintah grup (!tagall, !admins, !link, !kick)
        └── media.rs         # Perintah media (!poll, !location, !contact)
```

---

## 💖 Ucapan Terima Kasih & Referensi

- **Core Library:** [github.com/KazeDevID/whatsapp-rust](https://github.com/KazeDevID/whatsapp-rust)
- **Original Rust Client:** [oxidezap/whatsapp-rust](https://github.com/oxidezap/whatsapp-rust)
- **Inspirasi Logika & Paritas:** [WhiskeySockets/Baileys](https://github.com/WhiskeySockets/Baileys) & [tulir/whatsmeow](https://github.com/tulir/whatsmeow)

---

## 📄 Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).
