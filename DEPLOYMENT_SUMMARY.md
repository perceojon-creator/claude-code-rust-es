# Resumen de despliegue

## Estado

El proyecto puede compilarse y desplegarse como ejecutable nativo de Rust.

## Componentes

- CLI: `claude-code`.
- GUI: `claude-code-gui`.
- Web opcional: `claude-code-web`.

## Flujo sugerido

1. Ejecutar pruebas y validación.
2. Compilar en modo release.
3. Publicar binarios o contenedor.
4. Documentar variables de entorno necesarias.

## Validación previa

```bash
cargo fmt
cargo check
cargo test
cargo build --release
```
