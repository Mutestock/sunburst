Feature: Making article requests against the NodeJS Rest service

    Scenario: Sending a request to read all articles from the nodejs rest service
        Given mongodb is online
        And the distributor is online
        And the nodejs rest service is online
        When a request for reading all articles from the nodejs rest service is made
        Then we receive a list of all articles from the nodejs rest service

    Scenario: Sending a request to read all articles from the nodejs rest service by a search term
        Given mongodb is online
        And the distributor is online
        And the nodejs rest service is online
        When a request for reading all articles by search term from the nodejs rest service is made
        Then we receive a list of all articles which has the search term in the name or tags from the nodejs rest service

    Scenario: Sending a request to read all articles from the nodejs rest service by a site
        Given mongodb is online
        And the distributor is online
        And the nodejs rest service is online
        When a request for reading all articles by site from the nodejs rest service is made
        Then we receive a list of all articles which come from that site from the nodejs rest service

    Scenario: Sending a request to get article count statistics from the nodejs rest service
        Given mongodb is online
        And the distributor is online
        And the nodejs rest service is online
        When a request for reading article count statistics from the nodejs rest service is made
        Then we receive article count statistics from the nodejs rest service
