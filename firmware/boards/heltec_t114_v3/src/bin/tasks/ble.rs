// provide the shared crates via re-export
use common::*;

// provide access to esp32 hardware
use soc_esp32::*;

// provide logging primitives
use log::*;

// provide scheduling primitives
use embassy_sync::rwlock::RwLock;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;


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

    enmesh_firmware::ble::run(&global_state, ble_controller).await;

    error!("bluetooth thread ended");
}
