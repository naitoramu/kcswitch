#!/bin/bash

echo -e "\nBuilding KCSwitch"
if cargo build --release; then
	echo "Package built successfully"
else
	echo "Cannot build package"
fi

echo -e "\nCreating binary link"
SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
target_symlink="$SCRIPTPATH/target/release/kcswitch"
source_symlink="$HOME/.local/bin/kcswitch"
if ln -s $target_symlink $source_symlink; then 
	echo "Symlink $source_symlink -> $target_symlink created"
else
	echo "Symlink not created. File already exists or you need to add binary file to your PATH manually."
fi

echo -e "\nExporting KUBECONFIG variable"
current_shell=$(basename "$SHELL")
kubeconfig_path="$HOME/.kcswitch/.kubeconfig"
var="KUBECONFIG=${kubeconfig_path}"

case "$current_shell" in
  bash)
    config_file="$HOME/.bashrc"
    ;;
  zsh)
    config_file="$HOME/.zshrc"
    ;;
  ksh)
    config_file="$HOME/.kshrc"
    ;;
  *)
    echo "Unsupported shell: $current_shell"
    echo "You need to export variable '$var' manually"
    exit 1
    ;;
esac

if ! grep -q "export $var" "$config_file"; then
	echo -e "\n# KCSwitch configuration" >> "$config_file"
	echo "export $var" >> "$config_file"
	echo "Variables exported to $config_file"
else
	echo "KUBECONFIG variable already set in '$config_file'."
fi
