use crate::{peer::Peer, torrent::Torrent};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Piece {
    peers: HashSet<usize>,
    piece_i: usize,
    length: usize,
    hash: [u8; 20],
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.peers
            .len()
            .cmp(&other.peers.len())
            // tie-break by _random_ ordering of HashSet to avoid deterministic contention
            .then(self.peers.iter().cmp(other.peers.iter()))
            .then(self.hash.cmp(&other.hash))
            .then(self.length.cmp(&other.length))
            .then(self.piece_i.cmp(&other.piece_i))
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Piece {
    pub(crate) fn new(piece_i: usize, t: &Torrent, peers: &[Peer]) -> Self {
        let piece_hash = t.info.pieces.0[piece_i];
        let piece_size = if piece_i == t.info.pieces.0.len() - 1 {
            let md = t.length() % t.info.plength;
            if md == 0 {
                t.info.plength
            } else {
                md
            }
        } else {
            t.info.plength
        };

        let peers = peers
            .iter()
            .enumerate()
            .filter_map(|(peer_i, peer)| peer.has_piece(piece_i).then_some(peer_i))
            .collect();

        Self {
            peers,
            piece_i,
            length: piece_size,
            hash: piece_hash,
        }
    }

    pub(crate) fn peers(&self) -> &HashSet<usize> {
        &self.peers
    }

    pub(crate) fn index(&self) -> usize {
        self.piece_i
    }

    pub(crate) fn hash(&self) -> [u8; 20] {
        self.hash
    }

    pub(crate) fn length(&self) -> usize {
        self.length
    }
}
