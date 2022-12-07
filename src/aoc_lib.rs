use std::fs;

pub fn read_day(day: u8, variant: Option<u8>) -> Vec<String> {
    read_lines(make_file_name(true, day, variant))
}

#[allow(dead_code)]
pub fn read_probe(day: u8, variant: Option<u8>) -> Vec<String> {
    read_lines(make_file_name(false, day, variant))
}

pub fn make_file_name(is_day: bool, day: u8, variant: Option<u8>) -> String {
    let prefix = if is_day { "day_" } else { "probe_" };
    let suffix = if let Some(x) = variant {
        format!("_{}", x)
    } else {
        String::from("")
    };
    format!("input/{p}{d}{s}.txt", p = prefix, d = day, s = suffix)
}

pub fn read_lines(filename: String) -> Vec<String> {
    // println!("read file: {}", filename);
    let contents = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", &filename));

    contents.split('\n').map(String::from).collect()
}
