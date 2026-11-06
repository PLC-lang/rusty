FUNCTION print_extra : DINT
VAR_INPUT
    ex : LibExtraExercise;
END_VAR
    printf('%d %d %s %s$N', ex.pick_result, ex.grid_result, REF(ex.grid_text), REF(ex.written));
    printf('%d %s$N', ex.defaults_result, REF(ex.defaults_text));
    printf('%d %d %d$N', ex.triple[0], ex.triple[1], ex.triple[2]);
    printf('%s %d %d$N', REF(ex.name), ex.pair[0], ex.pair[1]);
END_FUNCTION
