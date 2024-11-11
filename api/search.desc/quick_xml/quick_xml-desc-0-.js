searchState.loadedDescShard("quick_xml", 0, "High performance XML reader/writer.\nSerde custom error\n(De)serialization error\nEmpty <code>Event::DocType</code>. <code>&lt;!doctype foo&gt;</code> is correct but ` is …\nEnd event mismatch\nContains the error value\nThe error type used by this crate.\nEscape error\nThis error indicates that <code>deserialize_struct</code> was called, …\nAttribute parsing error\nCannot parse specified value to boolean\nCannot parse to float\nCannot parse to integer\nXml parsing error\nIO error.\nThis error indicates an error in the <code>Deserialize</code> …\nInput decoding error. If <code>encoding</code> feature is disabled, …\nContains the success value\nA specialized <code>Result</code> type where the error is hard-wired to …\nText not found, expected <code>Event::Text</code>\nUnexpected &lt;!&gt;\nDeserializer encounter an end tag with a specified name …\nUnexpected End of File\nThe <code>Reader</code> produced <code>Event::Eof</code> when it is not expecting, …\nDeserializer encounter a start tag with a specified name …\nUnexpected token\nSpecified namespace prefix is unknown, cannot resolve …\nAn attempt to deserialize to a type, that is not supported …\n<code>Event::BytesDecl</code> must start with <em>version</em> attribute. …\nSerde <code>Deserializer</code> module.\nA module for wrappers that encode / decode data.\nManage xml character escapes\nDefines zero-copy XML events used throughout this library.\nReturns the argument unchanged.\nCreates a new <code>Error::EscapeError</code> from the given error\nCreates a new <code>Error::Utf8</code> from the given error\nCreates a new <code>Error::NonDecodable</code> from the given error\nCreates a new <code>Error::Io</code> from the given error\nCalls <code>U::from(self)</code>.\nModule for handling names according to the W3C Namespaces …\nContains high-level interface for a pull-based XML parser.\nModule to handle custom serde <code>Serializer</code>\nContains high-level interface for an events-based XML …\nExpected end event\nFound end event\nUnescaped character data stored in <code>&lt;![CDATA[...]]&gt;</code>.\nSerde custom error\n(De)serialization error\nSimplified event which contains only these variants that …\nA structure that deserializes XML into Rust values.\nDocument type definition data (DTD) stored in …\nEnd tag <code>&lt;/tag&gt;</code>.\nEnd tag <code>&lt;/tag&gt;</code>.\nUsed to resolve unknown entities while parsing\nEnd of XML document.\nEnd of XML document.\nThe error type that represents DTD parse error\nThis error indicates that <code>deserialize_struct</code> was called, …\nCannot parse specified value to boolean\nCannot parse to float\nCannot parse to integer\nXml parsing error\nXML input source that reads from a std::io input stream.\nThis error indicates an error in the <code>Deserialize</code> …\nAn <code>EntityResolver</code> that does nothing and always returns <code>None</code>…\nSimplified event which contains only these variants that …\nXML input source that reads from a slice of bytes and can …\nStart tag (with attributes) <code>&lt;tag attr=&quot;value&quot;&gt;</code>.\nStart tag (with attributes) <code>&lt;tag attr=&quot;value&quot;&gt;</code>.\nDecoded and concatenated content of consequent <code>Text</code> and …\nDecoded and concatenated content of consequent <code>Text</code> and …\nEscaped character data between tags.\nDeserializer encounter an end tag with a specified name …\nThe <code>Reader</code> produced <code>Event::Eof</code> when it is not expecting, …\nDeserializer encounter a start tag with a specified name …\nAn attempt to deserialize to a type, that is not supported …\nTrait used by the deserializer for iterating over input. …\nCalled on contents of <code>Event::DocType</code> to capture declared …\nA copy of the reader’s decoder used to decode strings.\nForwards deserialization to the <code>deserialize_bytes</code>.\nReturns <code>DeError::Unsupported</code>\nCharacter represented as strings.\nIdentifiers represented as strings.\nAlways call <code>visitor.visit_unit()</code> because returned value …\nRepresentation of the newtypes the same as one-element …\nRepresentation of owned strings the same as non-owned.\nRepresentation of tuples the same as sequences.\nRepresentation of named tuples the same as unnamed tuples.\nUnit represented in XML as a <code>xs:element</code> or text/CDATA …\nRepresentation of the named units the same as unnamed units…\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nDeserialize from a reader. This method will do internal …\nCreate new deserializer that will copy data from the …\nDeserialize an instance of type <code>T</code> from a string of XML …\nCreate new deserializer that will borrow data from the …\nCreate new deserializer that will borrow data from the …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturn an input-borrowing event.\nSkips until end element is found. Unlike <code>next()</code> it will …\nCalled when an entity needs to be resolved.\nCreate new deserializer that will copy data from the …\nDecoder of byte slices into strings.\nWithout <code>encoding</code> feature\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nEntity with Null character\nError for XML escape / unescape.\nNot a valid unicode codepoint\nCharacter is not a valid decimal value\nCharacter is not a valid hexadecimal value\nCannot convert decimal to hexa\nCannot convert Hexa to utf8\nUnrecognized escape symbol\nCannot find <code>;</code> after <code>&amp;</code>\nEscapes an <code>&amp;str</code> and replaces all xml special characters (<code>&lt;</code>…\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nEscapes an <code>&amp;str</code> and replaces xml special characters (<code>&lt;</code>, <code>&gt;</code>, …\nUnescape an <code>&amp;str</code> and replaces all xml escaped characters (…\nUnescape an <code>&amp;str</code> and replaces all xml escaped characters (…\nCDATA content contains unescaped data from the reader. If …\nAn XML declaration (<code>Event::Decl</code>).\nA struct to manage <code>Event::End</code> events\nOpening tag data (<code>Event::Start</code>), with optional attributes.\nData from various events (most notably, <code>Event::Text</code>) that …\nUnescaped character data stored in <code>&lt;![CDATA[...]]&gt;</code>.\nComment <code>&lt;!-- ... --&gt;</code>.\nXML declaration <code>&lt;?xml ...?&gt;</code>.\nDocument type definition data (DTD) stored in …\nEmpty element tag (with attributes) <code>&lt;tag attr=&quot;value&quot; /&gt;</code>.\nEnd tag <code>&lt;/tag&gt;</code>.\nEnd of XML document.\nEvent emitted by <code>Reader::read_event_into</code>.\nProcessing instruction <code>&lt;?...?&gt;</code>.\nStart tag (with attributes) <code>&lt;tag attr=&quot;value&quot;&gt;</code>.\nEscaped character data between tags.\nXml Attributes module\nReturns an iterator over the attributes of this tag.\nGets the undecoded raw string with the attributes of this …\nConverts the event into a borrowed event. Most useful when …\nConverts the event into a borrowed event.\nConverts the event into a borrowed event.\nConverts the event into a borrowed event.\nConverts the event into a borrowed event.\nConverts the event into a borrowed event.\nRemove all attributes from the ByteStart\nGets xml encoding, excluding quotes (<code>&#39;</code> or <code>&quot;</code>).\nConverts this CDATA content to an escaped version, that …\nAdd additional attributes to this tag using an iterator.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a new <code>BytesStart</code> from the given content (name + …\nCreates a new <code>BytesText</code> from an escaped string.\nCreates a <code>BytesDecl</code> from a <code>BytesStart</code>\nReturns an iterator over the HTML-like attributes of this …\nRemoves trailing XML whitespace bytes from text content.\nRemoves leading XML whitespace bytes from text content.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nExtracts the inner <code>Cow</code> from the <code>BytesText</code> event container.\nExtracts the inner <code>Cow</code> from the <code>BytesCData</code> event container.\nConverts the event into an owned event.\nConverts the event into an owned event.\nConverts the event into an owned event.\nEnsures that all data is owned to extend the object’s …\nEnsures that all data is owned to extend the object’s …\nConverts the event to an owned version, untied to the …\nGets the undecoded raw local tag name (excluding …\nGets the undecoded raw local tag name (excluding …\nGets the undecoded raw tag name, as present in the input …\nGets the undecoded raw tag name, as present in the input …\nCreates a new <code>BytesStart</code> from the given name.\nConstructs a new <code>XmlDecl</code> from the (mandatory) <em>version</em> …\nCreates a new <code>BytesEnd</code> borrowing a slice.\nCreates a new <code>BytesText</code> from a string. The string is …\nCreates a new <code>BytesCData</code> from a string.\nConverts this CDATA content to an escaped version, that …\nAdds an attribute to this element.\nEdit the name of the BytesStart in-place\nGets xml standalone, excluding quotes (<code>&#39;</code> or <code>&quot;</code>).\nCreates new paired close tag\nConverts the event into an owned event without taking …\nTry to get an attribute\nDecodes then unescapes the content of the event.\nDecodes then unescapes the content of the event with …\nGets xml version, excluding quotes (<code>&#39;</code> or <code>&quot;</code>).\nConsumes <code>self</code> and yield a new <code>BytesStart</code> with additional …\nA struct representing a key/value XML or HTML attribute.\nErrors that can be raised during parsing attributes.\nA struct representing a key/value XML attribute.\nIterator over XML attributes.\nAttribute with value enclosed in double quotes (<code>&quot;</code>). …\nAn attribute with the same name was already encountered. …\nAttribute without value. Attribute key provided. This is …\nAttribute key was not followed by <code>=</code>, position relative to …\nAttribute value was not finished with a matching quote, …\nAttribute value was not found after <code>=</code>, position relative …\nAttribute with value enclosed in single quotes (<code>&#39;</code>). …\nAttribute with value not enclosed in quotes. Attribute key …\nAttribute value is not quoted, position relative to the …\nDecodes then unescapes the value.\nDecodes then unescapes the value with custom entities.\nCreates new attribute from raw bytes. Does not apply any …\nCreates new attribute from text representation. Key is …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a new attribute iterator from a buffer, allowing …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the key value\nThe key to uniquely define the attribute.\nMaps an <code>Attr&lt;T&gt;</code> to <code>Attr&lt;U&gt;</code> by applying a function to a …\nCreates a new attribute iterator from a buffer.\nDecodes using UTF-8 then unescapes the value.\nDecodes using UTF-8 then unescapes the value, using custom …\nReturns the attribute value. For <code>Self::Empty</code> variant an …\nThe raw value of the attribute.\nChanges whether attributes should be checked for …\n<code>Prefix</code> resolved to the specified namespace\nXML attribute binds a default namespace. Corresponds to …\nA local (unqualified) name of an element or an attribute, …\nXML attribute binds a specified prefix to a namespace. …\nA namespace name that is declared in a …\nA namespace prefix part of the qualified name of an …\nA namespace prefix declaration, <code>xmlns</code> or <code>xmlns:&lt;name&gt;</code>, as …\nA qualified name of an element or an attribute, including …\nResult of prefix resolution which creates by …\nQualified name does not contain prefix, and resolver does …\nSpecified prefix was not found in scope\nIf that <code>QName</code> represents <code>&quot;xmlns&quot;</code> series of names, returns …\nThe same as <code>(qname.local_name(), qname.prefix())</code>, but does …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates <code>LocalName</code> from a <code>QName</code>\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts this name to an internal slice representation.\nConverts this name to an internal slice representation.\nExtracts internal slice\nConverts this namespace to an internal slice …\nReturns local part of this qualified name.\nReturns namespace part of this qualified name or <code>None</code> if …\nA low level encoding-agnostic XML event reader that …\nA low level encoding-agnostic XML event reader.\nRange of input in bytes, that corresponds to some piece of …\nGets the current byte position in the input data.\nChanges whether comments should be validated.\nChanges whether comments should be validated.\nChanges whether mismatched closing tag names should be …\nChanges whether mismatched closing tag names should be …\nGet the decoder, used to decode bytes, read by this …\nThe upper bound of the range (exclusive).\nChanges whether empty elements should be split into an <code>Open</code>…\nChanges whether empty elements should be split into an <code>Open</code>…\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates an XML reader from a file path.\nCreates an XML reader from a file path.\nCreates a <code>NsReader</code> that reads from a reader.\nCreates a <code>Reader</code> that reads from a given reader.\nCreates an XML reader from a string slice.\nCreates an XML reader from a string slice.\nGets a mutable reference to the underlying reader.\nGets a mutable reference to the underlying reader.\nGets a reference to the underlying reader.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes <code>NsReader</code> returning the underlying reader\nConsumes <code>Reader</code> returning the underlying reader\nReads the next event, borrow its content from the input …\nRead an event that borrows from the input rather than a …\nReads the next event into given buffer.\nReads the next <code>Event</code>.\nReads the next event, borrow its content from the input …\nReads the next event into given buffer and resolves its …\nReads content between start and end tags, including any …\nReads content between start and end tags, including any …\nReads until end element is found. This function is …\nReads until end element is found. This function is …\nReads until end element is found using provided buffer as …\nReads until end element is found using provided buffer as …\nResolves a potentially qualified <strong>element name</strong> or <strong>attribute </strong>…\nResolves a potentially qualified <strong>attribute name</strong> into …\nResolves a potentially qualified <strong>element name</strong> into …\nThe lower bound of the range (inclusive).\nChanges whether trailing whitespaces after the markup name …\nChanges whether trailing whitespaces after the markup name …\nChanges whether whitespace before and after character data …\nChanges whether whitespace before and after character data …\nChanges whether whitespace after character data should be …\nChanges whether whitespace after character data should be …\nPerforms escaping, escape all characters that could have …\nPerforms the minimal possible escaping, escape only …\nPerforms escaping that is compatible with SGML …\nDefines which characters would be escaped in <code>Text</code> events …\nA Serializer\nEnable or disable expansion of empty elements. Defaults to …\nReturns the argument unchanged.\nReturns the argument unchanged.\nConfigure indent for a serializer\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new <code>Serializer</code> that uses struct name as a root …\nSerialize struct into a <code>String</code>.\nSerialize struct into a <code>String</code> using specified root tag …\nSerialize struct into a <code>Write</code>r.\nSerialize struct into a <code>Write</code>r using specified root tag …\nCreates a new <code>Serializer</code> that uses specified root tag …\nA struct to write an element. Contains methods to add …\nXML writer. Writes XML <code>Event</code>s to a <code>std::io::Write</code> …\nProvides a simple, high-level API for writing XML elements.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the underlying writer.\nGet a reference to the underlying writer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes this <code>Writer</code>, returning the underlying writer.\nCreates a <code>Writer</code> from a generic writer.\nCreates a <code>Writer</code> with configured indents from a generic …\nAdds an attribute to this element.\nAdd additional attributes to this element using an …\nWrite a Byte-Order-Mark character to the document.\nWrite a CData event <code>&lt;![CDATA[...]]&gt;</code> inside the current …\nWrite an empty (self-closing) tag.\nWrites the given event to the underlying writer.\nManually write a newline and indentation at the proper …\nCreate a new scope for writing XML inside the current …\nWrite a processing instruction <code>&lt;?...?&gt;</code> inside the current …\nWrite an arbitrary serializable type\nWrite some text inside the current element.")