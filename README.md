# Claude Code Rust 🦀

> 🚀 **Implementación completa de Claude Code en Rust**: más rápida, liviana y preparada para uso local con CLI, GUI, MCP, plugins, memoria y servicios avanzados.

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen.svg)]()
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)]()

**[Inicio rápido](#-inicio-rápido) • [Características](#-características) • [Instalación](#-instalación) • [Uso](#-uso) • [Arquitectura](#-arquitectura)**

</div>

## 🌐 Sitios del proyecto

| Sitio | Descripción |
|:--|:--|
| [Claude Code Rust](https://claudecode-rust.netlify.app/) | Sitio del proyecto con demostración de rendimiento e instalación |
| [Claude Code Rust Landing](https://lorryjovens-hub.github.io/claude-code-rust-landing/) | Página de presentación y características |

## 🎯 Descripción general

Claude Code Rust es una reimplementación de alto rendimiento de Claude Code usando Rust. El objetivo es ofrecer una herramienta nativa, rápida y portable para programadores que quieren trabajar con asistentes de IA desde terminal o interfaz gráfica.

Ventajas principales:

- ⚡ **Alto rendimiento**: arranque rápido y bajo consumo de memoria.
- 📦 **Distribución ligera**: ejecutables nativos sin depender de Node.js en tiempo de ejecución.
- 🔒 **Seguridad de memoria**: ventajas del compilador de Rust para reducir errores de memoria.
- 🧰 **Herramientas completas**: CLI, REPL, GUI, MCP, plugins, memoria, voz y servicios.
- 🌍 **Localización al español**: textos principales de usuario, ayuda CLI y mensajes visibles traducidos.

## ✨ Características

### CLI y REPL

- Consultas rápidas desde línea de comandos.
- Sesiones REPL interactivas.
- Gestión de configuración.
- Información del sistema y ayuda integrada.

### Interfaz gráfica

- Panel de chat.
- Configuración de API y conexión.
- Temas visuales.
- Mensajes de estado y errores en español.

### MCP y herramientas

- Gestión de servidores MCP.
- Herramientas de archivos.
- Recursos, prompts y transporte MCP.

### Plugins y habilidades

- Instalación, búsqueda, activación y desactivación de plugins.
- Registro y ejecución de habilidades.
- Estructura preparada para extender funcionalidades.

### Memoria y servicios

- Memoria de sesiones.
- Consolidación de memoria.
- Sincronización de memoria de equipo.
- Servicios avanzados como Magic Docs, voz, agentes y pruebas de estrés.

## 🚀 Inicio rápido

### Requisitos

- **Rust 1.75 o superior**: <https://rustup.rs/>
- **Git**
- **Windows, Linux o macOS**

### Instalación

```bash
git clone https://github.com/lorryjovens-hub/claude-code-rust.git
cd claude-code-rust
cargo build --release
```

En Windows también puedes usar PowerShell:

```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
.\scripts\install-windows.ps1
```

## 💬 Uso

### Ver ayuda

```bash
cargo run -- --help
```

### Mostrar información del sistema

```bash
cargo run -- --info
```

### Ejecutar una consulta

```bash
cargo run -- query --prompt "Explica este proyecto"
```

### Iniciar modo interactivo

```bash
cargo run -- repl
```

### Ejecutar la GUI

```bash
cargo run --bin claude-code-gui
```

## ⚙️ Configuración

La herramienta puede usar variables de entorno o configuración local para la API.

Ejemplo:

```bash
claude-code config set api_key "tu-api-key"
```

Si la clave no está configurada, la aplicación mostrará instrucciones en español para completarla.

## 🏗️ Arquitectura

```text
claude-code-rust/
├── src/
│   ├── api/          # Cliente API
│   ├── cli/          # Línea de comandos y REPL
│   ├── config/       # Configuración
│   ├── gui/          # Interfaz gráfica egui
│   ├── i18n/         # Localización e idiomas
│   ├── mcp/          # Protocolo MCP
│   ├── memory/       # Memoria y sesiones
│   ├── plugins/      # Sistema de plugins
│   ├── services/     # Servicios avanzados
│   ├── skills/       # Habilidades integradas
│   ├── tools/        # Herramientas de archivos, búsqueda y git
│   ├── voice/        # Entrada por voz
│   ├── wasm/         # Soporte WebAssembly
│   └── web/          # Servidor web opcional
├── locales/          # Archivos de idioma
├── scripts/          # Scripts de instalación
├── Cargo.toml        # Configuración Rust
└── README.md         # Documentación principal
```

## 🌍 Idioma español

Este repositorio incluye:

- `locales/es.ftl` con traducciones de interfaz.
- Ayuda de comandos traducida al español.
- Mensajes principales de salida y errores traducidos.
- README principal en español para usuarios finales.

## 🧪 Validación

Antes de publicar cambios se recomienda ejecutar:

```bash
cargo fmt
cargo check
cargo test
```

## 🤝 Contribución

Las contribuciones son bienvenidas. Puedes abrir issues, enviar pull requests o mejorar traducciones, documentación y pruebas.

## 📄 Licencia

Este proyecto usa licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más información.
