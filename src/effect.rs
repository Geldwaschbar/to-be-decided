use crate::component::{
    market::Market,
    news::{Event, News},
    parlament::{Parlament, Party},
};
use macroquad::prelude::*;
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
    Transfer,
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
                match resolution {
                    ParlamentResolution::Approval => {
                        let party = parlament
                            .parties
                            .get_mut(*party)
                            .expect("expect party exists");
                        Self::resolve_modifier(modifier, *value, &mut party.approval);
                        party.approval = clamp(party.approval, 0.0, 1.0);
                    }
                    ParlamentResolution::Popularity => {
                        let party = parlament
                            .parties
                            .get_mut(*party)
                            .expect("expect party exists");
                        Self::resolve_modifier(modifier, *value, &mut party.popularity)
                    }
                    ParlamentResolution::Transfer => {
                        // We only transfer popularity if we have more than one party
                        if parlament.parties.len() > 1 {
                            let target_num = {
                                let mut target_num = rand::gen_range(0, parlament.parties.len());
                                while target_num == *party {
                                    target_num = rand::gen_range(0, parlament.parties.len());
                                }
                                target_num
                            };

                            let target = parlament
                                .parties
                                .get_mut(target_num)
                                .expect("expected party exists");
                            let old_value = target.popularity;
                            Self::resolve_modifier(modifier, -*value, &mut target.popularity);
                            target.popularity = clamp(target.popularity, 0.0, 1.0);
                            let diff = old_value - target.popularity;
                            let party = parlament
                                .parties
                                .get_mut(*party)
                                .expect("expect party exists");
                            Self::resolve_modifier(modifier, diff, &mut party.popularity);
                        }
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
