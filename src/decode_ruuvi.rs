// Data format 5 (RAWv2)
// https://docs.ruuvi.com/communication/bluetooth-advertisements/data-format-5-rawv2
pub fn decode_ruuvi_raw5(data: &[u8]) -> Option<(f32, f32, f32, f32)> {
    if data.len() < 7 || data[0] != 5 {
        return None;
    }
    let temp_raw = i16::from_be_bytes([data[1], data[2]]);
    let hum_raw = u16::from_be_bytes([data[3], data[4]]);
    let pres_raw = u16::from_be_bytes([data[5], data[6]]);
    let power_raw = u16::from_be_bytes([data[13], data[14]]);
    let battery_raw = power_raw >> 5; // 0..2046

    let temperature = f32::from(temp_raw) / 200.0;
    let humidity = f32::from(hum_raw) / 400.0; // already in %, no x100!
    let pressure = (f32::from(pres_raw) + 50000.0) / 100.0;
    let battery = (1600_f32 + f32::from(battery_raw)) / 1000.0;

    Some((temperature, humidity, pressure, battery))
}
