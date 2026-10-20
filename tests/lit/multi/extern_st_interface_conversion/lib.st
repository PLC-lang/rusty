INTERFACE ISpeaker
    METHOD say : DINT
    END_METHOD
END_INTERFACE

FUNCTION speak : DINT
    VAR_INPUT
        s : ISpeaker;
    END_VAR

    speak := s.say();
END_FUNCTION

FUNCTION_BLOCK Dog IMPLEMENTS ISpeaker
    METHOD say : DINT
        say := 1;
    END_METHOD

    METHOD describe : DINT
        VAR
            s : ISpeaker;
        END_VAR

        s := THIS^;
        describe := s.say();
    END_METHOD
END_FUNCTION_BLOCK
