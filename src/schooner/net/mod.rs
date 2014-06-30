use super::events::*;
pub use self::types::*;
pub use self::netmgmt::*;

// Private stuff, shouldn't be used elsewhere.
pub mod parsers;
pub mod netmgmt;
mod macros;
mod listeners;
mod peer;
mod types;

#[deriving(Clone)]
pub struct Peers<'ps> {
    peer_configs: Box<Vec<NetPeerConfig<'ps>>>,
    msg_peers: Sender<(u64, RaftRpc)>,
    shutdown_send: Sender<u64>,
}

impl<'ps> Peers<'ps> {
    /*
     * Spawn the peer controller submodule.
     *
     * Arguments:
     *     conf: configuration for this node
     *     peer_configs: vector of configs for the peers to initiate connections with
     *     from_peers_send: port on which this peer controller will send network RPCs to Raft
     *     from_client_send: port on which client commands and a client reply channel will be sent
     *
     * Returns a `Peers` object that manages all the fancy network callbacks for you.
     */
    pub fn new(conf: NetPeerConfig, peer_configs: &Vec<NetPeerConfig>, from_peers_send: Sender<RaftMsg>, from_client_send: Sender<(ClientCmdReq, Sender<ClientCmdRes>)>) -> Peers {
        let (msg_peers, shutdown_send) = NetManager::new(conf, peer_configs, from_peers_send, from_client_send);
        let this = Peers {
            peer_configs: box peer_configs.clone(),
            msg_peers: msg_peers,
            shutdown_send: shutdown_send,
        };
        this
    }

    /*
     * Message every peer.
     *     msg: A raft RPC to send to all peers
     */
    pub fn msg_all_peers(&mut self, msg: RaftRpc) {
        for id in self.get_peer_ids().iter() {
            self.msg_peer(*id, msg.clone());
        }
    }

    /*
     * Message a single peer.
     *     id: id of peer
     *     msg: RPC for peer
     */
    pub fn msg_peer(&mut self, id: u64, msg: RaftRpc) -> bool {
        if self.get_peer_ids().contains(&id) {
            self.msg_peers.send((id, msg));
            true
        }
        else {
            false
        }
    }

    /*
     * Get the ids of the known peers for this network listener.
     */
    pub fn get_peer_ids(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        for conf in self.peer_configs.iter() {
            ids.push(conf.id);
        }
        ids
    }
}
