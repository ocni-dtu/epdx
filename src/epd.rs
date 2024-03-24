use chrono::prelude::*;
use chrono::{DateTime, Utc};
use pkg_version::*;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::ilcd::{Exchange, LCIAResult, ModuleAnie, ILCD};

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

#[derive(Serialize, JsonSchema, Clone)]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct EPD {
    pub id: String,
    pub name: String,
    pub declared_unit: Unit,
    pub version: String,

    #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    pub published_date: DateTime<Utc>,

    #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    pub valid_until: DateTime<Utc>,
    pub format_version: String,
    pub source: Option<Source>,
    pub reference_service_life: Option<u32>,
    pub standard: Standard,
    pub comment: Option<String>,
    pub location: String,
    pub subtype: SubType,
    pub conversions: Option<Vec<Conversion>>,
    pub gwp: Option<ImpactCategory>,
    pub odp: Option<ImpactCategory>,
    pub ap: Option<ImpactCategory>,
    pub ep: Option<ImpactCategory>,
    pub pocp: Option<ImpactCategory>,
    pub adpe: Option<ImpactCategory>,
    pub adpf: Option<ImpactCategory>,
    pub penre: Option<ImpactCategory>,
    pub pere: Option<ImpactCategory>,
    pub perm: Option<ImpactCategory>,
    pub pert: Option<ImpactCategory>,
    pub penrt: Option<ImpactCategory>,
    pub penrm: Option<ImpactCategory>,
    pub sm: Option<ImpactCategory>,
    pub rsf: Option<ImpactCategory>,
    pub nrsf: Option<ImpactCategory>,
    pub fw: Option<ImpactCategory>,
    pub hwd: Option<ImpactCategory>,
    pub nhwd: Option<ImpactCategory>,
    pub rwd: Option<ImpactCategory>,
    pub cru: Option<ImpactCategory>,
    pub mfr: Option<ImpactCategory>,
    pub mer: Option<ImpactCategory>,
    pub eee: Option<ImpactCategory>,
    pub eet: Option<ImpactCategory>,
    pub meta_data: Option<HashMap<String, String>>,
}

impl EPD {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            declared_unit: Unit::UNKNOWN,
            version: "".to_string(),
            published_date: Default::default(),
            valid_until: Default::default(),
            format_version: "".to_string(),
            source: None,
            reference_service_life: None,
            standard: Standard::UNKNOWN,
            comment: None,
            location: "".to_string(),
            subtype: SubType::Generic,
            conversions: None,
            gwp: None,
            odp: None,
            ap: None,
            ep: None,
            pocp: None,
            adpe: None,
            adpf: None,
            penre: None,
            pere: None,
            perm: None,
            pert: None,
            penrt: None,
            penrm: None,
            sm: None,
            rsf: None,
            nrsf: None,
            fw: None,
            hwd: None,
            nhwd: None,
            rwd: None,
            cru: None,
            mfr: None,
            mer: None,
            eee: None,
            eet: None,
            meta_data: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
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
            _ => Unit::UNKNOWN,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Source {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum Standard {
    EN15804A1,
    EN15804A2,
    UNKNOWN,
}

impl From<&String> for Standard {
    fn from(value: &String) -> Self {
        if value.to_ascii_lowercase().contains("15804+a2") {
            Standard::EN15804A2
        } else if value.to_ascii_lowercase().contains("15804") {
            Standard::EN15804A1
        } else {
            Standard::UNKNOWN
        }
    }
}

#[derive(Debug, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum SubType {
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
        } else {
            SubType::Generic
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct ImpactCategory {
    pub a1a3: Option<f64>,
    pub a4: Option<f64>,
    pub a5: Option<f64>,
    pub b1: Option<f64>,
    pub b2: Option<f64>,
    pub b3: Option<f64>,
    pub b4: Option<f64>,
    pub b5: Option<f64>,
    pub b6: Option<f64>,
    pub b7: Option<f64>,
    pub c1: Option<f64>,
    pub c2: Option<f64>,
    pub c3: Option<f64>,
    pub c4: Option<f64>,
    pub d: Option<f64>,
}

impl ImpactCategory {
    pub fn new() -> Self {
        Self {
            a1a3: None,
            a4: None,
            a5: None,
            b1: None,
            b2: None,
            b3: None,
            b4: None,
            b5: None,
            b6: None,
            b7: None,
            c1: None,
            c2: None,
            c3: None,
            c4: None,
            d: None,
        }
    }

    pub fn add(self: &mut Self, key: &str, value: f64) {
        match key.to_lowercase().as_str() {
            "a1a3" => self.a1a3 = Some(value),
            "a4" => self.a4 = Some(value),
            "a5" => self.a5 = Some(value),
            "b1" => self.b1 = Some(value),
            "b2" => self.b2 = Some(value),
            "b3" => self.b3 = Some(value),
            "b4" => self.b4 = Some(value),
            "b5" => self.b5 = Some(value),
            "b6" => self.b6 = Some(value),
            "b7" => self.b7 = Some(value),
            "c1" => self.c1 = Some(value),
            "c2" => self.c2 = Some(value),
            "c3" => self.c3 = Some(value),
            "c4" => self.c4 = Some(value),
            "d" => self.d = Some(value),
            _ => (),
        }
    }
}

impl From<&Vec<ModuleAnie>> for ImpactCategory {
    fn from(anies: &Vec<ModuleAnie>) -> Self {
        let mut category = ImpactCategory::default();

        for anie in anies {
            match (&anie.module, &anie.value) {
                (Some(module), Some(value)) if module.to_lowercase() == String::from("a1-a3") => {
                    category.a1a3 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("a4") => {
                    category.a4 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("a5") => {
                    category.a5 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b1") => {
                    category.b1 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b2") => {
                    category.b2 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b3") => {
                    category.b3 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b4") => {
                    category.b4 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b5") => {
                    category.b5 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b6") => {
                    category.b6 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("b7") => {
                    category.b7 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("c1") => {
                    category.c1 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("c2") => {
                    category.c2 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("c3") => {
                    category.c3 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("c4") => {
                    category.c4 = Some(f64::from(value))
                }
                (Some(module), Some(value)) if module.to_lowercase() == String::from("d") => {
                    category.d = Some(f64::from(value))
                }
                _ => continue,
            }
        }
        category
    }
}

#[derive(Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Conversion {
    pub value: f64,
    pub to: Unit,
    pub meta_data: String,
}

impl<'de> Deserialize<'de> for EPD {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = ILCD::deserialize(deserializer)?;
        let subtype = helper
            .modelling_and_validation
            .lci_method_and_allocation
            .other
            .anies
            .iter()
            .find(|&anie| anie.name == "subType")
            .unwrap();
        let format_version = format!(
            "{}.{}.{}",
            pkg_version_major!(),
            pkg_version_minor!(),
            pkg_version_patch!()
        );
        let standard = get_ilcd_standard(&helper);

        let (gwp, odp, ap, ep, pocp, adpe, adpf) =
            collect_from_lcia_result(&helper.lcia_results.lcia_result);

        let (
            declared_unit,
            conversions,
            pere,
            perm,
            pert,
            penre,
            penrm,
            penrt,
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
            eet,
        ) = collect_from_exchanges(&helper.exchanges.exchange);

        Ok(EPD {
            id: helper.process_information.data_set_information.uuid,
            name: helper
                .process_information
                .data_set_information
                .name
                .base_name
                .first()
                .unwrap()
                .value
                .to_string(),
            version: helper.version,
            format_version,
            declared_unit,
            reference_service_life: None,
            conversions: Some(conversions),
            standard,
            comment: None,
            meta_data: None,
            source: None,
            published_date: Utc
                .with_ymd_and_hms(
                    helper.process_information.time.reference_year,
                    1,
                    1,
                    0,
                    0,
                    0,
                )
                .unwrap(),
            valid_until: Utc
                .with_ymd_and_hms(
                    helper.process_information.time.data_set_valid_until,
                    1,
                    1,
                    0,
                    0,
                    0,
                )
                .unwrap(),
            location: helper
                .process_information
                .geography
                .location_of_operation_supply_or_production
                .location,
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
    for compliance in &helper
        .modelling_and_validation
        .compliance_declarations
        .compliance
    {
        for description in &compliance.reference_to_compliance_system.short_description {
            match Standard::from(&description.value) {
                Standard::UNKNOWN => continue,
                standard => return standard,
            }
        }
    }

    return Standard::UNKNOWN;
}

fn get_converted_unit(unit_value: &String) -> Unit {
    let value = unit_value
        .split("/")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string();
    Unit::from(&value)
}

fn get_ilcd_conversion(exchange: &Exchange) -> Vec<Conversion> {
    let mut conversions: Vec<Conversion> = vec![];

    match &exchange.material_properties {
        Some(material_properties) => {
            for material_property in material_properties {
                let value = material_property.value.parse().unwrap_or_else(|_| 1.0);
                conversions.push(Conversion {
                    value,
                    to: get_converted_unit(&material_property.unit),
                    meta_data: serde_json::to_string(material_property).unwrap(),
                })
            }
        }
        _ => return conversions,
    }

    conversions
}

fn get_ilcd_declared_unit(exchange: &Exchange) -> Unit {
    for flow_property in exchange.flow_properties.as_ref().unwrap() {
        match (
            flow_property.reference_flow_property,
            &flow_property.reference_unit,
        ) {
            (Some(reference_flow), Some(reference_unit)) if reference_flow == true => {
                return Unit::from(reference_unit)
            }
            _ => continue,
        }
    }

    Unit::UNKNOWN
}

fn collect_from_lcia_result(
    lcia_result: &Vec<LCIAResult>,
) -> (
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
) {
    let mut gwp = None;
    let mut odp = None;
    let mut ap = None;
    let mut ep = None;
    let mut pocp = None;
    let mut adpe = None;
    let mut adpf = None;

    for lcia_result in lcia_result {
        for description in &lcia_result
            .reference_to_lcia_method_dataset
            .short_description
        {
            let impact_value = Some(ImpactCategory::from(&lcia_result.other.anies));
            match &description.value {
                value if value.contains("(GWP)") || value.contains("(GWP-total)") => {
                    gwp = impact_value
                }
                value if value.contains("(ODP)") => odp = impact_value,
                value if value.contains("(AP)") => ap = impact_value,
                value if value.contains("(EP)") => ep = impact_value,
                value if value.contains("(POCP)") => pocp = impact_value,
                value if value.contains("(ADPE)") => adpe = impact_value,
                value if value.contains("(ADPF)") => adpf = impact_value,
                _ => continue,
            }
        }
    }

    (gwp, odp, ap, ep, pocp, adpe, adpf)
}

fn collect_from_exchanges(
    exchanges: &Vec<Exchange>,
) -> (
    Unit,
    Vec<Conversion>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
    Option<ImpactCategory>,
) {
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
                        _ => continue,
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

    (
        declared_unit,
        conversions,
        pere,
        perm,
        pert,
        penre,
        penrm,
        penrt,
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
        eet,
    )
}
