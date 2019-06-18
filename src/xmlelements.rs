use ggez::filesystem::File;
use std::io::BufReader;
use xml::reader::{
    EventReader,
    XmlEvent::{self, StartElement},
};

pub struct XMLElements {
    pub events: Vec<XmlEvent>,
}

impl XMLElements {
    pub fn new(file: File) -> XMLElements {
        XMLElements {
            events: EventReader::new(BufReader::new(file))
                .into_iter()
                .map(Result::unwrap)
                .collect(),
        }
    }

    pub fn get_element(&self, element_name: &str) -> XmlEvent {
        self.events
            .clone()
            .into_iter()
            .find(|e| {
                if let StartElement { name, .. } = e {
                    name.local_name == element_name
                } else {
                    false
                }
            })
            .unwrap()
    }

    pub fn get_element_attribute(
        &self,
        element_name: &str,
        attribute_name: &str,
    ) -> Result<usize, ()> {
        let element = self.get_element(element_name);
        if let StartElement { attributes, .. } = element {
            Ok(attributes
                .iter()
                .find(|a| a.name.local_name == attribute_name)
                .unwrap()
                .value
                .parse()
                .unwrap())
        } else {
            Err(())
        }
    }
}
