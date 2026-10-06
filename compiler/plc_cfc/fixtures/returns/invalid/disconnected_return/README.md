What: a return with no wired condition can never fire and is rejected.

Illustrated:

    myCondition --> RETURN (1)
                    RETURN (2)   [unconnected]
