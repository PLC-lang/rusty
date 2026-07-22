What: a return with no wired condition can never fire and is rejected.

Illustrated:

    myCondition --> RETURN (0)
                    RETURN (1)   [unconnected]
