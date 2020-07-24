use std::error::Error;
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpStream};
use std::{thread, time};

fn main() {
    match do_knock() {
        Err(_) => std::process::exit(1),
        Ok(_) => {}
    }
}
fn do_knock() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    if args.len() < 3 {
        return Err("Usage: knockout host port...".into());
    }
    let host = args.nth(1).expect("host not found");
    let mut ports = vec![];
    for port in args {
        ports.push(port);
    }
    let mut errored = false;
    for (i, port) in ports.iter().enumerate() {
        if i > 0 {
            thread::sleep(time::Duration::from_millis(1000));
        }
        let target = format!("{:}:{:}", host, port);
        print!("Knocking at {:}", target);
        let result = knock_at(target);
        match result {
            Err(err) => {
                println!(" ... failed ({})", err);
                errored = true;
            }
            _ => println!(" ... knocked"),
        }
    }
    print!("All done!");
    if errored {
        print!(" Some knocks have failed.");
        return Err("Some knocks have failed.".into());
    }
    println!("");
    Ok(())
}

fn knock_at(target: String) -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = target.parse()?;
    match TcpStream::connect_timeout(&addr, time::Duration::from_millis(1)) {
        Err(err) if err.kind() == ErrorKind::TimedOut => Ok(()),
        Err(err) => Err(err.into()),
        Ok(_) => return Ok(()),
    }
}
