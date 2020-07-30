const TCP = require('libp2p-tcp')
const Multiplex = require('libp2p-mplex')
const SECIO = require('libp2p-secio')

const DEFAULT_OPTS = {
    modules: {
        transport: [
            TCP
        ],
        connEncryption: [
            SECIO
        ],
        streamMuxer: [
            Multiplex
        ],
    }
}

module.exports = { DEFAULT_OPTS }

