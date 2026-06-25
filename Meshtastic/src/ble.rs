use trouble_host::prelude::*;
use trouble_host_embassy_sync as embassy_sync;
use trouble_host_static_cell as static_cell;

/// Meshtastic Companion BLE protocol
///=============================================================================
/// https://meshtastic.org/docs/development/device/client-api/
pub const MESHTASTIC_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x6BA1B218_15A8_461F_9FA8_5DCAE273EAFD);
pub const MESHTASTIC_FROM_RADIO_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x2C55E69E_4993_11ED_B878_0242AC120002);
pub const MESHTASTIC_TO_RADIO_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0xF75C76D2_129E_4DAD_A1DD_7866124401E7);
pub const MESHTASTIC_FROM_NUM_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0xED9DA18C_A800_4F66_A670_AA7547E34453);
pub const MESHTASTIC_LOG_RECORD_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x5A3D6E49_06E6_4423_9944_E9DE8CDF9547);

pub const MESHTASTIC_OTA_IMAGE_SIZE_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0xE74DD9C0_A301_4A6F_95A1_F0E1DBEA8E1E);
pub const MESHTASTIC_OTA_BUFFER_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0xE272EBAC_D463_4B98_BC84_5CC1A39EE517);
pub const MESHTASTIC_OTA_CRC32_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x4826129C_C22A_43A3_B066_CE8F0D5BACC6);
pub const MESHTASTIC_OTA_RESULT_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x5E134862_7411_4424_AC4A_210937432C77);
pub const MESHTASTIC_OTA_REGION_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x5E134862_7411_4424_AC4A_210937432C67);

pub const MAXPACKET: usize = 256;

/// MeshCore Companion BLE protocol
#[gatt_service(uuid = MESHTASTIC_UUID)]
pub struct MeshtasticService {
    // standard BLE characteristics
    //-----------------------------------------------------------------------------
    /// Battery Level
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, name = "batteryLevel", read, value = "Battery Level", type = &'static str)]
    #[characteristic(uuid = characteristic::BATTERY_LEVEL, read, notify, value = 0)]
    pub battery_level: u8,

    #[characteristic(uuid = characteristic::SOFTWARE_REVISION_STRING,
        read, permissions(encrypted))]
    pub sw_version: &'static str,

    #[characteristic(uuid = characteristic::MANUFACTURER_NAME_STRING,
        read, permissions(encrypted))]
    pub manufacturer: &'static str,

    // #[characteristic(uuid = MESHTASTIC_HW_VERSION_UUID,
    #[characteristic(uuid = characteristic::HARDWARE_REVISION_STRING,
        read, permissions(encrypted))]
    pub hw_version: &'static str,

    // Meshtastic specialization
    //-----------------------------------------------------------------------------
    #[characteristic(uuid = MESHTASTIC_FROM_RADIO_UUID,
        read, permissions(encrypted), value = [0u8; MAXPACKET])]
    pub from_radio: [u8; MAXPACKET],

    #[characteristic(uuid = MESHTASTIC_TO_RADIO_UUID,
        write, permissions(encrypted), value = [0u8; MAXPACKET])]
    pub to_radio: [u8; MAXPACKET],

    #[characteristic(uuid = MESHTASTIC_FROM_NUM_UUID,
        read, notify, write, permissions(encrypted), value = 0)]
    pub from_num: u32,

    #[characteristic(uuid = MESHTASTIC_LOG_RECORD_UUID,
        notify, permissions(encrypted), value = [0u8; MAXPACKET])]
    pub log_record: [u8; MAXPACKET],

    // OTA - over the air update specialization
    //-----------------------------------------------------------------------------
    #[characteristic(uuid = MESHTASTIC_OTA_IMAGE_SIZE_UUID,
        write, read, permissions(encrypted))]
    pub image_size: u32,

    #[characteristic(uuid = MESHTASTIC_OTA_BUFFER_UUID,
        write, permissions(encrypted), value = [0u8; MAXPACKET])]
    pub ota_buffer: [u8; MAXPACKET],

    #[characteristic(uuid = MESHTASTIC_OTA_CRC32_UUID,
        write, permissions(encrypted))]
    pub image_crc32: u32,

    #[characteristic(uuid = MESHTASTIC_OTA_RESULT_UUID,
        read, notify, permissions(encrypted))]
    pub image_result: u32,

    #[characteristic(uuid = MESHTASTIC_OTA_REGION_UUID,
        write, permissions(encrypted))]
    pub image_region: u8,
}

// provide MeshCore BLE command serde
// pub mod commands;
