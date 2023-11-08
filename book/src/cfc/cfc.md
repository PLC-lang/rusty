# CFC (Continous Function Chart)

RuSTy is compatible with CFC, as per the FBD part detailed in the [IEC61131-3 XML-exchange format](https://www.plcopen.org/system/files/downloads/tc6_xml_v201_technical_doc.pdf). 
The CFC implementation borrows extensively from the [ST compiler-pipeline](../arch/architecture.md), with the exception that the lexical analysis and parsing phases are replaced by a model-to-model conversion process. 
This involves converting the XML into a structured model, which is then converted into ST AST statements. 


The next chapter will walk you through the CFC implementation, giving you a better understanding of underlying [code](https://github.com/PLC-lang/rusty/tree/master/compiler/plc_xml). 