const express = require('express');
const app = express();
const port = 8080;

// SEE: https://stackabuse.com/building-a-rest-api-with-node-and-express/

app.get('/api/v1/hello', (req, res) => {
    name = req.query.name ?? "World"
    res.send(`Hello, ${name}!`);
});

app.listen(port, () => console.log(`Hello world app listening on port ${port}!`))
