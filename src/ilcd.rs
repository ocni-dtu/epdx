use serde::{Deserialize};


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ILCD {
    pub process_information: ProcessInformation,
    pub modelling_and_validation: ModellingAndValidation,
    #[serde(alias = "LCIAResults")]
    pub lcia_results: LCIAResults,
    pub version: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModellingAndValidation {
    #[serde(alias = "LCIMethodAndAllocation")]
    pub lci_method_and_allocation: LCIMethodAndAllocation,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCIAResults {
    #[serde(alias = "LCIAResult")]
    pub lci_result: Vec<LCIAResult>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LCIAResult {
    #[serde(alias = "referenceToLCIAMethodDataSet")]
    pub reference_to_lcia_method_dataset: ReferenceToLCIAMethodDataSet,
    pub other: LCIAAnies
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LCIAAnies {
    pub anies: Vec<ModuleAnie>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleAnie {
    pub module: Option<String>,
    pub value: Option<AnieValue>
}


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnieValue {
    ValueString(String),
    ValueObject(ValueObject),
}

impl From<&AnieValue> for f64 {
    fn from(value: &AnieValue) -> Self {
        match value {
            AnieValue::ValueString(s) => {
                // Parse the string into a float
                let float_value = s.parse::<f64>().unwrap();
                float_value
            },
            AnieValue::ValueObject(_) => {
                panic!("Cannot convert AnieValue::ValueObject to MyStruct");
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueObject {
    _type: String,
    ref_object_id: String,
    uri: String,
}

#[derive(Deserialize, Debug)]
pub enum ModuleValue {
    Value(String),
    Name(ModuleMap)
}

#[derive(Deserialize, Debug)]
pub struct ModuleMap {
    name: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceToLCIAMethodDataSet {
    pub short_description: Vec<ValueLang>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCIMethodAndAllocation {
    pub other: Anies,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anies {
    pub anies: Vec<Anie>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anie {
    pub name: String,
    pub value: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInformation {
    pub data_set_information: DataSetInformation,
    pub time: TimeData,
    pub geography: Geography
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geography {
    pub location_of_operation_supply_or_production: LocationOfOperationSupplyOrProduction
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationOfOperationSupplyOrProduction {
    pub location: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeData {
    pub reference_year: i32,
    pub data_set_valid_until: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSetInformation {
    #[serde(alias = "UUID")]
    pub uuid: String,
    pub name: DataSetName
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSetName {
    pub base_name: Vec<ValueLang>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ValueLang {
    pub value: String,
    pub lang: String
}