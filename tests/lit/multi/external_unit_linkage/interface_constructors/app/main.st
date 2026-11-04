FUNCTION main : DINT
VAR
    lib_text : LibText;
    main_text : MainText;
    exercise : LibExercise;
    lib_extra : LibExtra;
    main_extra : MainExtra;
    extra_exercise : LibExtraExercise;
    lib_more : LibMore;
    main_more : MainMore;
END_VAR
    exercise(t := lib_text);
    print_results(exercise);
    exercise(t := main_text);
    print_results(exercise);
    extra_exercise(e := lib_extra);
    print_extra(extra_exercise);
    extra_exercise(e := main_extra);
    print_extra(extra_exercise);
    printf('%d %d$N', more_of(lib_more), more_of(main_more));
END_FUNCTION
