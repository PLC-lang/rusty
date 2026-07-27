FUNCTION main
VAR
    MyString : STRING;
    MyInt : INT := 1234;
    MyGuard : DINT := 24681357;
    retValue : ErrorCode;
END_VAR

retValue := Config_GetString('/service/config/logLevel', MyString);
printf('ret=%d my_int=%d my_guard=%d value_prefix=%.8s', retValue, MyInt, MyGuard, MyString);
// CHECK: ret=0 my_int=1234 my_guard=24681357 value_prefix=AAAAAAAA
END_FUNCTION
