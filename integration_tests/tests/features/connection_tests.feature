Feature: Making basic connections to mongodb
    Scenario: Get a connection to the mongodb testing database
        Given mongodb is up and running
        When a connection to the mongodb testing database is established
        Then we're capable of basic interactions with the testing database

    Scenario: Get a connection to the mongodb dev database
        Given mongodb is up and running
        When a connection to the mongodb dev database is established
        Then we're capable of basic interactions with the dev database

    Scenario: Get a connection to the mongodb production database
        Given mongodb is up and running
        When a connection to the mongodb production database is established
        Then we're capable of basic interactions with the production database