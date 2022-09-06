Show the databases:

	show dbs 

Use/create some database (e.g. called 'dbname'):
	
	use dbname

Show the collections of current database:

	show collections

To set an editor(e.g. vim):
	
	config.set("config", "vim")

Using an editor in mongosh to deal with multiple lines of instructions:

	edit

A query to #find all# documents in "person" with at least one constraint qualified(This works as "or" because the two conditions will be separately verified):

	db.person.find({age:{$gt: 30}, age:{$lt: 50}})

CAUTION: weird thing happens if you run the following query in the same database

	db.person.find({age:{$lt: 50}, age:{$gt: 30}})

feel free to check this weird result with community 6.0.1 version, and mongosh 1.5.4 version. The collection setup is 

	db.person.insertMany([{ "_id": ObjectId("5e457ff1511770d0bfab31b1"), "name": "A", "age": 20, "height": 5.5 }, { "_id": ObjectId("5e457ff1511770d0bfab31b3"), "name": "B", "age": 20, "height": 6 }, { "_id": ObjectId("5e457ff1511770d0bfab31b5"), "name": "D", "age": 20 }, { "_id": ObjectId("5e458449511770d0bfab31b6"), "name": "U", "age": 25, "height": 5.5 }, { "_id": ObjectId("5e458449511770d0bfab31b7"), "name": "S", "age": 41 }, { "_id": ObjectId("5e458449511770d0bfab31b8"), "age": 30, "height": 6.2 }, { "_id": ObjectId("5e458449511770d0bfab31ba"), "name": "Abdu ForeverYoung", "age": 21 }])

"Cursor methods" are methods like ```.count, .pretty, .sort```. Check [cursor methods](https://www.mongodb.com/docs/manual/reference/method/js-cursor/) for details.

Here're some commonly used cursor methods:

	cursor.forEach() # Applies a JavaScript function for every document in a cursor.
	cursor.limit() # Constrains the size of a cursor's result set.
	cursor.map() # Applies a function to each document in a cursor and collects the return values in an array.
	cursor.max() # Specifies an exclusive upper index bound for a cursor. For use with cursor.hint()
	cursor.hint() # Forces MongoDB to use a specific index for a query.
	cursor.min() # Specifies an inclusive lower index bound for a cursor. For use with cursor.hint()
	cursor.next() # Returns the next document in a cursor.
	cursor.pretty() # Configures the cursor to display results in an easy-to-read format.
	cursor.size() # Returns a count of the documents in the cursor after applying skip() and limit() methods.
	cursor.sort() # Returns results ordered according to a sort specification.
	cursor.toArray() # Returns an array that contains all documents returned by the cursor.

An example of aggregation:
	
	db.Movies.aggregate([
    {$unwind: "$actors"},
    {
        $match:
        {
            "ratings": {$gte: 8}
        }

    },
    {
        $group:
        {
            _id: "$director",
            unique_actor: {$addToSet: "$actors"}
        }
    },
    {
        $project:
        {
            director: "$_id",
            _id: 0,
            count_unique_actors: {$size: "$unique_actor"}
        }
    }])
