use crate::ws::response::EventId;

#[derive(PartialEq, Debug, Clone)]
pub enum Split {
    FirstStructure,
    SecondStructure,
    Blind,
    EyeSpy,
    EndEnter,
}

pub enum Structure {
    Bastion,
    Fortress,
}

impl Split {
    pub fn from_str(split: &str) -> Option<Split> {
        match split {
            "FS" => Some(Split::FirstStructure),
            "SS" => Some(Split::SecondStructure),
            "B" => Some(Split::Blind),
            "E" => Some(Split::EyeSpy),
            "EE" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_event_id(event_id: &EventId) -> Option<Split> {
        match event_id {
            EventId::RsgFirstPortal => Some(Split::Blind),
            EventId::RsgEnterStronghold => Some(Split::EyeSpy),
            EventId::RsgEnterEnd => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_command_param(param: &str) -> Option<Split> {
        match param {
            "first_structure" => Some(Split::FirstStructure),
            "second_structure" => Some(Split::SecondStructure),
            "blind" => Some(Split::Blind),
            "eye_spy" => Some(Split::EyeSpy),
            "end_enter" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn desc(&self, structure: &Option<Structure>) -> Option<String> {
        Some(
            match self {
                Split::FirstStructure => match structure {
                    Some(structure) => match structure {
                        Structure::Bastion => "Enter Bastion",
                        Structure::Fortress => "Enter Fortress",
                    },
                    None => return None,
                },
                Split::SecondStructure => match structure {
                    Some(structure) => match structure {
                        Structure::Bastion => "Enter Bastion",
                        Structure::Fortress => "Enter Fortress",
                    },
                    None => return None,
                },
                Split::Blind => "First Portal",
                Split::EyeSpy => "Enter Stronghold",
                Split::EndEnter => "Enter End",
            }
            .to_string(),
        )
    }

    pub fn alt_desc(&self) -> String {
        match self {
            Split::FirstStructure => "Structure 1",
            Split::SecondStructure => "Structure 2",
            Split::Blind => "Blind",
            Split::EyeSpy => "Eye Spy",
            Split::EndEnter => "End Enter",
        }
        .to_string()
    }

    pub fn to_str(&self) -> String {
        match self {
            Split::FirstStructure => "FS",
            Split::SecondStructure => "SS",
            Split::Blind => "B",
            Split::EyeSpy => "E",
            Split::EndEnter => "EE",
        }
        .to_string()
    }
}
