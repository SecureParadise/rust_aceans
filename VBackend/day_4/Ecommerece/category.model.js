import mongoose from "mongoose";

const categorySchema = new mongoose.Schema(
    {
        name: {
            type: String,
            rerquired: true,
        }
    },
    {
        timestamps: true,
    }
)

export const Category = mongoose.model("Category",categorySchema)