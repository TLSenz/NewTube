use std::fmt::Error;
use bcrypt::hash;
use diesel::associations::HasTable;
use diesel::prelude::*;
use tokio::task::id;
use crate::establish_connection;
use crate::model::usermodel::{LoginInfo, NewUser, User};
use crate::schema::users::dsl::users;
use crate::schema::users::password;

pub fn get_user(id: i32) -> Result<User,Error>{

    let connection = &mut establish_connection();

    let user = users
        .find(id)
        .select(User::as_select())
        .first(connection)
        .optional();


    match user {
        Ok(Some(user)) => Ok(user),
        Ok(None) => {
            println!("No User with this id");
            Err(Error)
        },
        Err(E) => {
            println!("Could not connect with Database");
            Err(Error)
        }
    }

    }


pub fn check_user(login_data :LoginInfo) -> Result<User,Error>{

    let conn = &mut establish_connection();

    let user = users
        .filter(&login_data.username)
        .select(User::as_select())
        .first(conn)
        .optional();

    match user {
        Ok(Some(user)) => {
            if user.password == login_data.password{
                Ok(user)
            }
            else {
                Err(Error)
            }
        },
        Ok(None) => {
            println!("No User with this id");
            Err(Error)

        },
        Err(E) => {
            println!("Could not connect with Database");
            Err(Error)
        }
    }





}



pub fn create_user(conn: &mut PgConnection, username: &str, passwort: &str, email: &str) -> User{

    let hashed_password = hash(passwort, 12).expect("Could not Hash");


    let new_user = NewUser {
        username: username,
        password: &hashed_password,
        email: email,
    };


    diesel::insert_into(users::table())
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error Creating New User ")

}
