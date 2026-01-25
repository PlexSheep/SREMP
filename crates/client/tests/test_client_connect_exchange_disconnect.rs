use async_channel::{Receiver, Sender};
use sremp_client::domain::{UiCommand, UiEvent};
use sremp_core::{
    chat::messages::{Message, SharedMessage},
    identity::{ContactId, UserIdentity},
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

fn killtimer(dur: u64) {
    std::thread::spawn(move || {
        wait(dur);
        std::process::abort()
    });
}

fn is_socket_bound_tcp(sock: &SocketAddr) -> bool {
    let b = std::net::TcpListener::bind(sock).is_err();
    info!("Socket {sock} is bound: {b}");
    b
}

fn start_client(rt: &mut tokio::runtime::Runtime) -> (Sender<UiCommand>, Receiver<UiEvent>) {
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

    l.filter_level(log::LevelFilter::Trace);

    let mut fmt = ConfigurableFormat::default();
    fmt.line_number(true)
        .timestamp(None)
        .module_path(false)
        .target(true);
    if let Some(prefix) = prefix {
        fmt.suffix(prefix);
    }

    l.parse_default_env()
        .format(move |f, r| fmt.format(f, r))
        .init();
}

macro_rules! assert_event {
    ($evt:expr, $var:pat) => {
        let evt = $evt;
        info!("TEST: Waiting for the next ui event");

        assert!(matches!(evt, $var));

        info!("TEST: ACK UiEvent {}", evt);
    };
}

fn prepare_for_chat(cid: ContactId, ui_tx: &Sender<UiCommand>, ui_rx: &Receiver<UiEvent>) {
    info!("starting chat");
    ui_tx
        .send_blocking(UiCommand::StartChat(cid.clone()))
        .unwrap();
    assert_event!(&ui_rx.recv_blocking().unwrap(), UiEvent::LoadedChats(_));

    info!("selecting chat");
    ui_tx
        .send_blocking(UiCommand::SelectChat(cid.clone()))
        .unwrap();
    assert_event!(&ui_rx.recv_blocking().unwrap(), UiEvent::OpenChat(_));
}

fn get_identity(ui_tx: &Sender<UiCommand>, ui_rx: &Receiver<UiEvent>) -> UserIdentity {
    let iden = UserIdentity::create("parent").unwrap();

    ui_tx
        .send_blocking(UiCommand::SetIdentity(Some(iden.clone().into())))
        .unwrap();
    // NOTE: set identity currently causes two events, the direct response and that the working copy was updated
    assert_event!(
        &ui_rx.recv_blocking().unwrap(),
        UiEvent::SetKnownIdentities(_)
    );
    assert_event!(&ui_rx.recv_blocking().unwrap(), UiEvent::IdentitySet(_));

    iden
}

fn send_msg(
    msg: &str,
    iden: &UserIdentity,
    cid: ContactId,
    ui_tx: &Sender<UiCommand>,
    ui_rx: &Receiver<UiEvent>,
) {
    info!("sending message");
    let msg: SharedMessage = Message::new(msg, Utc::now(), iden.id()).into();

    ui_tx
        .send_blocking(UiCommand::SendMessage(cid.clone(), msg))
        .unwrap();
    assert_event!(
        &ui_rx.recv_blocking().unwrap(),
        UiEvent::MessageSent(_, _, _)
    );
}

fn disconnect(remote_sock: SocketAddr, ui_tx: &Sender<UiCommand>, ui_rx: &Receiver<UiEvent>) {
    ui_tx
        .send_blocking(UiCommand::Disconnect(remote_sock))
        .unwrap();
    assert_event!(
        &ui_rx.recv_blocking().unwrap(),
        UiEvent::ConnectionLost(_, _)
    );
}

// NOTE: This is the first time I'm doing automated testing for client functionality with fork().
// The idea is that i have two processes that run my test code to talk over a loopback socket, but
// i'm not sure if that actually works for tests like this. I guess i can call this an integration
// test?
#[test]
#[timeout(500)]
fn test_client_connect_exchange_disconnect() {
    // surely nobody uses that specific port
    let lsock: SocketAddr = SocketAddr::from_str("127.0.0.1:31048").unwrap();

    let role = fork().unwrap();
    killtimer(600);

    let mut rt = tokio::runtime::Runtime::new().expect("could not create tokio runtime");
    let (ui_tx, ui_rx) = start_client(&mut rt);
    match role {
        Fork::Parent(_) => {
            setup_logging(Some(" | P\n"));

            let iden = get_identity(&ui_tx, &ui_rx);

            ui_tx
                .send_blocking(UiCommand::StartListener(lsock))
                .unwrap();
            assert_event!(&ui_rx.recv_blocking().unwrap(), UiEvent::ListenerStarted(_));

            // NOTE: we need to wait until we use is_socket_bound_tcp because it steals
            // our socket otherwise
            wait(100);
            assert!(is_socket_bound_tcp(&lsock));

            info!("Waiting for connection established event");
            assert_event!(
                &ui_rx.recv_blocking().unwrap(),
                UiEvent::SetKnownIdentities(_) // new identity from peer
            );
            let evt = ui_rx.recv_blocking().unwrap();
            assert_event!(&evt, UiEvent::ConnectionEstablished(_, _));
            if let UiEvent::ConnectionEstablished(remote_sock, cid) = evt {
                assert_ne!(remote_sock, lsock);

                // Now that we have established a connection and gotten their identity, we need to do
                // trust-on-first use. For this test, we just set the identity to trusted.
                ui_tx
                    .send_blocking(UiCommand::TrustContact(
                        cid.clone(),
                        sremp_core::identity::Trust::Trusted,
                    ))
                    .unwrap();

                prepare_for_chat(cid.clone(), &ui_tx, &ui_rx);

                send_msg("Wer das liest ist doof", &iden, cid, &ui_tx, &ui_rx);

                info!("receiving message");
                assert_event!(
                    &ui_rx.recv_blocking().unwrap(),
                    UiEvent::SetKnownIdentities(_)
                );

                disconnect(remote_sock, &ui_tx, &ui_rx);
            } else {
                unreachable!()
            }
        }
        Fork::Child => {
            setup_logging(Some(" | C\n"));
            // NOTE: This runs as a test with cargo test. Cargo test does not care about the return
            // status of a child process, and why should it. But that means that an error here is
            // not necessarily treated as a failed test!

            let iden = get_identity(&ui_tx, &ui_rx);

            ui_tx.send_blocking(UiCommand::Connect(lsock)).unwrap();

            info!("Waiting for connection established event");
            assert_event!(
                &ui_rx.recv_blocking().unwrap(),
                UiEvent::SetKnownIdentities(_) // new identity from peer
            );
            let evt = ui_rx.recv_blocking().unwrap();
            assert_event!(&evt, UiEvent::ConnectionEstablished(_, _));
            if let UiEvent::ConnectionEstablished(remote_sock, cid) = evt {
                assert_eq!(remote_sock, lsock);

                // Now that we have established a connection and gotten their identity, we need to do
                // trust-on-first use. For this test, we just set the identity to trusted.
                ui_tx
                    .send_blocking(UiCommand::TrustContact(
                        cid.clone(),
                        sremp_core::identity::Trust::Trusted,
                    ))
                    .unwrap();

                prepare_for_chat(cid.clone(), &ui_tx, &ui_rx);

                info!("receiving message");
                assert_event!(&ui_rx.recv_blocking().unwrap(), UiEvent::LoadedChats(_));

                send_msg(
                    "hallo 👉👈 富士山はロボトですか。",
                    &iden,
                    cid,
                    &ui_tx,
                    &ui_rx,
                );

                disconnect(remote_sock, &ui_tx, &ui_rx);
            } else {
                unreachable!()
            }
        }
    }
}
