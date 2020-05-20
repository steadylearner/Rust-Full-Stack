// $docker volume create mongodbdata
// $docker run -d -v mongodbdata:/data/db --name mongo -p 27017:27017 mongo

// https://rominirani.com/docker-tutorial-series-part-7-data-volumes-93073a1b5b72

const mongoose = require('mongoose');

const config = require('config');
const mongoURI = config.get("mongoURI");

const useMongo = async function() {
    try {
        const _connection = await mongoose.connect(mongoURI, {
            useNewUrlParser: true,
            useUnifiedTopology: true,
            useCreateIndex: true,
            useFindAndModify: false,
        });

        console.log('MongoDB Connected...');

        // [Should] handle when database is stopped or not working. It shows TimeoutError.
        // https://medium.com/@vsvaibhav2016/best-practice-of-mongoose-connection-with-mongodb-c470608483f0
        // https://mongoosejs.com/docs/api/connection.html#connection_Connection-readyState
        // https://mongoosejs.com/docs/api/connection.html#connection_Connection-watch

        // MongoTimeoutError: Server selection timed out after 30000 ms
        //     at Timeout._onTimeout (/home/steadylearner/Desktop/code/site/Rust-Full-Stack/database/node_mongoose/node_modules/mongodb/lib/core/sdam/server_selection.js:308:9)
        //     at listOnTimeout (internal/timers.js:531:17)
        //     at processTimers (internal/timers.js:475:7) {
        // name: 'MongoTimeoutError',
        // reason: Error: connect ECONNREFUSED 0.0.0.0:27017
        //     at TCPConnectWrap.afterConnect [as oncomplete] (net.js:1054:14) {
        //     name: 'MongoNetworkError',
        //     [Symbol(mongoErrorContextSymbol)]: {}
        // },
        // [Symbol(mongoErrorContextSymbol)]: {}
        // }

        // Test above is handled by logger.debug(e) parts at controllers/emailController.js

        // Use this to develop. https://mongoosejs.com/docs/connections.html#multiple_connections
        // Should close whenver necessary.
        process.on('SIGINT', function () {
            mongoose.connection.close(function () {
                console.log("Mongoose default connection is disconnected due to application termination");
                process.exit(0);
            });
        });

    } catch(e) {
        console.log(e); // Save it to a log file.
    }
};

module.exports = useMongo
