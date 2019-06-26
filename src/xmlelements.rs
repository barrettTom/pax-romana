use ggez::filesystem::File;
use std::io::BufReader;
use xml::reader::{
    EventReader,
    XmlEvent::{self, EndElement, StartElement},
};

#[derive(Debug)]
pub struct Property {
    entity: String,
    keyframe: usize,
    delay: usize,
}

impl Property {
    pub fn new(property_elements: Vec<XmlEvent>) -> Property {
        let entity = XMLElements::get_attribute_value(&property_elements, "entity")
            .unwrap()
            .parse()
            .unwrap();
        let keyframe = XMLElements::get_attribute_value(&property_elements, "keyframe")
            .unwrap()
            .parse()
            .unwrap();
        let delay = XMLElements::get_attribute_value(&property_elements, "delay")
            .unwrap()
            .parse()
            .unwrap();

        Property {
            entity,
            keyframe,
            delay,
        }
    }
}

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

    pub fn get_children(&self, element: &XmlEvent, children_name: &str) -> Vec<XmlEvent> {
        let start_index = self.events.iter().position(|e| e == element).unwrap();

        let element_name = if let StartElement { name, .. } = element {
            &name.local_name
        } else {
            ""
        };

        let end_index = self.events[start_index..]
            .iter()
            .position(|e| {
                if let EndElement { name } = e {
                    element_name == name.local_name
                } else {
                    false
                }
            })
            .unwrap()
            + start_index;

        self.events[start_index..end_index]
            .iter()
            .cloned()
            .filter(|e| {
                if let StartElement { name, .. } = e {
                    name.local_name == children_name
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn get_elements(&self, element_name: &str) -> Vec<XmlEvent> {
        self.events
            .clone()
            .into_iter()
            .filter(|e| {
                if let StartElement { name, .. } = e {
                    name.local_name == element_name
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn get_element_attribute(
        &self,
        element_name: &str,
        attribute_name: &str,
    ) -> Result<String, ()> {
        let element = self.get_element(element_name);
        XMLElements::get_attribute(&element, attribute_name)
    }

    pub fn get_attribute(element: &XmlEvent, attribute_name: &str) -> Result<String, ()> {
        if let StartElement { attributes, .. } = element {
            Ok(attributes
                .iter()
                .find(|a| a.name.local_name == attribute_name)
                .unwrap()
                .value
                .clone())
        } else {
            Err(())
        }
    }

    pub fn get_attribute_value(elements: &[XmlEvent], attribute_name: &str) -> Result<String, ()> {
        let element = elements
            .iter()
            .find(|e| {
                if let StartElement { attributes, .. } = e {
                    attributes.iter().any(|a| a.value == attribute_name)
                } else {
                    false
                }
            })
            .unwrap();

        if let StartElement { attributes, .. } = element {
            Ok(attributes
                .iter()
                .find(|a| a.name.local_name == "value")
                .unwrap()
                .value
                .clone())
        } else {
            Err(())
        }
    }
}
