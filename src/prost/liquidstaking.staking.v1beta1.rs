/// StakeAuthorization defines authorization for delegate/undelegate/redelegate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeAuthorization {
    /// max_tokens specifies the maximum amount of tokens can be delegate to a validator. If it is
    /// empty, there is no spend limit and any amount of coins can be delegated.
    #[prost(message, optional, tag = "1")]
    pub max_tokens: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    /// authorization_type defines one of AuthorizationType.
    #[prost(enumeration = "AuthorizationType", tag = "4")]
    pub authorization_type: i32,
    /// validators is the oneof that represents either allow_list or deny_list
    #[prost(oneof = "stake_authorization::Validators", tags = "2, 3")]
    pub validators: ::core::option::Option<stake_authorization::Validators>,
}
/// Nested message and enum types in `StakeAuthorization`.
pub mod stake_authorization {
    /// Validators defines list of validator addresses.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValidatorsVec {
        #[prost(string, repeated, tag = "1")]
        pub address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// validators is the oneof that represents either allow_list or deny_list
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Validators {
        /// allow_list specifies list of validator addresses to whom grantee can delegate tokens on behalf of granter's
        /// account.
        #[prost(message, tag = "2")]
        AllowList(ValidatorsVec),
        /// deny_list specifies list of validator addresses to whom grantee can not delegate tokens.
        #[prost(message, tag = "3")]
        DenyList(ValidatorsVec),
    }
}
/// AuthorizationType defines the type of staking module authorization type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorizationType {
    /// AUTHORIZATION_TYPE_UNSPECIFIED specifies an unknown authorization type
    Unspecified = 0,
    /// AUTHORIZATION_TYPE_DELEGATE defines an authorization type for Msg/Delegate
    Delegate = 1,
    /// AUTHORIZATION_TYPE_UNDELEGATE defines an authorization type for Msg/Undelegate
    Undelegate = 2,
    /// AUTHORIZATION_TYPE_REDELEGATE defines an authorization type for Msg/BeginRedelegate
    Redelegate = 3,
}
impl AuthorizationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthorizationType::Unspecified => "AUTHORIZATION_TYPE_UNSPECIFIED",
            AuthorizationType::Delegate => "AUTHORIZATION_TYPE_DELEGATE",
            AuthorizationType::Undelegate => "AUTHORIZATION_TYPE_UNDELEGATE",
            AuthorizationType::Redelegate => "AUTHORIZATION_TYPE_REDELEGATE",
        }
    }
}
/// HistoricalInfo contains header and validator information for a given block.
/// It is stored as part of staking module's state, which persists the `n` most
/// recent HistoricalInfo
/// (`n` is set by the staking module's `historical_entries` parameter).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalInfo {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<::tendermint_proto::types::Header>,
    #[prost(message, repeated, tag = "2")]
    pub valset: ::prost::alloc::vec::Vec<Validator>,
}
/// CommissionRates defines the initial commission rates to be used for creating
/// a validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommissionRates {
    /// rate is the commission rate charged to delegators, as a fraction.
    #[prost(string, tag = "1")]
    pub rate: ::prost::alloc::string::String,
    /// max_rate defines the maximum commission rate which validator can ever charge, as a fraction.
    #[prost(string, tag = "2")]
    pub max_rate: ::prost::alloc::string::String,
    /// max_change_rate defines the maximum daily increase of the validator commission, as a fraction.
    #[prost(string, tag = "3")]
    pub max_change_rate: ::prost::alloc::string::String,
}
/// Commission defines commission parameters for a given validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commission {
    /// commission_rates defines the initial commission rates to be used for creating a validator.
    #[prost(message, optional, tag = "1")]
    pub commission_rates: ::core::option::Option<CommissionRates>,
    /// update_time is the last time the commission rate was changed.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Description defines a validator description.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Description {
    /// moniker defines a human-readable name for the validator.
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    /// website defines an optional website link.
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
    /// security_contact defines an optional email for security contact.
    #[prost(string, tag = "4")]
    pub security_contact: ::prost::alloc::string::String,
    /// details define other optional details.
    #[prost(string, tag = "5")]
    pub details: ::prost::alloc::string::String,
}
/// Validator defines a validator, together with the total amount of the
/// Validator's bond shares and their exchange rate to coins. Slashing results in
/// a decrease in the exchange rate, allowing correct calculation of future
/// undelegations without iterating over delegators. When coins are delegated to
/// this validator, the validator is credited with a delegation whose number of
/// bond shares is based on the amount of coins delegated divided by the current
/// exchange rate. Voting power can be calculated as total bonded shares
/// multiplied by exchange rate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    /// operator_address defines the address of the validator's operator; bech encoded in JSON.
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    /// consensus_pubkey is the consensus public key of the validator, as a Protobuf Any.
    #[prost(message, optional, tag = "2")]
    pub consensus_pubkey: ::core::option::Option<::prost_types::Any>,
    /// jailed defined whether the validator has been jailed from bonded status or not.
    #[prost(bool, tag = "3")]
    pub jailed: bool,
    /// status is the validator status (bonded/unbonding/unbonded).
    #[prost(enumeration = "BondStatus", tag = "4")]
    pub status: i32,
    /// tokens define the delegated tokens (incl. self-delegation).
    #[prost(string, tag = "5")]
    pub tokens: ::prost::alloc::string::String,
    /// delegator_shares defines total shares issued to a validator's delegators.
    #[prost(string, tag = "6")]
    pub delegator_shares: ::prost::alloc::string::String,
    /// description defines the description terms for the validator.
    #[prost(message, optional, tag = "7")]
    pub description: ::core::option::Option<Description>,
    /// unbonding_height defines, if unbonding, the height at which this validator has begun unbonding.
    #[prost(int64, tag = "8")]
    pub unbonding_height: i64,
    /// unbonding_time defines, if unbonding, the min time for the validator to complete unbonding.
    #[prost(message, optional, tag = "9")]
    pub unbonding_time: ::core::option::Option<::prost_types::Timestamp>,
    /// commission defines the commission parameters.
    #[prost(message, optional, tag = "10")]
    pub commission: ::core::option::Option<Commission>,
    /// Number of shares marked_exempt_to_this_validator
    #[prost(string, tag = "11")]
    pub total_exempt_shares: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub total_tokenized_shares: ::prost::alloc::string::String,
}
/// ValAddresses defines a repeated set of validator addresses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValAddresses {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DVPair is struct that just has a delegator-validator pair with no other data.
/// It is intended to be used as a marshalable pointer. For example, a DVPair can
/// be used to construct the key to getting an UnbondingDelegation from state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvPair {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// DVPairs defines an array of DVPair objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvPairs {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<DvPair>,
}
/// DVVTriplet is struct that just has a delegator-validator-validator triplet
/// with no other data. It is intended to be used as a marshalable pointer. For
/// example, a DVVTriplet can be used to construct the key to getting a
/// Redelegation from state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvvTriplet {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
}
/// DVVTriplets defines an array of DVVTriplet objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DvvTriplets {
    #[prost(message, repeated, tag = "1")]
    pub triplets: ::prost::alloc::vec::Vec<DvvTriplet>,
}
/// Delegation represents the bond with tokens held by an account. It is
/// owned by one delegator, and is associated with the voting power of one
/// validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the bech32-encoded address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// shares define the delegation shares received.
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
    /// has this delegation been marked as exempt.
    #[prost(bool, tag = "4")]
    pub exempt: bool,
}
/// UnbondingDelegation stores all of a single delegator's unbonding bonds
/// for a single validator in an time-ordered list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingDelegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the bech32-encoded address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// entries are the unbonding delegation entries.
    ///
    /// unbonding delegation entries
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<UnbondingDelegationEntry>,
}
/// UnbondingDelegationEntry defines an unbonding object with relevant metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingDelegationEntry {
    /// creation_height is the height which the unbonding took place.
    #[prost(int64, tag = "1")]
    pub creation_height: i64,
    /// completion_time is the unix time for unbonding completion.
    #[prost(message, optional, tag = "2")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// initial_balance defines the tokens initially scheduled to receive at completion.
    #[prost(string, tag = "3")]
    pub initial_balance: ::prost::alloc::string::String,
    /// balance defines the tokens to receive at completion.
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
}
/// RedelegationEntry defines a redelegation object with relevant metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationEntry {
    /// creation_height  defines the height which the redelegation took place.
    #[prost(int64, tag = "1")]
    pub creation_height: i64,
    /// completion_time defines the unix time for redelegation completion.
    #[prost(message, optional, tag = "2")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// initial_balance defines the initial balance when redelegation started.
    #[prost(string, tag = "3")]
    pub initial_balance: ::prost::alloc::string::String,
    /// shares_dst is the amount of destination-validator shares created by redelegation.
    #[prost(string, tag = "4")]
    pub shares_dst: ::prost::alloc::string::String,
}
/// Redelegation contains the list of a particular delegator's redelegating bonds
/// from a particular source validator to a particular destination validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Redelegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_src_address is the validator redelegation source operator address.
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    /// validator_dst_address is the validator redelegation destination operator address.
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
    /// entries are the redelegation entries.
    ///
    /// redelegation entries
    #[prost(message, repeated, tag = "4")]
    pub entries: ::prost::alloc::vec::Vec<RedelegationEntry>,
}
/// Params defines the parameters for the staking module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// unbonding_time is the time duration of unbonding.
    #[prost(message, optional, tag = "1")]
    pub unbonding_time: ::core::option::Option<::prost_types::Duration>,
    /// max_validators is the maximum number of validators.
    #[prost(uint32, tag = "2")]
    pub max_validators: u32,
    /// max_entries is the max entries for either unbonding delegation or redelegation (per pair/trio).
    #[prost(uint32, tag = "3")]
    pub max_entries: u32,
    /// historical_entries is the number of historical entries to persist.
    #[prost(uint32, tag = "4")]
    pub historical_entries: u32,
    /// bond_denom defines the bondable coin denomination.
    #[prost(string, tag = "5")]
    pub bond_denom: ::prost::alloc::string::String,
    /// min_commission_rate is the chain-wide minimum commission rate that a validator can charge their delegators
    #[prost(string, tag = "6")]
    pub min_commission_rate: ::prost::alloc::string::String,
    /// exemption_factor is required for tokenize share and undelegation check for network safety
    #[prost(string, tag = "7")]
    pub exemption_factor: ::prost::alloc::string::String,
}
/// DelegationResponse is equivalent to Delegation except that it contains a
/// balance in addition to shares which is more suitable for client responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub delegation: ::core::option::Option<Delegation>,
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// RedelegationEntryResponse is equivalent to a RedelegationEntry except that it
/// contains a balance in addition to shares which is more suitable for client
/// responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationEntryResponse {
    #[prost(message, optional, tag = "1")]
    pub redelegation_entry: ::core::option::Option<RedelegationEntry>,
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
}
/// RedelegationResponse is equivalent to a Redelegation except that its entries
/// contain a balance in addition to shares which is more suitable for client
/// responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub redelegation: ::core::option::Option<Redelegation>,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<RedelegationEntryResponse>,
}
/// Pool is used for tracking bonded and not-bonded token supply of the bond
/// denomination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub not_bonded_tokens: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bonded_tokens: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenizeShareRecord {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// module account take the role of delegator
    #[prost(string, tag = "3")]
    pub module_account: ::prost::alloc::string::String,
    /// validator delegated to for tokenize share record creation
    #[prost(string, tag = "4")]
    pub validator: ::prost::alloc::string::String,
}
/// BondStatus is the status of a validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BondStatus {
    /// UNSPECIFIED defines an invalid validator status.
    Unspecified = 0,
    /// UNBONDED defines a validator that is not bonded.
    Unbonded = 1,
    /// UNBONDING defines a validator that is unbonding.
    Unbonding = 2,
    /// BONDED defines a validator that is bonded.
    Bonded = 3,
}
impl BondStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BondStatus::Unspecified => "BOND_STATUS_UNSPECIFIED",
            BondStatus::Unbonded => "BOND_STATUS_UNBONDED",
            BondStatus::Unbonding => "BOND_STATUS_UNBONDING",
            BondStatus::Bonded => "BOND_STATUS_BONDED",
        }
    }
}
/// GenesisState defines the staking module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of related to deposit.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// last_total_power tracks the total amounts of bonded tokens recorded during
    /// the previous end block.
    #[prost(bytes = "vec", tag = "2")]
    pub last_total_power: ::prost::alloc::vec::Vec<u8>,
    /// last_validator_powers is a special index that provides a historical list
    /// of the last-block's bonded validators.
    #[prost(message, repeated, tag = "3")]
    pub last_validator_powers: ::prost::alloc::vec::Vec<LastValidatorPower>,
    /// delegations defines the validator set at genesis.
    #[prost(message, repeated, tag = "4")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// delegations defines the delegations active at genesis.
    #[prost(message, repeated, tag = "5")]
    pub delegations: ::prost::alloc::vec::Vec<Delegation>,
    /// unbonding_delegations defines the unbonding delegations active at genesis.
    #[prost(message, repeated, tag = "6")]
    pub unbonding_delegations: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// redelegations defines the redelegations active at genesis.
    #[prost(message, repeated, tag = "7")]
    pub redelegations: ::prost::alloc::vec::Vec<Redelegation>,
    #[prost(bool, tag = "8")]
    pub exported: bool,
    /// store tokenize share records to provide reward to record owners
    #[prost(message, repeated, tag = "9")]
    pub tokenize_share_records: ::prost::alloc::vec::Vec<TokenizeShareRecord>,
    /// last tokenize share record id, used for next share record id calculation
    #[prost(uint64, tag = "10")]
    pub last_tokenize_share_record_id: u64,
}
/// LastValidatorPower required for validator set update logic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastValidatorPower {
    /// address is the address of the validator.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// power defines the power of the validator.
    #[prost(int64, tag = "2")]
    pub power: i64,
}
/// MsgCreateValidator defines a SDK message for creating a new validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateValidator {
    #[prost(message, optional, tag = "1")]
    pub description: ::core::option::Option<Description>,
    #[prost(message, optional, tag = "2")]
    pub commission: ::core::option::Option<CommissionRates>,
    #[prost(string, tag = "3")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub pubkey: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag = "6")]
    pub value: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgCreateValidatorResponse defines the Msg/CreateValidator response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateValidatorResponse {}
/// MsgEditValidator defines a SDK message for editing an existing validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEditValidator {
    #[prost(message, optional, tag = "1")]
    pub description: ::core::option::Option<Description>,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// We pass a reference to the new commission rate and min self delegation as
    /// it's not mandatory to update. If not updated, the deserialized rate will be
    /// zero with no way to distinguish if an update was intended.
    /// REF: #2373
    #[prost(string, tag = "3")]
    pub commission_rate: ::prost::alloc::string::String,
}
/// MsgEditValidatorResponse defines the Msg/EditValidator response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEditValidatorResponse {}
/// MsgDelegate defines a SDK message for performing a delegation of coins
/// from a delegator to a validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDelegateResponse defines the Msg/Delegate response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegateResponse {}
/// MsgBeginRedelegate defines a SDK message for performing a redelegation
/// of coins from a delegator and source validator to a destination validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginRedelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgBeginRedelegateResponse defines the Msg/BeginRedelegate response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginRedelegateResponse {
    #[prost(message, optional, tag = "1")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// MsgUndelegate defines a SDK message for performing an undelegation from a
/// delegate and a validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUndelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgUndelegateResponse defines the Msg/Undelegate response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUndelegateResponse {
    #[prost(message, optional, tag = "1")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// MsgCancelUnbondingDelegation defines the SDK message for performing a cancel unbonding delegation for delegator
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelUnbondingDelegation {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// amount is always less than or equal to unbonding delegation entry balance
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// creation_height is the height which the unbonding took place.
    #[prost(int64, tag = "4")]
    pub creation_height: i64,
}
/// MsgCancelUnbondingDelegationResponse
///
/// Since: cosmos-sdk 0.46
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelUnbondingDelegationResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTokenizeShares {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub tokenized_share_owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTokenizeSharesResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRedeemTokensforShares {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRedeemTokensforSharesResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferTokenizeShareRecord {
    #[prost(uint64, tag = "1")]
    pub tokenize_share_record_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferTokenizeShareRecordResponse {}
/// MsgExemptDelegation defines a SDK message for performing exemption of delegated coins
/// from a delegator to a validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExemptDelegation {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExemptDelegationResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the staking Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// CreateValidator defines a method for creating a new validator.
        pub async fn create_validator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateValidator>,
        ) -> Result<tonic::Response<super::MsgCreateValidatorResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/CreateValidator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// EditValidator defines a method for editing an existing validator.
        pub async fn edit_validator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgEditValidator>,
        ) -> Result<tonic::Response<super::MsgEditValidatorResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/EditValidator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delegate defines a method for performing a delegation of coins
        /// from a delegator to a validator.
        pub async fn delegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDelegate>,
        ) -> Result<tonic::Response<super::MsgDelegateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/Delegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// BeginRedelegate defines a method for performing a redelegation
        /// of coins from a delegator and source validator to a destination validator.
        pub async fn begin_redelegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBeginRedelegate>,
        ) -> Result<tonic::Response<super::MsgBeginRedelegateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/BeginRedelegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undelegate defines a method for performing an undelegation from a
        /// delegate and a validator.
        pub async fn undelegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUndelegate>,
        ) -> Result<tonic::Response<super::MsgUndelegateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/Undelegate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CancelUnbondingDelegation defines a method for performing canceling the unbonding delegation
        /// and delegate back to previous validator.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn cancel_unbonding_delegation(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelUnbondingDelegation>,
        ) -> Result<
            tonic::Response<super::MsgCancelUnbondingDelegationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/CancelUnbondingDelegation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TokenizeShares defines a method for tokenizing shares from a validator.
        pub async fn tokenize_shares(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTokenizeShares>,
        ) -> Result<tonic::Response<super::MsgTokenizeSharesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/TokenizeShares",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RedeemTokens defines a method for redeeming tokens from a validator for
        /// shares.
        pub async fn redeem_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRedeemTokensforShares>,
        ) -> Result<
            tonic::Response<super::MsgRedeemTokensforSharesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/RedeemTokens",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TransferTokenizeShareRecord defines a method to transfer ownership of
        /// TokenizeShareRecord
        pub async fn transfer_tokenize_share_record(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTransferTokenizeShareRecord>,
        ) -> Result<
            tonic::Response<super::MsgTransferTokenizeShareRecordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/TransferTokenizeShareRecord",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ExemptDelegation defines a method for performing exemption of
        /// delegated coins from a delegator to a validator.
        pub async fn exempt_delegation(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExemptDelegation>,
        ) -> Result<tonic::Response<super::MsgExemptDelegationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Msg/ExemptDelegation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// CreateValidator defines a method for creating a new validator.
        async fn create_validator(
            &self,
            request: tonic::Request<super::MsgCreateValidator>,
        ) -> Result<tonic::Response<super::MsgCreateValidatorResponse>, tonic::Status>;
        /// EditValidator defines a method for editing an existing validator.
        async fn edit_validator(
            &self,
            request: tonic::Request<super::MsgEditValidator>,
        ) -> Result<tonic::Response<super::MsgEditValidatorResponse>, tonic::Status>;
        /// Delegate defines a method for performing a delegation of coins
        /// from a delegator to a validator.
        async fn delegate(
            &self,
            request: tonic::Request<super::MsgDelegate>,
        ) -> Result<tonic::Response<super::MsgDelegateResponse>, tonic::Status>;
        /// BeginRedelegate defines a method for performing a redelegation
        /// of coins from a delegator and source validator to a destination validator.
        async fn begin_redelegate(
            &self,
            request: tonic::Request<super::MsgBeginRedelegate>,
        ) -> Result<tonic::Response<super::MsgBeginRedelegateResponse>, tonic::Status>;
        /// Undelegate defines a method for performing an undelegation from a
        /// delegate and a validator.
        async fn undelegate(
            &self,
            request: tonic::Request<super::MsgUndelegate>,
        ) -> Result<tonic::Response<super::MsgUndelegateResponse>, tonic::Status>;
        /// CancelUnbondingDelegation defines a method for performing canceling the unbonding delegation
        /// and delegate back to previous validator.
        ///
        /// Since: cosmos-sdk 0.46
        async fn cancel_unbonding_delegation(
            &self,
            request: tonic::Request<super::MsgCancelUnbondingDelegation>,
        ) -> Result<
            tonic::Response<super::MsgCancelUnbondingDelegationResponse>,
            tonic::Status,
        >;
        /// TokenizeShares defines a method for tokenizing shares from a validator.
        async fn tokenize_shares(
            &self,
            request: tonic::Request<super::MsgTokenizeShares>,
        ) -> Result<tonic::Response<super::MsgTokenizeSharesResponse>, tonic::Status>;
        /// RedeemTokens defines a method for redeeming tokens from a validator for
        /// shares.
        async fn redeem_tokens(
            &self,
            request: tonic::Request<super::MsgRedeemTokensforShares>,
        ) -> Result<
            tonic::Response<super::MsgRedeemTokensforSharesResponse>,
            tonic::Status,
        >;
        /// TransferTokenizeShareRecord defines a method to transfer ownership of
        /// TokenizeShareRecord
        async fn transfer_tokenize_share_record(
            &self,
            request: tonic::Request<super::MsgTransferTokenizeShareRecord>,
        ) -> Result<
            tonic::Response<super::MsgTransferTokenizeShareRecordResponse>,
            tonic::Status,
        >;
        /// ExemptDelegation defines a method for performing exemption of
        /// delegated coins from a delegator to a validator.
        async fn exempt_delegation(
            &self,
            request: tonic::Request<super::MsgExemptDelegation>,
        ) -> Result<tonic::Response<super::MsgExemptDelegationResponse>, tonic::Status>;
    }
    /// Msg defines the staking Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/liquidstaking.staking.v1beta1.Msg/CreateValidator" => {
                    #[allow(non_camel_case_types)]
                    struct CreateValidatorSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateValidator>
                    for CreateValidatorSvc<T> {
                        type Response = super::MsgCreateValidatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateValidator>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_validator(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateValidatorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/EditValidator" => {
                    #[allow(non_camel_case_types)]
                    struct EditValidatorSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgEditValidator>
                    for EditValidatorSvc<T> {
                        type Response = super::MsgEditValidatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgEditValidator>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_validator(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditValidatorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/Delegate" => {
                    #[allow(non_camel_case_types)]
                    struct DelegateSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDelegate>
                    for DelegateSvc<T> {
                        type Response = super::MsgDelegateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDelegate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delegate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/BeginRedelegate" => {
                    #[allow(non_camel_case_types)]
                    struct BeginRedelegateSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBeginRedelegate>
                    for BeginRedelegateSvc<T> {
                        type Response = super::MsgBeginRedelegateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBeginRedelegate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).begin_redelegate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BeginRedelegateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/Undelegate" => {
                    #[allow(non_camel_case_types)]
                    struct UndelegateSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUndelegate>
                    for UndelegateSvc<T> {
                        type Response = super::MsgUndelegateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUndelegate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).undelegate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UndelegateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/CancelUnbondingDelegation" => {
                    #[allow(non_camel_case_types)]
                    struct CancelUnbondingDelegationSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgCancelUnbondingDelegation>
                    for CancelUnbondingDelegationSvc<T> {
                        type Response = super::MsgCancelUnbondingDelegationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelUnbondingDelegation>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).cancel_unbonding_delegation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelUnbondingDelegationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/TokenizeShares" => {
                    #[allow(non_camel_case_types)]
                    struct TokenizeSharesSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgTokenizeShares>
                    for TokenizeSharesSvc<T> {
                        type Response = super::MsgTokenizeSharesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgTokenizeShares>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).tokenize_shares(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TokenizeSharesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/RedeemTokens" => {
                    #[allow(non_camel_case_types)]
                    struct RedeemTokensSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgRedeemTokensforShares>
                    for RedeemTokensSvc<T> {
                        type Response = super::MsgRedeemTokensforSharesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRedeemTokensforShares>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).redeem_tokens(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RedeemTokensSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/TransferTokenizeShareRecord" => {
                    #[allow(non_camel_case_types)]
                    struct TransferTokenizeShareRecordSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgTransferTokenizeShareRecord>
                    for TransferTokenizeShareRecordSvc<T> {
                        type Response = super::MsgTransferTokenizeShareRecordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgTransferTokenizeShareRecord,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transfer_tokenize_share_record(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferTokenizeShareRecordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Msg/ExemptDelegation" => {
                    #[allow(non_camel_case_types)]
                    struct ExemptDelegationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExemptDelegation>
                    for ExemptDelegationSvc<T> {
                        type Response = super::MsgExemptDelegationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExemptDelegation>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).exempt_delegation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExemptDelegationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "liquidstaking.staking.v1beta1.Msg";
    }
}
/// QueryValidatorsRequest is request type for Query/Validators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorsRequest {
    /// status enables to query for validators matching a given status.
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryValidatorsResponse is response type for the Query/Validators RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorsResponse {
    /// validators contains all the queried validators.
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryValidatorRequest is response type for the Query/Validator RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryValidatorResponse is response type for the Query/Validator RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorResponse {
    /// validator defines the the validator info.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
}
/// QueryValidatorDelegationsRequest is request type for the
/// Query/ValidatorDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorDelegationsRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryValidatorDelegationsResponse is response type for the
/// Query/ValidatorDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub delegation_responses: ::prost::alloc::vec::Vec<DelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryValidatorUnbondingDelegationsRequest is required type for the
/// Query/ValidatorUnbondingDelegations RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorUnbondingDelegationsRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryValidatorUnbondingDelegationsResponse is response type for the
/// Query/ValidatorUnbondingDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorUnbondingDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub unbonding_responses: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryDelegationRequest is request type for the Query/Delegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegationResponse is response type for the Query/Delegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationResponse {
    /// delegation_responses defines the delegation info of a delegation.
    #[prost(message, optional, tag = "1")]
    pub delegation_response: ::core::option::Option<DelegationResponse>,
}
/// QueryUnbondingDelegationRequest is request type for the
/// Query/UnbondingDelegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnbondingDelegationRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegationResponse is response type for the Query/UnbondingDelegation
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnbondingDelegationResponse {
    /// unbond defines the unbonding information of a delegation.
    #[prost(message, optional, tag = "1")]
    pub unbond: ::core::option::Option<UnbondingDelegation>,
}
/// QueryDelegatorDelegationsRequest is request type for the
/// Query/DelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorDelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryDelegatorDelegationsResponse is response type for the
/// Query/DelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorDelegationsResponse {
    /// delegation_responses defines all the delegations' info of a delegator.
    #[prost(message, repeated, tag = "1")]
    pub delegation_responses: ::prost::alloc::vec::Vec<DelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryDelegatorUnbondingDelegationsRequest is request type for the
/// Query/DelegatorUnbondingDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorUnbondingDelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryUnbondingDelegatorDelegationsResponse is response type for the
/// Query/UnbondingDelegatorDelegations RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorUnbondingDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub unbonding_responses: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryRedelegationsRequest is request type for the Query/Redelegations RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// src_validator_addr defines the validator address to redelegate from.
    #[prost(string, tag = "2")]
    pub src_validator_addr: ::prost::alloc::string::String,
    /// dst_validator_addr defines the validator address to redelegate to.
    #[prost(string, tag = "3")]
    pub dst_validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryRedelegationsResponse is response type for the Query/Redelegations RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub redelegation_responses: ::prost::alloc::vec::Vec<RedelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryDelegatorValidatorsRequest is request type for the
/// Query/DelegatorValidators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryDelegatorValidatorsResponse is response type for the
/// Query/DelegatorValidators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsResponse {
    /// validators defines the the validators' info of a delegator.
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryDelegatorValidatorRequest is request type for the
/// Query/DelegatorValidator RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegatorValidatorResponse response type for the
/// Query/DelegatorValidator RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorResponse {
    /// validator defines the the validator info.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
}
/// QueryHistoricalInfoRequest is request type for the Query/HistoricalInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalInfoRequest {
    /// height defines at which height to query the historical info.
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// QueryHistoricalInfoResponse is response type for the Query/HistoricalInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalInfoResponse {
    /// hist defines the historical info at the given height.
    #[prost(message, optional, tag = "1")]
    pub hist: ::core::option::Option<HistoricalInfo>,
}
/// QueryPoolRequest is request type for the Query/Pool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {}
/// QueryPoolResponse is response type for the Query/Pool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    /// pool defines the pool info.
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<Pool>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizeShareRecordByIdRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizeShareRecordByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<TokenizeShareRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizeShareRecordByDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizeShareRecordByDenomResponse {
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<TokenizeShareRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizeShareRecordsOwnedRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizeShareRecordsOwnedResponse {
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<TokenizeShareRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTokenizeShareRecordsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTokenizeShareRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<TokenizeShareRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastTokenizeShareRecordIdRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastTokenizeShareRecordIdResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalTokenizeSharedAssetsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalTokenizeSharedAssetsResponse {
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Validators queries all validators that match the given status.
        pub async fn validators(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorsRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/Validators",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Validator queries validator info for given validator address.
        pub async fn validator(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/Validator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ValidatorDelegations queries delegate info for given validator.
        pub async fn validator_delegations(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorDelegationsRequest>,
        ) -> Result<
            tonic::Response<super::QueryValidatorDelegationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/ValidatorDelegations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ValidatorUnbondingDelegations queries unbonding delegations of a validator.
        pub async fn validator_unbonding_delegations(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryValidatorUnbondingDelegationsRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryValidatorUnbondingDelegationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/ValidatorUnbondingDelegations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delegation queries delegate info for given validator delegator pair.
        pub async fn delegation(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegationRequest>,
        ) -> Result<tonic::Response<super::QueryDelegationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/Delegation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UnbondingDelegation queries unbonding info for given validator delegator
        /// pair.
        pub async fn unbonding_delegation(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUnbondingDelegationRequest>,
        ) -> Result<
            tonic::Response<super::QueryUnbondingDelegationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/UnbondingDelegation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DelegatorDelegations queries all delegations of a given delegator address.
        pub async fn delegator_delegations(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorDelegationsRequest>,
        ) -> Result<
            tonic::Response<super::QueryDelegatorDelegationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/DelegatorDelegations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DelegatorUnbondingDelegations queries all unbonding delegations of a given
        /// delegator address.
        pub async fn delegator_unbonding_delegations(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryDelegatorUnbondingDelegationsRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryDelegatorUnbondingDelegationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/DelegatorUnbondingDelegations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Redelegations queries redelegations of given address.
        pub async fn redelegations(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRedelegationsRequest>,
        ) -> Result<tonic::Response<super::QueryRedelegationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/Redelegations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DelegatorValidators queries all validators info for given delegator
        /// address.
        pub async fn delegator_validators(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorValidatorsRequest>,
        ) -> Result<
            tonic::Response<super::QueryDelegatorValidatorsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/DelegatorValidators",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DelegatorValidator queries validator info for given delegator validator
        /// pair.
        pub async fn delegator_validator(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorValidatorRequest>,
        ) -> Result<
            tonic::Response<super::QueryDelegatorValidatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/DelegatorValidator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// HistoricalInfo queries the historical info for given height.
        pub async fn historical_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryHistoricalInfoRequest>,
        ) -> Result<tonic::Response<super::QueryHistoricalInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/HistoricalInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Pool queries the pool info.
        pub async fn pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/Pool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Parameters queries the staking parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query for individual tokenize share record information by share by id
        pub async fn tokenize_share_record_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTokenizeShareRecordByIdRequest>,
        ) -> Result<
            tonic::Response<super::QueryTokenizeShareRecordByIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/TokenizeShareRecordById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query for individual tokenize share record information by share denom
        pub async fn tokenize_share_record_by_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryTokenizeShareRecordByDenomRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryTokenizeShareRecordByDenomResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/TokenizeShareRecordByDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query tokenize share records by address
        pub async fn tokenize_share_records_owned(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryTokenizeShareRecordsOwnedRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryTokenizeShareRecordsOwnedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/TokenizeShareRecordsOwned",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query for all tokenize share records
        pub async fn all_tokenize_share_records(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllTokenizeShareRecordsRequest>,
        ) -> Result<
            tonic::Response<super::QueryAllTokenizeShareRecordsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/AllTokenizeShareRecords",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query for last tokenize share record id
        pub async fn last_tokenize_share_record_id(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryLastTokenizeShareRecordIdRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryLastTokenizeShareRecordIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/LastTokenizeShareRecordId",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query for total tokenized staked assets
        pub async fn total_tokenize_shared_assets(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryTotalTokenizeSharedAssetsRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryTotalTokenizeSharedAssetsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/liquidstaking.staking.v1beta1.Query/TotalTokenizeSharedAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Validators queries all validators that match the given status.
        async fn validators(
            &self,
            request: tonic::Request<super::QueryValidatorsRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorsResponse>, tonic::Status>;
        /// Validator queries validator info for given validator address.
        async fn validator(
            &self,
            request: tonic::Request<super::QueryValidatorRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorResponse>, tonic::Status>;
        /// ValidatorDelegations queries delegate info for given validator.
        async fn validator_delegations(
            &self,
            request: tonic::Request<super::QueryValidatorDelegationsRequest>,
        ) -> Result<
            tonic::Response<super::QueryValidatorDelegationsResponse>,
            tonic::Status,
        >;
        /// ValidatorUnbondingDelegations queries unbonding delegations of a validator.
        async fn validator_unbonding_delegations(
            &self,
            request: tonic::Request<super::QueryValidatorUnbondingDelegationsRequest>,
        ) -> Result<
            tonic::Response<super::QueryValidatorUnbondingDelegationsResponse>,
            tonic::Status,
        >;
        /// Delegation queries delegate info for given validator delegator pair.
        async fn delegation(
            &self,
            request: tonic::Request<super::QueryDelegationRequest>,
        ) -> Result<tonic::Response<super::QueryDelegationResponse>, tonic::Status>;
        /// UnbondingDelegation queries unbonding info for given validator delegator
        /// pair.
        async fn unbonding_delegation(
            &self,
            request: tonic::Request<super::QueryUnbondingDelegationRequest>,
        ) -> Result<
            tonic::Response<super::QueryUnbondingDelegationResponse>,
            tonic::Status,
        >;
        /// DelegatorDelegations queries all delegations of a given delegator address.
        async fn delegator_delegations(
            &self,
            request: tonic::Request<super::QueryDelegatorDelegationsRequest>,
        ) -> Result<
            tonic::Response<super::QueryDelegatorDelegationsResponse>,
            tonic::Status,
        >;
        /// DelegatorUnbondingDelegations queries all unbonding delegations of a given
        /// delegator address.
        async fn delegator_unbonding_delegations(
            &self,
            request: tonic::Request<super::QueryDelegatorUnbondingDelegationsRequest>,
        ) -> Result<
            tonic::Response<super::QueryDelegatorUnbondingDelegationsResponse>,
            tonic::Status,
        >;
        /// Redelegations queries redelegations of given address.
        async fn redelegations(
            &self,
            request: tonic::Request<super::QueryRedelegationsRequest>,
        ) -> Result<tonic::Response<super::QueryRedelegationsResponse>, tonic::Status>;
        /// DelegatorValidators queries all validators info for given delegator
        /// address.
        async fn delegator_validators(
            &self,
            request: tonic::Request<super::QueryDelegatorValidatorsRequest>,
        ) -> Result<
            tonic::Response<super::QueryDelegatorValidatorsResponse>,
            tonic::Status,
        >;
        /// DelegatorValidator queries validator info for given delegator validator
        /// pair.
        async fn delegator_validator(
            &self,
            request: tonic::Request<super::QueryDelegatorValidatorRequest>,
        ) -> Result<
            tonic::Response<super::QueryDelegatorValidatorResponse>,
            tonic::Status,
        >;
        /// HistoricalInfo queries the historical info for given height.
        async fn historical_info(
            &self,
            request: tonic::Request<super::QueryHistoricalInfoRequest>,
        ) -> Result<tonic::Response<super::QueryHistoricalInfoResponse>, tonic::Status>;
        /// Pool queries the pool info.
        async fn pool(
            &self,
            request: tonic::Request<super::QueryPoolRequest>,
        ) -> Result<tonic::Response<super::QueryPoolResponse>, tonic::Status>;
        /// Parameters queries the staking parameters.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /// Query for individual tokenize share record information by share by id
        async fn tokenize_share_record_by_id(
            &self,
            request: tonic::Request<super::QueryTokenizeShareRecordByIdRequest>,
        ) -> Result<
            tonic::Response<super::QueryTokenizeShareRecordByIdResponse>,
            tonic::Status,
        >;
        /// Query for individual tokenize share record information by share denom
        async fn tokenize_share_record_by_denom(
            &self,
            request: tonic::Request<super::QueryTokenizeShareRecordByDenomRequest>,
        ) -> Result<
            tonic::Response<super::QueryTokenizeShareRecordByDenomResponse>,
            tonic::Status,
        >;
        /// Query tokenize share records by address
        async fn tokenize_share_records_owned(
            &self,
            request: tonic::Request<super::QueryTokenizeShareRecordsOwnedRequest>,
        ) -> Result<
            tonic::Response<super::QueryTokenizeShareRecordsOwnedResponse>,
            tonic::Status,
        >;
        /// Query for all tokenize share records
        async fn all_tokenize_share_records(
            &self,
            request: tonic::Request<super::QueryAllTokenizeShareRecordsRequest>,
        ) -> Result<
            tonic::Response<super::QueryAllTokenizeShareRecordsResponse>,
            tonic::Status,
        >;
        /// Query for last tokenize share record id
        async fn last_tokenize_share_record_id(
            &self,
            request: tonic::Request<super::QueryLastTokenizeShareRecordIdRequest>,
        ) -> Result<
            tonic::Response<super::QueryLastTokenizeShareRecordIdResponse>,
            tonic::Status,
        >;
        /// Query for total tokenized staked assets
        async fn total_tokenize_shared_assets(
            &self,
            request: tonic::Request<super::QueryTotalTokenizeSharedAssetsRequest>,
        ) -> Result<
            tonic::Response<super::QueryTotalTokenizeSharedAssetsResponse>,
            tonic::Status,
        >;
    }
    /// Query defines the gRPC querier service.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/liquidstaking.staking.v1beta1.Query/Validators" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryValidatorsRequest>
                    for ValidatorsSvc<T> {
                        type Response = super::QueryValidatorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).validators(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidatorsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/Validator" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryValidatorRequest>
                    for ValidatorSvc<T> {
                        type Response = super::QueryValidatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).validator(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidatorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/ValidatorDelegations" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorDelegationsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryValidatorDelegationsRequest,
                    > for ValidatorDelegationsSvc<T> {
                        type Response = super::QueryValidatorDelegationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryValidatorDelegationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).validator_delegations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidatorDelegationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/ValidatorUnbondingDelegations" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorUnbondingDelegationsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryValidatorUnbondingDelegationsRequest,
                    > for ValidatorUnbondingDelegationsSvc<T> {
                        type Response = super::QueryValidatorUnbondingDelegationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryValidatorUnbondingDelegationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).validator_unbonding_delegations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidatorUnbondingDelegationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/Delegation" => {
                    #[allow(non_camel_case_types)]
                    struct DelegationSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryDelegationRequest>
                    for DelegationSvc<T> {
                        type Response = super::QueryDelegationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delegation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/UnbondingDelegation" => {
                    #[allow(non_camel_case_types)]
                    struct UnbondingDelegationSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryUnbondingDelegationRequest>
                    for UnbondingDelegationSvc<T> {
                        type Response = super::QueryUnbondingDelegationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryUnbondingDelegationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).unbonding_delegation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnbondingDelegationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/DelegatorDelegations" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorDelegationsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryDelegatorDelegationsRequest,
                    > for DelegatorDelegationsSvc<T> {
                        type Response = super::QueryDelegatorDelegationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryDelegatorDelegationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delegator_delegations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegatorDelegationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/DelegatorUnbondingDelegations" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorUnbondingDelegationsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryDelegatorUnbondingDelegationsRequest,
                    > for DelegatorUnbondingDelegationsSvc<T> {
                        type Response = super::QueryDelegatorUnbondingDelegationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryDelegatorUnbondingDelegationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delegator_unbonding_delegations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegatorUnbondingDelegationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/Redelegations" => {
                    #[allow(non_camel_case_types)]
                    struct RedelegationsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryRedelegationsRequest>
                    for RedelegationsSvc<T> {
                        type Response = super::QueryRedelegationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRedelegationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).redelegations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RedelegationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/DelegatorValidators" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorValidatorsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryDelegatorValidatorsRequest>
                    for DelegatorValidatorsSvc<T> {
                        type Response = super::QueryDelegatorValidatorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryDelegatorValidatorsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delegator_validators(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegatorValidatorsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/DelegatorValidator" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorValidatorSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryDelegatorValidatorRequest>
                    for DelegatorValidatorSvc<T> {
                        type Response = super::QueryDelegatorValidatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryDelegatorValidatorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delegator_validator(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegatorValidatorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/HistoricalInfo" => {
                    #[allow(non_camel_case_types)]
                    struct HistoricalInfoSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryHistoricalInfoRequest>
                    for HistoricalInfoSvc<T> {
                        type Response = super::QueryHistoricalInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryHistoricalInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).historical_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HistoricalInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/Pool" => {
                    #[allow(non_camel_case_types)]
                    struct PoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPoolRequest>
                    for PoolSvc<T> {
                        type Response = super::QueryPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PoolSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest>
                    for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/TokenizeShareRecordById" => {
                    #[allow(non_camel_case_types)]
                    struct TokenizeShareRecordByIdSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTokenizeShareRecordByIdRequest,
                    > for TokenizeShareRecordByIdSvc<T> {
                        type Response = super::QueryTokenizeShareRecordByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTokenizeShareRecordByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).tokenize_share_record_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TokenizeShareRecordByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/TokenizeShareRecordByDenom" => {
                    #[allow(non_camel_case_types)]
                    struct TokenizeShareRecordByDenomSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTokenizeShareRecordByDenomRequest,
                    > for TokenizeShareRecordByDenomSvc<T> {
                        type Response = super::QueryTokenizeShareRecordByDenomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTokenizeShareRecordByDenomRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).tokenize_share_record_by_denom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TokenizeShareRecordByDenomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/TokenizeShareRecordsOwned" => {
                    #[allow(non_camel_case_types)]
                    struct TokenizeShareRecordsOwnedSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTokenizeShareRecordsOwnedRequest,
                    > for TokenizeShareRecordsOwnedSvc<T> {
                        type Response = super::QueryTokenizeShareRecordsOwnedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTokenizeShareRecordsOwnedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).tokenize_share_records_owned(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TokenizeShareRecordsOwnedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/AllTokenizeShareRecords" => {
                    #[allow(non_camel_case_types)]
                    struct AllTokenizeShareRecordsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryAllTokenizeShareRecordsRequest,
                    > for AllTokenizeShareRecordsSvc<T> {
                        type Response = super::QueryAllTokenizeShareRecordsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryAllTokenizeShareRecordsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).all_tokenize_share_records(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllTokenizeShareRecordsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/LastTokenizeShareRecordId" => {
                    #[allow(non_camel_case_types)]
                    struct LastTokenizeShareRecordIdSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryLastTokenizeShareRecordIdRequest,
                    > for LastTokenizeShareRecordIdSvc<T> {
                        type Response = super::QueryLastTokenizeShareRecordIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryLastTokenizeShareRecordIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).last_tokenize_share_record_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LastTokenizeShareRecordIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/liquidstaking.staking.v1beta1.Query/TotalTokenizeSharedAssets" => {
                    #[allow(non_camel_case_types)]
                    struct TotalTokenizeSharedAssetsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTotalTokenizeSharedAssetsRequest,
                    > for TotalTokenizeSharedAssetsSvc<T> {
                        type Response = super::QueryTotalTokenizeSharedAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTotalTokenizeSharedAssetsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).total_tokenize_shared_assets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TotalTokenizeSharedAssetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "liquidstaking.staking.v1beta1.Query";
    }
}
