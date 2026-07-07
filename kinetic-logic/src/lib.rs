use logigate_architecture::LogigateSandbox;
use zkp_coordinate_vault::ZkpAuthGate;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct TrajectoryState {
    pub transaction_id: usize,
    pub raw_metric_payload: String,
    pub scalar_weight: f64,
}

pub struct VectorDelta {
    pub displacement_magnitude: f64,
    pub strategic_mutation_log: String,
}

pub enum ProcessingTier {
    L1Tactical,
    L2Strategic,
    L3Sovereign,
}

pub struct KineticEngine {
    pub sandbox: LogigateSandbox,
}

impl KineticEngine {
    pub fn new() -> Self {
        Self {
            sandbox: LogigateSandbox::initialize_isolated_sandbox(),
        }
    }

    pub async fn compute_trajectory_delta(
        &self,
        tier: ProcessingTier,
        state: TrajectoryState,
        proof: &str,
    ) -> Result<VectorDelta, String> {
        ZkpAuthGate::verify_session_proof(proof, "TARGET_VALIDATION_HASH")
            .map_err(|e| e.to_string())?;

        self.sandbox.secure_write(&format!("tx_{}", state.transaction_id), &state.raw_metric_payload)?;

        let tier_multiplier = match tier {
            ProcessingTier::L1Tactical => 0.10,
            ProcessingTier::L2Strategic => 0.50,
            ProcessingTier::L3Sovereign => 1.00,
        };

        let absolute_delta = state.scalar_weight * tier_multiplier;

        let tracking_output = VectorDelta {
            displacement_magnitude: absolute_delta,
            strategic_mutation_log: format!("Delta successfully locked for state vector: {}", state.transaction_id),
        };

        self.sandbox.execute_forced_reset_trigger();

        Ok(tracking_output)
    }
}
