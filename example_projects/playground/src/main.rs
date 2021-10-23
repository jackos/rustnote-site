use Color::Hsv;
use Color::Rgb;
use Message::ChangeColor;
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    ChangeColor(Color),
}

impl Message {
    fn print_me(&self) {
        match self {
            ChangeColor(Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
            ChangeColor(Hsv(r, g, b)) => {
                println!(
                    "Change the color to hue {}, saturation {}, and value {}",
                    r, g, b
                )
            }
        }
    }
}

fn main() {
    let msg = ChangeColor(Rgb(25, 50, 50));
    msg.print_me();
    let msg = ChangeColor(Hsv(50, 45, 180));
    msg.print_me();
}
