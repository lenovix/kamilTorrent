# kamil torrent

Versi 0.0.1 — CLI Torrent Downloader berbasis Rust dengan `aria2c`

---

## Deskripsi

`kamil torrent` adalah aplikasi command-line sederhana untuk mendownload file torrent.  
Untuk versi 0.0.1 ini, `kamil torrent` memanfaatkan tool `aria2c` sebagai backend downloader yang powerful dan stabil.

---

## Fitur versi 0.0.1

- Scan folder berisi file `.torrent`  
- Otomatis jalankan `aria2c` untuk mendownload torrent ke folder output  
- Mendukung banyak file torrent sekaligus  
- Multi-platform: Linux dan Windows (dengan syarat `aria2c` terinstall)

---

## Prasyarat

- Rust toolchain ([Install Rust](https://rustup.rs/))  
- `aria2c` terinstall di sistem

### Cara install aria2c di Debian/Ubuntu

```bash
sudo apt install aria2
```
---
### Cara menjalankan
Siapkan folder berisi file .torrent, misal torrent-files
Siapkan folder tujuan untuk hasil download, misal downloads
Jalankan perintah berikut dari folder project kamil_torrent:
```
cargo run -- ./torrent-files ./downloads
```
Program akan membaca semua file .torrent di folder torrent-files, lalu menjalankan aria2c untuk mendownload file tersebut ke folder downloads.

---
### Rencana Pengembangan
- Implementasi native Rust torrent client tanpa dependensi eksternal
- UI GUI versi desktop multi-platform
- Fitur manajemen download (pause, resume, status progress) via RPC
- Dukungan magnet link dan torrent streaming