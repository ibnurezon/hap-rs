// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		supported_target_configuration::SupportedTargetConfigurationCharacteristic,
		target_list_configuration::TargetListConfigurationCharacteristic,
	},
    HapType,
};

/// Target Control Management Service.
#[derive(Debug, Default)]
pub struct TargetControlManagementService {
    /// Instance ID of the Target Control Management Service.
    id: u64,
    /// `HapType` of the Target Control Management Service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Supported Target Configuration Characteristic (required).
	pub supported_target_configuration: SupportedTargetConfigurationCharacteristic,
	/// Target List Configuration Characteristic (required).
	pub target_list_configuration: TargetListConfigurationCharacteristic,

}

impl TargetControlManagementService {
    /// Creates a new Target Control Management Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::TargetControlManagement,
			supported_target_configuration: SupportedTargetConfigurationCharacteristic::new(id + 1 + 0, accessory_id),
			target_list_configuration: TargetListConfigurationCharacteristic::new(id + 1 + 1, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for TargetControlManagementService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.supported_target_configuration,
			&self.target_list_configuration,
		];
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.supported_target_configuration,
			&mut self.target_list_configuration,
		];
		characteristics
    }
}

impl Serialize for TargetControlManagementService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
