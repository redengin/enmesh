// provide the shared crates via re-export
use common::*;

// provide access to esp32 hardware
use soc_esp32::*;

// provide logging primitives
use log::*;

// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;

#[embassy_executor::task]
pub async fn task_ble_companion(
    global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    bt_peripheral: esp_hal::peripherals::BT<'static>,
) {
    debug!("initializing bluetooth...");
    use esp_radio::ble::controller::BleConnector;
    let ble_connector = BleConnector::new(bt_peripheral, Default::default()).unwrap();
    use trouble_host::prelude::ExternalController;
    let ble_controller: ExternalController<_, 1> = ExternalController::new(ble_connector);

    use esp_hal::efuse::*;
    let mac_address = interface_mac_address(InterfaceMacAddress::Bluetooth);
    let mac: [u8; 6] = mac_address.as_bytes().try_into().expect("invalid mac length");
    enmesh_firmware::ble::run(&global_state, ble_controller, mac).await;

    error!("ble host stopped");
}
