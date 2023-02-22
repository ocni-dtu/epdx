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
//
// #[derive(Debug)]
// enum SubType {
//     Generic,
//     Specific,
//     Industry,
// }
//
// #[derive(Debug)]
// struct ImpactCategory {
//     a1a3: f64,
//     a4: f64,
//     a5: f64,
//     b1: f64,
//     b2: f64,
//     b3: f64,
//     b4: f64,
//     b5: f64,
//     b6: f64,
//     b7: f64,
//     c1: f64,
//     c2: f64,
//     c3: f64,
//     c4: f64,
//     d: f64,
// }
//
// #[derive(Debug)]
// struct Conversion {
//     value: f64,
//     to: String,
// }

use serde::{Deserialize, Deserializer, Serialize};


#[derive(Debug, Serialize)]
pub struct EPD {
    id: String,
    name: String,
//     declared_unit: Unit,
//     version: String,
//     published_date: String,
//     valid_until: String,
//     format_version: String,
//     source: String,
//     reference_service_life: u32,
//     standard: Standard,
//     comment: String,
//     location: String,
//     subtype: SubType,
//     conversions: Vec<Conversion>,
//     gwp: ImpactCategory,
//     odp: ImpactCategory,
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


impl<'de> Deserialize<'de> for EPD {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Outer {
            process_information: ProcessInformation,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ProcessInformation {
            data_set_information: DataSetInformation,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct DataSetInformation {
            #[serde(alias = "UUID")]
            uuid: String,
            name: DataSetName
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct DataSetName {
            base_name: Vec<ValueLang>,
        }

        #[derive(Deserialize)]
        struct ValueLang {
            value: String,
            lang: String,
        }

        let helper = Outer::deserialize(deserializer)?;
        //let default_value = ValueLang { value: "No Name", lang: "en".to_string() };
        Ok(EPD {
            id: helper.process_information.data_set_information.uuid,
            name: helper.process_information.data_set_information.name.base_name.first().unwrap().value.to_string()
        })
    }
}

pub fn parse_ilcd(json: String) -> EPD {
    let epd: EPD = serde_json::from_str(&json).unwrap();
    println!("deserialized = {:?}", epd);
    epd
}
