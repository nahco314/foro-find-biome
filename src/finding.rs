use alloc::fmt::format;
use std::path::PathBuf;

// if we can use musl binary, we should use it (it's a little bit faster)
const BIOME_TARGET_NAMES: [&str; 8] = [
    "win32-arm64", "darwin-arm64", "linux-arm64-musl", "linux-arm64", "win32-x64", "darwin-x64", "linux-x64-musl", "linux-x64"
];

fn pick_biome_in_node_modules(node_modules: &PathBuf) -> Option<PathBuf> {
    let mut node_modules = node_modules.clone();

    node_modules.push("@biomejs");
    if !node_modules.exists() {
        return None;
    }

    for target in BIOME_TARGET_NAMES.iter() {
        let cli_name = format!("cli-{}", target);

        let mut cli_path = node_modules.clone();
        cli_path.push(cli_name);
        cli_path.push("biome");

        if cli_path.exists() {
            return Some(cli_path);
        }
    }

    None
}

pub fn find_biome(current_dir: &PathBuf) -> Option<PathBuf> {
    let mut current_dir = current_dir.clone();

    loop {
        let node_modules = current_dir.join("node_modules");

        if node_modules.exists() {
            if let Some(biome) = pick_biome_in_node_modules(&node_modules) {
                return Some(biome);
            }
        }

        if !current_dir.pop() {
            break;
        }
    }

    None
}
