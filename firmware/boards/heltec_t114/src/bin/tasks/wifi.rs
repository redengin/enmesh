// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;

// use soc crate (it provides the panic handler)
use soc_esp32::*;

// provide scheduling primitives
use enmesh_firmware::prelude::*;

#[embassy_executor::task]
pub async fn task_wifi_bridge(
    _global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    wifi_peripheral: esp_hal::peripherals::WIFI<'static>,
) {
    debug!("initializing wifi bridge...");
    //================================================================================
    let _wifi_controller = esp_radio::wifi::new(
        wifi_peripheral,
        esp_radio::wifi::ControllerConfig::default()
            //TODO  .with_country_info(country_info)
            .with_initial_config(esp_radio::wifi::Config::AccessPointStation(
                esp_radio::wifi::sta::StationConfig::default(),
                // FIXME (set the SSID per the MAC)
                esp_radio::wifi::ap::AccessPointConfig::default(),
            )),
    )
    .unwrap();
    //================================================================================

    // debug!("initializing bluetooth...");
    // //================================================================================
    // use esp_radio::ble::controller::BleConnector;
    // let ble_connector = BleConnector::new(bt_peripheral, Default::default()).unwrap();
    // use trouble_host::prelude::ExternalController;
    // let _ble_controller: ExternalController<_, 1> = ExternalController::new(ble_connector);
    // //================================================================================

    // TODO run the wifi handler
    // enmesh_firmware::repeater::run(lora_radio).await;

    // panic!("LoRa task ended");

    // TODO...
    // https://github.com/esp-rs/esp-hal/blob/main/examples/wifi/embassy_access_point/src/main.rs

    warn!("wifi bridge task ended");
}

