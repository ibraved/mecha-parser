# Mecha Parser

<div align="center">

![Mecha Parser](https://img.shields.io/badge/Mecha-Parser-blue?style=for-the-badge&logo=windows)
![Version](https://img.shields.io/badge/Version-0.1.0-green?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)

**A modern, real-time roster tracking application for [MechaBREAK](https://mechabreak.seasungames.com/).**

</div>

---

## 📖 Overview

**Mecha Parser** is a Windows desktop application that provides live insights into your MechaBREAK matches. It monitors the game's log files in real-time to display team compositions, player ready states, and mecha selections in a clean, modern interface.

Built with **React** and **Tauri**, it combines a responsive frontend with a high-performance Rust backend for native system integration.

## ✨ Features

- **Live Roster Tracking**: Automatically detects team lineups as players join and select mechas.
- **Real-Time Updates**: Instant feedback on player ready status and mecha changes.
- **Auto-Discovery**: Automatically locates your MechaBREAK installation and log files—no manual setup required.
- **Modern UI**: Polished, dark-mode interface built with standardized components.
- **Low Resource Usage**: Efficient Rust-based file tailing and parsing.

## 🛠️ Tech Stack

- **Frontend**: [React 19](https://react.dev/), [TypeScript](https://www.typescriptlang.org/), [Vite](https://vitejs.dev/), [TailwindCSS](https://tailwindcss.com/)
- **Backend**: [Tauri v2](https://tauri.app/) (Rust)
- **UI Components**: [Radix UI](https://www.radix-ui.com/)
- **State Management**: React Hooks + Tauri Events

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (v18 or higher)
- **Rust** (via [rustup](https://rustup.rs/), ensure MSVC toolchain is installed on Windows)

### Installation

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/yourusername/mecha-tracker.git
    cd mecha-tracker
    ```

2.  **Install frontend dependencies:**

    ```bash
    npm install
    ```

### Running Locally

To start the application in development mode with hot-reloading:

```bash
npm run tauri:dev
```

This command will:
1.  Start the Vite development server.
2.  Compile the Rust backend.
3.  Launch the application window.

> **Note**: The web-only preview (`npm run dev`) works but will lack the backend functionality (log tracking) which relies on Tauri APIs.

### Building for Production

To create an optimized executable installer:

```bash
npm run tauri:build
```

The output installer will be located in `src-tauri/target/release/bundle/nsis/`.

## 📂 Project Structure

- **`src/`**: React frontend code.
    - `components/`: UI components and views.
    - `hooks/`: Custom hooks for tracking logic (`useTracker.ts`).
    - `lib/`: Utilities and Tauri IPC wrappers.
- **`src-tauri/`**: Rust backend code.
    - `src/parser/`: Log parsing logic.
    - `src/log_discovery.rs`: Game log detection.
    - `src/tailer.rs`: Real-time file monitoring.

## 🤝 Contributing

Contributions are welcome! Please check out the [ARCHITECTURE.md](ARCHITECTURE.md) for a deep dive into the codebase and design philosophy.

## 📄 License

This project is licensed under the MIT License.
