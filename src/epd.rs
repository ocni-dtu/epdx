use std::collections::HashMap;
use chrono::{DateTime, Utc};
use chrono::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ilcd::{ILCD, ModuleAnie};

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

#[derive(Debug, Serialize, JsonSchema)]
#[cfg_attr(feature = "jsbindings", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct EPD {
    id: String,
    name: String,
    declared_unit: Unit,
    version: String,

    #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    published_date: DateTime<Utc>,

    #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    valid_until: DateTime<Utc>,
    format_version: String,
    source: Option<Source>,
    reference_service_life: Option<u32>,
    standard: Standard,
    comment: Option<String>,
    location: String,
    subtype: SubType,
    conversions: Option<Vec<Conversion>>,
    gwp: Option<ImpactCategory>,
    odp: Option<ImpactCategory>,
    ap: Option<ImpactCategory>,
    ep: Option<ImpactCategory>,
    pocp: Option<ImpactCategory>,
    adpe: Option<ImpactCategory>,
    adpf: Option<ImpactCategory>,
    penre: Option<ImpactCategory>,
    pere: Option<ImpactCategory>,
    perm: Option<ImpactCategory>,
    pert: Option<ImpactCategory>,
    penrt: Option<ImpactCategory>,
    penrm: Option<ImpactCategory>,
    sm: Option<ImpactCategory>,
    rsf: Option<ImpactCategory>,
    nrsf: Option<ImpactCategory>,
    fw: Option<ImpactCategory>,
    hwd: Option<ImpactCategory>,
    nhwd: Option<ImpactCategory>,
    rwd: Option<ImpactCategory>,
    cru: Option<ImpactCategory>,
    mrf: Option<ImpactCategory>,
    mer: Option<ImpactCategory>,
    eee: Option<ImpactCategory>,
    eet: Option<ImpactCategory>,
    meta_data: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum Unit {
    M,
    M2,
    M3,
    KG,
    TONES,
    PCS,
    L,
    M2R1,
    UNKNOWN,
}

impl From<&String> for Unit {
    fn from(value: &String) -> Self {
        if value.to_ascii_lowercase() == "m" {
            Unit::M
        } else if value.to_ascii_lowercase() == "m2" {
            Unit::M2
        } else { Unit::UNKNOWN }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Source {
    name: String,
    url: Option<String>,
}


#[derive(Debug, Serialize, JsonSchema)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
enum Standard {
    EN15804A1,
    EN15804A2,
    UNKNOWN,
}

impl From<&String> for Standard {
    fn from(value: &String) -> Self {
        if value.to_ascii_lowercase() == "en 15804" {
            Standard::EN15804A1
        } else if value.to_ascii_lowercase() == "en 15804+a2" {
            Standard::EN15804A2
        } else { Standard::UNKNOWN }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
enum SubType {
    Generic,
    Specific,
    Industry,
    Representative,
}

impl From<&String> for SubType {
    fn from(value: &String) -> Self {
        if value.to_ascii_lowercase().contains("representative") {
            SubType::Representative
        } else if value.to_ascii_lowercase().contains("specific") {
            SubType::Specific
        } else if value.to_ascii_lowercase().contains("industry") {
            SubType::Industry
        } else { SubType::Generic }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, JsonSchema)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct ImpactCategory {
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


#[derive(Debug, Serialize, JsonSchema)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
struct Conversion {
    value: f64,
    to: Unit,
}


impl<'de> Deserialize<'de> for EPD {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let helper = ILCD::deserialize(deserializer)?;
        let subtype = helper.modelling_and_validation.lci_method_and_allocation.other.anies.iter().find(|&anie| anie.name == "subType").unwrap();
        let format_version = String::from("0.1");
        let mut standard = Standard::UNKNOWN;
        for compliance in helper.modelling_and_validation.compliance_declarations.compliance {
            let description = compliance.reference_to_compliance_system.short_description.iter().find(|&description| description.lang == "en").unwrap();
            standard = Standard::from(&description.value)
        }

        let mut gwp = None;
        let mut odp = None;
        let mut ap = None;
        let mut ep = None;
        let mut pocp = None;
        let mut adpe = None;
        let mut adpf = None;

        for lcia_result in helper.lcia_results.lcia_result.iter() {
            if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Global warming potential (GWP)").is_some() {
                gwp = Some(ImpactCategory::from(&lcia_result.other.anies))
            } else if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Depletion potential of the stratospheric ozone layer (ODP)").is_some() {
                odp = Some(ImpactCategory::from(&lcia_result.other.anies))
            } else if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Acidification potential of soil and water (AP)").is_some() {
                ap = Some(ImpactCategory::from(&lcia_result.other.anies))
            } else if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Eutrophication potential (EP)").is_some() {
                ep = Some(ImpactCategory::from(&lcia_result.other.anies))
            } else if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Formation potential of tropospheric ozone (POCP)").is_some() {
                pocp = Some(ImpactCategory::from(&lcia_result.other.anies))
            } else if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Abiotic depletion potential for non fossil resources (ADPE)").is_some() {
                adpe = Some(ImpactCategory::from(&lcia_result.other.anies))
            } else if lcia_result.reference_to_lcia_method_dataset.short_description.iter().find(|&description| description.value == "Abiotic depletion potential for fossil resources (ADPF)").is_some() {
                adpf = Some(ImpactCategory::from(&lcia_result.other.anies))
            }
        }

        let mut declared_unit = Unit::UNKNOWN;
        let mut conversions: Vec<Conversion> = vec![];
        let mut pere = None;
        let mut perm = None;
        let mut pert = None;
        let mut penre = None;
        let mut penrm = None;
        let mut penrt = None;
        let mut sm = None;
        let mut rsf = None;
        let mut nrsf = None;
        let mut fw = None;
        let mut hwd = None;
        let mut nhwd = None;
        let mut rwd = None;
        let mut cru = None;
        let mut mrf = None;
        let mut mer = None;
        let mut eee = None;
        let mut eet = None;

        for exchange in helper.exchanges.exchange {
            if exchange.reference_flow.is_some() && exchange.reference_flow.unwrap() {
                for flow_property in exchange.flow_properties.unwrap() {
                    if flow_property.reference_flow_property.is_some() && flow_property.reference_flow_property.unwrap() {
                        declared_unit = Unit::from(&flow_property.reference_unit.unwrap())
                    }
                }
                // TODO - Fix this conversion thing
                for material_property in exchange.material_properties.unwrap() {
                    conversions.push(Conversion { value: material_property.value.parse().unwrap(), to: Unit::from(&material_property.unit) })
                }
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Use of renewable primary energy (PERE)").is_some() {
                pere = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Use of renewable primary energy resources used as raw materials (PERM)").is_some() {
                perm = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Total use of renewable primary energy resources (PERT)").is_some() {
                pert = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Use of non renewable primary energy (PENRE)").is_some() {
                penre = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Use of non renewable primary energy resources used as raw materials (PENRM)").is_some() {
                penrm = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Total use of non renewable primary energy resource (PENRT)").is_some() {
                penrt = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Use of secondary material (SM)").is_some() {
                sm = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Use of renewable secondary fuels (RSF)").is_some() {
                rsf = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Use of non renewable secondary fuels (NRSF)").is_some() {
                nrsf = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Use of net fresh water (FW)").is_some() {
                fw = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Hazardous waste disposed (HWD)").is_some() {
                hwd = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Non hazardous waste dispose (NHWD)").is_some() {
                nhwd = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Radioactive waste disposed (RWD)").is_some() {
                rwd = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Components for re-use (CRU)").is_some() {
                cru = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Materials for recycling (MFR)").is_some() {
                mrf = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Materials for energy recovery (MER)").is_some() {
                mer = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Exported electrical energy (EEE)").is_some() {
                eee = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            } else if exchange.reference_to_flow_data_set.short_description.iter().find(|&description| description.value == "Exported thermal energy (EET)").is_some() {
                eet = Some(ImpactCategory::from(&exchange.other.unwrap().anies))
            }
        }

        Ok(EPD {
            id: helper.process_information.data_set_information.uuid,
            name: helper.process_information.data_set_information.name.base_name.first().unwrap().value.to_string(),
            version: helper.version,
            format_version,
            declared_unit,
            reference_service_life: None,
            conversions: Some(conversions),
            standard,
            comment: None,
            meta_data: None,
            source: None,
            published_date: Utc.with_ymd_and_hms(helper.process_information.time.reference_year, 1, 1, 0, 0, 0).unwrap(),
            valid_until: Utc.with_ymd_and_hms(helper.process_information.time.data_set_valid_until, 1, 1, 0, 0, 0).unwrap(),
            location: helper.process_information.geography.location_of_operation_supply_or_production.location,
            subtype: SubType::from(&subtype.value),
            gwp,
            odp,
            ap,
            ep,
            pocp,
            adpe,
            adpf,
            pere,
            perm,
            pert,
            penrt,
            penrm,
            sm,
            rsf,
            nrsf,
            fw,
            hwd,
            nhwd,
            rwd,
            cru,
            mrf,
            mer,
            eee,
            penre,
            eet,
        })
    }
}