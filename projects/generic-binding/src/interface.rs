use alloy::contract::ContractInstance;

pub type GenericContractInstance = ContractInstance<
    (),
    alloy::providers::fillers::FillProvider<
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::JoinFill<
                alloy::providers::Identity,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::GasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::BlobGasFiller,
                        alloy::providers::fillers::JoinFill<
                            alloy::providers::fillers::NonceFiller,
                            alloy::providers::fillers::ChainIdFiller,
                        >,
                    >,
                >,
            >,
            alloy::providers::fillers::WalletFiller<alloy::network::EthereumWallet>,
        >,
        alloy::providers::RootProvider,
    >,
>;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct TaskResponse<OUTPUT> {
    #[allow(missing_docs)]
    pub referenceTaskIndex: u32,
    #[allow(missing_docs)]
    pub response: OUTPUT,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Task<INPUT> {
    #[allow(missing_docs)]
    pub input: INPUT,
    #[allow(missing_docs)]
    pub taskCreatedBlock: u32,
    #[allow(missing_docs)]
    pub quorumNumbers: alloy::sol_types::private::Bytes,
    #[allow(missing_docs)]
    pub quorumThresholdPercentage: u32,
}

/// This is the documentation for the `NonSignerStakesAndSignature` struct.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
#[derive(Debug, Clone)]
pub struct NonSignerStakesAndSignature<G1Point, G2Point>
where
    G1Point: alloy::sol_types::SolType + Clone,
    G2Point: alloy::sol_types::SolType + Clone,
{
    #[allow(missing_docs)]
    pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
    #[allow(missing_docs)]
    pub nonSignerPubkeys: alloy::sol_types::private::Vec<G1Point>,
    #[allow(missing_docs)]
    pub quorumApks: alloy::sol_types::private::Vec<G1Point>,
    #[allow(missing_docs)]
    pub apkG2: G2Point, // as alloy::sol_types::SolType,
    #[allow(missing_docs)]
    pub sigma: G1Point, // as alloy::sol_types::SolType,
    #[allow(missing_docs)]
    pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
    #[allow(missing_docs)]
    pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
    #[allow(missing_docs)]
    pub nonSignerStakeIndices: alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
}

pub trait TaskManagerContract {
    /// Type for task indices
    //type Index;

    /// Type for inputs of each task
    type Input;

    /// Type for outputs of each task
    type Output;

    /// G1Point type with required trait bounds
    type G1Point: alloy::sol_types::SolType + Clone;

    /// G2Point type with required trait bounds
    type G2Point: alloy::sol_types::SolType + Clone;

    fn create_new_task(&self, task: Task<Self::Input>) -> Task<Self::Input>;

    fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature<Self::G1Point, Self::G2Point>,
    );
}
