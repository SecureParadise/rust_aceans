import { asyncHandler } from "../utils/asyncHandler.js";
import { ApiError } from "../utils/apiError.js";
import { User } from "../models/user.models.js";
import {uploadOnCloudinary} from "../utils/cloudinary.js"
import { ApiResponse } from "../utils/ApiResponse.js";
const registerUser = asyncHandler(async (req, res) => {
  /*

    res.status(200).json({
        message:"ok"
    })

     */

  // Lets register user logic

  // get user details from fromtend
  const { fullName, email, userName, password } = req.body;
  // console.log("email",email);
  // console.log("pass",password);

  // validation -ensure every field is fulfilled
  // if(fullName == ""){
  //     throw new ApiError(400,"fullname is required")

  // }
  if (
    [fullName, email, userName, password].some((field) => field?.trim() === "")
  ) {
    throw new ApiError(400, "All fields are required");
  }
  // check if user alredy exist: userName,email
  const existedUser = User.findOne({
    $or: [{userName }, { email }],
  });
  if (existedUser) {
    throw new ApiError(409, "User with email or userName alredy exist");
  }

  // check for image,check for avitar
  const avatarLocalPath = req.files?.avatar[0]?.path;
  const CoverImageLocalPath = req.files?.CoverImage[0]?.path;
  if (!avatarLocalPath) {
    throw new ApiError(400, "Avatar is nneeded");
  }
  // upload them to cloudanry,avatar
const avatar = await uploadOnCloudinary(avatarLocalPath)
const coverImage =  await uploadOnCloudinary(CoverImageLocalPath)
if(!avatar){
   throw new ApiError("Avatar field i shul");
   
}
  // create user object -> to store in mongo db
  const user = await User.create({
    fullName,
    avatar:avatar.url,
    coverImage:coverImage.url?.url || "",
    email,
    userName:userName.toLowerCase()
  })
  // remove password and refresh token field from response
  const createdusertest = await User.findById(user._id).select(
    "-password -refreshToken"
  )
  // check for user creation
  if(!createdusertest){
    throw new ApiError(500,"Something went wrong while regestering the user");
  }
  // return res
  return res.status(201).json(
    new ApiResponse(200,createdusertest,"User registered successfully")
  )
});

export { registerUser };
