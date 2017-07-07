const https = require('https');
const express = require('express');
const fs = require('fs');
const app = express();

app.use(express.static('public'));

https.createServer({
    key: fs.readFileSync('../backend/configs/certs/ia.key'),
    cert: fs.readFileSync('../backend/configs/certs/ia.crt')
}, app).listen(3005, function () {
    console.log('Ready');
    console.log('https://localhost:3005');
});
