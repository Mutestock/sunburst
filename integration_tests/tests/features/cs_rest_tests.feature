Feature: Making article requests against the Dotnet Rest service

    Scenario: Sending a request to read all articles from the dotnet rest service
        Given mongodb is online
        And the distributor is online
        And the dotnet rest service is online
        When a request for reading all articles from the dotnet rest service is made
        Then we receive a list of all articles from the dotnet rest service

    Scenario: Sending a request to read all articles from the dotnet rest service by a search term
        Given mongodb is online
        And the distributor is online
        And the dotnet rest service is online
        When a request for reading all articles by search term from the dotnet rest service is made
        Then we receive a list of all articles which has the search term in the name or tags from the dotnet rest service

    Scenario: Sending a request to read all articles from the dotnet rest service by a site
        Given mongodb is online
        And the distributor is online
        And the dotnet rest service is online
        When a request for reading all articles by site from the dotnet rest service is made
        Then we receive a list of all articles which come from that site from the dotnet rest service

    Scenario: Sending a request to get article count statistics from the dotnet rest service
        Given mongodb is online
        And the distributor is online
        And the dotnet rest service is online
        When a request for reading article count statistics from the dotnet rest service is made
        Then we receive article count statistics from the dotnet rest service
