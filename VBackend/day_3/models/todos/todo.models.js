import mongoose from "mongoose";

const todoSchema = mongoose.Schema(
    {
        title: {
            type: String,
            required: true,

        },
        complete:{
            type: Boolean,
            default: false

        },
        createdBy: {
            type: mongoose.Schema.Types.ObjectId,
            ref: "User"
        },
        subTodos: [
            {
                type: mongoose.Schema.Types.ObjectId,
                ref:"SubTodo"
            }
        ]
    },
    {timestamps: true}
)



export const Todo = new mongoose.model("Todo",todoSchema)

// In mongodb --> todos