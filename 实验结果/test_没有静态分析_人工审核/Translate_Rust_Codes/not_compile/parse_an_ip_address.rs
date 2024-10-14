use std::ptr;

fn _parse_decimal(pch_cursor: &mut &str) -> u32 {
    let mut n_val = 0;
    while let Some(ch_now) = pch_cursor.chars().next() {
        if ch_now >= '0' && ch_now <= '9' {
            n_val *= 10;
            n_val += (ch_now as u32) - ('0' as u32);
            *pch_cursor = &pch_cursor[1..];
        } else {
            break;
        }
    }
    n_val
}

fn _parse_hex(pch_cursor: &mut &str) -> u32 {
    let mut n_val = 0;
    while let Some(ch_now) = pch_cursor.chars().next() {
        let ch_now = (ch_now as u8 & 0x5f) as char; // collapses case, but mutilates digits
        if (ch_now >= ('0' as u8 & 0x5f) as char && ch_now <= ('9' as u8 & 0x5f) as char) || (ch_now >= 'A' && ch_now <= 'F') {
            let nybble_value; // Removed mut keyword as it is not necessary
            let ch_now = (ch_now as u8) - 0x10; // scootch digital values down; hex now offset by x31
            nybble_value = if ch_now > 9 { ch_now - (0x31 - 0x0a) } else { ch_now };
            n_val <<= 4;
            n_val += nybble_value as u32;
            *pch_cursor = &pch_cursor[1..];
        } else {
            break;
        }
    }
    n_val
}

fn parse_ipv4_or_ipv6(ppsz_text: &mut &str, aby_addr: &mut [u8; 16], pn_port: Option<&mut u16>, pb_is_ipv6: Option<&mut bool>) -> bool {
    let mut aby_addr_local: Option<&mut [u8; 16]> = Some(aby_addr);
    let mut aby_dummy_addr = [0u8; 16];

    if aby_addr_local.is_none() {
        aby_addr_local = Some(&mut aby_dummy_addr);
    }

    let pch_colon = ppsz_text.find(':');
    let pch_dot = ppsz_text.find('.');
    let pch_open_bracket = ppsz_text.find('[');
    let pch_close_bracket = ppsz_text.find(']');

    let b_is_ipv6_local = pch_open_bracket.is_some() || pch_dot.is_none() || (pch_colon.is_some() && (pch_dot.is_none() || pch_colon < pch_dot));

    if b_is_ipv6_local {
        if pch_open_bracket.is_some() && (pch_close_bracket.is_none() || pch_close_bracket < pch_open_bracket) {
            return false;
        }
    } else {
        if pch_dot.is_none() || (pch_colon.is_some() && pch_colon < pch_dot) {
            return false;
        }
    }

    if let Some(pb_is_ipv6) = pb_is_ipv6 {
        *pb_is_ipv6 = b_is_ipv6_local;
    }

    if !b_is_ipv6_local {
        let pby_addr_cursor = aby_addr_local.unwrap().as_mut_ptr(); // Removed mut keyword
        let mut n_val;
        let mut psz_text_before = *ppsz_text;
        n_val = _parse_decimal(ppsz_text);
        if ppsz_text.chars().next() != Some('.') || n_val > 255 || psz_text_before == *ppsz_text { // Changed comparison to use a single character from the string
            return false;
        }
        unsafe { *pby_addr_cursor = n_val as u8; }
        *ppsz_text = &ppsz_text[1..];

        psz_text_before = *ppsz_text;
        n_val = _parse_decimal(ppsz_text);
        if ppsz_text.chars().next() != Some('.') || n_val > 255 || psz_text_before == *ppsz_text { // Changed comparison to use a single character from the string
            return false;
        }
        unsafe { *pby_addr_cursor.offset(1) = n_val as u8; }
        *ppsz_text = &ppsz_text[1..];

        psz_text_before = *ppsz_text;
        n_val = _parse_decimal(ppsz_text);
        if ppsz_text.chars().next() != Some('.') || n_val > 255 || psz_text_before == *ppsz_text { // Changed comparison to use a single character from the string
            return false;
        }
        unsafe { *pby_addr_cursor.offset(2) = n_val as u8; }
        *ppsz_text = &ppsz_text[1..];

        psz_text_before = *ppsz_text;
        n_val = _parse_decimal(ppsz_text);
        if n_val > 255 || psz_text_before == *ppsz_text {
            return false;
        }
        unsafe { *pby_addr_cursor.offset(3) = n_val as u8; }

        if ppsz_text.chars().next() == Some(':') && pn_port.is_some() { // Changed comparison to use a single character from the string
            *ppsz_text = &ppsz_text[1..];
            psz_text_before = *ppsz_text;
            n_val = _parse_decimal(ppsz_text);
            if n_val > 65535 || psz_text_before == *ppsz_text {
                return false;
            }
            let us_port_network = ((n_val & 0xff00) >> 8) as u8 as u16 | (n_val & 0xff) as u8 as u16; // Removed unused assignment
            if let Some(pn_port) = pn_port {
                *pn_port = us_port_network;
            }
            return true;
        } else {
            if let Some(pn_port) = pn_port {
                *pn_port = 0;
            }
            return true;
        }
    } else {
        let pby_addr_cursor = aby_addr_local.unwrap().as_mut_ptr(); // Removed mut keyword
        let mut pby_zeros_loc = None;
        let mut b_ipv4_detected = false;
        let mut n_idx = 0;

        if pch_open_bracket.is_some() {
            *ppsz_text = &ppsz_text[pch_open_bracket.unwrap() + 1..];
        }

        while n_idx < 8 {
            let mut psz_text_before = *ppsz_text;
            let n_val = _parse_hex(ppsz_text);
            if psz_text_before == *ppsz_text {
                if pby_zeros_loc.is_some() {
                    if pby_zeros_loc.unwrap() == pby_addr_cursor {
                        n_idx -= 1;
                        break;
                    }
                    return false;
                }
                if ppsz_text.chars().next() != Some(':') { // Changed comparison to use a single character from the string
                    return false;
                }
                if n_idx == 0 {
                    *ppsz_text = &ppsz_text[1..];
                    if ppsz_text.chars().next() != Some(':') { // Changed comparison to use a single character from the string
                        return false;
                    }
                }
                pby_zeros_loc = Some(pby_addr_cursor);
                *ppsz_text = &ppsz_text[1..];
            } else {
                if ppsz_text.chars().next() == Some('.') { // Changed comparison to use a single character from the string
                    let mut psz_text_local = psz_text_before;
                    let mut aby_addr_local = [0u8; 16];
                    let mut b_is_ipv6_local = false;
                    let b_parse_result_local = parse_ipv4_or_ipv6(&mut psz_text_local, &mut aby_addr_local, None, Some(&mut b_is_ipv6_local));
                    *ppsz_text = psz_text_local;
                    if !b_parse_result_local || b_is_ipv6_local {
                        return false;
                    }
                    unsafe {
                        *pby_addr_cursor = aby_addr_local[0];
                        *pby_addr_cursor.offset(1) = aby_addr_local[1];
                        *pby_addr_cursor.offset(2) = aby_addr_local[2];
                        *pby_addr_cursor.offset(3) = aby_addr_local[3];
                    }
                    n_idx += 1;
                    b_ipv4_detected = true;
                    break;
                }

                if n_val > 65535 {
                    return false;
                }
                unsafe {
                    *pby_addr_cursor = (n_val >> 8) as u8;
                    *pby_addr_cursor.offset(1) = (n_val & 0xff) as u8;
                }
                pby_addr_cursor = unsafe { pby_addr_cursor.offset(2) };
                if ppsz_text.chars().next() == Some(':') { // Changed comparison to use a single character from the string
                    *ppsz_text = &ppsz_text[1..];
                } else {
                    break;
                }
            }
            n_idx += 1;
        }

        if let Some(pby_zeros_loc) = pby_zeros_loc {
            let n_head = (pby_zeros_loc as usize - aby_addr_local.unwrap().as_ptr() as usize) / std::mem::size_of::<u8>();
            let n_tail = n_idx * 2 - n_head;
            let n_zeros = 16 - n_tail - n_head;
            unsafe {
                ptr::copy(&aby_addr_local.unwrap()[16 - n_tail], &mut aby_addr_local.unwrap()[16 - n_tail], n_tail);
                ptr::write_bytes(pby_zeros_loc, 0, n_zeros);
            }
        }

        if b_ipv4_detected {
            const ABY_PFX: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0, 0, 0, 0, 0, 0]; // Modified ABY_PFX to have 16 elements
            if ptr::eq(aby_addr_local.unwrap(), &ABY_PFX) { // Removed unsafe block
                return false;
            }
        }

        if pch_open_bracket.is_some() {
            if ppsz_text.chars().next() != Some(']') { // Changed comparison to use a single character from the string
                return false;
            }
            *ppsz_text = &ppsz_text[1..];
        }

        if ppsz_text.chars().next() == Some(':') && pn_port.is_some() { // Changed comparison to use a single character from the string
            *ppsz_text = &ppsz_text[1..];
            let mut psz_text_before = *ppsz_text;
            let n_val = _parse_decimal(ppsz_text);
            if n_val > 65535 || psz_text_before == *ppsz_text {
                return false;
            }
            let us_port_network = ((n_val & 0xff00) >> 8) as u8 as u16 | (n_val & 0xff) as u8 as u16; // Removed unused assignment
            if let Some(pn_port) = pn_port {
                *pn_port = us_port_network;
            }
            return true;
        } else {
            if let Some(pn_port) = pn_port {
                *pn_port = 0;
            }
            return true;
        }
    }
}

fn parse_ipv4_or_ipv6_2(psz_text: &str, aby_addr: &mut [u8; 16], pn_port: Option<&mut u16>, pb_is_ipv6: Option<&mut bool>) -> bool {
    let mut psz_text_local = psz_text;
    parse_ipv4_or_ipv6(&mut psz_text_local, aby_addr, pn_port, pb_is_ipv6)
}

fn htons(us: u16) -> u16 {
    ((us >> 8) & 0xff) as u8 as u16 | ((us & 0xff) << 8) as u8 as u16
}

fn dumpbin(pby_bin: &[u8], n_len: usize) {
    for i in 0..n_len {
        print!("{:02x}", pby_bin[i]);
    }
}

fn testcase(psz_test: &str) {
    let mut aby_addr = [0u8; 16];
    let mut b_is_ipv6 = false;
    let mut n_port = 0u16;
    let b_success;

    println!("Test case '{}'", psz_test);
    let mut psz_text_cursor = psz_test;
    b_success = parse_ipv4_or_ipv6(&mut psz_text_cursor, &mut aby_addr, Some(&mut n_port), Some(&mut b_is_ipv6));
    if !b_success {
        println!("parse failed, at about index {}; rest: '{}'", psz_text_cursor.len(), psz_text_cursor);
        return;
    }

    print!("addr:  ");
    dumpbin(&aby_addr, if b_is_ipv6 { 16 } else { 4 });
    println!();
    if n_port == 0 {
        println!("port absent");
    } else {
        println!("port:  {}", htons(n_port));
    }
    println!();
}

fn main() {
    testcase("127.0.0.1");
    testcase("127.0.0.1:80");
    testcase("::1");
    testcase("[::1]:80");
    testcase("2605:2700:0:3::4713:93e3");
    testcase("[2605:2700:0:3::4713:93e3]:80");
    testcase("::ffff:192.168.173.22");
    testcase("[::ffff:192.168.173.22]:80");
    testcase("1::");
    testcase("[1::]:80");
    testcase("::");
    testcase("[::]:80");
}