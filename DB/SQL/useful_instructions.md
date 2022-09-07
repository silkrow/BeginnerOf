* SELECT-FROM-WHERE:
	SELECT *
	FROM (SELECT * FROM Customer WHERE name LIKE 'A%') as temp
	WHERE temp.phone LIKE '5%';
	---
	A parenthesized SFW statement(subquery) can be used as a value in a number of places, including FROM and WHERE clauses.


