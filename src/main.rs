mod ide;
mod textarea;
use ide::IDE;
// struct Document{
//     text_buffer;
// }

fn main() {
    let mut ide = IDE::new();
    ide.run();
}
