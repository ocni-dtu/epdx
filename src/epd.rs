use std::collections::HashMap;
use chrono::{DateTime, Utc};
use chrono::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use pkg_version::*;

use crate::ilcd::{Exchange, ILCD, LCIAResult, ModuleAnie};

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
    mfr: Option<ImpactCategory>,
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
    KM,
    TONES_KM,
    UNKNOWN,
}

impl From<&String> for Unit {
    fn from(unit: &String) -> Self {
        match unit.to_ascii_lowercase().as_str() {
            "m" => Unit::M,
            "m2" | "m^2" | "qm" => Unit::M2,
            "m3" | "m^3" => Unit::M3,
            "km" => Unit::KM,
            "kg" => Unit::KG,
            "tones" | "tonnes" => Unit::TONES,
            "pcs" | "stk" | "pcs." => Unit::PCS,
            "l" => Unit::L,
            "m2r1" => Unit::M2R1,
            "tones*km" => Unit::TONES_KM,
            _ => Unit::UNKNOWN
        }
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
        if value.to_ascii_lowercase().contains("15804") {
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
            match (&anie.module, &anie.value) {
                (Some(module), Some(value)) if module.to_lowercase() == String::from("a1-a3") => category.a1a3 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("a4") => category.a4 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("a5") => category.a5 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b1") => category.b1 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b2") => category.b2 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b3") => category.b3 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b4") => category.b4 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b5") => category.b5 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b6") => category.b6 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b7") => category.b7 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("c1") => category.c1 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("c2") => category.c2 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("c3") => category.c3 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("c4") => category.c4 = Some(f64::from(value)),
                (Some(module), Some(value)) if module.to_lowercase() == String::from("d") => category.d = Some(f64::from(value)),
                _ => continue
            }
        }
        category
    }
}


#[derive(Debug, Serialize, JsonSchema)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
struct Conversion {
    value: f64,
    to: Unit,
    meta_data: String,
}


impl<'de> Deserialize<'de> for EPD {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let helper = ILCD::deserialize(deserializer)?;
        let subtype = helper.modelling_and_validation.lci_method_and_allocation.other.anies.iter().find(|&anie| anie.name == "subType").unwrap();
        let format_version = format!("{}.{}.{}", pkg_version_major!(), pkg_version_minor!(), pkg_version_patch!());
        let standard = get_ilcd_standard(&helper);

        let (gwp, odp, ap, ep, pocp, adpe, adpf) = collect_from_lcia_result(&helper.lcia_results.lcia_result);

        let (declared_unit, conversions, pere, perm, pert, penre, penrm, penrt, sm ,rsf, nrsf, fw, hwd, nhwd, rwd, cru, mfr, mer, eee, eet) = collect_from_exchanges(&helper.exchanges.exchange);

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
            mfr,
            mer,
            eee,
            penre,
            eet,
        })
    }
}

fn get_ilcd_standard(helper: &ILCD) -> Standard {
    for compliance in &helper.modelling_and_validation.compliance_declarations.compliance {
        match compliance.reference_to_compliance_system.short_description.iter().find(|&description| description.lang == "en") {
            Some(description) => return Standard::from(&description.value),
            _ => continue
        }
    }

    return Standard::UNKNOWN;
}

fn get_converted_unit(unit_value: &String) -> Unit {
    let value = unit_value.split("/").collect::<Vec<&str>>().first().unwrap().to_string();
    Unit::from(&value)
}

fn get_ilcd_conversion(exchange: &Exchange) -> Vec<Conversion> {
    let mut conversions: Vec<Conversion> = vec![];

    match &exchange.material_properties {
        Some(material_properties) => {
            for material_property in material_properties {
                let value = material_property.value.parse().unwrap_or_else(|_| 1.0);
                conversions.push(Conversion { value, to: get_converted_unit(&material_property.unit), meta_data: serde_json::to_string(material_property).unwrap() })
            }
        }
        _ => return conversions
    }

    conversions
}

fn get_ilcd_declared_unit(exchange: &Exchange) -> Unit {
    for flow_property in exchange.flow_properties.as_ref().unwrap() {
        match (flow_property.reference_flow_property, &flow_property.reference_unit) {
            (Some(reference_flow), Some(reference_unit)) if reference_flow == true => return Unit::from(reference_unit),
            _ => continue
        }
    }

    Unit::UNKNOWN
}

fn collect_from_lcia_result(lcia_result: &Vec<LCIAResult>) -> (Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>) {
    let mut gwp = None;
    let mut odp = None;
    let mut ap = None;
    let mut ep = None;
    let mut pocp = None;
    let mut adpe = None;
    let mut adpf = None;

    for lcia_result in lcia_result {
        for description in &lcia_result.reference_to_lcia_method_dataset.short_description {
            let impact_value = Some(ImpactCategory::from(&lcia_result.other.anies));
            match &description.value {
                value if value.as_str() == "Global warming potential (GWP)" => gwp = impact_value,
                value if value.as_str() == "Depletion potential of the stratospheric ozone layer (ODP)" => odp = impact_value,
                value if value.as_str() == "Acidification potential of soil and water (AP)" => ap = impact_value,
                value if value.as_str() == "Eutrophication potential (EP)" => ep = impact_value,
                value if value.as_str() == "Formation potential of tropospheric ozone (POCP)" => pocp = impact_value,
                value if value.as_str() == "Abiotic depletion potential for non fossil resources (ADPE))" => adpe = impact_value,
                value if value.as_str() == "Abiotic depletion potential for fossil resources (ADPF)" => adpf = impact_value,
                _ => continue
            }
        }
    }

    (gwp, odp, ap, ep, pocp, adpe, adpf)
}

fn collect_from_exchanges(exchanges: &Vec<Exchange>) -> (Unit, Vec<Conversion>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>, Option<ImpactCategory>) {
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
    let mut mfr = None;
    let mut mer = None;
    let mut eee = None;
    let mut eet = None;

    for exchange in exchanges {
        match exchange.reference_flow {
            Some(flow) if flow == true => {
                declared_unit = get_ilcd_declared_unit(exchange);
                conversions = get_ilcd_conversion(exchange);
            }
            _ => {
                for description in &exchange.reference_to_flow_data_set.short_description {
                    let impact_value = match &exchange.other {
                        Some(_anies) => Some(ImpactCategory::from(&_anies.anies)),
                        _ => continue
                    };
                    match &description.value {
                        _description if _description == "Use of renewable primary energy (PERE)" => pere = impact_value,
                        _description if _description == "Use of renewable primary energy resources used as raw materials (PERM)" => perm = impact_value,
                        _description if _description == "Total use of renewable primary energy resources (PERT)" => pert = impact_value,
                        _description if _description == "Use of non renewable primary energy (PENRE)" => penre = impact_value,
                        _description if _description == "Use of non renewable primary energy resources used as raw materials (PENRM)" => penrm = impact_value,
                        _description if _description == "Total use of non renewable primary energy resource (PENRT)" => penrt = impact_value,
                        _description if _description == "Use of secondary material (SM)" => sm = impact_value,
                        _description if _description == "Use of renewable secondary fuels (RSF)" => rsf = impact_value,
                        _description if _description == "Use of non renewable secondary fuels (NRSF)" => nrsf = impact_value,
                        _description if _description == "Use of net fresh water (FW)" => fw = impact_value,
                        _description if _description == "Hazardous waste disposed (HWD)" => hwd = impact_value,
                        _description if _description == "Non hazardous waste dispose (NHWD)" => nhwd = impact_value,
                        _description if _description == "Radioactive waste disposed (RWD)" => rwd = impact_value,
                        _description if _description == "Components for re-use (CRU)" => cru = impact_value,
                        _description if _description == "Materials for recycling (MFR)" => mfr = impact_value,
                        _description if _description == "Materials for energy recovery (MER)" => mer = impact_value,
                        _description if _description == "Exported electrical energy (EEE)" => eee = impact_value,
                        _description if _description == "Exported thermal energy (EET)" => eet = impact_value,
                        _ => continue
                    }
                }
            }
        };
    }

    (declared_unit, conversions, pere, perm, pert, penre, penrm, penrt, sm ,rsf, nrsf, fw, hwd, nhwd, rwd, cru, mfr, mer, eee, eet)
}