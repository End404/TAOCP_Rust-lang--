use crate::models::coures::Course;
use sqlx::postgres::PgPool;
use crate::errors::MyError;

pub async fn get_courses_for_teacher_db(
    pool:&PgPool, teacher_id: i32
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT id, * FROM course WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &PgPool, teacher_id: i32, course_id: i32
) -> Result<Course,MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course
        WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    }else {
        Err(MyError::NotFound("Course Id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool:&PgPool, new_course:Course
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (id, teacher_id, name)
        VALUES ($1,$2,$3)
        RETURNING id, teacher_id, name, time"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) {
    let course_row = sqlx::query!(
        "DELETE FROM course where teacher_id = $1 and id =$2",
        teacher_id,
        id,
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool, teacher_id: i32, id: i32, update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM course whers teacher_id = $1 and id = $2",
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let description: Strign = if let Some(desc) = update_course.description {
        desc
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format: String = if let Some(format) update_course.format {
        format
    }else {
        current_course_row.format.unwrap_or_default()
    };

    let course_row = sqlx::query_as!(
        Course,
        "UPDATE course SET name = $1, description = $2, format = $3,
        RETURNING name, description, format",
        name,
        description,
        format,
    )
    .fetch_one(pool)
    .await;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}
