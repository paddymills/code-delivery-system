
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct JobData {
  pub webs: Option<Vec<WebFlangeData>>,
  pub flanges: Option<Vec<WebFlangeData>>,
  pub parts: Option<Vec<PartData>>
}

#[derive(Serialize, Deserialize)]
pub struct WebFlangeData {
    matl: String,
    matl_desc: String,
    grade: String,
    program: String,
    checked: Option<String>,
    printed: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct PartData {
    grade: String,
    thk: f32,
    sheet: String,
    program: String,
    printed: Option<String>
}

impl JobData {
    pub fn test_data(job: String) -> Self {
        let (webs, flanges) = match job.as_str() {
            "1210117" => (None, None),
            _ => {
                (
                    // webs
                    Some(vec![
                        WebFlangeData {
                            matl: String::from(&job) + "A01-03001",
                            matl_desc: "PL 5/8 x 99 x 63'-10".into(),
                            grade: "A709-50WT2".into(),
                            program: "50444".into(),
                            checked: None,
                            printed: None
                        },
                        WebFlangeData {
                            matl: String::from(&job) + "A01-03002",
                            matl_desc: "PL 5/8 x 122 x 44'-0".into(),
                            grade: "A709-HPS50WT2".into(),
                            program: "50445".into(),
                            checked: Some("dpainter".into()),
                            printed: None
                        },
                        WebFlangeData {
                            matl: String::from(&job) + "A01-03003",
                            matl_desc: "PL 3/4 x 122 x 63'-10".into(),
                            grade: "A709-50WT2".into(),
                            program: "50446".into(),
                            checked: Some("dpainter".into()),
                            printed: Some("aphillips".into())
                        }
                    ]),

                    // flanges
                    Some(vec![
                        WebFlangeData {
                            matl: String::from(&job) + "A01-04001",
                            matl_desc: "PL 1 1/2 x 99 x 63'-10".into(),
                            grade: "A709-50WT2".into(),
                            program: "55001".into(),
                            checked: None,
                            printed: None
                        },
                        WebFlangeData {
                            matl: String::from(&job) + "A01-04002",
                            matl_desc: "PL 2 x 122 x 44'-0".into(),
                            grade: "A709-HPS50WT2".into(),
                            program: "55002".into(),
                            checked: Some("dpainter".into()),
                            printed: None
                        },
                        WebFlangeData {
                            matl: String::from(&job) + "A01-04003",
                            matl_desc: "PL 1 1/2 x 122 x 63'-10".into(),
                            grade: "A709-50WT2".into(),
                            program: "55003".into(),
                            checked: Some("dpainter".into()),
                            printed: Some("aphillips".into())
                        }
                    ])
                )

            }
        };

        let parts = match job.as_str() {
            "1210105" => None,
            _ => Some(vec![
                PartData {
                    grade: "50/50WT2".into(),
                    thk: 0.625,
                    sheet: "S11052".into(), 
                    program: "50428".into(),
                    printed: None
                },
                PartData {
                    grade: "A709-50WT2".into(),
                    thk: 1.25,
                    sheet: "S99905".into(), 
                    program: "50429".into(),
                    printed: Some("lreese".into())
                },
                PartData {
                    grade: "50/50WT2".into(),
                    thk: 1.0,
                    sheet: "S11054".into(), 
                    program: "50430".into(),
                    printed: Some("kheiser".into())
                },
            ])
        };

        Self {
            webs: webs,
            flanges: flanges,
            parts: parts
        }
    }
}