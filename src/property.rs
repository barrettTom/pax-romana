use xml::reader::XmlEvent;

use crate::xmlelements::XMLElements;

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub entity: Option<String>,
    pub rotation: f32,
    pub keyframe: Option<usize>,
    pub delay: Option<usize>,
    pub spawn: Option<String>,
    pub visible: Option<bool>,
}

impl Property {
    pub fn new(property_elements: Vec<XmlEvent>) -> Property {
        let entity = match XMLElements::get_attribute_value(&property_elements, "entity") {
            Ok(entity) => entity.parse().ok(),
            Err(_) => None,
        };
        let keyframe = match XMLElements::get_attribute_value(&property_elements, "keyframe") {
            Ok(keyframe) => keyframe.parse().ok(),
            Err(_) => None,
        };
        let delay = match XMLElements::get_attribute_value(&property_elements, "delay") {
            Ok(delay) => delay.parse().ok(),
            Err(_) => None,
        };
        let spawn = XMLElements::get_attribute_value(&property_elements, "spawn").ok();
        let visible = match XMLElements::get_attribute_value(&property_elements, "visible") {
            Ok(visible) => visible.parse().ok(),
            Err(_) => None,
        };

        Property {
            rotation: 0.0,
            entity,
            keyframe,
            delay,
            spawn,
            visible,
        }
    }
}

impl Default for Property {
    fn default() -> Property {
        Property {
            rotation: 0.0,
            entity: None,
            keyframe: None,
            delay: None,
            spawn: None,
            visible: None,
        }
    }
}
