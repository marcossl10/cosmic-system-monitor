# COSMIC System Monitor Applet

<img src="resources/preview.png" width="400" alt="Preview">

Monitor de sistema limpo e leve para o COSMIC Desktop.

## Instalação Rápida

```bash
git clone https://github.com/marcossl10/cosmic-system-monitor.git
cd cosmic-system-monitor
sudo just install
```

## Funcionalidades
- Uso e Temperatura de CPU, RAM e GPU
- Rede em tempo real (B/s, KB/s, MB/s)
- Visual nativo do sistema COSMIC

## Pré-requisitos

Antes de compilar e instalar, certifique-se de que você tem as seguintes dependências instaladas em seu sistema.

### Instalando Rust
O projeto é escrito em Rust. Se você não tem o Rust instalado, pode instalá-lo usando `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Instalando `just`
`just` é um executor de comandos que simplifica as tarefas de compilação e instalação.

**Debian/Ubuntu:**
```bash
sudo apt update
sudo apt install just
```

**Fedora/CentOS/RHEL:**
```bash
sudo dnf install just
```

### Bibliotecas de Desenvolvimento do Sistema
Estas bibliotecas são frequentemente necessárias para funcionalidades de monitoramento do sistema e de rede.

**Debian/Ubuntu:**
```bash
sudo apt update
sudo apt install build-essential libsensors-dev libgtk-3-dev libdbus-1-dev
```

**Fedora/CentOS/RHEL:**
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install lm_sensors-devel gtk3-devel dbus-devel
```

---
*Nota: Se a compilação falhar, verifique se você tem o `just`, `rust` e as bibliotecas de sistema instaladas.*
