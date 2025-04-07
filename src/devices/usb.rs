use rusb::{Device, DeviceHandle, GlobalContext, UsbContext, Direction, TransferType};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use bytes::{BytesMut, Buf};
use std::time::Duration;
use crate::messages::Messages;
use crate::constants::*;

type Buffer = Vec<u8>;

#[derive(Clone)]
pub struct USBDriver {
    id_vendor: u16,
    id_product: u16,
    max_channels: usize,
    can_scan: bool,
    used_channels: isize,
    device: Arc<Mutex<Option<DeviceHandle<GlobalContext>>>>,
    attached_sensors: Arc<Mutex<Vec<Box<dyn BaseSensor>>>>,
    event_sender: UnboundedSender<USBEvent>,
}

pub trait BaseSensor: Send + Sync {
    fn detach(&self);
}

#[derive(Debug)]
pub enum USBEvent {
    Read(Buffer),
    Startup(Buffer),
    Shutdown,
}

impl USBDriver {
    pub fn new(id_vendor: u16, id_product: u16) -> (Self, UnboundedReceiver<USBEvent>) {
        let (tx, rx) = unbounded_channel();
        (
            USBDriver {
                id_vendor,
                id_product,
                max_channels: 0,
                can_scan: false,
                used_channels: 0,
                device: Arc::new(Mutex::new(None)),
                attached_sensors: Arc::new(Mutex::new(Vec::new())),
                event_sender: tx,
            },
            rx,
        )
    }

    fn get_devices(&self) -> Vec<Device<GlobalContext>> {
        rusb::devices()
            .unwrap()
            .iter()
            .filter(|d| {
                let desc = d.device_descriptor().unwrap();
                desc.vendor_id() == self.id_vendor && desc.product_id() == self.id_product
            })
            .collect()
    }

    pub fn is_present(&self) -> bool {
        !self.get_devices().is_empty()
    }

    pub fn open(&self) -> bool {
        let devices = self.get_devices();
        for device in devices {
            match device.open() {
                Ok(handle) => {
                    println!("Device opened successfully");
                    let iface = 0;
                    let _ = handle.claim_interface(iface);
                    let mut dev_lock = self.device.lock().unwrap();
                    *dev_lock = Some(handle);
                    return true;
                }
                Err(err) => {
                    println!("Failed to open device: {}", err);
                    continue;
                },
            }
        }
        false
    }

    pub fn close(&self) {
        let mut device = self.device.lock().unwrap();
        if let Some(mut handle) = device.take() {
            let _ = handle.release_interface(0);
            let _ = self.event_sender.send(USBEvent::Shutdown);
        }
    }

    pub fn reset(&mut self) {
        self.detach_all();
        self.max_channels = 0;
        self.used_channels = 0;
        self.write(Messages::reset_system());
    }

    pub fn is_scanning(&self) -> bool {
        self.used_channels == -1
    }

    pub fn attach(&mut self, sensor: Box<dyn BaseSensor>, for_scan: bool) -> bool {
        if self.used_channels < 0 {
            return false;
        }
        if for_scan {
            if self.used_channels != 0 {
                return false;
            }
            self.used_channels = -1;
        } else {
            if self.max_channels <= self.used_channels as usize {
                return false;
            }
            self.used_channels += 1;
        }
        self.attached_sensors.lock().unwrap().push(sensor);
        true
    }

    pub fn detach(&mut self, sensor_ptr: *const dyn BaseSensor) -> bool {
        let mut sensors = self.attached_sensors.lock().unwrap();
        if let Some(pos) = sensors.iter().position(|s| &**s as *const _ == sensor_ptr) {
            sensors.remove(pos);
            if self.used_channels < 0 {
                self.used_channels = 0;
            } else {
                self.used_channels -= 1;
            }
            return true;
        }
        false
    }

    pub fn detach_all(&mut self) {
        let mut sensors = self.attached_sensors.lock().unwrap();
        for sensor in sensors.iter() {
            sensor.detach();
        }
        sensors.clear();
    }

    pub fn write(&self, data: Buffer) {
        if let Some(ref handle) = *self.device.lock().unwrap() {
            // Simulate bulk write
            let _ = handle.write_bulk(0x01, &data, Duration::from_millis(100));
        }
    }

    pub fn read(&mut self, data: Buffer) {
        if data.len() < 3 {
            return;
        }
        let message_id = data[2];
        match message_id {
            Constants::MESSAGE_STARTUP => {
                self.write(Messages::request_message(0, Constants::MESSAGE_CAPABILITIES));
            }
            Constants::MESSAGE_CAPABILITIES => {
                self.max_channels = data[3] as usize;
                self.can_scan = (data[7] & 0x06) == 0x06;
                self.write(Messages::set_network_key());
            }
            Constants::MESSAGE_CHANNEL_EVENT if data[4] == Constants::MESSAGE_NETWORK_KEY => {
                let _ = self.event_sender.send(USBEvent::Startup(data));
            }
            _ => {
                let _ = self.event_sender.send(USBEvent::Read(data));
            }
        }
    }
}
