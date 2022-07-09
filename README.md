This is my solution in Rust for Cassidoo interview question from https://buttondown.email/cassidoo/archive/choose-people-who-lift-you-up-michelle-obama/

Examples of output:

    Your tray (sorted):
    - 1 Blue
    - 1 Black
    - 1 Yellow
    - 1 Yellow
    - 3 Blue
    - 3 Black
    - 3 Yellow
    - 4 Blue
    - 4 Blue
    - 5 Black
    - 7 Yellow
    - 10 Yellow
    - 13 Yellow
    - Wildcard
    Valid sets:
    -> 1 Blue 1 Black 1 Yellow Wildcard 
    -> 1 Black 1 Yellow Wildcard 
    -> 1 Blue 1 Yellow Wildcard 
    -> 1 Blue 1 Black Wildcard 
    -> 1 Blue 1 Black 1 Yellow 
    -> 3 Blue 3 Black 3 Yellow Wildcard 
    -> 3 Black 3 Yellow Wildcard 
    -> 3 Blue 3 Yellow Wildcard 
    -> 3 Blue 3 Black Wildcard 
    -> 3 Blue 3 Black 3 Yellow 
    -> 1 Blue Wildcard 3 Blue 4 Blue 
    -> 1 Blue Wildcard 3 Blue 
    -> Wildcard 3 Blue 4 Blue 
    -> 3 Blue 4 Blue Wildcard 
    -> 1 Black Wildcard 3 Black 
    -> 3 Black Wildcard 5 Black 
    -> 1 Yellow Wildcard 3 Yellow 


    Your tray:
    - 2 Black
    - 9 Red
    - 7 Yellow
    - 7 Yellow
    - 11 Blue
    - 2 Red
    - 3 Blue
    - Wildcard
    - 10 Black
    - 7 Black
    - 6 Yellow
    - 11 Red
    - 5 Blue
    - 1 Black
    Your tray (sorted):
    - 1 Black
    - 2 Red
    - 2 Black
    - 3 Blue
    - 5 Blue
    - 6 Yellow
    - 7 Black
    - 7 Yellow
    - 7 Yellow
    - 9 Red
    - 10 Black
    - 11 Red
    - 11 Blue
    - Wildcard
    Valid sets:
    -> 2 Red 2 Black Wildcard 
    -> 7 Black 7 Yellow Wildcard 
    -> 11 Red 11 Blue Wildcard 
    -> 9 Red Wildcard 11 Red 
    -> 3 Blue Wildcard 5 Blue 
    -> 1 Black 2 Black Wildcard 
    -> Wildcard 6 Yellow 7 Yellow 
    -> 6 Yellow 7 Yellow Wildcard 