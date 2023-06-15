#[derive(Debug)]
pub(crate) struct Interface {
    //TODO: return type, variables, etc..
    // we only care about the addData field for now, as it contains our text-declaration

    // the addData field is be application-specific and can be implemented/extended as needed
    add_data: Option<Data>,
}

// XXX: this implementation is very specific to our own format. we might want to make it generic/provide an interface via traits
// so it can be easily modified/extended for other purposes
impl Interface {
    pub fn new(content: &str) -> Self {
        Interface { add_data: Some(Data::new_implementation(content)) }
    }

    pub fn get_data_content(&self) -> Option<&str> {
        let Some(ref data) = self.add_data else {
            return None
        };
        Some(&data.content)
    }

    // We have to append a END_... to the declaration, as it is missing in our text declaration
    pub fn append_end_keyword(self, pou_type: &str) -> Self {
        let Some(old_data) = self.add_data else {
            // if we have no content, we have nothing to append to. return as is
            return self
        };
        Interface {
            add_data: Some(Data::new_implementation(&format!("{}END_{}", old_data.content, pou_type))),
        }
    }
}

/// Application specific data
#[derive(Debug)]
struct Data {
    content: String,
    handle: HandleUnknown,
}

impl Data {
    fn new_implementation(content: &str) -> Self {
        Data { content: content.to_string(), handle: HandleUnknown::Implementation }
    }
}

/// Recommended processor-handling for unknown data elements
#[derive(Debug)]
enum HandleUnknown {
    Preserve,
    Discard,
    Implementation,
}
