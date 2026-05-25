// provide the shared crates via re-export
use common::*;

// provide access to esp32 hardware
use soc_esp32::*;

// provide logging primitives
use log::*;

// provide scheduling primitives
use enmesh_firmware::prelude::*;

#[embassy_executor::task]
pub async fn task_ble_companion(
    global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    bt_peripheral: esp_hal::peripherals::BT<'static>,
    rng: esp_hal::peripherals::RNG<'static>,
    adc1: esp_hal::peripherals::ADC1<'static>,
) {
    debug!("initializing bluetooth...");
    use esp_radio::ble::controller::BleConnector;
    let ble_connector = BleConnector::new(bt_peripheral, Default::default()).unwrap();
    use trouble_host::prelude::ExternalController;
    let ble_controller: ExternalController<_, 1> = ExternalController::new(ble_connector);

    // start the enmesh firmware ble host (advertising per the MAC)
    use esp_hal::efuse::*;
    let mac_address = interface_mac_address(InterfaceMacAddress::Bluetooth);
    let mac: [u8; 6] = mac_address.as_bytes().try_into().expect("invalid mac length");
    // initialize the rng for security
    esp_hal::rng::TrngSource::new(rng, adc1);
    let mut random_generator = esp_hal::rng::Trng::try_new().unwrap();
    enmesh_firmware::ble::run(&global_state, ble_controller, mac, &mut random_generator).await;

    error!("ble host stopped");
}
