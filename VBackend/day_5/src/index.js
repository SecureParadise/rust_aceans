// connect database

// Problem may arise while connecting to the database. 
// Therefore, we must wrap the database connection logic in a (try...catch block or use Promises) and async await 
// to handle potential connection errors.

// Database connection errors can occur due to:
//   - Network issues: The database is often located remotely, leading to potential network connectivity problems.
//   - Incorrect credentials: Invalid username, password, or host details.
//   - Database server unavailability: The database server might be down or overloaded.
//   - Firewall restrictions: Firewalls might block the connection to the database.
//   - Connection timeouts: The connection attempt might time out if it takes too long.
//   - Resource exhaustion: The database server or the client application might run out of resources.

// require('dotenv').config({path:'./env'})
import dotenv from "dotenv"
// import mongoose from "mongoose";
// import { DB_NAME } from "./constants.js";
import connectDB from "./db/dbConnectionIndex.js";
import { app } from "./app.js"  // Import the configured app from app.js

dotenv.config({
    path: './.env'  // Fixed path to .env file
})

connectDB()
.then(() => {
    app.listen(process.env.PORT || 8080, () => {
        console.log(`Server is running at http://localhost:${process.env.PORT || 8080}`);
    })
})
.catch((err) => {
    console.log("MONGO DB connection failed ", err);
})






/*
// ifee which executes immedetly

;(async ()=>{
    try{
        await mongoose.connect(`${process.env.MONGODB_URI}/${DB_NAME}`)
        app.on("error",(error)=>{
            console.log("Error: ",error);
        })
        app.listen(process.env.PORT,()=>{
            console.log(`App is listening on port ${process.env.PORT}`);
            
        })
    } catch(error){
        console.error("Error",error);
        throw err 
    }
})()

mongoose.connect()
*/