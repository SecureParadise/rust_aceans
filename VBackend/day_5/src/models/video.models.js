import mongoose, { Schema } from "mongoose";
import moongooseAggregatePaginate from "mongoose-aggregate-paginate-v2";

// index:true is for searching opertion, ots is a expensive operation
const videoSchema = mongoose.Schema(
  {
    videoFile: {
      type: String, //cloudnary url
      required: true,
    },
    thumbnails: {
      type: String, //cloudnary url
      required: true,
    },
    title: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
    duration: {
      type: Number, // cloudnary
      required: true,
    },
    views: {
      type: Number, //cloudnary url
      default: 0,
    },
    isPublished: {
      type: Boolean,
      default: true,
    },

    owner: {
      type: Schema.Types.ObjectId,
      ref: "User",
    },
    refreshToken: {
      type: String,
    },
  },
  { timestamps: true }
);

videoSchema.plugin(moongooseAggregatePaginate);
export const Video = mongoose.model("Video", videoSchema);
