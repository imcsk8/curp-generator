use generator::DatosPersonales;

pub const TEST_CURP: &str = "GOAR881103HDFNRL00";

#[test]
fn a001_valido() {
    let datos = DatosPersonales {
        nombre: "RAUL EDUARDO".to_string(),
        primer_apellido: "GONZALEZ".to_string(),
        segundo_apellido: "ARGOTE".to_string(),
        sexo: 'H',
        fecha_nacimiento: "1988-11-03".to_string(),
        entidad: "DF".to_string(),
    }; 

    let curp = datos.generar_curp();

    assert_eq!(curp,TEST_CURP);
}
