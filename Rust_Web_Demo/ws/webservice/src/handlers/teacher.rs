use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::{CreateTacher, UpdateTeacher};
use crate::state::AppState;

use actix_web::{web, HttpResposne};

pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResposne, MyError> {
    get_all_teachers_db(&app_state.db)
    .await
    .map(|teachers| HttpResposne::Ok().json(teachers))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResposne, MyError> {
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResposne::Ok().json(teacher))
}

pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTacher>,
    app_state: web::Data<AppState>,
) -> Result<HttpResposne, MyError> {
    post_new_teacher_db(&app_state.db, CreateTacher::from(new_teacher))
        .await
        .map(|teacher| HttpResposne::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_teacher: web::Json<UpdateTeacher>,
) -> Result<HttpResposne, MyError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(
        &app_state.db, 
        UpdateTeacher::from(update_teacher),
    )
    .await
    .map(|teacher| HttpResposne::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResposne, MyError> {
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
    .await
    .map(|teacher| HttpResposne::Ok().json(teacher))
}

