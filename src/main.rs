extern crate nix;
use std::io;
use std::net::UdpSocket;
use std::result::Result;
use nix::sys::signal;

extern fn handle(_:i32) {
  println!("Interrupted!");
  panic!();
}


fn setup() -> Result<i32, std::io::Error> {
    // Cleanup when terminated
    unsafe {
        let sig_action = signal::SigAction::new(
            signal::SigHandler::Handler(handle),
            signal::SaFlag::empty(),
            signal::SigSet::empty());
        signal::sigaction(signal::SIGINT, &sig_action);
    }



    
   let mut socket = try!(UdpSocket::bind("127.0.0.1:8888"));

    // read from the socket
    let mut buf = [0; 10];
    let (amt, src) = try!(socket.recv_from(&mut buf));

    // send a reply to the socket we received data from
    let buf = &mut buf[..amt];
    buf.reverse();
    try!(socket.send_to(buf, &src));

    
    return Ok(2);
}

fn main() {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

}
