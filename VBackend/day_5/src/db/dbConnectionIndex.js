// Best practice to connect database


import mongoose from "mongoose"
import { DB_NAME } from "../constants.js";
import dotenv from 'dotenv'

dotenv.config()



const connectDB = async ()=>{
    try{
        const connectionInstance = await mongoose.connect(`${process.env.MONGO_URI}/${DB_NAME}`)
        console.log(`\n MongoDB connected !! DB Host: ${connectionInstance.connection.host}`);
        
    } catch(error){
        console.log("  mongoDB connection Error",error);
        process.exit(1)
    }
}

export default connectDB

