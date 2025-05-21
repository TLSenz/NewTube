
use rocket::*;
use crate::model::videomodel::UploadRequest;

#[post("/upload") data = "<data>"]
pub async fn upload(data: UploadRequest){


}