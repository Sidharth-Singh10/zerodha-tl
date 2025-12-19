use crate::models::{Depth, Mode, Tick};

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
    let ltp = i32::from_be_bytes(packet[4..8].try_into().ok()?) as f64 / 100.0;

    let mut tick = Tick {
        instrument_token: token,
        mode: Mode::LTP,
        ltp,
        ..Default::default()
    };

    // If packet is only 8 bytes (LTP Mode)
    if packet.len() == 8 {
        return Some(tick);
    }

    // Processing Quote (44 bytes) or Full (184 bytes)
    if packet.len() == 44 || packet.len() == 184 {
        tick.mode = if packet.len() == 184 {
            Mode::Full
        } else {
            Mode::Quote
        };

        tick.last_traded_quantity = Some(i32::from_be_bytes(packet[8..12].try_into().ok()?));
        tick.average_traded_price =
            Some(i32::from_be_bytes(packet[12..16].try_into().ok()?) as f64 / 100.0);
        tick.volume = Some(i32::from_be_bytes(packet[16..20].try_into().ok()?));
        tick.total_buy_quantity = Some(i32::from_be_bytes(packet[20..24].try_into().ok()?));
        tick.total_sell_quantity = Some(i32::from_be_bytes(packet[24..28].try_into().ok()?));

        // OHLC
        tick.open = Some(i32::from_be_bytes(packet[28..32].try_into().ok()?) as f64 / 100.0);
        tick.high = Some(i32::from_be_bytes(packet[32..36].try_into().ok()?) as f64 / 100.0);
        tick.low = Some(i32::from_be_bytes(packet[36..40].try_into().ok()?) as f64 / 100.0);
        tick.close = Some(i32::from_be_bytes(packet[40..44].try_into().ok()?) as f64 / 100.0);
    }

    // Processing Full specific fields (Timestamp, OI, Market Depth)
    if packet.len() == 184 {
        tick.last_traded_timestamp = Some(i32::from_be_bytes(packet[44..48].try_into().ok()?));
        tick.open_interest = Some(i32::from_be_bytes(packet[48..52].try_into().ok()?));
        tick.open_interest_day_high = Some(i32::from_be_bytes(packet[52..56].try_into().ok()?));
        tick.open_interest_day_low = Some(i32::from_be_bytes(packet[56..60].try_into().ok()?));
        tick.exchange_timestamp = Some(i32::from_be_bytes(packet[60..64].try_into().ok()?));

        // Parse Market Depth: 64-184
        // 10 entries total: First 5 are Bids, Next 5 are Offers
        // Each entry is 12 bytes: Qty (4), Price (4), Orders (2), Padding (2)
        let mut bids = Vec::with_capacity(5);
        let mut offers = Vec::with_capacity(5);

        let depth_start_offset = 64;

        for i in 0..10 {
            let offset = depth_start_offset + (i * 12);
            let qty = i32::from_be_bytes(packet[offset..offset + 4].try_into().ok()?);
            let price =
                i32::from_be_bytes(packet[offset + 4..offset + 8].try_into().ok()?) as f64 / 100.0;
            let orders = u16::from_be_bytes(packet[offset + 8..offset + 10].try_into().ok()?);
            // Bytes offset+10..offset+12 are padding, skip them.

            let entry = Depth {
                quantity: qty,
                price,
                orders,
            };

            if i < 5 {
                bids.push(entry);
            } else {
                offers.push(entry);
            }
        }

        tick.bids = Some(bids);
        tick.offers = Some(offers);
    }

    Some(tick)
}
