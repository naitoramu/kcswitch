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
To build package run a command below
```bash
cargo build --release
```
After that in `target/release/` directory should appear executable `kcswitch`. Add it to your PATH. The last step is to export variable `KUBECONFIG` to point to `$HOME/.kcswitch/.kubeconfig`

---
## Usage
```bash
╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch
No kubeconfigs defined.

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch add .kubeconfig.yaml --name kc1
Kubeconfig 'kc1' successfully added.

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch add .kubeconfig.yaml --name kc2
Kubeconfig 'kc2' successfully added.

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch
   kc2
   kc1

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch kc1

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch
-> kc1
   kc2

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch delete kc2

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch
-> kc1

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch delete kc1

╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch
No kubeconfigs defined.
```

Help
```bash
╭─ ~ ──────────────────────────────────────────────────────────────────────────────────────── Py base
╰─❯ kcswitch help
Switch easily between different kubernetes clusters

Usage: kcswitch [KUBECONFIG] [COMMAND]

Commands:
  add     Add new kubeconfig
  delete
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [KUBECONFIG]  Kubeconfig name you want to switch to

Options:
  -h, --help     Print help
  -V, --version  Print version
```