use generator::{DatosPersonales, DatosPersonalesFromC};
use std::os::raw::c_char;
use glib::translate::*;


/// Genera una cadena CURP desde una representación de DatosPersonalesFromC
#[no_mangle]
pub extern "C" fn generar_curp_c(datos: *const DatosPersonalesFromC) -> *mut c_char {
    if datos.is_null() {
        return std::ptr::null_mut();
    }

    let datos_personales_result = unsafe {
        let datos_c_ref = &*datos;
        DatosPersonales::from_c(datos_c_ref)
    };

    match datos_personales_result {
        Ok(dp) => {
            let curp = dp.generar_curp();
            curp.to_string()
                .to_glib_full()
        },
        Err(e) => format!("Error generando CURP: {}", e).to_glib_full(),
    }
}
