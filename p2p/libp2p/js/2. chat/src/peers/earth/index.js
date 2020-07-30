const PeerId = require("peer-id");
const PeerInfo = require("peer-info");
const { P2PNode } = require("../../p2p");

const pull = require("pull-stream");
const Pushable = require("pull-pushable");
const p = Pushable();

const async = require("async");
const { blue, green, bold } = require("chalk");
const emoji = require("node-emoji");


async.parallel([
	(callback) => {
		PeerId.createFromJSON(require("../ids/earthId"), (err, earthPeerId) => {
			if (err) {
				throw err;
			}
			callback(null, earthPeerId); // (err, ids)
		});
	},
	(callback) => {
		PeerId.createFromJSON(require("../ids/moonId"), (err, moonPeerId) => {
			if (err) {
				throw err;
			}
			callback(null, moonPeerId); // (err, ids)
		});
	}
], (err, ids) => {
	// earthPeerId = ids[0];
	// moonPeerId = ids[1];
	if (err) {
		console.log(err);
		throw err;
	}

	const chatProtocol = "/chat/1.0.0";

	const incoming_envelope = emoji.get("incoming_envelope");
	const earth = emoji.get("large_blue_circle");
	const moon = emoji.get("moon");

	const randomEarthAd = "/ip4/127.0.0.1/tcp/0";
	const moonAd = "/ip4/127.0.0.1/tcp/10333";

	const earthPeerInfo = new PeerInfo(ids[0]);
	earthPeerInfo.multiaddrs.add(randomEarthAd);

	const nodeDialer = new P2PNode({ peerInfo: earthPeerInfo });
	const moonPeerInfo = new PeerInfo(ids[1]);

	moonPeerInfo.multiaddrs.add(moonAd);

	nodeDialer.start((err) => {
		if (err) {
			console.log(err);
			throw err;
		}

		console.log(`${earth} ${blue("Earth Ready and Listening on:")} `);

		nodeDialer.dialProtocol(moonPeerInfo, chatProtocol, (err, conn) => {
			if (err) {
				throw err;
			}

			console.log(
				`\n${earth} ${blue("Earth dialed to Moon on protocol: /chat/1.0.0")}`
			);
			console.log(
				`${incoming_envelope} ${bold("Type a message and press enter. See what happens...")}`
			);

			// console.log(conn)

			// 1. Write operation. Data sent as a buffer
			pull(
				p,
				conn
			);

			// 2. Sink, data converted from buffer to utf8 string
			pull(
				conn,
				pull.map((data) => {
					return data.toString("utf8").replace("\n", "");
				}),
				pull.drain(console.log)
			);

			process.stdin.setEncoding("utf8");
			process.openStdin().on("data", (chunk) => {
				const data = blue(`\n${earth} sent ${chunk.toString()}`) + green(`\n${incoming_envelope} respond to it. `) + `Miss. ${moon}`;
				p.push(data);
			});
		});
	});
});

