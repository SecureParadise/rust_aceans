package p2p

import (
	"net"
	"sync"
)

type TCPTransport struct {
	//
	listenAdderess string
	listener       net.Listener
	mu             sync.RWMutex
	peers          map[net.Addr]Peer
}

func NewTCPTransport(listenAddr string) Transport {
	return &TCPTransport{
		listenAdderess: listenAddr,
	}
}
