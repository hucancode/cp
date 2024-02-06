# Write your MySQL query statement below
with cte as (select Department.id, Department.name, max(Employee.Salary) as salary
    from Employee left join Department on Employee.departmentId = Department.id
    group by Department.id, Department.name
)
select Employee.name as Employee, cte.name as Department, Employee.salary as Salary 
from Employee left join cte on Employee.departmentId = cte.id
where Employee.salary = cte.salary