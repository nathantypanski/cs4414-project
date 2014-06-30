use std::io::net::ip::SocketAddr;
use std::io::TcpStream;
use super::super::events::*;

// Bare RPC types. This is the incoming type before we set up channels
// to make Raft messages. Might not be necessary, provided we can setup
// those channels in the functions where the RPCs are built out of network
// bytes.
#[deriving(Decodable, Encodable, Eq, Clone, Show, PartialEq)]
pub enum RaftRpc {
    RpcARQ(AppendEntriesReq),
    RpcARS(AppendEntriesRes),
    RpcVRQ(VoteReq),
    RpcVRS(VoteRes),
    RpcStopReq,
}

pub enum MgmtMsg {
    // Peer ID and a TcpStream to attach to
    AttachStreamMsg(u64, TcpStream),
    SendMsg(RaftRpc),
    StopMsg,
}

#[deriving(Clone, Hash, Eq, Show, PartialEq)]
pub struct NetPeerConfig <'a> {
    pub id: u64,
    // Peer's Raft listening address, but not necessarily the port we will
    // get our request from. Hence the peer should send its id when it
    // makes its first connection to us.
    pub host_addr: &'a str,
    pub host_port: u16,
    // The port for this field is the peer's client listening port.
    pub client_addr: &'a str,
    pub client_port: u8
}
