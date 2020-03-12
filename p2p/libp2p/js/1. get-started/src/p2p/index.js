const Libp2p = require('libp2p')
const Ping = require('libp2p/src/ping')

const defaultsDeep = require('@nodeutils/defaults-deep')

const { DEFAULT_OPTS } = require("./options");

class P2PNode extends Libp2p {
  constructor (opts) {
    super(defaultsDeep(opts, DEFAULT_OPTS))
  }

  ping (remotePeerInfo, callback) {
    const p = new Ping(this._switch, remotePeerInfo)

    p.on('ping', time => {
      p.stop() // stop sending pings
      callback(null, time)
    })
    p.on('error', callback)

    p.start()
  }
}

module.exports = { P2PNode }

