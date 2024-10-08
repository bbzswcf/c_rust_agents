const INPUT: &str = "Character,Speech\n\
                    The multitude,The messiah! Show us the messiah!\n\
                    Brians mother,<angry>Now you listen here! He's not the messiah; \
                    he's a very naughty boy! Now go away!</angry>\n\
                    The multitude,Who are you?\n\
                    Brians mother,I'm his mother; that's who!\n\
                    The multitude,Behold his mother! Behold his mother!";

fn main() {
    println!("<table>\n<tr><td>");
    let mut in_angry_tag = false;
    let mut tag_buffer = String::new();

    for c in INPUT.chars() {
        match c {
            '\n' => {
                if in_angry_tag {
                    // Preserve newlines within <angry> tags
                    print!("{}", c);
                } else {
                    // Correctly format newlines outside <angry> tags
                    print!("</td></tr>\n<tr><td>");
                }
            },
            ',' => {
                if in_angry_tag {
                    // Preserve commas within <angry> tags
                    print!("{}", c);
                } else {
                    // Correctly format commas for table columns
                    print!("</td><td>");
                }
            },
            '<' => {
                // Start buffering the tag content to check if it's an <angry> tag
                tag_buffer.clear();
                tag_buffer.push(c);
            },
            '>' => {
                // Check if the buffered content is an <angry> or </angry> tag
                tag_buffer.push(c);
                if tag_buffer.eq_ignore_ascii_case("<angry>") {
                    in_angry_tag = true;
                } else if tag_buffer.eq_ignore_ascii_case("</angry>") {
                    in_angry_tag = false;
                }
                // Print the tag buffer as-is without escaping
                print!("{}", tag_buffer);
                tag_buffer.clear();
            },
            _ => {
                if !tag_buffer.is_empty() {
                    // Continue buffering the tag content
                    tag_buffer.push(c);
                } else {
                    if in_angry_tag {
                        // Print characters within <angry> tags as-is
                        print!("{}", c);
                    } else {
                        // Escape special characters outside of <angry> tags
                        match c {
                            '<' => print!("&lt;"),
                            '>' => print!("&gt;"),
                            '&' => print!("&amp;"),
                            _ => print!("{}", c),
                        }
                    }
                }
            },
        }
    }
    // Ensure any open tags are closed correctly at the end of the input
    if in_angry_tag {
        print!("</angry>");
    }
    println!("</td></tr>\n</table>");
}