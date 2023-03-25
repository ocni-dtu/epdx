use chrono::{DateTime, Utc};
use chrono::prelude::*;
use chrono::serde::ts_seconds;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ilcd::{AnieValue, ILCD, ModuleAnie};

#[derive(Debug, Serialize)]
pub struct EPD {
    id: String,
    name: String,
//     declared_unit: Unit,
    version: String,

    #[serde(with = "ts_seconds")]
    published_date: DateTime<Utc>,

    #[serde(with = "ts_seconds")]
    valid_until: DateTime<Utc>,
//     format_version: String,
//     source: String,
//     reference_service_life: u32,
//     standard: Standard,
    comment: Option<String>,
    location: String,
    // subtype: SubType,
    subtype: String,
//     conversions: Vec<Conversion>,
    gwp: Option<ImpactCategory>,
    odp: Option<ImpactCategory>,
//     ap: ImpactCategory,
//     ep: ImpactCategory,
//     pocp: ImpactCategory,
//     adpe: ImpactCategory,
//     adpf: ImpactCategory,
//     penre: ImpactCategory,
//     pere: ImpactCategory,
//     perm: ImpactCategory,
//     pert: ImpactCategory,
//     penrt: ImpactCategory,
//     penrm: ImpactCategory,
//     sm: ImpactCategory,
//     rsf: ImpactCategory,
//     nrsf: ImpactCategory,
//     fw: ImpactCategory,
//     hwd: ImpactCategory,
//     nhed: ImpactCategory,
//     rwd: ImpactCategory,
//     cru: ImpactCategory,
//     mrf: ImpactCategory,
//     mer: ImpactCategory,
//     eee: ImpactCategory,
//     eet: ImpactCategory,
}

// #[derive(Debug)]
// enum Unit {
//     M,
//     M2,
//     M3,
//     KG,
//     Tones,
//     PCS,
//     L,
//     M2R1,
// }
//
// #[derive(Debug)]
// enum Standard {
//     EN15804A1,
//     EN15804A2,
// }

#[derive(Debug, Serialize)]
enum SubType {
    Generic,
    Specific,
    Industry,
    Representative
}

#[derive(Debug, Serialize, Default)]
struct ImpactCategory {
    a1a3: Option<f64>,
    a4: Option<f64>,
    a5: Option<f64>,
    b1: Option<f64>,
    b2: Option<f64>,
    b3: Option<f64>,
    b4: Option<f64>,
    b5: Option<f64>,
    b6: Option<f64>,
    b7: Option<f64>,
    c1: Option<f64>,
    c2: Option<f64>,
    c3: Option<f64>,
    c4: Option<f64>,
    d: Option<f64>,
}

impl From<&Vec<ModuleAnie>> for ImpactCategory {
    fn from(anies: &Vec<ModuleAnie>) -> Self {
        // let a1a3_module = anies.iter().find(|anie| anie.module == Some(String::from("a1-a3"))).unwrap();
        // let a1a3 = match &a1a3_module.value {
        //     AnieValue::ValueString(value) => value.parse::<f64>().unwrap(),
        //     _ => None
        // };
        let mut category = ImpactCategory::default();

        for anie in anies {
            if anie.module == Some(String::from("A1-A3")) {
                category.a1a3 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("A4")) {
                category.a4 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("A5")) {
                category.a5 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("B1")) {
                category.b1 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("B2")) {
                category.b2 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("B3")) {
                category.b3 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("B4")) {
                category.b4 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("B5")) {
                category.b5 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("B6")) {
                category.b6 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("B7")) {
                category.b7 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("C1")) {
                category.c1 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("C2")) {
                category.c2 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("C3")) {
                category.c3 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("C4")) {
                category.c4 = Some(f64::from(anie.value.as_ref().unwrap()))
            } else if anie.module == Some(String::from("D")) {
                category.d = Some(f64::from(anie.value.as_ref().unwrap()))
            }
        };
        category
    }
}


// #[derive(Debug)]
// struct Conversion {
//     value: f64,
//     to: String,
// }


impl<'de> Deserialize<'de> for EPD {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {


        let helper = ILCD::deserialize(deserializer)?;
        let subtype = helper.modelling_and_validation.lci_method_and_allocation.other.anies.iter().find(|&anie| anie.name == "subType").unwrap();

        let mut gwp = None;
        let mut odp = None;
        for lcia_result in helper.lcia_results.lci_result.iter() {
            if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Global warming potential (GWP)").is_some() {
                gwp = Some(ImpactCategory::from(&lcia_result.other.anies))
            } else if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Depletion potential of the stratospheric ozone layer (ODP)").is_some() {
                odp = Some(ImpactCategory::from(&lcia_result.other.anies))
            }


        }


        Ok(EPD {
            id: helper.process_information.data_set_information.uuid,
            name: helper.process_information.data_set_information.name.base_name.first().unwrap().value.to_string(),
            version: helper.version,
            comment: None,
            published_date: Utc.with_ymd_and_hms(helper.process_information.time.reference_year, 1, 1,0,0,0).unwrap(),
            valid_until: Utc.with_ymd_and_hms(helper.process_information.time.data_set_valid_until, 1, 1,0,0,0).unwrap(),
            location: helper.process_information.geography.location_of_operation_supply_or_production.location,
            subtype: subtype.value.clone(),
            gwp,
            odp
        })
    }
}