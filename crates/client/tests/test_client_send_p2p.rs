use sremp_client::domain::{UiCommand, UiEvent};
use sremp_core::{
    chat::messages::{Message, SharedMessage},
    identity::UserIdentity,
};

use std::{net::SocketAddr, str::FromStr};

use chrono::Utc;
use env_logger::fmt::ConfigurableFormat;
use fork::{Fork, fork};
use log::{debug, info};
use ntest::timeout;

fn wait(dur: u64) {
    let dur = std::time::Duration::from_millis(dur);
    std::thread::sleep(dur);
}

fn ack_evt(evt: &UiEvent) {
    info!("TEST ACK UiEvent {evt}");
}

fn is_socket_bound_tcp(sock: &SocketAddr) -> bool {
    let b = std::net::TcpListener::bind(sock).is_err();
    info!("Socket {sock} is bound: {b}");
    b
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

fn setup_logging(prefix: Option<&'static str>) {
    let mut l = env_logger::builder();

    l.filter_level(log::LevelFilter::Info);

    let mut fmt = ConfigurableFormat::default();
    if let Some(prefix) = prefix {
        fmt.suffix(prefix);
    }

    l.parse_default_env()
        .format(move |f, r| fmt.format(f, r))
        .init();
}

// NOTE: This is the first time I'm doing automated testing for client functionality with fork().
// The idea is that i have two processes that run my test code to talk over a loopback socket, but
// i'm not sure if that actually works for tests like this. I guess i can call this an integration
// test?
#[test]
#[timeout(500)]
fn test_client_send_p2p() {
    // surely nobody uses that specific port
    let lsock: SocketAddr = SocketAddr::from_str("127.0.0.1:31048").unwrap();

    let role = fork().unwrap();

    let mut rt = tokio::runtime::Runtime::new().expect("could not create tokio runtime");
    let (ui_tx, ui_rx) = start_client(&mut rt);
    match role {
        Fork::Parent(_) => {
            setup_logging(Some(" | P\n"));
            let iden = UserIdentity::create("parent").unwrap();

            ui_tx
                .send_blocking(UiCommand::SetIdentity(Some(iden.clone().into())))
                .unwrap();
            // NOTE: set identity currently causes two events, the direct response and that the working copy was updated
            ack_evt(&ui_rx.recv_blocking().unwrap());
            ack_evt(&ui_rx.recv_blocking().unwrap());

            ui_tx
                .send_blocking(UiCommand::StartListener(lsock))
                .unwrap();
            ack_evt(&ui_rx.recv_blocking().unwrap());

            // NOTE: we need to wait until we use is_socket_bound_tcp because it steals
            // our socket otherwise
            wait(100);
            assert!(is_socket_bound_tcp(&lsock));

            // TODO: assert that a connection is made

            ack_evt(&ui_rx.recv_blocking().unwrap()); // set identities
            info!("Waiting for connection established event");
            let evt = ui_rx.recv_blocking().unwrap();
            ack_evt(&evt);
            if let UiEvent::ConnectionEstablished(remote_sock, cid) = evt {
                assert_ne!(remote_sock, lsock);

                // TODO: check and accept identity
                // Now that we have established a connection and gotten their identity, we need to do
                // trust-on-first use. For this test, we just set the identity to trusted.
                ui_tx
                    .send_blocking(UiCommand::TrustContact(
                        cid.clone(),
                        sremp_core::identity::Trust::Trusted,
                    ))
                    .unwrap();

                info!("starting chat");
                ui_tx
                    .send_blocking(UiCommand::StartChat(cid.clone()))
                    .unwrap();
                ack_evt(&ui_rx.recv_blocking().unwrap());

                info!("selecting chat");
                ui_tx
                    .send_blocking(UiCommand::SelectChat(cid.clone()))
                    .unwrap();
                ack_evt(&ui_rx.recv_blocking().unwrap());

                info!("sending message");
                let msg: SharedMessage =
                    Message::new("your parents are worried", Utc::now(), iden.id()).into();

                ui_tx
                    .send_blocking(UiCommand::SendMessage(cid.clone(), msg))
                    .unwrap();
                ack_evt(&ui_rx.recv_blocking().unwrap());

                info!("receiving message");
                ack_evt(&ui_rx.recv_blocking().unwrap());

                ui_tx
                    .send_blocking(UiCommand::Disconnect(remote_sock))
                    .unwrap();
                ack_evt(&ui_rx.recv_blocking().unwrap());
            } else {
                panic!("No connection established?")
            }
        }
        Fork::Child => {
            setup_logging(Some(" | C\n"));
            // NOTE: This runs as a test with cargo test. Cargo test does not care about the return
            // status of a child process, and why should it. But that means that an error here is
            // not necessarily treated as a failed test!

            let iden = UserIdentity::create("child").unwrap();

            ui_tx
                .send_blocking(UiCommand::SetIdentity(Some(iden.clone().into())))
                .unwrap();
            // NOTE: set identity currently causes two events, the direct response and that the working copy was updated
            ack_evt(&ui_rx.recv_blocking().unwrap());
            ack_evt(&ui_rx.recv_blocking().unwrap());

            ui_tx.send_blocking(UiCommand::Connect(lsock)).unwrap();
            ack_evt(&ui_rx.recv_blocking().unwrap()); // set identities
            info!("Waiting for connection established event");
            let evt = ui_rx.recv_blocking().unwrap();
            ack_evt(&evt);

            debug!("entering conn");
            if let UiEvent::ConnectionEstablished(remote_sock, cid) = evt {
                info!("in conn");
                assert_eq!(remote_sock, lsock);

                // TODO: check and accept identity
                // Now that we have established a connection and gotten their identity, we need to do
                // trust-on-first use. For this test, we just set the identity to trusted.
                ui_tx
                    .send_blocking(UiCommand::TrustContact(
                        cid.clone(),
                        sremp_core::identity::Trust::Trusted,
                    ))
                    .unwrap();

                info!("starting chat");
                ui_tx
                    .send_blocking(UiCommand::StartChat(cid.clone()))
                    .unwrap();
                ack_evt(&ui_rx.recv_blocking().unwrap());

                info!("selecting chat");
                ui_tx
                    .send_blocking(UiCommand::SelectChat(cid.clone()))
                    .unwrap();
                ack_evt(&ui_rx.recv_blocking().unwrap());

                info!("receiving message");
                ack_evt(&ui_rx.recv_blocking().unwrap());

                info!("sending message");
                let msg: SharedMessage =
                    Message::new("your parents are worried", Utc::now(), iden.id()).into();

                ui_tx
                    .send_blocking(UiCommand::SendMessage(cid.clone(), msg))
                    .unwrap();
                ack_evt(&ui_rx.recv_blocking().unwrap());

                ui_tx
                    .send_blocking(UiCommand::Disconnect(remote_sock))
                    .unwrap();
                ack_evt(&ui_rx.recv_blocking().unwrap());
            } else {
                panic!("No connection established?")
            }
        }
    }
    wait(20);
}
