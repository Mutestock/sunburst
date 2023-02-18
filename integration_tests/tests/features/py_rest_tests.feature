Feature: Making article requests against the python Rest service

    Scenario: Sending a request to read all articles from the python rest service
        Given mongodb is online
        And the distributor is online
        And the python rest service is online
        When a request for reading all articles from the python rest service is made
        Then we receive a list of all articles from the python rest service

    Scenario: Sending a request to read all articles from the python rest service by a search term
        Given mongodb is online
        And the distributor is online
        And the python rest service is online
        When a request for reading all articles by search term from the python rest service is made
        Then we receive a list of all articles which has the search term in the name or tags from the python rest service

    Scenario: Sending a request to read all articles from the python rest service by a site
        Given mongodb is online
        And the distributor is online
        And the python rest service is online
        When a request for reading all articles by site from the python rest service is made
        Then we receive a list of all articles which come from that site from the python rest service

    Scenario: Sending a request to get article count statistics from the python rest service
        Given mongodb is online
        And the distributor is online
        And the python rest service is online
        When a request for reading article count statistics from the python rest service is made
        Then we receive article count statistics from the python rest service
