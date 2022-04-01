
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
  pub name: String,
  pub shipments: Vec<u32>
}

pub fn test_data() -> Vec<Project> {
    let mut result = Vec::new();
    
    result.push(Project { name: "1200055".into(), shipments: vec![1, 2, 3, 4, 5] });
    result.push(Project { name: "1200248".into(), shipments: vec![1, 2, 3] });
    result.push(Project { name: "1210105".into(), shipments: vec![2, 3, 4] });
    result.push(Project { name: "1210117".into(), shipments: vec![1] });

    result
}

