import mongoose from "mongoose";

const sub_todoSchema = mongoose.Schema(
    {
        title: {
            type:String,
            required: true
        },
        complete: {
            type: Boolean,
            default: false
        },
        createdBy: {
            type: mongoose.Schema.Types.ObjectId,
            ref:"User"
        }
    },
    {
        timestamps: true
    })

export const SubTodo = new mongoose.model("SubTodo",sub_todoSchema)