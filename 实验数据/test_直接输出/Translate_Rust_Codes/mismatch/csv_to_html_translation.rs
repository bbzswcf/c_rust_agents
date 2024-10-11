const INPUT: &str = 
    "Character,Speech\n\
    The multitude,The messiah! Show us the messiah!\n\
    Brians mother,<angry>Now you listen here! He's not the messiah; \
    he's a very naughty boy! Now go away!</angry>\n\
    The multitude,Who are you?\n\
    Brians mother,I'm his mother; that's who!\n\
    The multitude,Behold his mother! Behold his mother!";

fn main() {
    println!("<table>\n<tr><td>");
    for c in INPUT.chars() {
        match c {
            '\n' => print!("</td></tr>\n<tr><td>"),
            ',' => print!("</td><td>"),
            '<' => print!("&lt;"),
            '>' => print!("&gt;"),
            '&' => print!("&amp;"),
            _ => print!("{}", c),
        }
    }
    println!("</td></tr>\n</table>");
}