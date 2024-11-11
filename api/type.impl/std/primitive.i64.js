(function() {
    var type_impls = Object.fromEntries([["iec61131std",[]],["rustix",[]],["rusty",[]],["serde",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[18,14,13,13]}