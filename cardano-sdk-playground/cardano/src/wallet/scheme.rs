//! wallet scheme interfaces. provide common interfaces to manage wallets
//! generate addresses and sign transactions.
//!
use alloc::vec::Vec;

use address::ExtendedAddr;
use coin::Coin;
use config::{NetworkMagic, ProtocolMagic};
use tx::{TxId, TxInWitness};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
pub enum SelectionPolicy {
    /// select the first inputs that matches, no optimization
    FirstMatchFirst,

    /// Order the given inputs from the largest input and pick the largest ones first
    LargestFirst,

    /// select only the inputs that are below the targeted output
    ///
    /// the value in this setting represents the accepted dust threshold
    /// to lose or ignore in fees.
    Blackjack(Coin),
}
impl Default for SelectionPolicy {
    fn default() -> Self {
        SelectionPolicy::FirstMatchFirst
    }
}

/// main wallet scheme, provides all the details to manage a wallet:
/// from managing wallet [`Account`](./trait.Account.html)s and
/// signing transactions.
///
pub trait Wallet {
    /// associated `Account` type, must implement the [`Account`](./trait.Account.html)
    /// trait.
    type Account: Account;

    /// the associated type for the stored accounts. Some wallet may
    /// provide different model to handle accounts.
    ///
    type Accounts;

    /// addressing model associated to this wallet scheme.
    ///
    /// provides a description about how to derive a public key
    /// from a wallet point of view.
    type Addressing: Clone;

    /// create an account with the associated alias.
    ///
    /// The alias may not be used in some wallets which does not support
    /// accounts such as the daedalus wallet.
    ///
    fn create_account(&mut self, alias: &str, id: u32) -> Self::Account;

    /// list all the accounts known of this wallet
    fn list_accounts<'a>(&'a self) -> &'a Self::Accounts;
    fn sign_tx<I>(
        &self,
        protocol_magic: ProtocolMagic,
        txid: &TxId,
        addresses: I,
    ) -> Vec<TxInWitness>
    where
        I: Iterator<Item = Self::Addressing>;
}

/// account level scheme, provides all the details to manage an account:
/// i.e. generate new addresses associated to this account.
pub trait Account {
    /// addressing model associated to this account scheme.
    ///
    /// provides a description about how to derive a public key
    /// from a wallet point of view.
    type Addressing;

    fn generate_addresses<'a, I>(
        &'a self,
        addresses: I,
        network_magic: NetworkMagic,
    ) -> Vec<ExtendedAddr>
    where
        I: Iterator<Item = &'a Self::Addressing>;
}
