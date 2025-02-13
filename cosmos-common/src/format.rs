use num_format::{Locale, ToFormattedString};

pub fn bytes_to_pretty(bytes: &u64, add_bytes: bool) -> String {
    let mut steps = 0;
    let mut val: f64 = bytes.clone() as f64;

    while val > 1024. && steps <= 8 {
        val = val / 1024.;
        steps += 1;
    }

    let unit = match steps {
        0 => "B",
        1 => "KB",
        2 => "MB",
        3 => "GB",
        4 => "TB",
        5 => "PB",
        6 => "EB",
        7 => "ZB",
        8 => "YB",
        _ => "Not Supported",
    };

    if add_bytes {
        let bytes_str = bytes.to_formatted_string(&Locale::en); //TODO: Accept locale as a parameter.
        return format!("{:.2} {} ({} bytes)", val, unit, bytes_str);
    } else {
        return format!("{:.2} {}", val, unit);
    }
}
