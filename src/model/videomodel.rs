use diesel::*;



#[derive(Insertable)]
#[diesel(table_name = videos)]
pub struct UploadRequest{
    title: String,
    description: String,
    visibility: bool
}


