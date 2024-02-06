# Write your MySQL query statement below
with a as (select Users.name
    from (
        select user_id, count(movie_id) as rate_count 
        from MovieRating 
        group by user_id 
        order by count(movie_id) desc) as Rating left join Users on Users.user_id = Rating.user_id
    order by Rating.rate_count desc, Users.name
    limit 1),
b as (select Movies.title 
    from (select movie_id, avg(rating) as rating
        from MovieRating
        where month(created_at)=2 and year(created_at)=2020
        group by movie_id
        order by avg(rating) desc) as Rating left join Movies on Movies.movie_id = Rating.movie_id
    order by Rating.rating desc, Movies.title
    limit 1)
select name as results from a union all select title as results from b