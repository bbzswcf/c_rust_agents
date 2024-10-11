use std::ptr;

fn _parse_decimal(pch_cursor: &mut &str) -> u32 {
    let mut n_val: u32 = 0; // Modified: Specify the type to resolve ambiguity
    while let Some(ch_now) = pch_cursor.chars().next() {
        if ch_now >= '0' && ch_now <= '9' {
            n_val = n_val.checked_mul(10).and_then(|v| v.checked_add((ch_now as u32) - ('0' as u32))).unwrap_or(u32::MAX); // Modified: Use checked arithmetic to avoid overflow
            *pch_cursor = &pch_cursor[1..];
        } else {
            break;
        }
    }
    n_val
}

fn _parse_hex(pch_cursor: &mut &str) -> u32 {
    let mut n_val: u32 = 0; // Modified: Specify the type to resolve ambiguity
    while let Some(ch_now) = pch_cursor.chars().next() {
        let ch_now = ch_now.to_ascii_uppercase(); // Modified: Use char::to_ascii_uppercase for case conversion
        if ch_now >= '0' && ch_now <= '9' || ch_now >= 'A' && ch_now <= 'F' {
            let nybble_value = if ch_now >= 'A' { (ch_now as u32) - ('A' as u32) + 10 } else { (ch_now as u32) - ('0' as u32) };
            n_val = n_val.checked_shl(4).and_then(|v| v.checked_add(nybble_value)).unwrap_or(u32::MAX); // Modified: Use checked arithmetic to avoid overflow
            *pch_cursor = &pch_cursor[1..];
        } else {
            break;
        }
    }
    n_val
}

fn parse_ipv4_or_ipv6(ppsz_text: &mut &str, aby_addr: Option<&mut [u8; 16]>, pn_port: Option<&mut u16>, pb_is_ipv6: Option<&mut bool>) -> bool {
    let mut binding = [0u8; 16]; // Modified: Create a longer-lived value to avoid temporary value being dropped
    let mut aby_addr_local = aby_addr.as_mut().unwrap_or(binding); // Modified: Dereference binding to match the expected type
    let mut aby_dummy_addr = [0u8; 16];

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

    if aby_addr.as_ref().is_none() { // Modified: Use as_ref() to borrow the contents of aby_addr
        *aby_addr_local = &mut aby_dummy_addr; // Modified: Directly assign the reference
    }

    if !b_is_ipv6_local {
        let mut pby_addr_cursor = aby_addr_local.as_mut_ptr();
        let mut n_val;
        let mut psz_text_before = *ppsz_text;
        n_val = _parse_decimal(ppsz_text);
        if ppsz_text.chars().next() != Some('.') || n_val > 255 || psz_text_before == *ppsz_text {
            return false;
        }
        unsafe { *pby_addr_cursor = n_val as u8; }
        *ppsz_text = &ppsz_text[1..];
        unsafe { pby_addr_cursor = pby_addr_cursor.offset(1); } // Modified: Use pointer arithmetic within an `unsafe` block

        psz_text_before = *ppsz_text;
        n_val = _parse_decimal(ppsz_text);
        if ppsz_text.chars().next() != Some('.') || n_val > 255 || psz_text_before == *ppsz_text {
            return false;
        }
        unsafe { *pby_addr_cursor = n_val as u8; }
        *ppsz_text = &ppsz_text[1..];
        unsafe { pby_addr_cursor = pby_addr_cursor.offset(1); } // Modified: Use pointer arithmetic within an `unsafe` block

        psz_text_before = *ppsz_text;
        n_val = _parse_decimal(ppsz_text);
        if ppsz_text.chars().next() != Some('.') || n_val > 255 || psz_text_before == *ppsz_text {
            return false;
        }
        unsafe { *pby_addr_cursor = n_val as u8; }
        *ppsz_text = &ppsz_text[1..];
        unsafe { pby_addr_cursor = pby_addr_cursor.offset(1); } // Modified: Use pointer arithmetic within an `unsafe` block

        psz_text_before = *ppsz_text;
        n_val = _parse_decimal(ppsz_text);
        if n_val > 255 || psz_text_before == *ppsz_text {
            return false;
        }
        unsafe { *pby_addr_cursor = n_val as u8; }
        unsafe { pby_addr_cursor = pby_addr_cursor.offset(1); } // Modified: Use pointer arithmetic within an `unsafe` block

        if ppsz_text.chars().next() == Some(':') && pn_port.is_some() {
            let mut us_port_network = 0u16;
            *ppsz_text = &ppsz_text[1..];
            psz_text_before = *ppsz_text;
            n_val = _parse_decimal(ppsz_text);
            if n_val > 65535 || psz_text_before == *ppsz_text {
                return false;
            }
            us_port_network = ((n_val & 0xff00) >> 8) as u8 as u16 | ((n_val & 0xff) as u8 as u16) << 8;
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
        let mut pby_addr_cursor = aby_addr_local.as_mut_ptr();
        let mut pby_zeros_loc = ptr::null_mut();
        let mut b_ipv4_detected = false;
        let mut n_idx = 0;

        if pch_open_bracket.is_some() {
            *ppsz_text = &ppsz_text[pch_open_bracket.unwrap() + 1..];
        }

        while n_idx < 8 {
            let mut psz_text_before = *ppsz_text;
            let n_val = _parse_hex(ppsz_text);
            if psz_text_before == *ppsz_text {
                if !pby_zeros_loc.is_null() {
                    if pby_zeros_loc == pby_addr_cursor {
                        n_idx -= 1;
                        break;
                    }
                    return false;
                }
                if ppsz_text.chars().next() != Some(':') {
                    return false;
                }
                if n_idx == 0 {
                    *ppsz_text = &ppsz_text[1..];
                    if ppsz_text.chars().next() != Some(':') {
                        return false;
                    }
                }
                pby_zeros_loc = pby_addr_cursor;
                *ppsz_text = &ppsz_text[1..];
            } else {
                if ppsz_text.chars().next() == Some('.') {
                    let mut psz_text_local = psz_text_before;
                    let mut aby_addr_local = [0u8; 16];
                    let mut b_is_ipv6_local = false;
                    let b_parse_result_local = parse_ipv4_or_ipv6(&mut psz_text_local, Some(&mut aby_addr_local), None, Some(&mut b_is_ipv6_local));
                    *ppsz_text = psz_text_local;
                    if !b_parse_result_local || b_is_ipv6_local {
                        return false;
                    }
                    unsafe {
                        *pby_addr_cursor = aby_addr_local[0];
                        pby_addr_cursor = pby_addr_cursor.offset(1);
                        *pby_addr_cursor = aby_addr_local[1];
                        pby_addr_cursor = pby_addr_cursor.offset(1);
                        *pby_addr_cursor = aby_addr_local[2];
                        pby_addr_cursor = pby_addr_cursor.offset(1);
                        *pby_addr_cursor = aby_addr_local[3];
                        pby_addr_cursor = pby_addr_cursor.offset(1);
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
                    pby_addr_cursor = pby_addr_cursor.offset(1);
                    *pby_addr_cursor = (n_val & 0xff) as u8;
                    pby_addr_cursor = pby_addr_cursor.offset(1);
                }
                if ppsz_text.chars().next() == Some(':') {
                    *ppsz_text = &ppsz_text[1..];
                } else {
                    break;
                }
            }
            n_idx += 1;
        }

        if !pby_zeros_loc.is_null() {
            let n_head = (pby_zeros_loc as usize - aby_addr_local.as_mut_ptr() as usize) / std::mem::size_of::<u8>();
            let n_tail = n_idx * 2 - n_head;
            let n_zeros = 16 - n_tail - n_head;
            unsafe {
                ptr::copy(pby_addr_cursor.offset(-(n_tail as isize)), pby_addr_cursor.offset(n_zeros as isize), n_tail);
                ptr::write_bytes(pby_zeros_loc, 0, n_zeros);
            }
        }

        if b_ipv4_detected {
            const ABY_PFX: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0, 0];
            if unsafe { ptr::eq(aby_addr_local.as_ptr(), ABY_PFX.as_ptr()) } {
                return false;
            }
        }

        if pch_open_bracket.is_some() {
            if ppsz_text.chars().next() != Some(']') {
                return false;
            }
            *ppsz_text = &ppsz_text[1..];
        }

        if ppsz_text.chars().next() == Some(':') && pn_port.is_some() {
            let mut us_port_network = 0u16;
            *ppsz_text = &ppsz_text[1..];
            let mut psz_text_before = *ppsz_text;
            let n_val = _parse_decimal(ppsz_text);
            if n_val > 65535 || psz_text_before == *ppsz_text {
                return false;
            }
            us_port_network = ((n_val & 0xff00) >> 8) as u8 as u16 | ((n_val & 0xff) as u8 as u16) << 8;
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

fn parse_ipv4_or_ipv6_2(psz_text: &str, aby_addr: Option<&mut [u8; 16]>, pn_port: Option<&mut u16>, pb_is_ipv6: Option<&mut bool>) -> bool {
    let mut psz_text_local = psz_text;
    parse_ipv4_or_ipv6(&mut psz_text_local, aby_addr, pn_port, pb_is_ipv6)
}

fn htons(us: u16) -> u16 {
    ((us & 0xff) << 8) | ((us & 0xff00) >> 8)
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
    let mut b_success;

    println!("Test case '{}'", psz_test);
    let mut psz_text_cursor = psz_test;
    b_success = parse_ipv4_or_ipv6(&mut psz_text_cursor, Some(&mut aby_addr), Some(&mut n_port), Some(&mut b_is_ipv6));
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