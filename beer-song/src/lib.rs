pub fn verse(n: u32) -> String {
    if n == 2 {
        return "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.".to_string();
    }
    else if n == 1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.".to_string();
    }
    else if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.".to_string();
    }
    let minus = n - 1;
    return format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {minus} bottles of beer on the wall.");
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s : String = "".to_string();
    let mut i: u32 = start;
    loop {
        if end > i {
            break;
        }
        if s != "".to_string(){
            s.push_str("\n\n");
        }
        s.push_str(verse(i).as_str());
        i -= 1;
    }
    return s;
}
