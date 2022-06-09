use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pizza {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price_inr: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub pizza_id: i32,
    pub mobile_no: Option<String>,
    pub remarks: Option<String>,
    pub price_inr: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub pizza: Option<Pizza>,
}

impl Order {
    pub fn new(
        id: i32,
        pizza_id: i32,
        mobile_no: &Option<String>,
        remarks: &Option<String>,
        price_inr: i32,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        deleted_at: &Option<NaiveDateTime>,
    ) -> Self {
        Order {
            id,
            pizza_id,
            mobile_no: mobile_no.clone(),
            remarks: remarks.clone(),
            price_inr,
            created_at,
            updated_at,
            deleted_at: deleted_at.clone(),
            pizza: None,
        }
    }

    pub fn put_pizza(&mut self, pizza: &Pizza) {
        self.pizza = Some(pizza.clone());
    }
}
