use clap::{Parser, Subcommand};

/// Fabric Package Manager (fpm)
#[derive(Debug, Parser)]
#[command(
    name = "fpm",
    version,
    about = "A Fabric mod package manager powered by Modrinth",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize a new Fabric project
    Init {
        /// Minecraft version (e.g. 1.20.1)
        minecraft: String,
    },

    /// Search for Fabric mods on Modrinth
    Search {
        /// Search query
        query: String,
    },

    /// Add a mod to the project
    Add {
        /// Mod slug or project ID
        mod_name: String,

        /// Optional version constraint (e.g. ^0.5.8)
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Remove a mod from the project
    Remove {
        /// Mod slug or project ID
        mod_name: String,
    },

    /// List installed mods
    List,

    /// Resolve dependencies without installing
    Resolve,

}
