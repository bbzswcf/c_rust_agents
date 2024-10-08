fn main() {
    let mut n = 99;

    while n > 2 {
        print!(
            "{n} bottles of beer on the wall, {n} bottles of beer.\n\
            Take one down and pass it around, {} bottles of beer on the wall.\n\n",
            n - 1
        );
        n -= 1;
    }

    print!(
        "2 bottles of beer on the wall, 2 bottles of beer.\n\
        Take one down and pass it around, 1 bottle of beer on the wall.\n\n\
        \
        1 bottle of beer on the wall, 1 bottle of beer.\n\
        Take one down and pass it around, no more bottles of beer on the wall.\n\n\
        \
        No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n"
    );
}