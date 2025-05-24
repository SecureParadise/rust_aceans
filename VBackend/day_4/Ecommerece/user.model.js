import mongoose from "mongoose";

const userSchema = new mongoose.Schema(
    {
        userName:{
            type: String,
            rerquired: true,
            unique:true,
            lowercase: true
        },
        email: {
            type: String,
            rerquired: true,
            unique:true,
            lowercase: true
        },
        password:{
            type: String,
            rerquired: true,
        }
    },
    {
        timestamps: true,
        // gives field created at, updated at
    }
)

export const User = mongoose.model("User",userSchema)