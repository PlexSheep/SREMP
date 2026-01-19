use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

use sremp_client::domain::{UiCommand, UiEvent};

use fork::{Fork, fork};
use sremp_core::identity::UserIdentity;

fn wait(dur: u64) {
    let dur = std::time::Duration::from_mins(dur);
    std::thread::sleep(dur);
}

fn ack_evt(evt: UiEvent, prefix: Option<&str>) {
    if let Some(prefix) = prefix {
        print!("{prefix}: ")
    }
    println!("ACK UiEvent {evt}");
}

fn is_socket_bound_tcp(sock: &SocketAddr) -> bool {
    std::net::TcpListener::bind(sock).is_ok()
}

fn start_client(
    rt: &mut tokio::runtime::Runtime,
) -> (
    async_channel::Sender<UiCommand>,
    async_channel::Receiver<UiEvent>,
) {
    let (net_command_tx, net_command_rx) = async_channel::unbounded();
    let (net_event_tx, net_event_rx) = async_channel::unbounded();

    let (ui_command_tx, ui_command_rx) = async_channel::unbounded();
    let (ui_event_tx, ui_event_rx) = async_channel::unbounded();

    let net_domain = sremp_core::domain::NetworkDomain::new();
    net_domain
        .start(net_command_rx, net_event_tx, rt)
        .expect("could not start network domain");

    let domain = sremp_client::domain::ClientDomain::new();
    domain
        .start(net_command_tx, net_event_rx, ui_command_rx, ui_event_tx, rt)
        .expect("could not start application domain");

    (ui_command_tx, ui_event_rx)
}

// TODO: find a way to time this out! Sending and receiving a message over localhost should take
// a fraction of a second at most.
// NOTE: This is the first time I'm doing automated testing for client functionality with fork().
// The idea is that i have two processes that run my test code to talk over a loopback socket, but
// i'm not sure if that actually works for tests like this. I guess i can call this an integration
// test?
#[test]
fn test_client_send_p2p() {
    if let Err(err) = fork() {
        panic!("{err}")
    }
    let mut rt = tokio::runtime::Runtime::new().expect("could not create tokio runtime");
    let (ui_tx, ui_rx) = start_client(&mut rt);

    // surely nobody uses that specific port
    let lsock: SocketAddr = SocketAddr::from_str("127.0.0.1:31048").unwrap();

    match fork() {
        Ok(Fork::Parent(_)) => {
            println!("P: Creating identity");
            let iden = UserIdentity::create("parent").unwrap();

            println!("P: Setting identity");
            ui_tx
                .send_blocking(UiCommand::SetIdentity(Some(iden.into())))
                .unwrap();
            // NOTE: set identity currently causes two events, the direct response and that the working copy was updated
            ack_evt(ui_rx.recv_blocking().unwrap(), Some("P"));
            ack_evt(ui_rx.recv_blocking().unwrap(), Some("P"));

            println!("P: Starting listener");
            ui_tx
                .send_blocking(UiCommand::StartListener(lsock))
                .unwrap();
            ack_evt(ui_rx.recv_blocking().unwrap(), Some("P"));

            assert!(is_socket_bound_tcp(&lsock));
        }
        Ok(Fork::Child) => {
            // NOTE: This runs as a test with cargo test. Cargo test does not care about the return
            // status of a child process, and why should it. But that means that an error here is
            // not necessarily treated as a failed test!

            println!("C: Creating identity");
            let iden = UserIdentity::create("child").unwrap();

            println!("C: Setting identity");
            ui_tx
                .send_blocking(UiCommand::SetIdentity(Some(iden.into())))
                .unwrap();
            // NOTE: set identity currently causes two events, the direct response and that the working copy was updated
            ack_evt(ui_rx.recv_blocking().unwrap(), Some("P"));
            ack_evt(ui_rx.recv_blocking().unwrap(), Some("P"));

            println!("C: Connect to listener");
            ui_tx.send_blocking(UiCommand::Connect(lsock)).unwrap();
            ack_evt(ui_rx.recv_blocking().unwrap(), Some("P"));
        }
        Err(err) => panic!("{err}"),
    }
}
