#[macro_use]
extern crate maplit;
use address_formatter::{Address, Component, Formatter};

#[test]
pub fn basic_test() {
    let formatter = Formatter::default();

    let mut addr = Address::default();
    addr[Component::City] = Some("Toulouse".to_owned());
    addr[Component::Country] = Some("France".to_owned());
    addr[Component::CountryCode] = Some("FR".to_owned());
    addr[Component::County] = Some("Toulouse".to_owned());
    addr[Component::HouseNumber] = Some("17".to_owned());
    addr[Component::Neighbourhood] = Some("Lafourguette".to_owned());
    addr[Component::Postcode] = Some("31000".to_owned());
    addr[Component::Road] = Some("Rue du Médecin-Colonel Calbairac".to_owned());
    addr[Component::State] = Some("Midi-Pyrénées".to_owned());
    addr[Component::Suburb] = Some("Toulouse Ouest".to_owned());

    assert_eq!(
        formatter.format(addr).unwrap(),
        r#"17 Rue du Médecin-Colonel Calbairac
31000 Toulouse
France
"#
        .to_owned()
    )
}

#[test]
pub fn easier_init_test() {
    use Component::*;
    let formatter = Formatter::default();

    let addr = Address::from_hashmap(hashmap!(
        City => "Toulouse",
        Country => "France",
        CountryCode => "FR",
        County => "Toulouse",
        HouseNumber => "17",
        Neighbourhood => "Lafourguette",
        Postcode => "31000",
        Road => "Rue du Médecin-Colonel Calbairac",
        State => "Midi-Pyrénées",
        Suburb => "Toulouse Ouest",
    ));

    assert_eq!(
        formatter.format(addr).unwrap(),
        r#"17 Rue du Médecin-Colonel Calbairac
31000 Toulouse
France
"#
        .to_owned()
    )
}

#[test]
pub fn empty_address() {
    let formatter = Formatter::default();
    let addr = Address::default();
    assert_eq!(formatter.format(addr).unwrap(), "\n".to_owned())
}

#[test]
pub fn pouet() {
    let regex = regex::RegexBuilder::new(", United Kingdom$")
        .multi_line(true)
        .build()
        .unwrap();

    let bob = r#"DG1
Seabreeze Village
British Indian Ocean Territory, United Kingdom"#;

    let res = regex.replace_all(&bob, "\nUnited Kingdom");
    assert_eq!(
        res,
        r#"DG1
Seabreeze Village
British Indian Ocean Territory
United Kingdom"#
    )
}
