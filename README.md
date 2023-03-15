# Prime - Terminal based Task Driven Work Management

## Config
  - $PRIME_ROOT Root Directory of all your task directories
  - $PRIME_COMPLETED Directory to putt all your completed tasks (default $PRIME_ROOT/completed)

## Logical Objects

## Args
- prime
  - create
    - task-name
    - deadline (timeleft)
        ```
          Should be of the form tx
            - x is the unit of time in consideration. [can be w,d,m,h or s]
            - t is the number of units of the x specified.

            Example :
              2w - 2 Weeks
              5d - 5 Days
              3h - 3 Hours
              10m - 10 Minutes
              15s - 15 Seconds
        ```
    - priority (default 0) (can be 0,1 or 2)
  
  - read
    - (default - show all tasks (sort by deadlines))
    - filter
      - task-name
      - deadline
      - priority
      - count (show only top $count number of tasks)

  - update (user can select from read mode/enter task-name)
    - identifier (task-name)
      - task-name
      - deadline
      - priority
    - new value
  - delete
    - done (default true)
    - archive (default true)

## Other Utilities
- notify using desktop notifications