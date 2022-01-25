// cargo_audit_mod.rs

use serde::{Deserialize, Serialize};

/*
command: `cargo audit --json`
cargo_crev_reviews/samples/cargo_audit.json
cargo_crev_reviews/samples/cargo_audit_minimal.json
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct CargoAudit {
    vulnerabilities: Vulnerabilities,
    warnings: Warnings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vulnerabilities {
    list: Vec<ListItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItem {
    advisory: Advisory,
    package: Package,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Advisory {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Warnings {
    unmaintained: Option<Vec<ListItem>>,
}

pub fn run_cargo_audit() -> anyhow::Result<CargoAudit> {
    let output = std::process::Command::new("cargo").arg("audit").arg("--json").output()?;
    let ret_val = format!("{} {}", String::from_utf8(output.stdout)?, String::from_utf8(output.stderr)?);
    let cargo_audit: CargoAudit = serde_json::from_str(&ret_val)?;
    Ok(cargo_audit)
}

pub fn get_audit_id_for_crate_version(cargo_audit: &CargoAudit, crate_name: &str, crate_version: &str) -> Option<String> {
    let mut ret;
    // first find vulnerability
    ret = match cargo_audit
        .vulnerabilities
        .list
        .iter()
        .find(|&x| x.package.name.as_str() == crate_name && x.package.version.as_str() == crate_version)
    {
        None => None,
        Some(item) => Some(item.advisory.id.to_string()),
    };
    //if there is no vulnerability, then maybe a warning
    if ret.is_none() {
        match &cargo_audit.warnings.unmaintained {
            None => {
                return None;
            }
            Some(unmaintained) => {
                ret = match unmaintained
                    .iter()
                    .find(|&x| x.package.name.as_str() == crate_name && x.package.version.as_str() == crate_version)
                {
                    None => None,
                    Some(item) => Some(item.advisory.id.to_string()),
                };
            }
        }
    }
    // return
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_audit_deserialize_sample() {
        let sample = std::fs::read_to_string("samples/cargo_audit.json").unwrap();
        let cargo_audit: CargoAudit = serde_json::from_str(&sample).unwrap();
        println!("{:?}", &cargo_audit);
    }
}
