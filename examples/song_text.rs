fn main() {
    let song = get_song_text();
    println!("{}", song);
    for st in ["Hello world", "Cat and dog", "Good morning"].iter() {
        println!("{}", get_first_word(st));
    }
}
// FIXME Write myself func
fn claude_first_char_to_lowercase(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn get_song_text() -> String {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelth",
    ];
    let mut result_song = String::new();
    for (i, day) in days.iter().enumerate() {
        let first_str = format!("On the {} day of Christmas,\n", day);
        result_song.push_str(&first_str);
        result_song.push_str("my true love sent to me\n");
        for j in 0..i + 1 {
            if j == i {
                result_song.push_str(&format!("And {}\n", gifts[j]));
            } else {
                result_song.push_str(&format!("{}\n", gifts[j]));
            }
        }
        result_song.push_str("\n");
    }

    return result_song;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_song_text() {
        // Test equals the original text of song and the generated text of song
        let song = String::from(
            "
            On the first day of Christmas,
            my true love sent to me
            A partridge in a pear tree.

            On the second day of Christmas,
            my true love sent to me
            Two turtle doves,
            And a partridge in a pear tree.

            On the third day of Christmas,
            my true love sent to me
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the fourth day of Christmas,
            my true love sent to me
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the fifth day of Christmas,
            my true love sent to me
            Five gold rings,
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the sixth day of Christmas,
            my true love sent to me
            Six geese a-laying,
            Five gold rings,
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the seventh day of Christmas,
            my true love sent to me
            Seven swans a-swimming,
            Six geese a-laying,
            Five gold rings,
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the eighth day of Christmas,
            my true love sent to me
            Eight maids a-milking,
            Seven swans a-swimming,
            Six geese a-laying,
            Five gold rings,
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the ninth day of Christmas,
            my true love sent to me
            Nine ladies dancing,
            Eight maids a-milking,
            Seven swans a-swimming,
            Six geese a-laying,
            Five gold rings,
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the tenth day of Christmas,
            my true love sent to me
            Ten lords a-leaping,
            Nine ladies dancing,
            Eight maids a-milking,
            Seven swans a-swimming,
            Six geese a-laying,
            Five gold rings,
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the eleventh day of Christmas,
            my true love sent to me
            Eleven pipers piping,
            Ten lords a-leaping,
            Nine ladies dancing,
            Eight maids a-milking,
            Seven swans a-swimming,
            Six geese a-laying,
            Five gold rings,
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree.

            On the twelfth day of Christmas,
            my true love sent to me
            Twelve drummers drumming,
            Eleven pipers piping,
            Ten lords a-leaping,
            Nine ladies dancing,
            Eight maids a-milking,
            Seven swans a-swimming,
            Six geese a-laying,
            Five gold rings,
            Four calling birds,
            Three French hens,
            Two turtle doves,
            And a partridge in a pear tree!",
        );
        assert_eq!(get_song_text(), song);
    }
}
