// include!("gen/mod.rs");
mod gen;
pub use gen::*;

pub mod prelude {
    use std::{num::ParseIntError, str::FromStr};

    use crate::cosmos::base::v1beta1::DecCoin;

    use super::*;
    use ethnum::U256;

    use cosmos::base::v1beta1::Coin;

    pub trait CoinExt<'a> {
        fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a str>;
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

    impl<'a> CoinExt<'a> for Option<Coin> {
        fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a str> {
            self.as_ref()
                .filter(|c| c.denom == denom.into())
                .map(|c| c.denom.as_str())
        }
    }

    impl<'a> CoinExt<'a> for Vec<Coin> {
        fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a str> {
            let denom = denom.into();
            for coin in self.iter() {
                if coin.denom == denom {
                    return Some(coin.amount.as_str());
                }
            }
            None
        }
    }

    impl<'a> CoinExt<'a> for Vec<DecCoin> {
        fn get_denom(&'a self, denom: impl Into<String>) -> Option<&'a str> {
            let denom = denom.into();
            for coin in self.iter() {
                if coin.denom == denom {
                    return Some(coin.amount.as_str());
                }
            }
            None
        }
    }

    pub trait AmountExt {
        fn to_u256(&self) -> Result<U256, ParseIntError>;
    }

    impl AmountExt for String {
        fn to_u256(&self) -> Result<U256, ParseIntError> {
            U256::from_str(&self)
        }
    }

    impl<'a> AmountExt for &'a str {
        fn to_u256(&self) -> Result<U256, ParseIntError> {
            U256::from_str(&self)
        }
    }

    impl AmountExt for Coin {
        fn to_u256(&self) -> Result<U256, ParseIntError> {
            self.amount.to_u256()
        }
    }
}

#[cfg(test)]
mod test {
    use super::cosmos::base::v1beta1::Coin;
    use super::prelude::*;
    use ethnum::U256;
    use std::str::FromStr;

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

        assert_eq!(
            "12345".to_string().to_u256().unwrap(),
            U256::from_str("12345").unwrap()
        );
        assert_eq!("12345".to_u256().unwrap(), U256::from_str("12345").unwrap());
    }
}
