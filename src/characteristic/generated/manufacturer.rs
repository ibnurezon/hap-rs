// this file is auto-generated by build.rs

use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;

use crate::{
    characteristic::{
        AsyncCharacteristicCallbacks,
        Characteristic,
        CharacteristicCallbacks,
        Format,
        HapCharacteristic,
        HapCharacteristicSetup,
        HapType,
        OnReadFn,
        OnReadFuture,
        OnUpdateFn,
        OnUpdateFuture,
        Perm,
        Unit,
    },
    pointer,
    Error,
    Result,
};

/// Manufacturer Characteristic.
#[derive(Debug, Default, Serialize)]
pub struct ManufacturerCharacteristic(Characteristic<String>);

impl ManufacturerCharacteristic {
    /// Creates a new Manufacturer Characteristic.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self(Characteristic::<String> {
            id,
            accessory_id,
            hap_type: HapType::Manufacturer,
            format: Format::String,
            perms: vec![
					Perm::PairedRead,
            ],
            ..Default::default()
        })
    }
}

#[async_trait]
impl HapCharacteristic for ManufacturerCharacteristic {
    fn get_id(&self) -> u64 { self.0.get_id() }

    fn get_type(&self) -> HapType { self.0.get_type() }

    fn get_format(&self) -> Format { self.0.get_format() }

    fn get_perms(&self) -> Vec<Perm> { self.0.get_perms() }

    fn get_event_notifications(&self) -> Option<bool> { self.0.get_event_notifications() }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) {
        self.0.set_event_notifications(event_notifications)
    }

    async fn get_value(&mut self) -> Result<serde_json::Value> {
        let value = self.0.get_value().await?;
        Ok(json!(value))
    }

    async fn set_value(&mut self, value: serde_json::Value) -> Result<()> {
        let v;
        // for whatever reason, the controller is setting boolean values either as a boolean or as an integer
        if self.0.format == Format::Bool && value.is_number() {
            let num_v: u8 = serde_json::from_value(value)?;
            if num_v == 0 {
                v = serde_json::from_value(json!(false))?;
            } else if num_v == 1 {
                v = serde_json::from_value(json!(true))?;
            } else {
                return Err(Error::InvalidValue(self.get_format()));
            }
        } else {
            v = serde_json::from_value(value).map_err(|_| Error::InvalidValue(self.get_format()))?;
        }
        self.0.set_value(v).await
    }

    fn get_unit(&self) -> Option<Unit> { self.0.get_unit() }

    fn get_max_value(&self) -> Option<serde_json::Value> { self.0.get_max_value().map(|v| json!(v)) }

    fn get_min_value(&self) -> Option<serde_json::Value> { self.0.get_min_value().map(|v| json!(v)) }

    fn get_step_value(&self) -> Option<serde_json::Value> { self.0.get_step_value().map(|v| json!(v)) }

    fn get_max_len(&self) -> Option<u16> { self.0.get_max_len() }
}

impl HapCharacteristicSetup for ManufacturerCharacteristic {
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        self.0.set_event_emitter(event_emitter)
    }
}

impl CharacteristicCallbacks<String> for ManufacturerCharacteristic {
    fn on_read(&mut self, f: Option<impl OnReadFn<String>>) { self.0.on_read(f) }

    fn on_update(&mut self, f: Option<impl OnUpdateFn<String>>) { self.0.on_update(f) }
}

impl AsyncCharacteristicCallbacks<String> for ManufacturerCharacteristic {
    fn on_read_async(&mut self, f: Option<impl OnReadFuture<String>>) { self.0.on_read_async(f) }

    fn on_update_async(&mut self, f: Option<impl OnUpdateFuture<String>>) { self.0.on_update_async(f) }
}