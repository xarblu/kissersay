use std::io::{self, BufRead, BufReader};
use std::cmp::Ordering::Greater;
use clap::Parser;

const KISSER: &str = 
"⠀⡿⠀⠀⠀⠀⠀⠈⢷⡀⢻⡀⠀⠀⠙⢦⣰⠏⠀⠀⠀⠀⠀⠀⢸⠀
⠀⡇⠀⠀⠀⠀⠀⠀⢀⣻⠞⠛⠀⠀⠀⠀⠻⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⡇⠀⠀⠀⠀⠀⠀⠛⠓⠒⠓⠓⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀
⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠀
⠀⢿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⠀⠀⢀⡟⠀
⠀⠘⣇⠀⠘⣿⠋⢹⠛⣿⡇⠀⠀⠀⠀⣿⣿⡇⠀⢳⠉⠀⣠⡾⠁⠀
⣦⣤⣽⣆⢀⡇⠀⢸⡇⣾⡇⠀⠀⠀⠀⣿⣿⡷⠀⢸⡇⠐⠛⠛⣿⠀
⠹⣦⠀⠀⠸⡇⠀⠸⣿⡿⠁⢀⡀⠀⠀⠿⠿⠃⠀⢸⠇⠀⢀⡾⠁⠀
⠀⠈⡿⢠⢶⣡⡄⠀⠀⠀⠀⠉⠁⠀⠀⠀⠀⠀⣴⣧⠆⠀⢻⡄⠀⠀
⠀⢸⠃⠀⠘⠉⠀⠀⠀⠠⣄⡴⠲⠶⠴⠃⠀⠀⠀⠉⡀⠀⠀⢻⡄⠀
⠀⠘⠒⠒⠻⢦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⠞⠛⠒⠛⠋⠁⠀
⠀⠀⠀⠀⠀⠀⠸⣟⠓⠒⠂⠀⠀⠀⠀⠀⠈⢷⡀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠙⣦⠀⠀⠀⠀⠀⠀⠀⠀⠈⢷⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣼⣃⡀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣆⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠉⣹⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡆
";

// let boykisser say stuff
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // the message
    #[arg(index = 1, default_value_t = String::new())]
    msg: String,
}

// get width of the biggest line in s
fn max_line_width(s: &str) -> usize {
    let mut max: usize = 0;
    for line in s.lines() {
        let cur: usize = line.chars().count();
        match cur.cmp(&max) {
            Greater => max = cur,
            _ => continue,
        }
    };
    return max;
}

fn append_string_repeat(s: &mut String, append: &str, repeat: usize) {
    for _ in 0..repeat {
        s.push_str(append);
    }
}

// format the msg
fn format_msg(msg: &str) -> String {
    let msg_max_width: usize = max_line_width(&msg);
    let indent: usize = max_line_width(&KISSER) / 2;
    
    // create a msg box
    let mut msg_fmt: String = String::new();

    // indent
    append_string_repeat(&mut msg_fmt, " ", indent );
    
    // top line
    msg_fmt.push_str("+");
    append_string_repeat(&mut msg_fmt, "-", msg_max_width);
    msg_fmt.push_str("+\n");

    for line in msg.lines() {
        // indent
        append_string_repeat(&mut msg_fmt, " ", indent);
        
        msg_fmt.push_str("|");
        msg_fmt.push_str(line);
        append_string_repeat(&mut msg_fmt, " ", msg_max_width - line.chars().count());
        msg_fmt.push_str("|\n");
    }

    // indent
    append_string_repeat(&mut msg_fmt, " ", indent);

    // bottom line
    msg_fmt.push_str("+");
    append_string_repeat(&mut msg_fmt, "-", msg_max_width);
    msg_fmt.push_str("+\n");

    // indent
    append_string_repeat(&mut msg_fmt, " ", indent);

    // box arrow
    msg_fmt.push_str("|/\n\n");

    // add character
    msg_fmt.push_str(KISSER);

    return msg_fmt
}

fn main() {
    let args = Args::parse();

    let mut msg = String::new();
    
    if args.msg.len() > 0 {
        msg = args.msg;
    } else {
        let reader = BufReader::new(io::stdin());

        for line in reader.lines() {
            msg.push_str(&line.unwrap());
            msg.push_str("\n");
        }
    }

    print!("{}", format_msg(&msg));
}
