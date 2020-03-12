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
    } catch(e) {
        console.log(e);
    }
};

module.exports = useMongo
