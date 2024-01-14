use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::coures::*;
use crate::errors::MyError;



pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = 
        format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

use crate::models::course::{CreateCourse, UpdateCourse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
    .await
    .map(|course| HttpResponse::Ok().json(course));
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result< HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_course_details_db(&app_state.db, teacher_id)
    .await
    .map(|coures| HttpResponse::Ok().json(coures))
    
}


pub async fn get_courses_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, teacher_id,course_id)
    .await
    .map(|coures| HttpResponse::Ok().json(course))
    
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|resp| HttpResponse::Ok().json(course))
}
