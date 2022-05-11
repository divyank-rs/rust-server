
Create table if not exists pizza (
    id Serial primary key,
    name TEXT,
    description TEXT,
    price_inr Integer,
    created_at timestamp not null,
    updated_at timestamp not null,
    deleted_at timestamp
)