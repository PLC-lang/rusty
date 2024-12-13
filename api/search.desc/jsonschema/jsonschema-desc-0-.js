searchState.loadedDescShard("jsonschema", 0, "jsonschema\nFull configuration to guide the <code>JSONSchema</code> compilation.\nJSON Schema Draft version\nJSON Schema Draft 4\nJSON Schema Draft 6\nJSON Schema Draft 7\nThe structure that holds a JSON Schema compiled into a …\nA resolver that resolves external schema references. …\nAn opaque error type that is returned by resolvers on …\nApply the schema and return an <code>Output</code>. No actual work is …\nCompile <code>schema</code> into <code>JSONSchema</code> using the currently defined …\nCompile the input schema into a validation tree.\nThe <code>CompilationOptions</code> that were used to compile this …\nThe <code>Draft</code> which this schema was compiled against\nError types\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA shortcut for validating <code>instance</code> against <code>schema</code>. Draft …\nRun validation against <code>instance</code> but return a boolean …\nReturn a default <code>CompilationOptions</code> that can configure …\nImplementation of json schema output formats specified in …\nFacilities for working with paths within schemas or …\nPrimitive types for property type validators\nResolve an external schema via an URL.\nSet the <code>false</code> if unrecognized formats should be reported …\nForce enable or disable format validation. The default …\nRun validation against <code>instance</code> and return an iterator …\nEnsure that compiled schema is going to support the …\nEnsure that compiled schema is going to support the …\nAdd a new document to the store. It works as a cache to …\nEnsure that the schema is going to be compiled using the …\nRegister a custom “format” validator.\nAdd meta schemas for supported JSON Schema drafts. It is …\nUse a custom resolver for resolving external schema …\nEnsure that compiled schema is not supporting the provided …\nEnsure that compiled schema is not supporting the provided …\nThe input array contain more items than expected.\nUnexpected properties.\nThe input value is not valid under any of the schemas …\nResults from a [<code>fancy_regex::Error::BacktrackLimitExceeded</code>]…\nThe input value doesn’t match expected constant.\nThe input array doesn’t contain items conforming to the …\nThe input value does not respect the defined …\nThe input value does not respect the defined …\nThe input value doesn’t match any of specified options.\nAn iterator over instances of <code>ValidationError</code> that …\nValue is too large.\nValue is too small.\nEverything is invalid for <code>false</code> schema.\nIf the referenced file is not found during ref resolution.\nWhen the input doesn’t match to the specified format.\nMay happen in <code>contentEncoding</code> validation if <code>base64</code> encoded …\n<code>ref</code> value is not valid.\nInvalid URL, e.g. invalid port number or IP address\nMay happen during ref resolution when remote document is …\nToo many items in an array.\nString is too long.\nToo many properties in an object.\nValue is too large.\nToo few items in an array.\nString is too short.\nNot enough properties in an object.\nValue is too small.\nWhen some number is not a multiple of another number.\nNegated schema failed validation.\nThe given schema is valid under more than one of the …\nThe given schema is not valid under any of the schemas …\nWhen the input doesn’t match to a pattern.\nObject property names are invalid.\nWhen a required property is missing.\nError during schema ref resolution.\nResolved schema failed to compile.\nWhen the input value doesn’t match one or multiple …\nUnexpected properties.\nWhen the input array has non-unique elements.\nReference contains unknown scheme.\nInvalid UTF-8 string during percent encoding when …\nAn error that can occur during validation.\nKinds of errors that may happen during validation\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nValue of the property that failed validation.\nPath to the value that failed validation.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nType of validation error.\nPath to the JSON Schema keyword that failed validation.\nThe resolution error.\nThe url that was tried to be resolved.\nAnnotations associated with an output unit.\nThe “basic” output format. See the documentation for …\nAn error associated with an <code>OutputUnit</code>\nThe schema was invalid\nThe output format resulting from the application of a …\nAn output unit is a reference to a place in a schema and a …\nThe schema was valid, collected annotations can be examined\nThe absolute location in the schema of the keyword. This …\nOutput a list of errors and annotations for each element …\nThe error for this output unit\nIndicates whether the schema was valid, corresponds to the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe location in the instance\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA shortcut to check whether the output represents passed …\nThe location in the schema of the keyword\nThe annotations found at this output unit\nThe <code>serde_json::Value</code> of the annotation\nAn absolute reference\nIndex within a JSON array.\nJSON Pointer as a wrapper around individual path …\nJSON Schema keyword.\nA key within a JSON object or an index within a JSON array.\nProperty name within a JSON object.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nJSON pointer as a vector of strings. Each component is …\nReturn an iterator over the underlying vector of path …\nTake the last pointer chunk.\nFor faster error handling in “type” keyword validator …\nCompact representation of multiple <code>PrimitiveType</code>\nIterator over all <code>PrimitiveType</code> present in a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")