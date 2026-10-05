FUNCTION main : DINT
VAR
    fb : ColorFb;
    c : Color;
END_VAR
    printf('%d$N', gColor);
    fb();
    printf('%d$N', fb.out);
    c := next_color(fb.out);
    printf('%d$N', c);
    CASE c OF
        Green: printf('green$N');
        ELSE printf('other$N');
    END_CASE
    gColor := Blue;
    printf('%d$N', gColor);
END_FUNCTION
