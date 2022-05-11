
Create table if not exists orders(
    id Serial primary key,
    pizza_id integer references pizza(id),
    mobile_no TEXT,
    remarks TEXT,
    price_inr integer,
    created_at timestamp not null,
    updated_at timestamp not null,
    deleted_at timestamp 
)