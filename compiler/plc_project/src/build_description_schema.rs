pub(crate) const PLC_JSON_SCHEMA: &str = r#"
        {
            "$schema": "http://json-schema.org/draft-06/schema#",
            "title": "Schema for plc.json",
            "type": "object",
            "properties": {
              "name": {
                "type": "string"
              },
              "version": {
                "type": "string"
              },
              "format-version": {
                "type": "string"
              },
              "files": {
                "type": "array",
                "items": {
                  "type": "string"
                },
                "minItems": 1
              },
              "compile_type": {
                "type": "string"
              },
              "output": {
                "type": "string"
              },
              "libraries": {
                "type": "array",
                "items": {
                  "type": "object",
                  "properties": {
                    "name": {
                      "type": "string"
                    },
                    "path": {
                      "type": "string"
                    },
                    "package": {
                      "type": "string"
                    },
                    "include_path": {
                      "type": "array",
                      "items": {
                        "type": "string"
                      }
                    },
                    "architectures": {
                      "type": "array",
                      "items": {
                        "type": "object"
                      }
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "name",
                    "path",
                    "package",
                    "include_path"
                  ]
                }
              },
              "package_commands": {
                  "type": "array",
                  "items": {
                      "type": "string"
                  }
              }
            },
            "additionalProperties": false,            
            "required": [
              "name",
              "files",
              "compile_type"
            ]
        }"#;
