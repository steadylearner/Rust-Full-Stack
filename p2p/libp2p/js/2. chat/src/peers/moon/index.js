const PeerId = require("peer-id");
const PeerInfo = require("peer-info");
const { P2PNode } = require("../../p2p");

const pull = require("pull-stream");
const Pushable = require("pull-pushable");
const p = Pushable();

const { blue, green } = require("chalk");
const emoji = require("node-emoji");

PeerId.createFromJSON(require("../ids/moonId"), (err, peerId) => {
	if (err) {
		throw err;
	}

	const chatProtocol = "/chat/1.0.0";

	const moon = emoji.get("moon");
	const earth = emoji.get("large_blue_circle");
	const incoming_envelope = emoji.get("incoming_envelope");

	const moonAd = "/ip4/127.0.0.1/tcp/10333";

	const peerInfo = new PeerInfo(peerId);
	peerInfo.multiaddrs.add(moonAd);

	const nodeListener = new P2PNode({ peerInfo });

	nodeListener.start((err) => {
		if (err) {
			throw err;
		}

		nodeListener.on("peer:connect", (peerInfo) => {
			console.log(
				blue(`${moon} "Moon could find Earth" ${earth} on: ${peerInfo.id.toB58String()}`)
			);
			console.log(
				`\n${moon} ${green("Moon waiting for message from Earth")} ${earth}`
			);
		});

		nodeListener.handle(chatProtocol, (protocol, conn) => {
			// 1. To connect to the peer 'earth'
			pull(
				p,
				conn
			);
			pull(
				conn,
				pull.map((data) => {
					return data.toString("utf8").replace("\n", "");
				}),
				pull.drain(console.log)
			);

			// 2. To have a chat with "earth"
			process.stdin.setEncoding("utf8");
			process.openStdin().on("data", (chunk) => {
				const data = blue(`\n${moon} sent ${chunk.toString()}`) + green(`\n${incoming_envelope} respond to it. `) + `Mr. ${earth}`;
				p.push(data);
			});
		});

		console.log(`${moon} ${blue("Moon ready and Listening on:")}`);
		peerInfo.multiaddrs.forEach((ma) => {
			console.log(ma.toString() + "/p2p/" + peerId.toB58String());
		});
		console.log(`\n${moon} ${blue("Moon trying to connect with Earth")} ${earth}`);
	});
});
