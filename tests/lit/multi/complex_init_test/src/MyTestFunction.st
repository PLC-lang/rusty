FUNCTION MyTestFunction : complex
VAR_INPUT
	iComplexDefault	: complex;
	icomplexPartial	: complex := (id := 9, partialSimple := (a := 10), state := element0);
	icomplexFull	: complex := (
	id := 42,
	enabled := FALSE,
	defaultInt := 77,
	name := 'FULL',
	timeout := T#1s,
	bigTime := LT#10s,
	state := element0,
	states := [ element1,element0,element1],
	simple := (a := 11, b := TRUE),
	partialSimple := (a := 12, b := TRUE),

	arr := [ (a := 5, b := FALSE),(a := 6, b := TRUE),(a := 7, b := FALSE)],
	(*	arr := gcArrSimple_Full,  *)
	partialArr :=	[ (a := 19),(b:= TRUE),(a := 7)],
	(*	partialArr:= gcArrSimple_Partial, *)
	
	gains := [ 9.0,8.0,7.0]);

	iComplexConst	: complex := cComplexpartial;

END_VAR

VAR
	simple	: simpleDefault := (a := 17, b := TRUE);

	complexDefault	: complex;
	complexPartial	: complex := (id := 9, partialSimple := (a := 10), state := element0);
	complexFull	: complex := (
	id := 42,
	enabled := FALSE,
	defaultInt := 77,
	name := 'FULL',
	timeout := T#1s,
	bigTime := LT#10s,
	state := element0,
(*	states := [ element1,element0,element1], // NOT working *)
	simple := (a := 11, b := TRUE),
	partialSimple := (a := 12, b := TRUE),

(*	arr := [ (a := 5, b := FALSE),(a := 6, b := TRUE),(a := 7, b := FALSE)],// NOT working *)
(*	arr := gcArrSimple_Full, //NOT working *)
(*	partialArr :=	[ (a := 19),(b:= TRUE),(a := 7)], //NOT working *)
(*	partialArr:= gcArrSimple_Partial, //NOT working *)

	gains := [ 9.0,8.0,7.0]);
	ComplexConst	: complex := cComplexpartial;

	Simple_Override	: simpleOverride := cSimple_Override;
	Simple_Partial	: simpleOverride := cSimple_Partial;

	ArrElem1	: simpleOverride := gcSimple_Override;
	ArrElem3	: simpleOverride := (a := 5, b := TRUE);
	ArrSimple_Full	: ARRAY[1..3] OF simpleOverride := cArrSimple_Full;
	ArrSimple_Partial	: ARRAY[1..3] OF simpleOverride := cArrSimple_Partial;

	Gains_Test	: ARRAY[1..3] OF REAL := cGains_Test;

END_VAR
VAR CONSTANT
	cComplexpartial	: complex := (id := 9, partialSimple := (a := 15), state := element0);

	cSimple_Override	: simpleOverride := (a := 77, b := FALSE);
	cSimple_Partial	: simpleOverride := (a := 30);

	cArrElem1	: simpleOverride := gcSimple_Override;
	cArrElem3	: simpleOverride := (a := 5, b := TRUE);
	cArrSimple_Full	: ARRAY[1..3] OF simpleOverride := [ (gcArrElem1),(a := 18, b := FALSE),(gcArrElem3)];
	cArrSimple_Partial	: ARRAY[1..3] OF simpleOverride :=     [ (a := 27),(b := FALSE),(gcSimple_Partial)];

	cGains_Test	: ARRAY[1..3] OF REAL := [ 20.0,30.0,40.0];

END_VAR
//DEBUG
iComplexDefault;
icomplexPartial;
icomplexFull;
iComplexConst;
simple;
complexDefault;
complexPartial;
complexFull;
ComplexConst;

MyTestFunction := complexFull;

END_FUNCTION
