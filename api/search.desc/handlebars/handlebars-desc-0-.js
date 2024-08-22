searchState.loadedDescShard("handlebars", 0, "Handlebars\nA data structure holds contextual data for current block …\nA map holds block parameters. The parameter can be either …\nThe context wrap data you render on your templates.\nRender-time Decorator data when using in a decorator …\nDecorator Definition\nContains the error value\nThis type represents an <em>escape fn</em>, that is a function …\nEvaluate decorator\nThe single entry point of your Handlebars templates\nRender-time Helper data when using in a helper definition\nHelper Definition\nA type alias for <code>Result&lt;(), RenderError&gt;</code>\nRender Json data with default format\nContains the success value\nThe Output API.\nRepresents the Json path in templates.\nJson wrapper that holds the Json value and reference path …\nThe context of a render call\nError when rendering data on template.\nTemplate rendering error\nRender trait\nA JSON wrapper designed for handlebars internal use case\nError on parsing template.\nTemplate parsing error\nAdd a path reference as the parameter. The <code>path</code> is a …\nAdd a value as parameter.\nget the JSON reference\nborrow a reference to current scope’s base path all …\nborrow a mutable reference to the base path\nborrow the base value\nBorrow a reference to current block context\nBorrow a mutable reference to current block context in …\nReturns block param if any\nReturn block param pair (for example |key, val|) if any\nA complex version of helper interface.\nA complex version of helper interface.\nA simplified api to define helper\nA simplified api to define helper\nUnregister all templates\nGet the modified context data if any\nReturns full path to this value if any\nReturn the Json data wrapped in context\nReturn the mutable reference to Json data wrapped in …\nReturn dev mode state, default is false\nEvaluate a Json path in current scope.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a block parameter by its name.\nGet a block parameter from this block. Block parameters …\nReturns the current template name. Note that the name can …\nGet a reference to the current <em>escape fn</em>.\nAttempt to get a helper from current render context.\nget a local variable from current scope\nGet registered partial in this render context\nGet root template name if any. This is the template name …\nReturn a registered template,\nReturn all templates registered\nMacro that allows you to quickly define a handlebars …\nReturns if the helper has either a block param or block …\nReturn <code>true</code> if a template is registered for the given name\nReturns hash, resolved within the context\nReturns hash, resolved within the context\nReturn hash value of a given key, resolved within the …\nReturn hash value of a given key, resolved within the …\nThe default <em>escape fn</em> replaces the characters <code>&amp;&quot;&lt;&gt;</code> with …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the template of <code>else</code> branch if any\nReturns if the helper is a block one <code>{{#helper}}{{/helper}}</code>…\nTest if given template name is current template.\nGet the escape toggle\nTest if value is missing\nGet mutable access to the local variables\nGet template name of this error Returns <code>None</code> when the …\nReturns helper name\nReturns helper name\nCreate a empty block parameter map.\ncreate a new <code>BlockContext</code> with default data\nCreate a render context\n<code>EscapeFn</code> that does not change anything. Useful when using …\nCreate a context with null data\nReturns nth helper param, resolved within the context.\nReturns nth helper param, resolved within the context\nReturns all helper params, resolved within the context\nReturns all helper params, resolved within the context\nPop and drop current block context. This is typically …\nGet the line number and column number of this error\nReturn state for <code>prevent_indent</code> option, default to <code>false</code>.\nPush a block context into render context stack. This is …\nGet <code>RenderErrorReason</code> for this error\nGet underlying reason for the error\nRegister a decorator\nRegister a new <em>escape fn</em> to be used from now on by this …\nRegister a helper\nRegister a helper in this render context. This is a …\nRegister a partial string\nRegister a <code>Template</code>\nRegister a template from a path on file system\nRegister a template string\nReturns relative path when the value is referenced If the …\nRemove a registered partial\nrender into RenderContext’s <code>writer</code>\nRender a registered template with some data into a string\nRender a template string using current registry without …\nRender a template string using current registry without …\nRender a template string using reusable context data\nRender a template string using resuable context, and write …\nRender a registered template and write data to the …\nRender a registered template with reused context\nRender a registered template using reusable <code>Context</code>, and …\nrender into string\nrender into string\nset the base value\nSet a block parameter into this block.\nSet new context data into the render process. This is …\nSet the current template name.\nEnable or disable dev mode\nSet the escape toggle. When toggle is on, escape_fn will …\nset a local variable into current scope\nRegister a partial for this context\nEnable or disable indent for partial include tag <code>{{&gt;}}</code>\nEnable or disable handlebars strict mode\nReturn strict mode state, default is false.\nReturns the default inner template if the helper is a …\nReturns the default inner template if any\nConvert any serializable data into Serde Json type\nRestore the default <em>escape fn</em>.\nRemove a helper from render context\nRemove a template from the registry\nReturns the value\nCreate a context with given data\nDesigned to be used with <code>write!</code> macro. for backward …\nDesigned to be used with <code>write!</code> macro. for backward …\nA handlebars template\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")