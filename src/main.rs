use std::io::{self, BufRead, BufReader};
use std::cmp::Ordering::Greater;
use clap::Parser;

mod kissers;

// let boykisser say stuff
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // the message
    #[arg(index = 1, default_value_t = String::from("-"), help = "What to say (- reads from stdin)")]
    msg: String,

    // which kisser
    #[arg(short, long, default_value_t = String::from("boykisser"), help = format!("One of: {}", kissers::KISSERS))]
    kisser: String,
}

// append "s" with "append" repeated "repeat" times
fn append_string_repeat(s: &mut String, append: &str, repeat: usize) {
    for _ in 0..repeat {
        s.push_str(append);
    }
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

// format the msg
fn format_msg(msg: &str, kisser: &str) -> String {
    let msg_max_width: usize = max_line_width(&msg);
    let indent: usize = max_line_width(&kisser) / 2;
    
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
    msg_fmt.push_str(kisser);

    return msg_fmt
}

fn main() {
    let args = Args::parse();

    let mut msg = String::new();
    
    if args.msg == "-" {
        let reader = BufReader::new(io::stdin());

        for line in reader.lines() {
            msg.push_str(&line.unwrap());
            msg.push_str("\n");
        }
    } else {
        msg = args.msg;
    }

    let kisser = kissers::get_kisser(&args.kisser);

    if kisser.len() == 0 {
        println!("Invalid kisser - possible values: {}", kissers::KISSERS);
        return;
    }

    print!("{}", format_msg(&msg, &kisser));
}
