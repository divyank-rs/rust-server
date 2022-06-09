use crate::{
    errors::ServiceError,
    helpers,
    models::{Order, Pizza},
    DbState,
};
use actix_web::{
    get, post,
    web::{self, Json},
    Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PizzaRequest {
    name: String,
    description: String,
    price_inr: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderRequest {
    pizza_id: i32,
    mobile_no: String,
    remarks: String,
    price_inr: i32,
}

#[get("/pizza")]
async fn get_pizzas(db_conn: web::Data<DbState>) -> Result<impl Responder, ServiceError> {
    let pool = db_conn.db_pool.clone();
    let pizzas = match sqlx::query_as!(
        Pizza,
        "
        SELECT id, name, description, price_inr,created_at,updated_at,deleted_at from pizza"
    )
    .fetch_all(&pool)
    .await
    {
        Ok(x) => x,
        Err(_) => {
            return Err(ServiceError::InternalServorError);
        }
    };
    Ok(Json(pizzas))
}

#[get("/order/{id}")]
async fn get_order(
    db_conn: web::Data<DbState>,
    path: web::Path<i32>,
) -> Result<impl Responder, ServiceError> {
    let pool = db_conn.db_pool.clone();
    let id = path.into_inner();
    let order = match sqlx::query!("
    Select id, pizza_id, mobile_no, remarks, price_inr, created_at, updated_at, deleted_at from orders where id = $1
    and deleted_at is null
    ",id)
    .fetch_one(&pool)
    .await
    {
        Ok(x) => x,
        Err(y) => {
            match y {
                sqlx::Error::RowNotFound => {
                    return Err(ServiceError::NoContent)
                },
                _ => {return Err(ServiceError::InternalServorError)}
            }
        }
    };
    let mut order = Order::new(
        order.id,
        order.pizza_id,
        &order.mobile_no,
        &order.remarks,
        order.price_inr,
        order.created_at,
        order.updated_at,
        &order.deleted_at,
    );
    order.pizza = match sqlx::query_as!(Pizza,
        "
        SELECT id, name, description, price_inr,created_at,updated_at,deleted_at from pizza where id = $1",
        order.pizza_id
    ).fetch_one(&pool)
    .await {
        Ok(x) => {Some(x)},
        Err(y) => {
            return Err(ServiceError::InternalServorError);
        }
    };
    Ok(Json(order))
}

#[post("/pizza")]
async fn put_pizza(
    db_conn: web::Data<DbState>,
    pizza_req: web::Json<PizzaRequest>,
) -> Result<impl Responder, ServiceError> {
    let pool = db_conn.db_pool.clone();
    let pizza = match sqlx::query_as!(
        Pizza,
        "INSERT INTO pizza (name,description,price_inr,created_at,updated_at) VALUES 
        ($1,$2,$3,now(),now()) returning id,name,description,price_inr,created_at,updated_at,deleted_at
        ",pizza_req.name,pizza_req.description,pizza_req.price_inr)
        .fetch_one(&pool)
        .await {
            Ok(x) => {x},
            Err(_) => {return Err(ServiceError::InternalServorError);}
        };
    Ok(Json(pizza))
}

#[post("/order")]
async fn put_order(
    db_conn: web::Data<DbState>,
    order_req: web::Json<OrderRequest>,
) -> Result<impl Responder, ServiceError> {
    let pool = db_conn.db_pool.clone();
    let order = match sqlx::query!(
        "INSERT INTO orders (pizza_id, mobile_no, remarks, price_inr, created_at, updated_at)
        VALUES ($1,$2,$3,$4,now(),now()) returning id, pizza_id, mobile_no, remarks, price_inr,
        created_at, updated_at, deleted_at",
        order_req.pizza_id,
        order_req.mobile_no,
        order_req.remarks,
        order_req.price_inr
    )
    .fetch_one(&pool)
    .await
    {
        Ok(x) => x,
        Err(_) => {
            return Err(ServiceError::InternalServorError);
        }
    };
    let order = Order::new(
        order.id,
        order.pizza_id,
        &order.mobile_no,
        &order.remarks,
        order.price_inr,
        order.created_at,
        order.updated_at,
        &order.deleted_at,
    );
    Ok(Json(order))
}

#[get("/price/{id}")]
async fn get_price(
    db_conn: web::Data<DbState>,
    path: web::Path<i32>,
) -> Result<impl Responder, ServiceError> {
    let pool = db_conn.db_pool.clone();
    let id = path.into_inner();
    let count = match sqlx::query!(
        "Select count(*) as count from orders 
    where created_at > (NOW() - INTERVAL '1 DAY')"
    )
    .fetch_one(&pool)
    .await
    {
        Err(_) => {
            return Err(ServiceError::InternalServorError);
        }
        Ok(x) => x,
    };
    if count.count == None {
        return Err(ServiceError::InternalServorError);
    }
    let price = match sqlx::query!("Select price_inr from pizza where id = $1", id)
        .fetch_one(&pool)
        .await
    {
        Err(_) => {
            return Err(ServiceError::InternalServorError);
        }
        Ok(x) => x,
    };
    let final_price = helpers::pricing_logic(count.count.expect("Haha") as i32, price.price_inr);
    Ok(Json(final_price))
}
