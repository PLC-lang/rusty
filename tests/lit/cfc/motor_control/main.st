// Boolean -> integer, so we can print the motor state with printf's %d.
FUNCTION b2i : DINT
VAR_INPUT
    v : BOOL;
END_VAR
    IF v THEN b2i := 1; ELSE b2i := 0; END_IF;
END_FUNCTION

// Entry point: drive the motor through a scan sequence and observe `running`.
FUNCTION main : DINT
VAR
    m : Motor;
END_VAR
    // Press start (no stop, no e-stop) -> seal-in latches the motor on.
    m(start := TRUE, stop := FALSE, estop := FALSE);
    printf('after_start = %d$N', b2i(m.running));

    // Release start -> still running (the latch holds).
    m(start := FALSE, stop := FALSE, estop := FALSE);
    printf('still_running = %d$N', b2i(m.running));

    // Press stop -> motor stops.
    m(start := FALSE, stop := TRUE, estop := FALSE);
    printf('after_stop = %d$N', b2i(m.running));

    // Restart, then assert e-stop -> interlock keeps it stopped.
    m(start := TRUE, stop := FALSE, estop := FALSE);
    m(start := TRUE, stop := FALSE, estop := TRUE);
    printf('after_estop = %d$N', b2i(m.running));
END_FUNCTION
