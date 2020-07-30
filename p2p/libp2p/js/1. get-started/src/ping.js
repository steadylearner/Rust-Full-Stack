const process = require("process")
const multiaddr = require('multiaddr')
const PeerInfo = require('peer-info')
const PeerId = require('peer-id')

function pingRemotePeer(localPeer) {
    if (process.argv.length < 3) {
        return console.log('no remote peer address given, skipping ping')
    }
    const remoteAddr = multiaddr(process.argv[2])

    // Convert the multiaddress into a PeerInfo object
    const peerId = PeerId.createFromB58String(remoteAddr.getPeerId())
    const remotePeerInfo = new PeerInfo(peerId)
    remotePeerInfo.multiaddrs.add(remoteAddr)

    console.log('pinging remote peer at ', remoteAddr.toString())
    localPeer.ping(remotePeerInfo, (err, time) => {
        if (err) {
            return console.error('error pinging: ', err)
        }
        console.log(`pinged ${remoteAddr.toString()} in ${time}ms`)
    })
}

module.exports = {
    pingRemotePeer,
}

