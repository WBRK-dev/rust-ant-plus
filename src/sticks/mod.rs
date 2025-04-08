use std::{sync::{Arc, Mutex}, time::Duration};

use rusb::{Device, DeviceHandle, GlobalContext};
use crate::{constants::{MESSAGE_CAPABILITIES, MESSAGE_CHANNEL_EVENT, MESSAGE_NETWORK_KEY, MESSAGE_STARTUP}, events::EventEmitter, messages::Messages};

pub mod garmin_stick_3;

pub struct Driver {
    events: EventEmitter,
    id_vendor: u16,
    id_product: u16,
    device: Arc<Mutex<Option<DeviceHandle<GlobalContext>>>>,
    max_channels: usize,
    can_scan: bool,
}

impl Driver {
    pub fn new(id_vendor: u16, id_product: u16) -> Self {
        Driver {
            events: EventEmitter::new(),
            id_vendor,
            id_product,
            device: Arc::new(Mutex::new(None)),
            max_channels: 0,
            can_scan: false,
        }
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

    pub fn open(&mut self) -> bool {
        let devices = self.get_devices();
        for device in devices {
            match device.open() {
                Ok(handle) => {
                    let _ = handle.claim_interface(0);

                    println!("Device opened successfully");

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

    pub fn close(&mut self) {
        let mut device = self.device.lock().unwrap();
        if let Some(handle) = device.take() {
            let _ = handle.release_interface(0);
            self.events.emit("shutdown", None);
        }
    }

    pub fn write(&self, data: Vec<u8>) {
        if let Some(ref handle) = *self.device.lock().unwrap() {
            let _ = handle.write_bulk(0x01, &data, Duration::from_millis(100));
        }
    }

    pub fn read(&mut self, data: &Vec<u8>) {
        if data.len() < 3 {
            return;
        }
        let message_id = data[2];
        match message_id {
            MESSAGE_STARTUP => {
                self.write(Messages::request_message(0, MESSAGE_CAPABILITIES));
            }
            MESSAGE_CAPABILITIES => {
                self.max_channels = data[3] as usize;
                self.can_scan = (data[7] & 0x06) == 0x06;
                self.write(Messages::set_network_key());
            }
            MESSAGE_CHANNEL_EVENT if data[4] == MESSAGE_NETWORK_KEY => {
                let _ = self.events.emit("startup", Some(data));
            }
            _ => {
                let _ = self.events.emit("read", Some(data));
            }
        }
    }

    pub fn on<F>(&mut self, event: &str, callback: F)
    where
        F: Fn(Option<&Vec<u8>>) + Send + Sync + 'static,
    {
        self.events.on(event, callback)
    }
}