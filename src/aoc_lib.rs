use std::fs;

pub fn make_file_name(is_day: bool, day: u8, variant: Option<u8>) -> String {
    let prefix = if is_day { "day_" } else { "probe_" };
    let suffix = if let Some(x) = variant { format!("_{}", x) } else { String::from("") };
    format!("input/{p}{d}{s}.txt", p = prefix, d = day, s = suffix)
}

pub fn read_lines(filename: String) -> Vec<String> {
    // println!("read file: {}", filename);
    let contents = fs::read_to_string(&filename)
        .expect(&format!("Something went wrong reading the file {}", &filename));

    contents.split("\n")
        .map(|x| String::from(x.trim()))
        .collect()
}
