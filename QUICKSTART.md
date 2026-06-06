# Inicio rápido

Esta guía resume cómo instalar y usar Claude Code Rust en español.

## 1. Requisitos

- Rust 1.75 o superior.
- Git.
- Windows, Linux o macOS.

## 2. Clonar el repositorio

```bash
git clone https://github.com/perceojon-creator/claude-code-rust-es.git
cd claude-code-rust
```

## 3. Compilar

```bash
cargo build --release
```

## 4. Ver ayuda

```bash
cargo run -- --help
```

## 5. Ejecutar el chat interactivo

```bash
cargo run -- repl
```

## 6. Ejecutar la interfaz gráfica

```bash
cargo run --bin claude-code-gui
```

## 7. Configurar la API

Configura tu clave antes de usar consultas reales:

```bash
claude-code config set api_key "tu-clave-api"
```
