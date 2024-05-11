How to reproduce the issue:

- Open a context menu by right-clicking anywhere in the window.
- Click on "item1"
- Open the "main-menu" on the main menu bar.
- Click on "sub-item1"
- Click on "sub-item2"
- The app crashes.

## Environment
- OS: Windows 10.0.19045 X64
- WebView2: 124.0.2478.80
- MSVC: Visual Studio Community 2022
- rustc: 1.78.0 (9b00956e5 2024-04-29)
- cargo: 1.78.0 (54d8815d0 2024-03-26)
- rustup: 1.27.1 (54dd3d00f 2024-04-24)
- Rust toolchain: stable-x86_64-pc-windows-msvc (environment override by RUSTUP_TOOLCHAIN)

## Packages
- tauri [RUST]: 2.0.0-beta.19
- tauri-build [RUST]: 2.0.0-beta.15
- wry [RUST]: 0.39.5
- tao [RUST]: 0.28.0
- tauri-cli [RUST]: 2.0.0-beta.13
- @tauri-apps/api [NPM]: 2.0.0-beta.11
- @tauri-apps/cli [NPM]: 2.0.0-beta.13
