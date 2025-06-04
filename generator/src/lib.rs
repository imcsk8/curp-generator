use std::ffi::{CStr, CString};
use std::os::raw::c_char;


/// Representa los datos que componen la CURP
/// https://sre.gob.mx/component/phocadownload/category/2-marco-normativo?download=1116:instructivo-normativo-para-la-asignacion-de-la-clave-unica-de-registro-de-poblacion-dof-18-10-2021-texto-vigente
#[derive(Debug)]
pub struct DatosPersonales {
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub sexo: char, // 'H' o 'M'
    pub fecha_nacimiento: String, // "AAAA-MM-DD"
    pub entidad: String,
}


/// Representación en tipos de datos del lenguaje C de `generator::DatosPersonales`
#[repr(C)]
pub struct DatosPersonalesFromC {
    pub nombre: *const c_char,
    pub primer_apellido: *const c_char,
    pub segundo_apellido: *const c_char,
    pub sexo: c_char, // 'H' o 'M'
    pub fecha_nacimiento: *const c_char, // "AAAA-MM-DD"
    pub entidad: *const c_char,
}


/// Funciones para manejar la CURP
impl DatosPersonales {

    /// Convierte la estructura proveniente del tipo de llamada del lenguaje C
    /// a la estructura segura de Rust: DatosPersonales
    pub fn from_c(datos_c: &DatosPersonalesFromC) -> Result<Self, std::str::Utf8Error> {
        unsafe {
            Ok(DatosPersonales {
                nombre: CStr::from_ptr(datos_c.nombre).to_str()?.to_owned(),
                primer_apellido: CStr::from_ptr(datos_c.primer_apellido).to_str()?.to_owned(),
                segundo_apellido: CStr::from_ptr(datos_c.segundo_apellido).to_str()?.to_owned(),
                sexo: (datos_c.sexo as u8 as char),
                fecha_nacimiento: CStr::from_ptr(datos_c.fecha_nacimiento).to_str()?.to_owned(),
                entidad: CStr::from_ptr(datos_c.entidad).to_str()?.to_owned(),
            })
        }
    }


    /// Genera un CURP a partir de los datos personales
    pub fn generar_curp(&self) -> String {
        let mut curp = String::new();

        let inicial1 = self.primer_apellido.chars().next().unwrap_or('X');
        let vocal_interna = self.primer_apellido.chars().skip(1).find(|c| "AEIOUaeiou".contains(*c)).unwrap_or('X');
        let inicial2 = self.segundo_apellido.chars().next().unwrap_or('X');
        let inicial3 = self.nombre.chars().next().unwrap_or('X');

        curp.push(inicial1);
        curp.push(vocal_interna);
        curp.push(inicial2);
        curp.push(inicial3);

        let partes: Vec<&str> = self.fecha_nacimiento.split('-').collect();
        if partes.len() == 3 {
            curp.push_str(&partes[0][2..4]); // AA
            curp.push_str(partes[1]);        // MM
            curp.push_str(partes[2]);        // DD
        } else {
            curp.push_str("000000");
        }

        curp.push(self.sexo.into());
        curp.push_str(&self.entidad.to_uppercase());

        curp.push(extraer_consonante_interna(&self.primer_apellido));
        curp.push(extraer_consonante_interna(&self.segundo_apellido));
        curp.push(extraer_consonante_interna(&self.nombre));

        let siglo = partes[0].parse::<i32>().unwrap_or(1900);
        curp.push(if siglo < 2000 { '0' } else { 'A' });

        curp.push('0');

        curp.to_uppercase()
    }
}


/// Extrae la primera consonante interna de una cadena
fn extraer_consonante_interna(cadena: &str) -> char {
    cadena.chars()
        .skip(1)
        .find(|c| !"AEIOUaeiou".contains(*c) && c.is_alphabetic())
        .unwrap_or('X')
}
