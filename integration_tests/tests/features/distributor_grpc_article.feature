Feature: Making grpc article requests to the distributor

    Scenario: Sending a request to read all articles from the distributor
        Given mongodb and the distributor are both online
        When a request for reading all articles from the distributor is made
        Then we receive a list of all articles

    Scenario: Sending a request to read articles by a search term
        Given mongodb and the distributor are both online
        When a request for reading all articles by search term from the distributor is made
        Then we receive a list of articles which has the search term in the name

    Scenario: Sending a request to read articles by a site
        Given mongodb and the distributor are both online
        When a request for reading all articles by a site from the distributor is made
        Then we receive a list of articles which come from that site

    Scenario: Sending a request to get article count statistics by a site and search term
        Given mongodb and the distributor are both online
        When a request for create article count statistics is made
        Then we receive article count statistics