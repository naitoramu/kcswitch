use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;

use clap::Parser;

use crate::cli::{Cli, Command};

mod cli;

fn main() {
    let cli: Cli = Cli::parse();
    let kcdir: PathBuf = std::env::home_dir().unwrap().join(".kcswitch");
    let symlink: PathBuf = kcdir.join(".kubeconfig");

    let configs_dir: PathBuf = kcdir.join(".kubeconfigs");
    create_dir_all(&configs_dir).expect("Cannot create $HOME/.kubeconfigs directory");

    let available_kubeconfigs: HashMap<String, PathBuf> = load_available_kubeconfigs(&configs_dir);

    match &cli.command {
        Some(Command::Add { kubeconfig_file, name }) => {
            add_new_kubeconfig(kubeconfig_file, name, available_kubeconfigs, configs_dir);
        }
        Some(Command::Delete { kubeconfig }) => {
            delete_kubeconfig(kubeconfig, available_kubeconfigs, symlink);
        }
        None => switch_or_list_kubeconfigs(cli.kubeconfig, available_kubeconfigs, symlink)
    }
}

fn add_new_kubeconfig(file: &PathBuf, name: &Option<String>, available_kubeconfigs: HashMap<String, PathBuf>, configs_dir: PathBuf) {
    let kubeconfig_name: String = name.clone().unwrap_or_else(|| extract_filename(file));

    if available_kubeconfigs.contains_key(&kubeconfig_name) {
        println!("Kubeconfig named '{}' already exists. Specify other name for new kubeconfig and try again.", kubeconfig_name);
    } else {
        match std::fs::copy(file, configs_dir.join(PathBuf::from(kubeconfig_name.as_str().to_string() + ".yaml"))) {
            Ok(_) => println!("Kubeconfig '{kubeconfig_name}' successfully added."),
            Err(_) => println!("Unable to add kubeconfig '{kubeconfig_name}'.")
        };
    }
}

fn delete_kubeconfig(kubeconfig: &String, available_kubeconfigs: HashMap<String, PathBuf>, symlink: PathBuf) {
    if available_kubeconfigs.contains_key(kubeconfig) {
        let selected_kubeconfig = std::fs::read_link(&symlink).unwrap_or(PathBuf::from(""));
        let kubeconfig_file = PathBuf::from(available_kubeconfigs.get(kubeconfig).unwrap());
        if selected_kubeconfig.eq(&kubeconfig_file) {
            std::fs::remove_file(symlink).expect("Cannot deselect kubeconfig");
        }
        std::fs::remove_file(kubeconfig_file).expect("Cannot delete kubeconfig");
    } else {
        println!("Given kubeconfig '{kubeconfig}' does not exists.");
    }
}

fn switch_or_list_kubeconfigs(kubeconfig: Option<String>, available_kubeconfigs: HashMap<String, PathBuf>, symlink: PathBuf) {
    match kubeconfig {
        Some(kubeconfig_name) => switch_to_kubeconfig(kubeconfig_name, available_kubeconfigs, symlink),
        None => list_kubeconfigs(available_kubeconfigs, symlink)
    }
}

fn switch_to_kubeconfig(kubeconfig: String, available_kubeconfigs: HashMap<String, PathBuf>, symlink: PathBuf) {
    if available_kubeconfigs.contains_key(&kubeconfig) {
        std::fs::remove_file(&symlink).ok();

        let target_kubeconfig = available_kubeconfigs.get(&kubeconfig)
            .unwrap()
            .to_str()
            .expect("Filename not correct");

        std::os::unix::fs::symlink(target_kubeconfig, symlink)
            .expect("Unable to switch to kubeconfig");
    } else {
        println!("Given kubeconfig '{kubeconfig}' does not exists.");
    }
}

fn load_available_kubeconfigs(configs_dir: &PathBuf) -> HashMap<String, PathBuf> {
    let mut config_map: HashMap<String, PathBuf> = HashMap::new();
    let dir_content = std::fs::read_dir(configs_dir).expect("Cannot read directory content");
    for entry in dir_content {
        let path = entry.unwrap().path();
        if path.is_file() {
            config_map.insert(extract_filename(&path), path);
        }
    }
    return config_map;
}

fn extract_filename(path: &PathBuf) -> String {
    if let Some(stem) = path.file_stem() {
        if let Some(stem_str) = stem.to_str() {
            return stem_str.to_string();
        } else {
            panic!("File name is not valid UTF-8");
        }
    } else {
        panic!("No file name found");
    }
}

fn list_kubeconfigs(available_kubeconfigs: HashMap<String, PathBuf>, symlink: PathBuf) {
    if available_kubeconfigs.is_empty() {
        println!("No kubeconfigs defined.")
    } else {
        let selected_kubeconfig = std::fs::read_link(symlink).unwrap_or(PathBuf::from(""));
        for (name, path) in &available_kubeconfigs {
            if selected_kubeconfig.eq(path) {
                println!("-> {name}");
            } else {
                println!("   {name}");
            }
        }
    }
}
