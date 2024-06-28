# Kubeconfig Switch
Simple Rust cli application for easily switching between Kubernetes config files.

---
## Install
### Install via script
The easiest way to install kcswitch is to run script `install.sh`. Remember to add executable permission to the script if necessary.
```bash
chmod +x install.sh
./install.sh
```

---
### Manual installation
#### Build package
To install program manually run commands below in the cloned repository directory
```bash
make build
make install
make clean
```

After that in `$HOME/.local/bin` directory should appear executable `kcswitch`. To use tool globally add `$HOME/.local/bin` to your PATH (if not present yet). 
```bash
export PATH="$HOME/.local/bin:$PATH"
```

The last step is to export variable `KUBECONFIG` to point to `$HOME/.kcswitch/.kubeconfig`:
```bash
export KUBECONFIG="$HOME/.kcswitch/.kubeconfig"
```

---
## Usage

#### List available kubeconfigs and the selected one
```bash
╭─ ~ ─────────────────────────────────────────────────────────
╰─❯ kcswitch
-> kc1
   kc2
```

#### Add new kubeconfig to list
```bash
╭─ ~ ─────────────────────────────────────────────────────────
╰─❯ kcswitch add .kubeconfig.yaml --name kc1
Kubeconfig 'kc1' successfully added.
```

Select kubeconfig
```bash
╭─ ~ ─────────────────────────────────────────────────────────
╰─❯ kcswitch kc1
```

Delete kubeconfig from list
```bash
╭─ ~ ─────────────────────────────────────────────────────────
╰─❯ kcswitch delete kc2
```

Help
```bash
╭─ ~ ─────────────────────────────────────────────────────────
╰─❯ kcswitch help
Switch easily between different kubernetes clusters

Usage: kcswitch [KUBECONFIG] [COMMAND]

Commands:
  add     Add new kubeconfig
  delete  Delete kubeconfig
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [KUBECONFIG]  Kubeconfig name you want to switch to

Options:
  -h, --help     Print help
  -V, --version  Print version
```