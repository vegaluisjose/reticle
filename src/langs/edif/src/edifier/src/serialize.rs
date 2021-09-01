/*
Copyright 2021 Pedro M. Torruella N.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
use crate::ast::*;
use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;

impl Serialize for Rename {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"rename".to_string())?;
        seq.serialize_element(&self.from)?;
        let mut with_quotes = r#"""#.to_string();
        with_quotes.push_str(&self.to);
        with_quotes.push_str(&r#"""#.to_string());
        seq.serialize_element(&with_quotes)?;
        seq.end()
    }
}

impl Serialize for GenericRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.reference)?;
        seq.end()
    }
}

impl Serialize for PropertyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;

        match self {
            PropertyValue::Integer(val) => {
                seq.serialize_element(&"integer".to_string())?;
                seq.serialize_element(&val)?;
                seq.end()
            }
            PropertyValue::String(val) => {
                seq.serialize_element(&"string".to_string())?;
                let mut with_quotes = r#"""#.to_string();
                with_quotes.push_str(val);
                with_quotes.push_str(&r#"""#.to_string());
                seq.serialize_element(&with_quotes)?;
                seq.end()
            }
        }
    }
}

impl Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"property".to_string())?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.property)?;
        seq.end()
    }
}

impl Serialize for PortMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"member".to_string())?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.index)?;
        seq.end()
    }
}

impl Serialize for PortRefToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PortRefToken::Name(elem) => elem.serialize(serializer),
            PortRefToken::Member(elem) => elem.serialize(serializer),
        }
    }
}

impl Serialize for PortRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"portref".to_string())?;
        seq.serialize_element(&self.token)?;
        if !self.instanceref.is_empty() {
            seq.serialize_element(&self.instanceref)?
        }
        seq.end()
    }
}

impl Serialize for PortList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_empty() {
            let mut seq = serializer.serialize_seq(Some(self.len() + 1))?;
            seq.serialize_element(&"joined".to_string())?;
            for portref in self.iter() {
                seq.serialize_element(&portref)?;
            }
            seq.end()
        } else {
            serializer.serialize_none()
        }
    }
}

impl Serialize for StringToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            StringToken::Name(elem) => elem.serialize(serializer),
            StringToken::Rename(elem) => elem.serialize(serializer),
        }
    }
}

impl Serialize for ContentNet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"net".to_string())?;
        seq.serialize_element(&self.token)?;

        if !self.portlist.is_empty() {
            seq.serialize_element(&self.portlist)?;
        }
        seq.end()
    }
}

impl Serialize for CellRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"cellref".to_string())?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.libraryref)?;
        seq.end()
    }
}

impl Serialize for ContentInstance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3 + self.properties.len()))?;
        let all = ("viewref", &self.viewref, &self.cellref);

        seq.serialize_element(&"instance".to_string())?;
        seq.serialize_element(&self.token)?;
        seq.serialize_element(&all)?;
        if !self.properties.is_empty() {
            for prop in self.properties.iter() {
                seq.serialize_element(&prop)?;
            }
        }
        seq.end()
    }
}

impl Serialize for ContentElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ContentElement::Instance(elem) => elem.serialize(serializer),
            ContentElement::Net(elem) => elem.serialize(serializer),
        }
    }
}

impl Serialize for CellContents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_empty() {
            let mut seq = serializer.serialize_seq(Some(2))?;
            seq.serialize_element(&"contents".to_string())?;

            for element in self.iter() {
                seq.serialize_element(&element)?;
            }
            seq.end()
        } else {
            serializer.serialize_none()
        }
    }
}

impl Serialize for PortDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&"direction".to_string())?;
        match self {
            PortDirection::Input => {
                seq.serialize_element(&"INPUT".to_string())?;
            }
            PortDirection::Output => {
                seq.serialize_element(&"OUTPUT".to_string())?;
            }
        }
        seq.end()
    }
}

impl Serialize for PortArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;

        seq.serialize_element(&"array".to_string())?;
        seq.serialize_element(&self.rename)?;
        seq.serialize_element(&self.length)?;
        seq.end()
    }
}

impl Serialize for PortToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PortToken::Name(name) => name.serialize(serializer),
            PortToken::Array(array) => array.serialize(serializer),
        }
    }
}

impl Serialize for InterfacePort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"port".to_string())?;
        seq.serialize_element(&self.token)?;
        seq.serialize_element(&self.direction)?;
        seq.end()
    }
}

impl Serialize for CellInterface {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_empty() {
            let mut seq = serializer.serialize_seq(Some(1 + self.len()))?;
            seq.serialize_element(&"interface".to_string())?;

            for port in self.iter() {
                seq.serialize_element(&port)?;
            }
            seq.end()
        } else {
            serializer.serialize_none()
        }
    }
}

impl Serialize for CellView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len_interface = self.interface.len();
        let len_contents = self.contents.len();
        let len_properties = self.properties.len();
        let local_size = 2 + len_interface + len_contents + len_properties;
        let mut seq = serializer.serialize_seq(Some(local_size))?;
        let viewtype = ("viewtype", "NETLIST");

        seq.serialize_element("view")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&viewtype)?;

        if !self.interface.is_empty() {
            seq.serialize_element(&self.interface)?;
        }

        if !self.contents.is_empty() {
            seq.serialize_element(&self.contents)?;
        }

        if !self.properties.is_empty() {
            for prop in self.properties.iter() {
                seq.serialize_element(&prop)?;
            }
        }

        seq.end()
    }
}

impl Serialize for Cell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(4))?;
        let celltype = ("celltype", "GENERIC");

        seq.serialize_element("cell")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&celltype)?;
        if !self.views.is_empty() {
            for view in self.views.iter() {
                seq.serialize_element(&view)?;
            }
        }
        seq.end()
    }
}

impl Serialize for Library {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        let fixed1 = ("edifLevel", "0");
        //Little hack to make serialize to insert braces around numberDefinition.
        let fixed2 = ("technology", ("numberDefinition",));

        seq.serialize_element("Library")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&fixed1)?;
        seq.serialize_element(&fixed2)?;
        if !self.elements.is_empty() {
            for elem in self.elements.iter() {
                seq.serialize_element(&elem)?;
            }
        }
        seq.end()
    }
}

impl Serialize for Design {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;

        seq.serialize_element("design")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.cellref)?;
        if !self.properties.is_empty() {
            for prop in self.properties.iter() {
                seq.serialize_element(&prop)?;
            }
        }
        seq.end()
    }
}

impl Serialize for EdifElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EdifElement::Library(lib) => lib.serialize(serializer),
            EdifElement::Design(des) => des.serialize(serializer),
        }
    }
}

impl Serialize for Edif {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 1) Need to count the number of fields we will have on the
        //    main branch.
        //
        // 3 is the number of fields in the struct.
        let mut seq = serializer.serialize_seq(Some(6))?;
        let version = ("edifversion", "2", "0", "0");
        let level = ("edifLevel", 0);
        let kword = ("keywordmap", ("keywordlevel", 0));

        seq.serialize_element("edif")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&version)?;
        seq.serialize_element(&level)?;
        seq.serialize_element(&kword)?;

        if !self.elements.is_empty() {
            for elem in self.elements.iter() {
                seq.serialize_element(&elem)?;
            }
        }

        seq.end()
    }
}
