use crate::component::{
    market::Market,
    news::{Event, News},
    parlament::{Parlament, Party},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum MarketResolution {
    #[default]
    Money,
    Price,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum ParlamentResolution {
    #[default]
    Approval,
    Popularity,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum ModifierType {
    // v = n
    Setter,
    // v += n
    #[default]
    Constant,
    // v *= n
    Multiplier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Effect {
    CreateEvent {
        source: String,
        description: String,
    },
    MarketEffect {
        resolution: MarketResolution,
        modifier: ModifierType,
        value: f32,
    },
    ParlamentEffect {
        resolution: ParlamentResolution,
        modifier: ModifierType,
        value: f32,
        party: usize,
    },
}

impl Effect {
    pub fn resolve(&self, market: &mut Market, parlament: &mut Parlament, news: &mut News) {
        match &self {
            Self::CreateEvent {
                source,
                description,
            } => news
                .current
                .push_front(Event::new(source.to_string(), description.to_string())),
            Self::MarketEffect {
                resolution,
                modifier,
                value,
            } => {
                match resolution {
                    MarketResolution::Money => {
                        Self::resolve_modifier(modifier, *value, &mut market.money)
                    }
                    MarketResolution::Price => {
                        Self::resolve_modifier(modifier, *value, &mut market.price)
                    }
                };
            }
            Self::ParlamentEffect {
                resolution,
                modifier,
                value,
                party,
            } => {
                let mut party: &mut Party = parlament
                    .parties
                    .get_mut(*party)
                    .expect("expect party exists");
                match resolution {
                    ParlamentResolution::Approval => {
                        Self::resolve_modifier(modifier, *value, &mut party.approval)
                    }
                    ParlamentResolution::Popularity => {
                        Self::resolve_modifier(modifier, *value, &mut party.popularity)
                    }
                };
            }
        };
    }

    fn resolve_modifier(modifier: &ModifierType, value: f32, destination: &mut f32) {
        match &modifier {
            ModifierType::Setter => *destination = value,
            ModifierType::Constant => *destination += value,
            ModifierType::Multiplier => *destination *= value,
        };
    }
}
