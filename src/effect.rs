use serde::{Deserialize, Serialize};

use crate::{law::Parlament, market::Market};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(tag = "type", content = "party")]
pub enum Target {
    #[default]
    MarketMoney,
    MarketPrice,
    MarketStocks,
    PartyApproval(usize),
    PartyPopularity(usize),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Modifier {
    // v = n
    Setter,
    // v += n
    #[default]
    Constant,
    // v *= n
    Multiplier,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Effect {
    target: Target,
    modifier: Modifier,
    value: f32,
}

impl Effect {
    pub fn resolve_target(&self, market: &mut Market, parlament: &mut Parlament) {
        match &self.target {
            Target::MarketMoney => self.resolve_modifier(&mut market.money),
            Target::MarketPrice => self.resolve_modifier(&mut market.price),
            Target::MarketStocks => self.resolve_modifier(&mut market.stocks),
            Target::PartyApproval(number) => self.resolve_modifier(
                &mut parlament
                    .parties
                    .get_mut(*number)
                    .expect("expect party exists")
                    .approval,
            ),
            Target::PartyPopularity(number) => self.resolve_modifier(
                &mut parlament
                    .parties
                    .get_mut(*number)
                    .expect("expect party exists")
                    .popularity,
            ),
        };
    }

    pub fn resolve_modifier(&self, destination: &mut f32) {
        match &self.modifier {
            Modifier::Setter => *destination = self.value,
            Modifier::Constant => *destination += self.value,
            Modifier::Multiplier => *destination *= self.value,
        };
    }
}
