#!/bin/bash

NC='\033[0m'
RED='\033[0;31m'
GREEN='\033[0;32m'

print_success() {
	echo -e "${GREEN}${*}${NC}"
}

print_error() {
	echo -e "${RED}${*}${NC}"
}

# Making package
echo -e "\nBuilding package"
if make build; then
	print_success "Package built successfully"
else
	print_error "Cannot build package"
	exit 1
fi

echo -e "\nInstalling package"
if make install; then
	print_success "Package installed successfully"
else
	print_error "Cannot install package"
	exit 1
fi

echo -e "\nCleaning build artifacts"
if make clean; then
	print_success "Artifacts cleaned successfully"
else
	print_error "Cannot clean artifacts"
fi

# Detecting shell
current_shell=$(basename "$SHELL")
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
    print_error "Unsupported shell: $current_shell"
    exit 1
    ;;
esac

config_header="# Generated for KCSwitch. Do not remove"
if ! grep -q "${config_header}" "$config_file"; then
  echo -e "\n${config_header}" >> "$config_file"
fi

# Adding ~/.local/bin to directory
echo -e "\nAdding $HOME/.local/bin directory to PATH"
if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
	if echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$config_file"; then
	  print_success "$HOME/.local/bin directory successfully added to PATH"
	else
	  print_error "Cannot add $HOME/.local/bin directory to PATH"
	fi
else
	print_success "$HOME/.local/bin directory already in PATH"
fi

# Exporting KUBECONFIG variable
echo -e "\nExporting KUBECONFIG variable"
kubeconfig_path="$HOME/.kcswitch/.kubeconfig"
var="KUBECONFIG=${kubeconfig_path}"
if ! grep -q "export $var" "$config_file"; then
	if echo "export $var" >> "$config_file"; then
	  print_success "Variables exported to $config_file"
	else
	  print_error "KUBECONFIG variable not exported"
	fi
else
	print_success "KUBECONFIG variable already set in '$config_file'."
fi
