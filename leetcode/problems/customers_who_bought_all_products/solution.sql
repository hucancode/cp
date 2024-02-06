with a as (select customer_id,count(distinct(product_key)) as product_count
    from Customer
    group by customer_id)
select customer_id from a where product_count in (select count(*) from Product)