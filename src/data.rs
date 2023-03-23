use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Recipe {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) ingress: String,
    pub(crate) image_url: String,
    pub(crate) steps: Vec<String>,
    pub(crate) ingredients: Vec<Ingredient>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Ingredient {
    pub(crate) name: String,
    pub(crate) amount: Amount,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Amount {
    pub(crate) quantity: u8,
    pub(crate) unit: String,
}