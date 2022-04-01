use std::collections::HashMap;
use std::str::FromStr;
fn main() {
    let qr_string = "00020101021226660014ID.LINKAJA.WWW011893600911002134900102152011130911280020303UME51450015ID.OR.GPNQR.WWW0215ID20210981903520303UME520454995802ID5906kai-vm6007Jakarta6105401176217011312124776878575303360540421006304E98F";
    let result = parsing_qris(qr_string);
    match result {
        Ok(v) => println!("{}",v.get("54").unwrap()[1]),
        Err(e) => println!("{}",e),
    };
}


fn parsing_qris(mut qr_string: &str) -> Result<HashMap<String, [String;2]>, String> {
    let mut result = HashMap::<String, [String;2]>::new();
    let check_first = &qr_string[..2];
    if check_first != "00" {
        return Err("Invalid QR String".to_string());
    }
    let mut buff_length = qr_string.len();
    while buff_length > 0 {
        let guard = &qr_string[..4];
        let size_as_str = &guard[2..];
        let length;
        if &size_as_str[0..1] == "0" {
            length = usize::from_str(&size_as_str[1..2]).unwrap();
        }else {
            length = usize::from_str(size_as_str).unwrap();
        }
        let end_data_length: usize = 4 + length;
        let data = &qr_string[4..end_data_length];
        let key_data = &guard[0..2];
        result.insert(key_data.to_string(), [size_as_str.to_string(),data.to_string()]);
        qr_string = &qr_string[end_data_length..];
        buff_length = qr_string.len();
        
    }
    Ok(result)
}