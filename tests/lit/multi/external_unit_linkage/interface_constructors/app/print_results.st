FUNCTION print_results : DINT
VAR_INPUT
    exercise : LibExercise;
END_VAR
    printf('%d$N', exercise.put_result);
    printf('%s$N', REF(exercise.got));
    printf('%d$N', exercise.split_result);
    printf('%s %s$N', REF(exercise.head), REF(exercise.tail));
    printf('%d$N', exercise.by_ref_result);
    printf('%s$N', REF(exercise.target));
    printf('%d$N', exercise.rename_result);
    printf('%s$N', REF(exercise.label));
END_FUNCTION
