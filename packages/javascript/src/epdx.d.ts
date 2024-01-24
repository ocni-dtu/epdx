/* tslint:disable */
/* eslint-disable */
/**
* @param {string} json
* @returns {EPD}
*/
export function convertIlcd(json: string): EPD;
export interface Conversion {
    value: number;
    to: Unit;
}

export interface ImpactCategory {
    a1a3: number | null;
    a4: number | null;
    a5: number | null;
    b1: number | null;
    b2: number | null;
    b3: number | null;
    b4: number | null;
    b5: number | null;
    b6: number | null;
    b7: number | null;
    c1: number | null;
    c2: number | null;
    c3: number | null;
    c4: number | null;
    d: number | null;
}

export type SubType = "Generic" | "Specific" | "Industry" | "Representative";

export type Standard = "EN15804A1" | "EN15804A2" | "UNKNOWN";

export interface Source {
    name: string;
    url: string | null;
}

export type Unit = "M" | "M2" | "M3" | "KG" | "TONES" | "PCS" | "L" | "M2R1" | "UNKNOWN";

export interface EPD {
    id: string;
    name: string;
    declared_unit: Unit;
    version: string;
    published_date: DateTime<Utc>;
    valid_until: DateTime<Utc>;
    format_version: string;
    source: Source | null;
    reference_service_life: number | null;
    standard: Standard;
    comment: string | null;
    location: string;
    subtype: SubType;
    conversions: Conversion[] | null;
    gwp: ImpactCategory | null;
    odp: ImpactCategory | null;
    ap: ImpactCategory | null;
    ep: ImpactCategory | null;
    pocp: ImpactCategory | null;
    adpe: ImpactCategory | null;
    adpf: ImpactCategory | null;
    penre: ImpactCategory | null;
    pere: ImpactCategory | null;
    perm: ImpactCategory | null;
    pert: ImpactCategory | null;
    penrt: ImpactCategory | null;
    penrm: ImpactCategory | null;
    sm: ImpactCategory | null;
    rsf: ImpactCategory | null;
    nrsf: ImpactCategory | null;
    fw: ImpactCategory | null;
    hwd: ImpactCategory | null;
    nhwd: ImpactCategory | null;
    rwd: ImpactCategory | null;
    cru: ImpactCategory | null;
    mrf: ImpactCategory | null;
    mer: ImpactCategory | null;
    eee: ImpactCategory | null;
    eet: ImpactCategory | null;
    meta_data: Record<string, string> | null;
}

