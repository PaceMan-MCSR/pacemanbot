#[derive(PartialEq, Debug, Clone)]
pub enum Split {
    AdventuringTime,
    Beaconator,
    HDWGH,
}

impl Split {
    pub fn from_str(split: &str) -> Option<Split> {
        match split {
            "AT" => Some(Split::AdventuringTime),
            "B" => Some(Split::Beaconator),
            "H" => Some(Split::HDWGH),
            _ => None,
        }
    }

    pub fn from_command_param(param: &str) -> Option<Split> {
        match param {
            "adventuring_time" => Some(Split::AdventuringTime),
            "beaconator" => Some(Split::Beaconator),
            "hdwgh" => Some(Split::HDWGH),
            _ => None,
        }
    }

    pub fn desc(&self) -> String {
        match self {
            Split::AdventuringTime => "Adventuring Time",
            Split::HDWGH => "How Did We Get Here?",
            Split::Beaconator => "Beaconator",
        }
        .to_string()
    }

    pub fn get_emoji(&self) -> Option<String> {
        // TODO: Add emojis.
        todo!()
        // Some(match self {}.to_string())
    }

    pub fn to_str(&self) -> String {
        match self {
            Split::AdventuringTime => "AT",
            Split::Beaconator => "B",
            Split::HDWGH => "H",
        }
        .to_string()
    }
}
