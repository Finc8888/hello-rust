fn main() {
    let song = get_song_text();
    println!("{}", song);
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
        "tenth", "eleventh", "twelfth",
    ];
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
    return song;
}
