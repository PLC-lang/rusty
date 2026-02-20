FUNCTION main : DINT
mainProg();

// Print simple variable
printf('simple.a = %d$N', mainProg.simple.a); //CHECK: simple.a = 17
printf('simple.b = %s$N', SEL(mainProg.simple.b, 'FALSE', 'TRUE')); //CHECK: simple.b = TRUE

// Print complexDefault (uses type defaults from complex.dt)
printf('complexDefault.id = %d$N', mainProg.complexDefault.id); //CHECK: complexDefault.id = 123
printf('complexDefault.enabled = %s$N', SEL(mainProg.complexDefault.enabled, 'FALSE', 'TRUE')); //CHECK: complexDefault.enabled = TRUE
printf('complexDefault.defaultInt = %d$N', mainProg.complexDefault.defaultInt); //CHECK: complexDefault.defaultInt = 0
printf('complexDefault.name = %s$N', mainProg.complexDefault.name); //CHECK: complexDefault.name = AXIS1
printf('complexDefault.state = %d$N', mainProg.complexDefault.state); //CHECK: complexDefault.state = 1
printf('complexDefault.stateDefault = %d$N', mainProg.complexDefault.stateDefault); //CHECK: complexDefault.stateDefault = 0
printf('complexDefault.simpleDefault.a = %d$N', mainProg.complexDefault.simpleDefault.a); //CHECK: complexDefault.simpleDefault.a = 10
printf('complexDefault.simpleDefault.b = %s$N', SEL(mainProg.complexDefault.simpleDefault.b, 'FALSE', 'TRUE')); //CHECK: complexDefault.simpleDefault.b = TRUE
printf('complexDefault.simple.a = %d$N', mainProg.complexDefault.simple.a); //CHECK: complexDefault.simple.a = 99
printf('complexDefault.simple.b = %s$N', SEL(mainProg.complexDefault.simple.b, 'FALSE', 'TRUE')); //CHECK: complexDefault.simple.b = FALSE
printf('complexDefault.partialSimple.a = %d$N', mainProg.complexDefault.partialSimple.a); //CHECK: complexDefault.partialSimple.a = 7
printf('complexDefault.partialSimple.b = %s$N', SEL(mainProg.complexDefault.partialSimple.b, 'FALSE', 'TRUE')); //CHECK: complexDefault.partialSimple.b = TRUE
printf('complexDefault.gains[1] = %f$N', mainProg.complexDefault.gains[1]); //CHECK: complexDefault.gains[1] = 1.000000
printf('complexDefault.gains[2] = %f$N', mainProg.complexDefault.gains[2]); //CHECK: complexDefault.gains[2] = 0.000000
printf('complexDefault.gains[3] = %f$N', mainProg.complexDefault.gains[3]); //CHECK: complexDefault.gains[3] = 0.100000

// Print complexPartial (partial init: id=9, partialSimple.a=10, state=element0)
printf('complexPartial.id = %d$N', mainProg.complexPartial.id); //CHECK: complexPartial.id = 9
printf('complexPartial.enabled = %s$N', SEL(mainProg.complexPartial.enabled, 'FALSE', 'TRUE')); //CHECK: complexPartial.enabled = TRUE
printf('complexPartial.state = %d$N', mainProg.complexPartial.state); //CHECK: complexPartial.state = 0
printf('complexPartial.partialSimple.a = %d$N', mainProg.complexPartial.partialSimple.a); //CHECK: complexPartial.partialSimple.a = 10
printf('complexPartial.partialSimple.b = %s$N', SEL(mainProg.complexPartial.partialSimple.b, 'FALSE', 'TRUE')); //CHECK: complexPartial.partialSimple.b = TRUE

// Print complexFull
printf('complexFull.id = %d$N', mainProg.complexFull.id); //CHECK: complexFull.id = 42
printf('complexFull.enabled = %s$N', SEL(mainProg.complexFull.enabled, 'FALSE', 'TRUE')); //CHECK: complexFull.enabled = FALSE
printf('complexFull.defaultInt = %d$N', mainProg.complexFull.defaultInt); //CHECK: complexFull.defaultInt = 77
printf('complexFull.name = %s$N', mainProg.complexFull.name); //CHECK: complexFull.name = FULL
printf('complexFull.state = %d$N', mainProg.complexFull.state); //CHECK: complexFull.state = 0
printf('complexFull.simple.a = %d$N', mainProg.complexFull.simple.a); //CHECK: complexFull.simple.a = 11
printf('complexFull.simple.b = %s$N', SEL(mainProg.complexFull.simple.b, 'FALSE', 'TRUE')); //CHECK: complexFull.simple.b = TRUE
printf('complexFull.partialSimple.a = %d$N', mainProg.complexFull.partialSimple.a); //CHECK: complexFull.partialSimple.a = 12
printf('complexFull.partialSimple.b = %s$N', SEL(mainProg.complexFull.partialSimple.b, 'FALSE', 'TRUE')); //CHECK: complexFull.partialSimple.b = TRUE
printf('complexFull.gains[1] = %f$N', mainProg.complexFull.gains[1]); //CHECK: complexFull.gains[1] = 9.000000
printf('complexFull.gains[2] = %f$N', mainProg.complexFull.gains[2]); //CHECK: complexFull.gains[2] = 8.000000
printf('complexFull.gains[3] = %f$N', mainProg.complexFull.gains[3]); //CHECK: complexFull.gains[3] = 7.000000

// Print ComplexConst (uses cComplexpartial: id=9, partialSimple.a=15, state=element0)
printf('ComplexConst.id = %d$N', mainProg.ComplexConst.id); //CHECK: ComplexConst.id = 9
printf('ComplexConst.partialSimple.a = %d$N', mainProg.ComplexConst.partialSimple.a); //CHECK: ComplexConst.partialSimple.a = 15
printf('ComplexConst.state = %d$N', mainProg.ComplexConst.state); //CHECK: ComplexConst.state = 0

// Print Simple_Override (cSimple_Override: a=77, b=FALSE)
printf('Simple_Override.a = %d$N', mainProg.Simple_Override.a); //CHECK: Simple_Override.a = 77
printf('Simple_Override.b = %s$N', SEL(mainProg.Simple_Override.b, 'FALSE', 'TRUE')); //CHECK: Simple_Override.b = FALSE

// Print Simple_Partial (cSimple_Partial: a=30, b=TRUE from type default)
printf('Simple_Partial.a = %d$N', mainProg.Simple_Partial.a); //CHECK: Simple_Partial.a = 30
printf('Simple_Partial.b = %s$N', SEL(mainProg.Simple_Partial.b, 'FALSE', 'TRUE')); //CHECK: Simple_Partial.b = TRUE

// Print ArrElem1 (gcSimple_Override: a=77, b=FALSE)
printf('ArrElem1.a = %d$N', mainProg.ArrElem1.a); //CHECK: ArrElem1.a = 77
printf('ArrElem1.b = %s$N', SEL(mainProg.ArrElem1.b, 'FALSE', 'TRUE')); //CHECK: ArrElem1.b = FALSE

// Print ArrElem3 (a=5, b=TRUE)
printf('ArrElem3.a = %d$N', mainProg.ArrElem3.a); //CHECK: ArrElem3.a = 5
printf('ArrElem3.b = %s$N', SEL(mainProg.ArrElem3.b, 'FALSE', 'TRUE')); //CHECK: ArrElem3.b = TRUE

// Print ArrSimple_Full (cArrSimple_Full: [gcArrElem1, (a=18,b=FALSE), gcArrElem3])
printf('ArrSimple_Full[1].a = %d$N', mainProg.ArrSimple_Full[1].a); //CHECK: ArrSimple_Full[1].a = 77
printf('ArrSimple_Full[1].b = %s$N', SEL(mainProg.ArrSimple_Full[1].b, 'FALSE', 'TRUE')); //CHECK: ArrSimple_Full[1].b = FALSE
printf('ArrSimple_Full[2].a = %d$N', mainProg.ArrSimple_Full[2].a); //CHECK: ArrSimple_Full[2].a = 18
printf('ArrSimple_Full[2].b = %s$N', SEL(mainProg.ArrSimple_Full[2].b, 'FALSE', 'TRUE')); //CHECK: ArrSimple_Full[2].b = FALSE
printf('ArrSimple_Full[3].a = %d$N', mainProg.ArrSimple_Full[3].a); //CHECK: ArrSimple_Full[3].a = 5
printf('ArrSimple_Full[3].b = %s$N', SEL(mainProg.ArrSimple_Full[3].b, 'FALSE', 'TRUE')); //CHECK: ArrSimple_Full[3].b = TRUE

// Print ArrSimple_Partial (cArrSimple_Partial: [(a=27), (b=FALSE), gcSimple_Partial])
printf('ArrSimple_Partial[1].a = %d$N', mainProg.ArrSimple_Partial[1].a); //CHECK: ArrSimple_Partial[1].a = 27
printf('ArrSimple_Partial[1].b = %s$N', SEL(mainProg.ArrSimple_Partial[1].b, 'FALSE', 'TRUE')); //CHECK: ArrSimple_Partial[1].b = TRUE
printf('ArrSimple_Partial[2].a = %d$N', mainProg.ArrSimple_Partial[2].a); //CHECK: ArrSimple_Partial[2].a = 10
printf('ArrSimple_Partial[2].b = %s$N', SEL(mainProg.ArrSimple_Partial[2].b, 'FALSE', 'TRUE')); //CHECK: ArrSimple_Partial[2].b = FALSE
printf('ArrSimple_Partial[3].a = %d$N', mainProg.ArrSimple_Partial[3].a); //CHECK: ArrSimple_Partial[3].a = 30
printf('ArrSimple_Partial[3].b = %s$N', SEL(mainProg.ArrSimple_Partial[3].b, 'FALSE', 'TRUE')); //CHECK: ArrSimple_Partial[3].b = TRUE

// Print Gains_Test (cGains_Test: [20.0, 30.0, 40.0])
printf('Gains_Test[1] = %f$N', mainProg.Gains_Test[1]); //CHECK: Gains_Test[1] = 20.000000
printf('Gains_Test[2] = %f$N', mainProg.Gains_Test[2]); //CHECK: Gains_Test[2] = 30.000000
printf('Gains_Test[3] = %f$N', mainProg.Gains_Test[3]); //CHECK: Gains_Test[3] = 40.000000

// Print myfbDefault outputs (default initialized)
printf('myfbDefault.oComplexDefault.id = %d$N', mainProg.myfbDefault.oComplexDefault.id); //CHECK: myfbDefault.oComplexDefault.id = 123
printf('myfbDefault.ocomplexPartial.id = %d$N', mainProg.myfbDefault.ocomplexPartial.id); //CHECK: myfbDefault.ocomplexPartial.id = 9
printf('myfbDefault.ocomplexFull.id = %d$N', mainProg.myfbDefault.ocomplexFull.id); //CHECK: myfbDefault.ocomplexFull.id = 42
printf('myfbDefault.oComplexConst.id = %d$N', mainProg.myfbDefault.oComplexConst.id); //CHECK: myfbDefault.oComplexConst.id = 9

// Print myfbOverride outputs (inputs overridden with cComplexpartial)
printf('myfbOverride.iComplexDefault.id = %d$N', mainProg.myfbOverride.iComplexDefault.id); //CHECK: myfbOverride.iComplexDefault.id = 9
printf('myfbOverride.icomplexPartial.id = %d$N', mainProg.myfbOverride.icomplexPartial.id); //CHECK: myfbOverride.icomplexPartial.id = 9
printf('myfbOverride.icomplexFull.id = %d$N', mainProg.myfbOverride.icomplexFull.id); //CHECK: myfbOverride.icomplexFull.id = 9
printf('myfbOverride.iComplexConst.id = %d$N', mainProg.myfbOverride.iComplexConst.id); //CHECK: myfbOverride.iComplexConst.id = 9

// Print OutComplexDefault (from MyTestFunction with default inputs)
printf('OutComplexDefault.id = %d$N', mainProg.OutComplexDefault.id); //CHECK: OutComplexDefault.id = 42
printf('OutComplexDefault.name = %s$N', mainProg.OutComplexDefault.name); //CHECK: OutComplexDefault.name = FULL
printf('OutComplexDefault.simple.a = %d$N', mainProg.OutComplexDefault.simple.a); //CHECK: OutComplexDefault.simple.a = 11

// Print OutComplexOverride (from MyTestFunction with cComplexpartial inputs)
printf('OutComplexOverride.id = %d$N', mainProg.OutComplexOverride.id); //CHECK: OutComplexOverride.id = 42
printf('OutComplexOverride.name = %s$N', mainProg.OutComplexOverride.name); //CHECK: OutComplexOverride.name = FULL
printf('OutComplexOverride.simple.a = %d$N', mainProg.OutComplexOverride.simple.a); //CHECK: OutComplexOverride.simple.a = 11

// Print global variables
printf('gComplexDefault.id = %d$N', gComplexDefault.id); //CHECK: gComplexDefault.id = 123
printf('gComplexDefault.enabled = %s$N', SEL(gComplexDefault.enabled, 'FALSE', 'TRUE')); //CHECK: gComplexDefault.enabled = TRUE
printf('gComplexPartial.id = %d$N', gComplexPartial.id); //CHECK: gComplexPartial.id = 9
printf('gComplexPartial.partialSimple.a = %d$N', gComplexPartial.partialSimple.a); //CHECK: gComplexPartial.partialSimple.a = 10
printf('gComplexFull.id = %d$N', gComplexFull.id); //CHECK: gComplexFull.id = 42
printf('gComplexFull.enabled = %s$N', SEL(gComplexFull.enabled, 'FALSE', 'TRUE')); //CHECK: gComplexFull.enabled = FALSE
printf('gComplexFull.defaultInt = %d$N', gComplexFull.defaultInt); //CHECK: gComplexFull.defaultInt = 77
printf('gComplexFull.name = %s$N', gComplexFull.name); //CHECK: gComplexFull.name = FULL
printf('gComplexFull.simple.a = %d$N', gComplexFull.simple.a); //CHECK: gComplexFull.simple.a = 77
printf('gComplexFull.simple.b = %s$N', SEL(gComplexFull.simple.b, 'FALSE', 'TRUE')); //CHECK: gComplexFull.simple.b = FALSE
printf('gComplexFull.partialSimple.a = %d$N', gComplexFull.partialSimple.a); //CHECK: gComplexFull.partialSimple.a = 12
printf('gComplexFull.partialSimple.b = %s$N', SEL(gComplexFull.partialSimple.b, 'FALSE', 'TRUE')); //CHECK: gComplexFull.partialSimple.b = TRUE
printf('gComplexFull.states[1] = %d$N', gComplexFull.states[1]); //CHECK: gComplexFull.states[1] = 1
printf('gComplexFull.states[2] = %d$N', gComplexFull.states[2]); //CHECK: gComplexFull.states[2] = 0
printf('gComplexFull.states[3] = %d$N', gComplexFull.states[3]); //CHECK: gComplexFull.states[3] = 1
printf('gComplexFull.arr[1].a = %d$N', gComplexFull.arr[1].a); //CHECK: gComplexFull.arr[1].a = 77
printf('gComplexFull.arr[1].b = %s$N', SEL(gComplexFull.arr[1].b, 'FALSE', 'TRUE')); //CHECK: gComplexFull.arr[1].b = FALSE
printf('gComplexFull.arr[2].a = %d$N', gComplexFull.arr[2].a); //CHECK: gComplexFull.arr[2].a = 18
printf('gComplexFull.arr[2].b = %s$N', SEL(gComplexFull.arr[2].b, 'FALSE', 'TRUE')); //CHECK: gComplexFull.arr[2].b = FALSE
printf('gComplexFull.arr[3].a = %d$N', gComplexFull.arr[3].a); //CHECK: gComplexFull.arr[3].a = 5
printf('gComplexFull.arr[3].b = %s$N', SEL(gComplexFull.arr[3].b, 'FALSE', 'TRUE')); //CHECK: gComplexFull.arr[3].b = TRUE
printf('gComplexFull.partialArr[1].a = %d$N', gComplexFull.partialArr[1].a); //CHECK: gComplexFull.partialArr[1].a = 27
printf('gComplexFull.partialArr[1].b = %s$N', SEL(gComplexFull.partialArr[1].b, 'FALSE', 'TRUE')); //CHECK: gComplexFull.partialArr[1].b = TRUE
printf('gComplexFull.partialArr[2].a = %d$N', gComplexFull.partialArr[2].a); //CHECK: gComplexFull.partialArr[2].a = 10
printf('gComplexFull.partialArr[2].b = %s$N', SEL(gComplexFull.partialArr[2].b, 'FALSE', 'TRUE')); //CHECK: gComplexFull.partialArr[2].b = FALSE
printf('gComplexFull.partialArr[3].a = %d$N', gComplexFull.partialArr[3].a); //CHECK: gComplexFull.partialArr[3].a = 30
printf('gComplexFull.partialArr[3].b = %s$N', SEL(gComplexFull.partialArr[3].b, 'FALSE', 'TRUE')); //CHECK: gComplexFull.partialArr[3].b = TRUE
printf('gComplexFull.gains[1] = %f$N', gComplexFull.gains[1]); //CHECK: gComplexFull.gains[1] = 20.000000
printf('gComplexFull.gains[2] = %f$N', gComplexFull.gains[2]); //CHECK: gComplexFull.gains[2] = 30.000000
printf('gComplexFull.gains[3] = %f$N', gComplexFull.gains[3]); //CHECK: gComplexFull.gains[3] = 40.000000
printf('gComplexConst.id = %d$N', gComplexConst.id); //CHECK: gComplexConst.id = 9
printf('gComplexConst.partialSimple.a = %d$N', gComplexConst.partialSimple.a); //CHECK: gComplexConst.partialSimple.a = 15
printf('gComplexConstMix.simple.a = %d$N', gComplexConstMix.simple.a); //CHECK: gComplexConstMix.simple.a = 77
printf('gComplexConstMix.simple.b = %s$N', SEL(gComplexConstMix.simple.b, 'FALSE', 'TRUE')); //CHECK: gComplexConstMix.simple.b = FALSE
printf('gComplexConstMix.partialSimple.a = %d$N', gComplexConstMix.partialSimple.a); //CHECK: gComplexConstMix.partialSimple.a = 30
printf('gComplexConstMix.partialSimple.b = %s$N', SEL(gComplexConstMix.partialSimple.b, 'FALSE', 'TRUE')); //CHECK: gComplexConstMix.partialSimple.b = TRUE
printf('gComplexConstMix.gains[1] = %f$N', gComplexConstMix.gains[1]); //CHECK: gComplexConstMix.gains[1] = 20.000000
printf('gComplexConstMix.gains[2] = %f$N', gComplexConstMix.gains[2]); //CHECK: gComplexConstMix.gains[2] = 30.000000
printf('gComplexConstMix.gains[3] = %f$N', gComplexConstMix.gains[3]); //CHECK: gComplexConstMix.gains[3] = 40.000000

// Print global constants
printf('gcSimple_Override.a = %d$N', gcSimple_Override.a); //CHECK: gcSimple_Override.a = 77
printf('gcSimple_Override.b = %s$N', SEL(gcSimple_Override.b, 'FALSE', 'TRUE')); //CHECK: gcSimple_Override.b = FALSE
printf('gcSimple_Partial.a = %d$N', gcSimple_Partial.a); //CHECK: gcSimple_Partial.a = 30
printf('gcSimple_Partial.b = %s$N', SEL(gcSimple_Partial.b, 'FALSE', 'TRUE')); //CHECK: gcSimple_Partial.b = TRUE
printf('gcGains_Test[1] = %f$N', gcGains_Test[1]); //CHECK: gcGains_Test[1] = 20.000000
printf('gcGains_Test[2] = %f$N', gcGains_Test[2]); //CHECK: gcGains_Test[2] = 30.000000
printf('gcGains_Test[3] = %f$N', gcGains_Test[3]); //CHECK: gcGains_Test[3] = 40.000000

END_FUNCTION
