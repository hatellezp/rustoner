#[derive(Debug, Clone, PartialEq)]
pub struct AboxQItemCDb {
    pub(crate) id_db: i64,
    pub(crate) constant_name_db: String,
    pub(crate) concept_name_db: String,
    pub(crate) constant_db: i64,
    pub(crate) concept_db: i64,
    pub(crate) prevalue: f64,
    pub(crate) value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AboxQItemRDb {
    pub(crate) id_db: i64,
    pub(crate) constant1_name_db: String,
    pub(crate) constant2_name_db: String,
    pub(crate) role_name_db: String,
    pub(crate) constant1_db: i64,
    pub(crate) constant2_db: i64,
    pub(crate) role_db: i64,
    pub(crate) prevalue: f64,
    pub(crate) value: Option<f64>,
}
