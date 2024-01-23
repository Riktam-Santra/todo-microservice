use std::sync::Arc;

use actix_web::{Responder, HttpResponse, web::{Data, Json, Path}, http::header::ContentType};

use crate::models::{app_data::AppData, json_add_request::JsonAddRequest, json_update_request::JsonUpdateRequest};

#[actix_web::get("/todos")]
pub async fn get_todos(data: Data<Arc<AppData>>) -> impl Responder {
    let todos = data.get_db_controller().get_todos().await;
    HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&todos).unwrap())
}

#[actix_web::post("/todos")]
pub async fn add_todos(data: Data<Arc<AppData>>, req: Json<JsonAddRequest>) -> impl Responder {
    let _ = data.get_db_controller().add_todo(req.get_title(), req.get_task_completed()).await.unwrap();
    HttpResponse::Ok().content_type(ContentType::json()).finish()
}

#[actix_web::put("/todos/{id}")]
pub async fn update_todo(data: Data<Arc<AppData>>, id: Path<String>, req: Json<JsonUpdateRequest>) -> impl Responder {
    let id = uuid::Uuid::parse_str(&id).unwrap();
    data.get_db_controller().update_todo(id, req.get_title(), req.get_has_completed()).await;
    HttpResponse::Ok().content_type(ContentType::json()).finish()
}

#[actix_web::delete("/todos/{id}")]
pub async fn delete_todo(data: Data<Arc<AppData>>, id: Path<String>) -> impl Responder {
    let uuid = uuid::Uuid::parse_str(&id).unwrap();
    data.get_db_controller().delete_todo(uuid).await;

    HttpResponse::Ok().content_type(ContentType::json()).finish()
}

#[actix_web::post("/todos/{id}/complete")]
pub async fn complete_todo(data: Data<Arc<AppData>>, id: Path<String>) -> impl Responder {
    let uuid = uuid::Uuid::parse_str(&id).unwrap();
    data.get_db_controller().complete_todo(uuid).await;

    HttpResponse::Ok().content_type(ContentType::json()).finish()
}

#[actix_web::post("/todos/{id}/revert")]
pub async fn revert_todo(data: Data<Arc<AppData>>, id: Path<String>) -> impl Responder {
    let uuid = uuid::Uuid::parse_str(&id).unwrap();
    data.get_db_controller().revert_todo(uuid).await;

    HttpResponse::Ok().content_type(ContentType::json()).finish()
}