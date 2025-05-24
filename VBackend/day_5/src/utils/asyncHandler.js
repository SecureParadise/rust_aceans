const asyncHandler = (requestHAndler) => {
  return (req, res, next) => {
    Promise.resolve(requestHAndler(req, res, next)).catch((err) => next(err));
  };
};

export { asyncHandler };

// heigher order function in js
/*
const asyncHandler = (fn)=>async(req,res,next)=>{
    try{
        await fn(req,res,next)
    }catch(error){
        res.status(error.code || 500).json({
            success:false,
            message: error.message
        })
    }
}
    */
