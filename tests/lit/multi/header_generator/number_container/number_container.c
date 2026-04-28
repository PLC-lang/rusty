#include "number_container.h"
#include <stdio.h>

NumberContainerMembers* GetMembers(NumberContainer_type *t);

void PrintNumber(int16_t valueToPrint) {
    printf("The number you asked for: %d\n", valueToPrint);
}

void __NumberContainer__init(NumberContainer_type* self) { }

void NumberContainer(NumberContainer_type* self) { }

void NumberContainer__FB_INIT(NumberContainer_type* self) {
    NumberContainerMembers member = { 21 };
    GlobalMember = member;
    self->_Internal_ = &GlobalMember;
    GetMembers(self)->value = 42;
}

int16_t NumberContainer__GetMemberValue(NumberContainer_type* self) {
    return GetMembers(self)->value;
}

NumberContainerMembers* GetMembers(NumberContainer_type *t) {
    return (NumberContainerMembers*)(t->_Internal_);
}
