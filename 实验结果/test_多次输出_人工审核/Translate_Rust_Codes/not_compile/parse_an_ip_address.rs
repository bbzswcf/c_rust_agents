```rust
use std::mem;

fn _parse_decimal(cursor: &mut &str) -> u32 {
    let mut val = 0;
    while let Some(ch) = cursor.chars().next() {
        if ch.is_ascii_digit() {
            val = val * 10 + (ch as u32 - '0' as u32);
            *cursor = &cursor[1..];
        } else {
            break;
        }
    }
    val
}

fn _parse_hex(cursor: &mut &str) -> u32 {
    let mut val = 0;
    while let Some(ch) = cursor.chars().next() {
        let ch = ch.to_ascii_uppercase();
        if ch.is_ascii_hexdigit() {
            let nybble_value = if ch.is_ascii_digit() {
                ch as u32 - '0' as u32
            } else {
                ch as u32 - 'A' as u32 + 10
            };
            val = (val << 4) + nybble_value;
            *cursor = &cursor[1..];
        } else {
            break;
        }
    }
    val
}

fn parse_ipv4_or_ipv6(
    text: &mut &str,
    addr: Option<&mut [u8; 16]>,
    port: Option<&mut u16>,
    is_ipv6: Option<&mut bool>,
) -> bool {
    let mut addr_local = addr.unwrap_or(&mut [0; 16]);
    let mut dummy_addr = [0; 16];

    let mut colon_pos = text.find(':');
    let mut dot_pos = text.find('.');
    let mut open_bracket_pos = text.find('[');
    let mut close_bracket_pos = text.find(']');

    let is_ipv6_local = open_bracket_pos.is_some()
        || dot_pos.is_none()
        || (colon_pos.is_some() && (dot_pos.is_none() || colon_pos < dot_pos));

    if is_ipv6_local {
        if let Some(open_bracket_pos) = open_bracket_pos {
            if close_bracket_pos.is_none() || close_bracket_pos < open_bracket_pos {
                return false;
            }
        }
    } else {
        if dot_pos.is_none() || (colon_pos.is_some() && colon_pos < dot_pos) {
            return false;
        }
    }

    if let Some(is_ipv6) = is_ipv6 {
        *is_ipv6 = is_ipv6_local;
    }

    if addr.is_none() {
        addr_local = &mut dummy_addr;
    }

    if !is_ipv6_local {
        let mut addr_cursor = addr_local.as_mut_ptr();
        let mut val;
        let mut text_before;

        text_before = *text;
        val = _parse_decimal(text);
        if text.is_empty() || text.chars().next() != Some('.') || val > 255 {
            return false;
        }
        unsafe { *addr_cursor = val as u8 };
        addr_cursor = unsafe { addr_cursor.add(1) };
        *text = &text[1..];

        text_before = *text;
        val = _parse_decimal(text);
        if text.is_empty() || text.chars().next() != Some('.') || val > 255 {
            return false;
        }
        unsafe { *addr_cursor = val as u8 };
        addr_cursor = unsafe { addr_cursor.add(1) };
        *text = &text[1..];

        text_before = *text;
        val = _parse_decimal(text);
        if text.is_empty() || text.chars().next() != Some('.') || val > 255 {
            return false;
        }
        unsafe { *addr_cursor = val as u8 };
        addr_cursor = unsafe { addr_cursor.add(1) };
        *text = &text[1..];

        text_before = *text;
        val = _parse_decimal(text);
        if val > 255 {
            return false;
        }
        unsafe { *addr_cursor = val as u8 };

        if text.chars().next() == Some(':') && port.is_some() {
            *text = &text[1..];
            text_before = *text;
            val = _parse_decimal(text);
            if val > 65535 {
                return false;
            }
            if let Some(port) = port {
                *port = ((val >> 8) & 0xff) as u8 | ((val & 0xff) as u8) << 8;
            }
            return true;
        } else {
            if let Some(port) = port {
                *port = 0;
            }
            return true;
        }
    } else {
        if let Some(open_bracket_pos) = open_bracket_pos {
            *text = &text[open_bracket_pos + 1..];
        }

        let mut addr_cursor = addr_local.as_mut_ptr();
        let mut zeros_loc = None;
        let mut ipv4_detected = false;

        for n_idx in 0..8 {
            let text_before = *text;
            let val = _parse_hex(text);
            if text_before == *text {
                if let Some(zeros_loc) = zeros_loc {
                    if zeros_loc == addr_cursor {
                        break;
                    }
                    return false;
                }
                if text.chars().next() != Some(':') {
                    return false;
                }
                if n_idx == 0 {
                    *text = &text[1..];
                    if text.chars().next() != Some(':') {
                        return false;
                    }
                }
                zeros_loc = Some(addr_cursor);
                *text = &text[1..];
            } else {
                if text.chars().next() == Some('.') {
                    let mut text_local = text_before;
                    let mut addr_local = [0; 16];
                    let mut is_ipv6_local = false;
                    let parse_result_local = parse_ipv4_or_ipv6(
                        &mut text_local,
                        Some(&mut addr_local),
                        None,
                        Some(&mut is_ipv6_local),
                    );
                    *text = text_local;
                    if !parse_result_local || is_ipv6_local {
                        return false;
                    }
                    unsafe {
                        *addr_cursor = addr_local[0];
                        addr_cursor = addr_cursor.add(1);
                        *addr_cursor = addr_local[1];
                        addr_cursor = addr_cursor.add(1);
                        *addr_cursor = addr_local[2];
                        addr_cursor = addr_cursor.add(1);
                        *addr_cursor = addr_local[3];
                        addr_cursor = addr_cursor.add(1);
                    }
                    ipv4_detected = true;
                    break;
                }
                if val > 65535 {
                    return false;
                }
                unsafe {
                    *addr_cursor = (val >> 8) as u8;
                    addr_cursor = addr_cursor.add(1);
                    *addr_cursor = val as u8;
                    addr_cursor = addr_cursor.add(1);
                }
                if text.chars().next() == Some(':') {
                    *text = &text[1..];
                } else {
                    break;
                }
            }
        }

        if let Some(zeros_loc) = zeros_loc {
            let head = zeros_loc as usize - addr_local.as_ptr() as usize;
            let tail = n_idx * 2 - head;
            let zeros = 16 - tail - head;
            addr_local.copy_within(head..head + tail, 16 - tail);
            addr_local[head..head + zeros].fill(0);
        }

        if ipv4_detected {
           