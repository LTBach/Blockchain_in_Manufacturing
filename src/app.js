import express, { json } from 'express';
import bodyParser from "body-parser";
import morgan from 'morgan';
import mongoose from 'mongoose';
import config from "./configs";
import apiRouter from "./routers";

mongoose.connect(config.mongodb.url, config.mongodb.options)
.then(function(){
    console.log("Connected mongodb");
}).catch(function(error){
    console.log("Connect error: ", error);
});

const app = express();

app.use(json())
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({extended: true}));
app.use(morgan("combined"));

const PORT = process.env.PORT || 3000;

app.get('/', async (req, res) => {
    res.json({ status: true, message: "Our node.js app works" })
});

app.use("/api", apiRouter);

app.listen(PORT, () => console.log(`App listening at port ${PORT}`));