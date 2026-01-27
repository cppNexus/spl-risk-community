use crate::config::Config;
use crate::model::{RiskReport, TokenData};

pub trait RiskRule: Send + Sync {
    fn evaluate(&self, token: &TokenData, config: &Config, report: &mut RiskReport);
    fn name(&self) -> &str;
}
