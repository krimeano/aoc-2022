pub fn make_file_name(is_day: bool, day: u8, variant: Option<u8>) -> String {
    let prefix = if is_day { "day_" } else { "probe_" };
    let suffix = if let Some(x) = variant { format!("_{}", x) } else { String::from("") };
    format!("input/{p}{d}{s}.txt", p = prefix, d = day, s = suffix)
}
