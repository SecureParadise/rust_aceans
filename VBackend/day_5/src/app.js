import express from "express";
import cors from "cors"
import cookieParser from "cookie-parser" 

const app = express();
app.use(cors({
    origin: process.env.CORS_ORIGIN,
    credentials: true
}))

// app.use() -->for middleware and configuration
app.use(express.json({limit:"16kb"}))
app.use(express.urlencoded({extended:true,limit:"16kb"}))
// here public is folder name where we store favicon image and static data
app.use(express.static("public"))
app.use(cookieParser())

//routes import
// file segrigation
// import router from "./routes/user.route.js";
import userRouter from "./routes/user.route.js";
//routes decleration
app.use("/api/v1/users",userRouter)

export { app };
/*
(err,req,res,next)
here next is flag, majorly indicating middleware
*/