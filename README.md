# Kubeconfig Switch

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

