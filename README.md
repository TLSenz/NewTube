# 🎬 Rust Video Streaming Platform

A self-hosted video streaming platform built with [Rocket](https://rocket.rs/) in Rust — inspired by YouTube. The goal is to allow users to upload and stream videos easily and securely.

---

## 🛠️ Toolchain

- **Language & Framework:** [Rust](https://www.rust-lang.org/) + [Rocket](https://rocket.rs/)
- **Database:** [PostgreSQL](https://www.postgresql.org/)
- **ORM:** [Diesel](https://diesel.rs/)

---

## 🚀 Roadmap

### ✅ Stage 1: Core Functionality
- [x] User login system
- [ ] JWT-based authentication
- [ ] PostgreSQL integration
- [ ] Uploading video files

### 🔄 Stage 2: Storage & Streaming
- [ ] Storing videos on the server
- [ ] HTTP-based video streaming

### 🎨 Stage 3: UI & Extra Features
- [ ] Build the web GUI
- [ ] Add features like comments, likes, playlists, etc.

---

## 📦 Installation

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Diesel CLI](https://diesel.rs/guides/getting-started/)
- Have a running [PostgreSQL server](https://www.postgresql.org/download/)

### Getting Started

```bash
git clone https://github.com/your-username/your-project.git
cd your-project
diesel setup
cargo build
