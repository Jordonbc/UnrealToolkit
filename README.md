[![GitHub release](https://img.shields.io/github/release/Jordonbc/UnrealToolkit)](https://github.com/Jordonbc/UnrealToolkit/releases/) ![open issues](https://img.shields.io/github/issues-raw/Jordonbc/UnrealToolkit) ![Open Source](https://badges.frapsoft.com/os/v1/open-source.svg?v=104) ![made-with-rust](https://img.shields.io/badge/Made%20With-Rust-Green) ![GitHub last commit](https://img.shields.io/github/last-commit/jordonbc/UnrealToolkit)
# UnrealToolkit
Unreal Toolkit aims to make the process of packaging easier and potentially faster. With this program you’re able to package an Unreal Engine game without ever needing to open up the Editor.

This program uses the Rust programming language for the backend and uses the [Tauri](https://tauri.app) windowing system which allows me to make the GUI from HTML and JavaScript. I aim for it to be cross-compatible with Windows, Mac and Linux.

Unreal Toolkit allows you to target multiple configurations and platforms, this makes it a versatile tool for game developers like myself.

<p align="center"><img src=resources/Unreal_Toolkit_window.png/></p>

## Features
- Easy to use
- Package multiple configurations
- Package client and server builds
- Uses less RAM than Unreal Editor (Which allow more threads for compiling faster)
- Ability to remove the crash reporter (reduces game size by 1.5GB, however makes crashes less user-friendly)

## Planned Features
- Ability to build lighting for all or selected maps
- Show and parse through log files from Unreal
- Allow user to automatically create zipped archive of builds after packaging
- Upload completed builds to external server
- Create manifest file with error-checking for multi-version production