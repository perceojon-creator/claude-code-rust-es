//! CLI Module - Command Line Interface

pub mod args;
pub mod commands;
pub mod repl;
pub mod ui;

pub use args::Cli;
pub use repl::Repl;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Claude Code - asistente de programación con IA
#[derive(Parser, Debug)]
#[command(name = "claude-code")]
#[command(author = "Anthropic")]
#[command(version = "0.1.0")]
#[command(about = "Implementación en Rust de alto rendimiento de Claude Code CLI")]
#[command(disable_version_flag = true)]
#[command(disable_help_subcommand = true)]
pub struct CliArgs {
    /// Ruta del directorio del proyecto
    #[arg(short, long, value_name = "PATH")]
    pub path: Option<PathBuf>,

    /// Modelo a usar (sonnet, opus, haiku)
    #[arg(short, long, default_value = "sonnet")]
    pub model: String,

    /// Activar registro detallado
    #[arg(short, long)]
    pub verbose: bool,

    /// Ejecutar en modo no interactivo
    #[arg(short, long)]
    pub no_interactive: bool,

    /// Mostrar información de versión
    #[arg(long)]
    pub version: bool,

    /// Mostrar información del sistema
    #[arg(long)]
    pub info: bool,

    /// Subcomandos
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Iniciar una sesión REPL interactiva
    Repl {
        /// Prompt inicial para enviar
        #[arg(short, long)]
        prompt: Option<String>,
    },

    /// Ejecutar una consulta única
    Query {
        /// Consulta a ejecutar
        #[arg(short, long)]
        prompt: String,
    },

    /// Gestionar la configuración
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },

    /// Gestionar servidores MCP
    Mcp {
        #[command(subcommand)]
        action: McpCommands,
    },

    /// Gestionar plugins
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },

    /// Gestionar memoria y sesiones
    Memory {
        #[command(subcommand)]
        action: MemoryCommands,
    },

    /// Modo de entrada por voz
    Voice {
        /// Activar modo pulsar para hablar
        #[arg(short, long)]
        push_to_talk: bool,
    },

    /// Inicializar un proyecto nuevo
    Init {
        /// Nombre del proyecto
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Actualizar a la última versión
    Update,

    /// Mostrar ayuda e información de uso
    Help {
        /// Tema sobre el que mostrar ayuda
        #[arg(short, long)]
        topic: Option<String>,
    },

    /// Gestionar servicios en segundo plano
    Services {
        #[command(subcommand)]
        action: ServiceCommands,
    },

    /// Ejecutar un agente
    Agent {
        /// Tipo de agente (guide, explore, plan, verify, general)
        #[arg(short, long)]
        agent_type: String,
        /// Prompt para el agente
        #[arg(short, long)]
        prompt: String,
    },

    /// Gestionar Magic Docs
    MagicDocs {
        #[command(subcommand)]
        action: MagicDocsCommands,
    },

    /// Sincronización de memoria de equipo
    TeamSync {
        #[command(subcommand)]
        action: TeamSyncCommands,
    },

    /// Gestionar habilidades
    Skills {
        #[command(subcommand)]
        action: SkillsCommands,
    },

    /// Ejecutar pruebas de estrés
    StressTest {
        /// Número de solicitudes concurrentes
        #[arg(short, long, default_value = "5")]
        concurrency: usize,
        /// Número de iteraciones por solicitud
        #[arg(short, long, default_value = "10")]
        iterations: usize,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Mostrar configuración actual
    Show,

    /// Establecer un valor de configuración
    Set {
        /// Clave de configuración
        key: String,
        /// Valor de configuración
        value: String,
    },

    /// Restablecer configuración predeterminada
    Reset,
}

#[derive(Subcommand, Debug)]
pub enum McpCommands {
    /// Listar servidores MCP configurados
    List,

    /// Agregar un servidor MCP nuevo
    Add {
        /// Nombre del servidor (ej. filesystem)
        name: String,
        /// Server command (可选，filesystem 可只用 --path)
        command: Option<String>,
        /// Filesystem 专用路径（新增 --path / -p 支持）
        #[arg(long, short = 'p', value_name = "PATH")]
        path: Option<String>,
    },

    /// Eliminar un servidor MCP
    Remove {
        /// Server name
        name: String,
    },

    /// Reiniciar un servidor MCP
    Restart {
        /// Server name
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum PluginCommands {
    /// Listar plugins instalados
    List,

    /// Instalar un plugin
    Install {
        /// Nombre o URL del plugin
        plugin: String,
    },

    /// Eliminar un plugin
    Remove {
        /// Nombre del plugin
        name: String,
    },

    /// Actualizar todos los plugins
    Update,

    /// Buscar plugins
    Search {
        /// Consulta de búsqueda
        query: String,
    },

    /// Activar un plugin
    Enable {
        /// Nombre del plugin
        name: String,
    },

    /// Desactivar un plugin
    Disable {
        /// Nombre del plugin
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum MemoryCommands {
    /// Mostrar estado de memoria
    Status,

    /// Borrar todas las memorias
    Clear,

    /// Exportar memorias
    Export {
        /// Ruta del archivo de salida
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Importar memorias
    Import {
        /// Ruta del archivo de entrada
        input: PathBuf,
    },

    /// Ejecutar consolidación de memoria (dream)
    Dream,

    /// Forzar consolidación AutoDream
    AutoDream,
}

#[derive(Subcommand, Debug)]
pub enum ServiceCommands {
    /// Mostrar estado de todos los servicios
    Status,

    /// Iniciar todos los servicios
    Start,

    /// Detener todos los servicios
    Stop,

    /// Comprobar estado de AutoDream
    AutoDream,

    /// Comprobar estado de Voz
    Voice,

    /// Comprobar estado de Magic Docs
    MagicDocs,

    /// Comprobar estado de Team Sync
    TeamSync,

    /// Comprobar estado de Plugins
    Plugins,

    /// Comprobar estado de Agentes
    Agents,
}

#[derive(Subcommand, Debug)]
pub enum MagicDocsCommands {
    /// Listar Magic Docs rastreados
    List,

    /// Comprobar encabezado Magic Doc en un archivo
    Check {
        /// Ruta del archivo a comprobar
        file: String,
    },

    /// Actualizar un Magic Doc
    Update {
        /// Ruta del archivo a actualizar
        file: String,
        /// Contexto para la actualización
        #[arg(short, long)]
        context: Option<String>,
    },

    /// Borrar todos los Magic Docs rastreados
    Clear,
}

#[derive(Subcommand, Debug)]
pub enum TeamSyncCommands {
    /// Mostrar estado de sincronización
    Status,

    /// Autenticarse con el equipo
    Auth {
        /// ID del equipo
        team_id: String,
    },

    /// Sincronizar memorias
    Sync,

    /// Listar memorias del equipo
    List,

    /// Crear una memoria de equipo
    Create {
        /// Título de la memoria
        title: String,
        /// Contenido de la memoria
        #[arg(short, long)]
        content: String,
        /// Etiquetas (separadas por comas)
        #[arg(short, long)]
        tags: Option<String>,
    },

    /// Eliminar una memoria de equipo
    Delete {
        /// ID de memoria
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum SkillsCommands {
    /// Listar todas las habilidades disponibles
    List,

    /// Ejecutar una habilidad
    Execute {
        /// Nombre de la habilidad
        skill: String,
        /// Argumentos para la habilidad
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Obtener ayuda de una habilidad
    Help {
        /// Nombre de la habilidad
        skill: String,
    },

    /// Buscar habilidades
    Search {
        /// Consulta de búsqueda
        query: String,
    },
}
