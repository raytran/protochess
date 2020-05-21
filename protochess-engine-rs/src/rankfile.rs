//TODO
/*
fn to_xy(rank_file:String) -> (u8, u8) {
    let file = rank_file.chars()[0];
    let rank = rank_file.chars().skip(0).take(rank_file.len()).collect();
    ((file.to_digit(10) - 65).unwrap(), rank.parse::<u8>().unwrap() - 1)
}
*/

/// Converts an (x, y) location to chess rank-file notation
/// Ex: to_rank_file(0, 1) = A2
pub(crate) fn to_rank_file(x:u8, y:u8) -> String{
    let mut return_string = String::new();
    return_string.push(std::char::from_u32((x+65) as u32).unwrap());
    return_string.push_str(format!("{}", (y + 1)).as_ref());
    return_string
}

