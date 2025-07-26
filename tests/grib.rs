use tempestas::grib::GribFile;
use tempestas::grib::open;

#[test]
fn test_open() {
    let mut buffer = [0u8; 16];
    let result = open("tests/20250717_021449_GFS_P25_18.grb2", &mut buffer);

    // Ensure the result is Ok
    assert!(result.is_ok());

    // Unwrap the result to get the GribFile
    let grib_file = result.unwrap();

    // Assert the version is 2
    assert_eq!(grib_file.get_edition(), 2);

    // Assert the discipline is Meteorological
    assert_eq!(
        grib_file.get_discipline(),
        tempestas::grib::discipline::Discipline::Meteorological
    );
}
