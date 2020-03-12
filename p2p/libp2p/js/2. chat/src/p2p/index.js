const Libp2p = require("libp2p");
const defaultsDeep = require("@nodeutils/defaults-deep");

const { DEFAULT_OPTS } = require("./options");

class P2PNode extends Libp2p {
	constructor (opts) {
		super(defaultsDeep(opts, DEFAULT_OPTS));
	}

}

module.exports = { P2PNode };

