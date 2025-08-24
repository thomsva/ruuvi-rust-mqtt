pub fn decode_ruuvi_raw5(data: &[u8]) -> Option<(f32, f32, f32, f32)> {
    if data.len() < 7 || data[0] != 5 {
        return None;
    }
    let temp_raw = i16::from_be_bytes([data[1], data[2]]);
    let hum_raw = u16::from_be_bytes([data[3], data[4]]);
    let pres_raw = u16::from_be_bytes([data[5], data[6]]);
    let power_raw = u16::from_be_bytes([data[13], data[14]]);

    let temperature = temp_raw as f32 / 200.0;
    let humidity = hum_raw as f32 / 400.0; // already in %, no x100!
    let pressure = (pres_raw as f32 + 50000.0) / 100.0;
    let battery = (power_raw & 0x07FF) as f32 / 1000.0;

    Some((temperature, humidity, pressure, battery))
}
