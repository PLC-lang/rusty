FUNCTION main : DINT
VAR
    square : Square;
    triangle : Triangle;
    shape : POINTER TO AbstractShape;
    counter : ByTwo;
    named : Named;
    pair : ARRAY[0..1] OF DINT;
    acc : Accumulator;
    rec : Record;
    words : Words := ['p', 'q'];
    text : Text := 'text';
    grid : Grid;
    name : STRING[20];
END_VAR
    shape := REF(square);
    printf('%d %d %d$N', shape^.scaled(), shape^.describe('sq'), square.sides());
    printf('%s$N', REF(square.label));
    shape := REF(triangle);
    printf('%d %d %d$N', shape^.scaled(), shape^.describe('tri'), area_of(triangle));

    counter.tick();
    counter.tick();
    printf('%d %d %d$N', counter.current(), counter.step(), area_of(counter));

    name := named.name;
    printf('%s$N', REF(name));
    named.name := 'changed';
    name := named.name;
    pair := named.pair;
    printf('%s %d %d$N', REF(name), pair[0], pair[1]);

    Prog();
    Prog();
    printf('%d %s$N', Prog.twice('x'), REF(Prog.words[1]));
    Prog.reset();
    printf('%d$N', Prog.runs);

    acc.push(5, 'a');
    acc.push(7, 'b');
    printf('%d %d %s$N', acc.total, acc.inner.hits, REF(acc.items[0]));
    acc.clear();
    printf('%d$N', acc.total);

    printf('%d %d$N', pick_weight(1, TRUE, words), pick_weight(1, FALSE, words));

    printf('%s %s %d %d %d$N', REF(g_text), REF(g_words[1]), g_state, g_point.x, g_point.y);

    printf('%d %s %s$N', rec.pos.x, REF(words[1]), REF(text));

    grid.fill();
    printf('%d$N', grid.sum());

    printf('%d$N', larger(DINT#3, DINT#8));
END_FUNCTION
