use crate::models::{Mode, Tick};

pub fn parse_binary(data: &[u8]) -> Vec<Tick> {
    let mut ticks = Vec::new();
    if data.len() < 2 {
        return ticks;
    }

    let count = u16::from_be_bytes([data[0], data[1]]);
    let mut offset = 2;

    for _ in 0..count {
        if offset + 2 > data.len() {
            break;
        }

        let packet_len = u16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;

        if offset + packet_len > data.len() {
            break;
        }
        let packet = &data[offset..offset + packet_len];

        if let Some(tick) = parse_packet_bytes(packet) {
            ticks.push(tick);
        }

        offset += packet_len;
    }
    ticks
}

fn parse_packet_bytes(packet: &[u8]) -> Option<Tick> {
    if packet.len() < 8 {
        return None;
    }

    let token = u32::from_be_bytes(packet[0..4].try_into().ok()?);
    // LTP is always at bytes 4-8
    let ltp = i32::from_be_bytes(packet[4..8].try_into().ok()?) as f64 / 100.0;

    let mut tick = Tick {
        instrument_token: token,
        mode: Mode::LTP,
        ltp,
        ..Default::default()
    };

    // If packet is larger, parse extra fields
    if packet.len() == 44 || packet.len() == 184 {
        tick.mode = if packet.len() == 184 {
            Mode::Full
        } else {
            Mode::Quote
        };

        // Example: Volume is at 16-20
        if let Ok(vol_bytes) = packet[16..20].try_into() {
            tick.volume = i32::from_be_bytes(vol_bytes);
        }
        // You can add Open/High/Low parsing here based on offsets
    }

    Some(tick)
}
