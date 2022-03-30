
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct JobData {
  pub webs: Option<Vec<NestData>>,
  pub flanges: Option<Vec<NestData>>,
  pub parts: Option<Vec<NestData>>
}

#[derive(Serialize, Deserialize, Default)]
pub struct NestData {
    parts: Vec<Part>,
    sheet: Option<Sheet>,
    program: Option<String>,
    checked: Option<String>,
    printed: Option<String>
}

#[derive(Serialize, Deserialize, Default)]
pub struct Sheet {
    name: Option<String>,
    matl: String,
    matl_desc: Option<String>,
    thk: f32,
    grade: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Part {
    name: String,
    qty: u32
}

impl JobData {
    pub fn test_data(job: String) -> Self {
        let (webs, flanges) = match job.as_str() {
            "1210117" => (None, None),
            _ => {
                (
                    // webs
                    Some(vec![
                        NestData {
                            sheet: Some(Sheet {
                                matl: String::from(&job) + "A01-03001",
                                matl_desc: Some("PL 5/8 x 99 x 63'-10".into()),
                                grade: Some("A709-50WT2".into()),

                                ..Default::default()
                            }),

                            program: Some("50444".into()),
                            checked: None,
                            printed: None,

                            ..Default::default()
                        },
                        NestData {
                            sheet: Some(Sheet {
                                matl: String::from(&job) + "A01-03002",
                                matl_desc: Some("PL 5/8 x 122 x 44'-0".into()),
                                grade: Some("A709-HPS50WT2".into()),

                                ..Default::default()
                            }),

                            program: Some("50445".into()),
                            checked: Some("dpainter".into()),
                            printed: None,

                            ..Default::default()
                        },
                        NestData {
                            sheet: Some(Sheet {
                                matl: String::from(&job) + "A01-03003",
                                matl_desc: Some("PL 3/4 x 122 x 63'-10".into()),
                                grade: Some("A709-50WT2".into()),

                                ..Default::default()
                            }),

                            program: Some("50446".into()),
                            checked: Some("dpainter".into()),
                            printed: Some("aphillips".into()),

                            ..Default::default()
                        }
                    ]),

                    // flanges
                    Some(vec![
                        NestData {
                            sheet: Some(Sheet {
                                matl: String::from(&job) + "A01-04001",
                                matl_desc: Some("PL 1 1/2 x 99 x 63'-10".into()),
                                grade: Some("A709-50WT2".into()),

                                ..Default::default()
                            }),

                            program: Some("55001".into()),
                            checked: None,
                            printed: None,

                            ..Default::default()
                        },
                        NestData {
                            sheet: Some(Sheet {
                                matl: String::from(&job) + "A01-04002",
                                matl_desc: Some("PL 2 x 122 x 44'-0".into()),
                                grade: Some("A709-HPS50WT2".into()),

                                ..Default::default()
                            }),

                            program: Some("55002".into()),
                            checked: Some("dpainter".into()),
                            printed: None,

                            ..Default::default()
                        },
                        NestData {
                            sheet: Some(Sheet {
                                matl: String::from(&job) + "A01-04003",
                                matl_desc: Some("PL 1 1/2 x 122 x 63'-10".into()),
                                grade: Some("A709-50WT2".into()),

                                ..Default::default()
                            }),
                            
                            program: Some("55003".into()),
                            checked: Some("dpainter".into()),
                            printed: Some("aphillips".into()),

                            ..Default::default()
                        }
                    ])
                )

            }
        };

        let parts = match job.as_str() {
            "1210105" => None,
            _ => Some(vec![
                NestData {
                    sheet: Some(Sheet {
                        name: Some("S11052".into()),
                        grade: Some("50/50WT2".into()),
                        thk: 0.625,

                        ..Default::default()
                    }),

                    program: Some("50428".into()),
                    printed: None,

                    ..Default::default()
                },
                NestData {
                    sheet: Some(Sheet {
                        name: Some("S99905".into()),
                        grade: Some("A709-50WT2".into()),
                        thk: 1.25,

                        ..Default::default()
                    }),

                    program: Some("50429".into()),
                    printed: Some("lreese".into()),

                    ..Default::default()
                },
                NestData {
                    sheet: Some(Sheet {
                        name: Some("S11054".into()),
                        grade: Some("50/50WT2".into()),
                        thk: 1.0,

                        ..Default::default()
                    }),

                    program: Some("50430".into()),
                    printed: Some("kheiser".into()),

                    ..Default::default()
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