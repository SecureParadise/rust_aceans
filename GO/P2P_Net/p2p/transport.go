package p2p

// Peer is an interface thet represent remote node
type Peer interface {
}

// Transport is anything that can handle the communicartion between nodesin the network
// This can be of form(TCP,UDP,WebSockets...)
type Transport interface {
}
