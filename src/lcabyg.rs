use serde::{Deserialize, Serialize};

use crate::epd::{Conversion, ImpactCategory, Source, Standard, SubType, Unit, EPD};
use crate::utils::get_version;

#[derive(Deserialize, Serialize)]
pub enum Nodes {
    Node(Node),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Node {
    Stage(Stage),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Stage {
    pub id: String,
    pub name: Languages,
    pub comment: Languages,
    pub source: String,
    pub valid_to: String,
    pub stage: String,
    pub stage_unit: String,
    pub stage_factor: f64,
    pub mass_factor: f64,
    pub scale_factor: f64,
    pub external_source: String,
    pub external_id: String,
    pub external_version: String,
    pub external_url: String,
    pub compliance: String,
    pub data_type: String,
    pub indicators: StageIndicators,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Languages {
    pub english: Option<String>,
    pub german: Option<String>,
    pub norwegian: Option<String>,
    pub danish: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct StageIndicators {
    pub ser: f64,
    pub ep: f64,
    pub odp: f64,
    pub pocp: f64,
    pub per: f64,
    pub adpe: f64,
    pub ap: f64,
    pub gwp: f64,
    pub adpf: f64,
    pub penr: f64,
    pub senr: f64,
}

impl From<&Vec<Stage>> for EPD {
    fn from(stages: &Vec<Stage>) -> Self {
        let mut ep = ImpactCategory::new();
        let mut odp = ImpactCategory::new();
        let mut pocp = ImpactCategory::new();
        let mut per = ImpactCategory::new();
        let mut adpe = ImpactCategory::new();
        let mut ap = ImpactCategory::new();
        let mut gwp = ImpactCategory::new();
        let mut adpf = ImpactCategory::new();
        let mut penr = ImpactCategory::new();

        for stage in stages {
            let stage_name = if stage.stage == "A1to3" {
                "A1A3"
            } else {
                &stage.stage
            };

            ep.add(stage_name, stage.indicators.ep);
            odp.add(stage_name, stage.indicators.odp);
            pocp.add(stage_name, stage.indicators.pocp);
            per.add(stage_name, stage.indicators.per);
            adpe.add(stage_name, stage.indicators.adpe);
            ap.add(stage_name, stage.indicators.ap);
            gwp.add(stage_name, stage.indicators.gwp);
            adpf.add(stage_name, stage.indicators.adpf);
            penr.add(stage_name, stage.indicators.penr);
        }
        let node = &stages[0];
        let epd = EPD {
            id: node.id.to_string(),
            name: node.name.english.clone().unwrap(),
            declared_unit: Unit::from(&node.stage_unit),
            version: node.external_version.clone(),
            published_date: Default::default(),
            valid_until: Default::default(),
            comment: node.comment.english.clone(),
            source: Some(Source {
                name: node.external_source.clone(),
                url: Some(node.external_url.clone()),
            }),
            subtype: SubType::from(&node.data_type),
            standard: if node.compliance == "A1" {
                Standard::EN15804A1
            } else {
                Standard::EN15804A2
            },
            ep: Some(ep),
            odp: Some(odp),
            pocp: Some(pocp),
            pert: Some(per),
            adpe: Some(adpe),
            ap: Some(ap),
            gwp: Some(gwp),
            adpf: Some(adpf),
            penre: None,
            pere: None,
            penrt: Some(penr),
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
            format_version: get_version(),
            reference_service_life: None,
            location: "".to_string(),
            conversions: Some(vec![Conversion {
                to: Unit::KG,
                value: node.mass_factor,
                meta_data: "".to_string(),
            }]),
            perm: None,
            meta_data: None,
        };

        epd
    }
}
