// Declared in a separate compilation unit at the same hardware address as
// file1.st's `foo`. Without the cross-unit dedup of synthetic `__PI_*`
// globals, this used to fail to link with an ambiguous-symbol error
// (#1307).
VAR_GLOBAL
    bar AT %IX1.2.3.4 : BOOL;
END_VAR
