# Instalación

## Requisitos

- Rust 1.75 o superior.
- Git.
- Conexión a internet para descargar dependencias de Rust.

## Windows

Abre PowerShell en la carpeta del proyecto y ejecuta:

```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
.\scripts\install-windows.ps1
```

También puedes compilar manualmente:

```powershell
cargo build --release
```

## Linux y macOS

```bash
chmod +x ./install.sh
./install.sh
```

O manualmente:

```bash
cargo build --release
```

## Verificación

```bash
cargo run -- --version
cargo run -- --help
```
