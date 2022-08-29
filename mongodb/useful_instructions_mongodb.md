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

"Cusor methods" are methods like ```.count, .pretty, .sort```.
