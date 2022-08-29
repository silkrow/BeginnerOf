To run MongoDB(i.e. the mongod process) as a macOS service:

	brew services start mongodb-community@6.0

To stop a mongod running running as a macOS service:

	brew services stop mongodb-community@6.0

To list macOS services:

	brew services list 

To delete a database, first switch into is, then run:

	db.dropDatabase()

To connect and use MongoDB:

	mongosh
