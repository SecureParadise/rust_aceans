import mongoose from "mongoose";

// mini model
const orderItemSchema = new mongoose({
    productId: {
        type: mongoose.Schema.Types,ObjectId,
        ref:"Product",
    },
    quantity:{
        type: Number,
        rerquired: true,
    }
})



const categorySchema = mongoose.Schema(
    {
        orderPrice: {
            type: Number,
            rerquired: true,
        },
        customer: {
            type: mongoose.Schema.Types.ObjectId,
            ref: "User"
        },
        orderItems:{
            type: [orderItemSchema]
        },
        address:
        {
            type: String,
            rerquired: true,
        },
        status:{
            type: String,
            enum: ["PENDING","CANCELLED","DELIVERED"],
            default: "PENDING"

        }
    } 
)

export const Category = mongoose.model("Category",categorySchema)