// import mongoose
import mongoose from "mongoose"

// design model
const user = new mongoose.Schema(
    {
        // username: String,
        // email: String,
        // isActive: Boolean
        username: {
            type: String,
            required : true,
            unique: true,
            lowercase:true
            },
        email:{
            type: String,
            required: true,
            unique: true,
            lowercase: true

        },
        password: {
            type: String,
            required: true,
        },
        isActive: Boolean
    },
    {
        timestamps: true
    }
)


// export mongoose schema
export const User = mongoose.model("User",user)

// "User" --> In mongodbb database "users"