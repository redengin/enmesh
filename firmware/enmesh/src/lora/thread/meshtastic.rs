// // provide the common crates via re-export
// use common::*;

// // provide logging primitives
// use log::*;
// const TAG: &str = "LoRa-Meshstatic";

// use crate::lora::EnmeshLoRaConfig;

// pub async fn cycle(
//     lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
//     lora_config: &EnmeshLoRaConfig,
// ) {
//     let modulation_params = lora_radio
//         .create_modulation_params(
//             lora_config.modulation_config.spreading_factor,
//             lora_config.modulation_config.bandwidth,
//             lora_config.modulation_config.coding_rate,
//             lora_config.modulation_config.frequency_hz,
//         )
//         .unwrap();

//     // prioritize sending
//     let has_transmit_packets = false; // FIXME - check the packet queue
//     if has_transmit_packets {
//         match crate::lora::is_channel_clear(lora_radio, &modulation_params).await {
//             Ok(is_clear) => {
//                 if is_clear {
//                     tx(lora_radio, lora_config).await;
//                 }
//             }
//             Err(e) => {
//                 warn!("{TAG} failed to detect channel activity (CAD): {:?}", e);
//                 return;
//             }
//         }

//         // receive packets
//         rx(lora_radio, lora_config).await;
//     }
// }

// async fn tx(lora_radio: &mut impl lora_phy::mod_traits::RadioKind, lora_config: &EnmeshLoRaConfig) {
//     // send until our air time is up
//     let _tx_start = embassy_time::Instant::now();

//     // prepare for tx
//     lora_radio
//         .ensure_ready(lora_phy::mod_params::RadioMode::Transmit)
//         .await
//         .unwrap();
//     lora_radio.set_standby().await.unwrap();
//     let modulation_params = lora_radio
//         .create_modulation_params(
//             lora_config.modulation_config.spreading_factor,
//             lora_config.modulation_config.bandwidth,
//             lora_config.modulation_config.coding_rate,
//             lora_config.modulation_config.frequency_hz,
//         )
//         .unwrap();
//     lora_radio
//         .set_modulation_params(&modulation_params)
//         .await
//         .unwrap();

//     // set transmit power
//     // TODO - upon acceptance of RFC, scale down power
//     crate::lora::set_tx_power(lora_radio, lora_config.modulation_config.tx_power_dbm).await.unwrap();

//     lora_radio
//         .ensure_ready(lora_phy::mod_params::RadioMode::Transmit)
//         .await
//         .unwrap();
//     lora_radio.set_standby().await.unwrap();


//     // FIXME lora_radio.set_packet_params(pkt_params);
//     lora_radio
//         .set_channel(lora_config.modulation_config.frequency_hz)
//         .await
//         .unwrap();
//     // lora_radio.set_payload(buffer).await.unwrap();
//     lora_radio
//         .set_irq_params(Some(lora_phy::mod_params::RadioMode::Transmit))
//         .await
//         .unwrap();

//     // while self.tx_queue.len() > 0 {
//     while false {
//         // let packet = self.tx_queue.get(0).unwrap();

//         // prepare for transmit
//         lora_radio.create_packet_params(
//             lora_config.packet_config.preamble_length,
//             lora_config.packet_config.implicit_header,
//             // FIXME - should use packet.len()
//             0,
//             lora_config.packet_config.crc,
//             lora_config.packet_config.iq_inverted,
//             &modulation_params,
//         ).unwrap();

//         // transmit packet
//         match lora_radio.do_tx().await {
//             Ok(_) => {
//                 // self.tx_queue.pop_front();
//             }
//             Err(err) => {
//                 warn!("{TAG} failed to send packet: {:?}", err);
//                 break;
//             }
//         }

//         // TODO - upon acceptance of RFC
//         // stop transmitting if we've exceeded airtime
//         // if (embassy_time::Instant::now() - tx_start)
//         //     > lora_config.modulation_config.air_time
//         // {
//         //     break;
//         // }
//     }
// }

// async fn rx(lora_radio: &mut impl lora_phy::mod_traits::RadioKind, lora_config: &EnmeshLoRaConfig) {


// }
