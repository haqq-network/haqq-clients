// include!("gen/mod.rs");
mod gen;
use std::{num::ParseIntError, str::FromStr};

use ethnum::U256;
pub use gen::*;

pub type ClientChannel = tonic::transport::Channel;
pub use tonic;

use cosmos::base::v1beta1::Coin;

pub trait CoinOptionExt<'a> {
    fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a Coin>;
}

// impl<'a, I> CoinOptionExt<'a> for I
// where
//     I: AsRef<[Coin]>,
// {
//     fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a Coin> {
//         let denom = denom.into();
//         self.as_ref().iter().find(move |c| c.denom == denom)
//     }
// }

impl<'a> CoinOptionExt<'a> for Option<Coin> {
    fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a Coin> {
        self.as_ref().filter(|c| c.denom == denom.into())
    }
}

impl<'a> CoinOptionExt<'a> for Vec<Coin> {
    fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a Coin> {
        let denom = denom.into();
        for coin in self.iter() {
            if coin.denom == denom {
                return Some(&coin);
            }
        }
        None
    }
}

pub trait CoinExt {
    fn to_u256(&self) -> Result<U256, ParseIntError>;
}

impl CoinExt for Coin {
    fn to_u256(&self) -> Result<U256, ParseIntError> {
        U256::from_str(&self.amount)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ethnum::U256;

    #[test]
    fn coin_option_ext() {
        let coin = Coin {
            denom: "aISLM".to_string(),
            amount: "0".to_string(),
        };

        assert!(vec![coin.clone()].get_denom("aISLM").is_some());
        assert!(Some(coin).get_denom("aISLM").is_some());
    }

    #[test]
    fn coin_ext() {
        let mut coin = Coin {
            denom: "ISLM".to_string(),
            amount: "1.2345".to_string(),
        };

        assert!(coin.to_u256().is_err());

        coin.amount = "12345".to_string();
        assert_eq!(coin.to_u256().unwrap(), U256::from_str("12345").unwrap());
    }
}
