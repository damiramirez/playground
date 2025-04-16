use alloy::{
    network::{Ethereum, Network},
    primitives::U256,
    providers::Provider,
    transports::Transport,
};

use crate::{
    bindings::incrediblesquaringtaskmanager::{
        IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
        IIncredibleSquaringTaskManager::{
            Task as TaskContract, TaskResponse as TaskResponseContract,
        },
        IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance,
        BN254::{G1Point, G2Point},
    },
    interface::{Task, TaskManagerContract, TaskResponse},
};

type ContractInstance = IncredibleSquaringTaskManagerInstance<
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

struct ContractImpl {
    task_manager: ContractInstance,
}

impl ContractImpl {
    fn new(task_manager: ContractInstance) -> Self {
        task_manager.address();
        Self { task_manager }
    }
}

impl TaskManagerContract for ContractImpl {
    type Input = U256;
    type Output = U256;
    type G1Point = G1Point;
    type G2Point = G2Point;

    fn create_new_task(&self, task: Task<Self::Input>) -> Task<Self::Input> {
        task
    }

    fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: crate::interface::NonSignerStakesAndSignature<
            Self::G1Point,
            Self::G2Point,
        >,
    ) {
        // self.task_manager
        //     .respondToTask(task, response, non_signer_stakes_and_signature)
        //     .send();

        let contract_task = TaskContract {
            numberToBeSquared: task.input,
            taskCreatedBlock: task.taskCreatedBlock,
            quorumNumbers: task.quorumNumbers,
            quorumThresholdPercentage: task.quorumThresholdPercentage,
        };

        let contract_task_response = TaskResponseContract {
            referenceTaskIndex: response.referenceTaskIndex,
            numberSquared: response.response,
        };

        let non_signer_contract = NonSignerStakesAndSignature {
            nonSignerQuorumBitmapIndices: non_signer_stakes_and_signature
                .nonSignerQuorumBitmapIndices,
            nonSignerPubkeys: non_signer_stakes_and_signature.nonSignerPubkeys,
            quorumApks: non_signer_stakes_and_signature.quorumApks,
            apkG2: non_signer_stakes_and_signature.apkG2,
            sigma: non_signer_stakes_and_signature.sigma,
            quorumApkIndices: non_signer_stakes_and_signature.quorumApkIndices,
            totalStakeIndices: non_signer_stakes_and_signature.totalStakeIndices,
            nonSignerStakeIndices: non_signer_stakes_and_signature.nonSignerStakeIndices,
        };

        self.task_manager
            .respondToTask(contract_task, contract_task_response, non_signer_contract)
            .send();
    }
}

pub struct TaskManagerAdapter<T, P, N = Ethereum> {
    pub contract: IncredibleSquaringTaskManagerInstance<T, P, N>,
}

impl<T, P, N> TaskManagerAdapter<T, P, N>
where
    T: Transport + Clone,
    P: Provider<N>,
    N: Network,
{
    fn respond_to_task(&self) {
        &self.contract;
    }
}
