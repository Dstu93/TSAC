
use std::sync::mpsc::{channel,Sender,Receiver};

pub struct USBDeviceWatcher {
    vid: u16,
    pid: u16,
    is_connected: bool,
    event_channel: Sender<USBEvent>,
}

impl USBDeviceWatcher {

    pub fn new(vid: u16,pid: u16) -> (Self,Receiver<USBEvent>) {
        let (rx,tx) = channel();
        let watcher = USBDeviceWatcher{vid,pid,is_connected: false,event_channel: rx};
        (watcher,tx)
    }

    /// listens for usb device. This function blocks
    pub fn listen(self) -> Result<(),rusb::Error> {
        loop {
            let device_list = rusb::devices()?;
            for device in device_list.iter() {
                let descriptor = device.device_descriptor()?;
                let vid = descriptor.vendor_id();
                let pid = descriptor.product_id();
                if vid == self.vid && pid == self.pid {

                }
            }
        }
    }
}


pub enum USBEvent{
    DeviceConnected,
    DeviceDisconnected,
}