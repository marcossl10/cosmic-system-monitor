use cosmic::iced::{Alignment, Task, font};
use cosmic::iced::window::{self, Id};
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup};
use cosmic::cosmic_config::{ConfigGet, ConfigSet};
use cosmic::prelude::*;
use cosmic::widget;
use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind, Networks, Components, Disks};
use crate::config::Config;
use std::time::Duration;
use cosmic::iced::time;
use std::fs;
use std::sync::LazyLock;

// NVML para NVIDIA
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
static NVML: LazyLock<Result<nvml_wrapper::Nvml, nvml_wrapper::error::NvmlError>> = LazyLock::new(nvml_wrapper::Nvml::init);

// Temperatura GPU via NVML
fn get_nvidia_temp() -> Option<f32> {
    if let Ok(nvml) = &*NVML {
        if let Some(pci_slot) = get_nvidia_pci_slot() {
            if let Ok(device) = nvml.device_by_pci_bus_id(pci_slot.as_str()) {
                if let Ok(temp) = device.temperature(TemperatureSensor::Gpu) {
                    return Some(temp as f32);
                }
            }
        }
        if let Ok(device) = nvml.device_by_index(0) {
            if let Ok(temp) = device.temperature(TemperatureSensor::Gpu) {
                return Some(temp as f32);
            }
        }
    }
    None
}

pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    config: Config,
    #[allow(dead_code)]
    config_handler: cosmic::cosmic_config::Config,
    system: System,
    networks: Networks,
    components: Components,
    disks: Disks,
    cpu_usage: f32,
    ram_usage: f32,
    gpu_usage: f32,
    cpu_temp: f32,
    gpu_temp: f32,
    gpu_vram_used: u64,
    gpu_vram_total: u64,
    download_speed: String,
    upload_speed: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick,
    TogglePopup,
    PopupClosed(Id),
    ToggleCpu,
    ToggleCpuTemp,
    ToggleRam,
    ToggleGpu,
    ToggleGpuTemp,
    ToggleGpuVram,
    ToggleDisk,
    ToggleDiskSpace,
    ToggleNet,
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "io.github.marcos.SysMonitor";

    fn core(&self) -> &cosmic::Core { &self.core }
    fn core_mut(&mut self) -> &mut cosmic::Core { &mut self.core }

    fn init(core: cosmic::Core, _flags: Self::Flags) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let config_handler = cosmic::cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
            .unwrap_or_else(|_| cosmic::cosmic_config::Config::new(Self::APP_ID, 0).unwrap());
        let config = config_handler.get("config").unwrap_or_default();

        let mut system = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        system.refresh_all();

        let networks = Networks::new_with_refreshed_list();
        let components = Components::new_with_refreshed_list();
        let disks = Disks::new_with_refreshed_list();

        let app = AppModel {
            core,
            popup: None,
            config,
            config_handler,
            system,
            networks,
            components,
            disks,
            cpu_usage: 0.0,
            ram_usage: 0.0,
            gpu_usage: 0.0,
            cpu_temp: 0.0,
            gpu_temp: 0.0,
            gpu_vram_used: 0,
            gpu_vram_total: 0,
            download_speed: "0 B/s".to_string(),
            upload_speed: "0 B/s".to_string(),
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(self.config.update_interval_ms)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::Tick => {
                self.system.refresh_cpu_all();
                self.system.refresh_memory();
                self.networks.refresh(true);
                self.components.refresh(true);
                self.disks.refresh(true);
                
                self.cpu_usage = self.system.global_cpu_usage();
                self.ram_usage = (self.system.used_memory() as f32 / self.system.total_memory() as f32) * 100.0;

                for component in &self.components {
                    let label = component.label();
                    let temp = component.temperature().unwrap_or(0.0);
                    
                    if label == "Tctl" || label.contains("CPU") || label.contains("Package id 0") {
                        self.cpu_temp = temp;
                    } else if label == "edge" {
                        self.gpu_temp = temp;
                    } else if label.contains("nvidia") {
                        self.gpu_temp = temp;
                    }
                }

                self.gpu_usage = read_gpu_usage().unwrap_or(0.0);
                
                if let Some((used, total)) = read_gpu_vram() {
                    self.gpu_vram_used = used;
                    self.gpu_vram_total = total;
                }
                
                if self.gpu_temp == 0.0 {
                    if let Some(temp) = get_nvidia_temp() {
                        self.gpu_temp = temp;
                    }
                }

                let mut total_down = 0;
                let mut total_up = 0;
                for (_, data) in &self.networks {
                    total_down += data.received();
                    total_up += data.transmitted();
                }
                
                self.download_speed = format_speed(total_down);
                self.upload_speed = format_speed(total_up);
                
                Task::none()
            }
            Message::TogglePopup => {
                if let Some(p) = self.popup.take() {
                    return destroy_popup(p);
                } else {
                    let new_id = window::Id::unique();
                    self.popup.replace(new_id);

                    let popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        Some((1, 1)),
                        None,
                        None,
                    );
                    return get_popup(popup_settings);
                }
            }
            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
                Task::none()
            }
            Message::ToggleCpu | Message::ToggleCpuTemp | Message::ToggleRam | Message::ToggleGpu | Message::ToggleGpuTemp | Message::ToggleGpuVram | Message::ToggleDisk | Message::ToggleDiskSpace | Message::ToggleNet => {
                match message {
                    Message::ToggleCpu => self.config.show_cpu = !self.config.show_cpu,
                    Message::ToggleCpuTemp => self.config.show_cpu_temp = !self.config.show_cpu_temp,
                    Message::ToggleRam => self.config.show_ram = !self.config.show_ram,
                    Message::ToggleGpu => self.config.show_gpu = !self.config.show_gpu,
                    Message::ToggleGpuTemp => self.config.show_gpu_temp = !self.config.show_gpu_temp,
                    Message::ToggleGpuVram => self.config.show_gpu_vram = !self.config.show_gpu_vram,
                    Message::ToggleDisk => self.config.show_disk = !self.config.show_disk,
                    Message::ToggleDiskSpace => self.config.show_disk_space = !self.config.show_disk_space,
                    Message::ToggleNet => self.config.show_net = !self.config.show_net,
                    _ => {}
                }
                let _ = self.config_handler.set("config", &self.config);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let text_size = 12;
        let bold_font = font::Font { weight: font::Weight::Bold, ..Default::default() };
        let config = &self.config;
        
        let mut row_children: Vec<Element<'_, Message>> = Vec::new();
        
        if config.show_cpu {
            let cpu_info = if config.show_cpu_temp {
                format!("{:.0}%|{:.0}°C", self.cpu_usage, self.cpu_temp)
            } else {
                format!("{:.0}%", self.cpu_usage)
            };
            row_children.push(
                widget::row(vec![
                    widget::text("CPU ").size(text_size).font(bold_font).into(),
                    widget::text(cpu_info).size(text_size).into(),
                ]).spacing(0).align_y(Alignment::Center).into()
            );
            row_children.push(widget::text(" | ").size(text_size).into());
        }
        
        if config.show_ram {
            let ram_info = if self.system.total_memory() > 0 {
                let used_gb = self.system.used_memory() as f64 / 1024.0 / 1024.0;
                let total_gb = self.system.total_memory() as f64 / 1024.0 / 1024.0;
                format!("{:.0}% | {:.1}/{:.1} GB", self.ram_usage, used_gb, total_gb)
            } else {
                format!("{:.0}%", self.ram_usage)
            };
            row_children.push(
                widget::row(vec![
                    widget::text("Ram ").size(text_size).font(bold_font).into(),
                    widget::text(ram_info).size(text_size).into(),
                ]).spacing(0).align_y(Alignment::Center).into()
            );
            row_children.push(widget::text(" | ").size(text_size).into());
        }
        
        if config.show_gpu {
            let gpu_info = if self.gpu_usage > 0.0 || self.cpu_temp > 0.0 {
                let mut info = format!("{:.0}%", self.gpu_usage);
                if config.show_gpu_temp && self.gpu_temp > 0.0 {
                    info = format!("{} |{:.0}°C", info, self.gpu_temp);
                }
                if config.show_gpu_vram && self.gpu_vram_total > 0 {
                    let used = self.gpu_vram_used / 1024 / 1024;
                    let total = self.gpu_vram_total / 1024 / 1024;
                    info = format!("{}|{}/{}GB", info, used, total);
                }
                info
            } else {
                format!("{:.0}%", self.gpu_usage)
            };
            row_children.push(
                widget::row(vec![
                    widget::text("GPU ").size(text_size).font(bold_font).into(),
                    widget::text(gpu_info).size(text_size).into(),
                ]).spacing(0).align_y(Alignment::Center).into()
            );
            row_children.push(widget::text(" | ").size(text_size).into());
        }
        
        if config.show_disk {
            let disk_info = {
                let mut disk_str = String::new();
                for d in &self.disks {
                    let mount = d.mount_point().to_string_lossy().to_string();
                    if mount == "/" || mount.starts_with("/home") {
                        let total = d.total_space() / 1024 / 1024 / 1024;
                        let available = d.available_space() / 1024 / 1024 / 1024;
                        let used = total - available;
                        let usage = if total > 0 { (used as f32 / total as f32) * 100.0 } else { 0.0 };
                        disk_str = if config.show_disk_space {
                            format!("{usage:.0}%| {used}/{total} GB")
                        } else {
                            format!("{usage:.0}%")
                        };
                        break;
                    }
                }
                if disk_str.is_empty() { "N/A".to_string() } else { disk_str }
            };
            row_children.push(
                widget::row(vec![
                    widget::text("DISK ").size(text_size).font(bold_font).into(),
                    widget::text(disk_info).size(text_size).into(),
                ]).spacing(0).align_y(Alignment::Center).into()
            );
            row_children.push(widget::text(" | ").size(text_size).into());
        }
        
        if config.show_net {
            let net_info = format!("↓{} ↑{}", self.download_speed, self.upload_speed);
            row_children.push(
                widget::row(vec![
                    widget::text("NET ").size(text_size).font(bold_font).into(),
                    widget::text(net_info).size(text_size).into(),
                ]).spacing(0).align_y(Alignment::Center).into()
            );
        }
        
        let content = widget::row(row_children).spacing(0).align_y(Alignment::Center);
        let main_btn = widget::button::custom(content)
            .on_press(Message::TogglePopup)
            .class(cosmic::theme::Button::AppletIcon);
        
        widget::autosize::autosize(main_btn, widget::Id::unique()).into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let config = &self.config;
        let bold_font = font::Font { weight: font::Weight::Bold, ..Default::default() };
        
        let header = widget::row(vec![
            widget::text("Configurações").size(16).font(bold_font).into(),
            widget::space().into(),
        ]);
        
        let cpu_row = widget::row(vec![
            widget::text("CPU").size(13).into(),
            widget::toggler(config.show_cpu).on_toggle(move |_| Message::ToggleCpu).into(),
        ]).spacing(8);
        
        let cpu_temp_row = widget::row(vec![
            widget::text("  CPU Temp").size(13).into(),
            widget::toggler(config.show_cpu_temp).on_toggle(move |_| Message::ToggleCpuTemp).into(),
        ]).spacing(8);
        
        let ram_row = widget::row(vec![
            widget::text("RAM").size(13).into(),
            widget::toggler(config.show_ram).on_toggle(move |_| Message::ToggleRam).into(),
        ]).spacing(8);
        
        let gpu_row = widget::row(vec![
            widget::text("GPU").size(13).into(),
            widget::toggler(config.show_gpu).on_toggle(move |_| Message::ToggleGpu).into(),
        ]).spacing(8);
        
        let gpu_temp_row = widget::row(vec![
            widget::text("  GPU Temp").size(13).into(),
            widget::toggler(config.show_gpu_temp).on_toggle(move |_| Message::ToggleGpuTemp).into(),
        ]).spacing(8);
        
        let gpu_vram_row = widget::row(vec![
            widget::text("  GPU VRAM").size(13).into(),
            widget::toggler(config.show_gpu_vram).on_toggle(move |_| Message::ToggleGpuVram).into(),
        ]).spacing(8);
        
        let disk_row = widget::row(vec![
            widget::text("Disk").size(13).into(),
            widget::toggler(config.show_disk).on_toggle(move |_| Message::ToggleDisk).into(),
        ]).spacing(8);

        let disk_space_row = widget::row(vec![
            widget::text("  Disk Space").size(13).into(),
            widget::toggler(config.show_disk_space).on_toggle(move |_| Message::ToggleDiskSpace).into(),
        ]).spacing(8);
        
        let net_row = widget::row(vec![
            widget::text("Net").size(13).into(),
            widget::toggler(config.show_net).on_toggle(move |_| Message::ToggleNet).into(),
        ]).spacing(8);
        
        let content = widget::column(vec![
            header.into(),
            cpu_row.into(),
            cpu_temp_row.into(),
            ram_row.into(),
            gpu_row.into(),
            gpu_temp_row.into(),
            gpu_vram_row.into(),
            disk_row.into(),
            disk_space_row.into(),
            net_row.into(),
        ])
            .spacing(12)
            .padding(16);
        
        self.core.applet.popup_container(content).into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

fn format_speed(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B/s", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB/s", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB/s", bytes as f64 / 1024.0 / 1024.0)
    }
}

fn read_gpu_usage() -> Option<f32> {
    if let Ok(nvml) = &*NVML {
        if let Some(pci_slot) = get_nvidia_pci_slot() {
            if let Ok(device) = nvml.device_by_pci_bus_id(pci_slot.as_str()) {
                if let Ok(util) = device.utilization_rates() {
                    return Some(u64::from(util.gpu) as f32);
                }
            }
        }
        if let Ok(device) = nvml.device_by_index(0) {
            if let Ok(util) = device.utilization_rates() {
                return Some(u64::from(util.gpu) as f32);
            }
        }
    }
    
    for card in 0..=1 {
        let path = format!("/sys/class/drm/card{}/device/gpu_busy_percent", card);
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(usage) = content.trim().parse::<f32>() {
                return Some(usage);
            }
        }
    }
    
    None
}

fn read_gpu_vram() -> Option<(u64, u64)> {
    if let Ok(nvml) = &*NVML {
        if let Some(pci_slot) = get_nvidia_pci_slot() {
            if let Ok(device) = nvml.device_by_pci_bus_id(pci_slot.as_str()) {
                if let Ok(mem) = device.memory_info() {
                    return Some((mem.used, mem.total));
                }
            }
        }
        if let Ok(device) = nvml.device_by_index(0) {
            if let Ok(mem) = device.memory_info() {
                return Some((mem.used, mem.total));
            }
        }
    }
    
    for card in 0..=1 {
        let used_path = format!("/sys/class/drm/card{}/device/mem_info_vram_used", card);
        let total_path = format!("/sys/class/drm/card{}/device/mem_info_vram_total", card);
        
        if let (Ok(used_str), Ok(total_str)) = (fs::read_to_string(&used_path), fs::read_to_string(&total_path)) {
            if let (Ok(used), Ok(total)) = (used_str.trim().parse::<u64>(), total_str.trim().parse::<u64>()) {
                if total > 0 {
                    return Some((used, total));
                }
            }
        }
    }
    
    None
}

fn get_nvidia_pci_slot() -> Option<String> {
    for card in 0..=1 {
        let uevent_path = format!("/sys/class/drm/card{}/device/uevent", card);
        if let Ok(content) = fs::read_to_string(&uevent_path) {
            for line in content.lines() {
                if line.starts_with("PCI_SLOT_NAME=") {
                    return Some(line.strip_prefix("PCI_SLOT_NAME=")?.to_string());
                }
            }
        }
    }
    None
}
