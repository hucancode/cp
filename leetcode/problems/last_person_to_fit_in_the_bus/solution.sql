# Write your MySQL query statement below
with bus as (select person_name, sum(weight) over (order by turn) as capacity 
    from Queue)
select person_name
from bus
where capacity <= 1000
order by capacity desc
limit 1 