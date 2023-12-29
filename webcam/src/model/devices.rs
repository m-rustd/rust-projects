use std::ops::Deref;

use tracing::info;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaDeviceKind, MediaDevices, MediaStreamConstraints, MediaDeviceInfo, console};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Devices(Vec<Device>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub kind: MediaDeviceKind,
    pub label: String,
    pub id: String,
}

impl Deref for Devices {
    type Target = Vec<Device>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Iterator for Devices {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl From<&JsValue> for Devices {
    fn from(value: &JsValue) -> Self {
        match js_sys::try_iter(value) {
            Ok(Some(v)) => {
                let devices = v.into_iter()
                  .filter(|item|item.is_ok())
                  .map(|v| Device::from(v.unwrap()))
                  .collect();
                Self(devices)
            },
            _ => Default::default(),
        }
    }
}

impl From<JsValue> for Device {
    fn from(value: JsValue) -> Self {
        let device = value.dyn_into::<MediaDeviceInfo>().unwrap();
        Device { 
          kind: device.kind(), 
          label: device.label(), 
          id: device.device_id()
        }
    }
}

impl Devices {
    pub async fn load() -> Self {
        console::log_1(&JsValue::from_str("load===1"));
        let devices = Self::get_media_devices().await;

        if let Ok(promise) = devices.enumerate_devices() {
            let all_devices = JsFuture::from(promise)
                .await
                .unwrap();
            return Self::from(&all_devices);
        }
        Default::default()
        // let all_devices = JsFuture::from(devices.enumerate_devices().unwrap()).await.unwrap();
        // Self::from(&all_devices)
    }

    // 获取媒体设备列表
    pub async fn get_media_devices() -> MediaDevices {
        info!("get_media_devices=====");
        console::log_1(&JsValue::from_str("get_media_devices===1"));
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let devices = navigator.media_devices().expect("no navigator.media_devices exists");
        info!("devices={:?}", devices);
        console::log_1(&JsValue::from_str(format!("devices={:?}", devices).as_str()));

        match devices.get_user_media_with_constraints(&MediaStreamConstraints::new().video(&true.into())) {
            Ok(promise) => {
                console::log_1(&JsValue::from_str("promise===1"));
                JsFuture::from(promise)
                .await
                .unwrap();
            },
            Err(e) => {
                console::log_2(&JsValue::from_str("不支持devices"),&e);
            },
        }

        devices
    }

    pub fn video_devices(&self) -> impl Iterator<Item=&Device> {
        self.iter_by_kind(MediaDeviceKind::Videoinput)
    }

    pub fn audio_devices(&self) -> impl Iterator<Item=&Device> {
        self.iter_by_kind(MediaDeviceKind::Audioinput)
    }

    fn iter_by_kind(&self, kind: MediaDeviceKind) -> impl Iterator<Item=&Device> {
        self.iter().filter(move |item| item.kind == kind)
    }


}