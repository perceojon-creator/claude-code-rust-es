# Benchmarks de rendimiento

Claude Code Rust está diseñado para ser rápido, liviano y portable.

## Objetivos

- Arranque rápido.
- Bajo consumo de memoria.
- Ejecutable nativo.
- Menos dependencias de tiempo de ejecución.

## Comparación esperada

| Métrica | Rust | Implementaciones con runtime |
|:--|:--|:--|
| Inicio | Rápido | Depende del runtime |
| Memoria base | Baja | Mayor |
| Distribución | Ejecutable nativo | Requiere dependencias |
| Seguridad de memoria | Garantías de Rust | Depende del entorno |

## Cómo medir

```bash
cargo build --release
./target/release/claude-code --version
./target/release/claude-code --help
```

En Windows usa:

```powershell
.\target\release\claude-code.exe --version
```
