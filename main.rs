use std::thread;
use std::time::Duration;

fn main() {
    println!("April: what distro you use?");
    thread::sleep(Duration::from_millis(1500));
    println!("Moe: debian");
    thread::sleep(Duration::from_millis(500));
    println!("April: DEBIAN???");
    thread::sleep(Duration::from_millis(2000));
    println!("Moe: Lesbian");
}