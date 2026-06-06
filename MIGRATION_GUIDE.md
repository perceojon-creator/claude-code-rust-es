# Guía de migración

Esta guía ayuda a migrar desde una versión previa o desde una implementación basada en TypeScript/Node hacia Claude Code Rust.

## Pasos recomendados

1. Instala Rust.
2. Clona este repositorio.
3. Compila con `cargo build --release`.
4. Copia tu configuración de API.
5. Prueba comandos básicos.
6. Migra plugins o herramientas según sea necesario.

## Configuración

Revisa claves como:

- `api_key`
- `base_url`
- `model`

## Validación

```bash
cargo run -- --help
cargo run -- --info
cargo check
```
