use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 7,
        agency_name: "International Civil Aviation Organization (ICAO)".to_string(),
        project_name: "ICAO AeroLedger AI".to_string(),
        portfolio: "Global Civil Aviation Safety & Emissions".to_string(),
        strategy: "Soulbound Verifiable Credential -- International Pilot and Aviation Airworthiness Credentials".to_string(),
        target_chain: "Internet Computer (ICP) + Base L2".to_string(),
        is_active: true,
    }
}
