// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		configured_name::ConfiguredNameCharacteristic,
		managed_network_enable::ManagedNetworkEnableCharacteristic,
		network_access_violation_control::NetworkAccessViolationControlCharacteristic,
		network_client_control::NetworkClientControlCharacteristic,
		network_client_status_control::NetworkClientStatusControlCharacteristic,
		router_status::RouterStatusCharacteristic,
		supported_router_configuration::SupportedRouterConfigurationCharacteristic,
		wan_configuration_list::WanConfigurationListCharacteristic,
		wan_status_list::WanStatusListCharacteristic,
	},
    HapType,
};

/// Wi-Fi Router Service.
#[derive(Debug, Default)]
pub struct WiFiRouterService {
    /// Instance ID of the Wi-Fi Router Service.
    id: u64,
    /// `HapType` of the Wi-Fi Router Service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Configured Name Characteristic (required).
	pub configured_name: ConfiguredNameCharacteristic,
	/// Managed Network Enable Characteristic (required).
	pub managed_network_enable: ManagedNetworkEnableCharacteristic,
	/// Network Access Violation Control Characteristic (required).
	pub network_access_violation_control: NetworkAccessViolationControlCharacteristic,
	/// Network Client Control Characteristic (required).
	pub network_client_control: NetworkClientControlCharacteristic,
	/// Network Client Status Control Characteristic (required).
	pub network_client_status_control: NetworkClientStatusControlCharacteristic,
	/// Router Status Characteristic (required).
	pub router_status: RouterStatusCharacteristic,
	/// Supported Router Configuration Characteristic (required).
	pub supported_router_configuration: SupportedRouterConfigurationCharacteristic,
	/// WAN Configuration List Characteristic (required).
	pub wan_configuration_list: WanConfigurationListCharacteristic,
	/// WAN Status List Characteristic (required).
	pub wan_status_list: WanStatusListCharacteristic,

}

impl WiFiRouterService {
    /// Creates a new Wi-Fi Router Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::WiFiRouter,
			configured_name: ConfiguredNameCharacteristic::new(id + 1 + 0, accessory_id),
			managed_network_enable: ManagedNetworkEnableCharacteristic::new(id + 1 + 1, accessory_id),
			network_access_violation_control: NetworkAccessViolationControlCharacteristic::new(id + 1 + 2, accessory_id),
			network_client_control: NetworkClientControlCharacteristic::new(id + 1 + 3, accessory_id),
			network_client_status_control: NetworkClientStatusControlCharacteristic::new(id + 1 + 4, accessory_id),
			router_status: RouterStatusCharacteristic::new(id + 1 + 5, accessory_id),
			supported_router_configuration: SupportedRouterConfigurationCharacteristic::new(id + 1 + 6, accessory_id),
			wan_configuration_list: WanConfigurationListCharacteristic::new(id + 1 + 7, accessory_id),
			wan_status_list: WanStatusListCharacteristic::new(id + 1 + 8, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for WiFiRouterService {
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
			&self.configured_name,
			&self.managed_network_enable,
			&self.network_access_violation_control,
			&self.network_client_control,
			&self.network_client_status_control,
			&self.router_status,
			&self.supported_router_configuration,
			&self.wan_configuration_list,
			&self.wan_status_list,
		];
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.configured_name,
			&mut self.managed_network_enable,
			&mut self.network_access_violation_control,
			&mut self.network_client_control,
			&mut self.network_client_status_control,
			&mut self.router_status,
			&mut self.supported_router_configuration,
			&mut self.wan_configuration_list,
			&mut self.wan_status_list,
		];
		characteristics
    }
}

impl Serialize for WiFiRouterService {
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
