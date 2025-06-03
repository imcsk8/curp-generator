pub mod logic {
    use std::os::raw::c_char;

    /// Esta constante representa el "Cat·logo de Palabras Inconvenientes"
    ///
    /// El cual forma parte del Anexo 2.
    ///
    /// # Referencias
    ///
    /// - INSTRUCTIVO NORMATIVO PARA LA ASIGNACIÓN DE LA CLAVE ÚNICA DE REGISTRO DE POBLACIÓN:
    ///   [ver pág. 59](https://www.ordenjuridico.gob.mx/Federal/PE/APF/APC/SEGOB/Instructivos/InstructivoNormativo.pdf)
    const PALABRAS_INCONVENIENTES: &[&str] = &[
        "BACA", "BAKA", "BUEI", "BUEY", "CACA", "CACO", "CAGA", "CAGO", "CAKA", "CAKO", "COGE",
        "COGI", "COJA", "COJE", "COJI", "COJO", "COLA", "CULO", "FALO", "FETO", "GETA", "GUEI",
        "GUEY", "JETA", "JOTO", "KACA", "KACO", "KAGA", "KAGO", "KAKA", "KAKO", "KOGE", "KOGI",
        "KOJA", "KOJE", "KOJI", "KOJO", "KOLA", "KULO", "LILO", "LOCA", "LOCO", "LOKA", "LOKO",
        "MALA", "MALO", "MAME", "MAMO", "MEAR", "MEAS", "MEON", "MIAR", "MION", "MOCO", "MOKO",
        "MULA", "MULO", "NACA", "NACO", "PEDA", "PEDO", "PENE", "PIPI", "PITO", "POPO", "PUTA",
        "PUTO", "QULO", "RATA", "ROBA", "ROBE", "ROBO", "RUIN", "SENO", "TETA", "VACA", "VAGA",
        "VAGO", "VAKA", "VUEI", "VUEY", "WUEI", "WUEY",
    ];

    #[repr(C)]
    pub struct DatosPersonales {
        pub nombre: *const c_char,
        pub primer_apellido: *const c_char,
        pub segundo_apellido: *const c_char,
        pub sexo: c_char, // 'H' o 'M'
        pub fecha_nacimiento: *const c_char, // "AAAA-MM-DD"
        pub entidad: *const c_char,
    }

    pub fn generar_curp(nombre: &str, primer_apellido: &str, segundo_apellido: &str, sexo: char, fecha_nacimiento: &str, entidad: &str) -> String {
        let mut curp = String::new();

        let inicial1 = primer_apellido.chars().next().unwrap_or('X');
        let vocal_interna = primer_apellido.chars().skip(1).find(|c| "AEIOUaeiou".contains(*c)).unwrap_or('X');
        let inicial2 = segundo_apellido.chars().next().unwrap_or('X');
        let inicial3 = nombre.chars().next().unwrap_or('X');

        curp.push(inicial1);
        curp.push(vocal_interna);
        curp.push(inicial2);
        curp.push(inicial3);

        let partes: Vec<&str> = fecha_nacimiento.split('-').collect();
        if partes.len() == 3 {
            curp.push_str(&partes[0][2..4]); // AA
            curp.push_str(partes[1]);        // MM
            curp.push_str(partes[2]);        // DD
        } else {
            curp.push_str("000000");
        }

        curp.push(sexo);
        curp.push_str(&entidad.to_uppercase());

        curp.push(extraer_consonante_interna(primer_apellido));
        curp.push(extraer_consonante_interna(segundo_apellido));
        curp.push(extraer_consonante_interna(nombre));

        let siglo = partes[0].parse::<i32>().unwrap_or(1900);
        curp.push(if siglo < 2000 { '0' } else { 'A' });

        curp.push('0');

        // Convertir a mayúsculas antes de filtrar
        let mut curp_mayus = curp.to_uppercase();

        // Aplicar filtro de palabras inconvenientes
        filtrar_palabra_inconveniente(&mut curp_mayus);

        curp_mayus
    }

    /// Filtra palabras inconvenientes.
    ///
    /// # Cita:
    ///
    /// En la estructura de la CURP (posiciones 1-4) que en ocasiones forma una
    /// palabra cuya pronunciación se considera ofensiva para los patrones socialmente
    /// establecidos, en cuyo caso la letra de la segunda posición se sustituye por una 'X'.
    ///
    /// # Referencias
    ///
    /// - INSTRUCTIVO NORMATIVO PARA LA ASIGNACIÓN DE LA CLAVE ÚNICA DE REGISTRO DE POBLACIÓN:
    ///   [ver pág. 59](https://www.ordenjuridico.gob.mx/Federal/PE/APF/APC/SEGOB/Instructivos/InstructivoNormativo.pdf)
    ///
    /// # Ejemplo
    /// ```
    /// let curp = filtrar_palabra_inconveniente("PENE660720HDFNTN00");
    /// assert_eq!(curp, "PXNE660720HDFNTN00");
    /// ```
    fn filtrar_palabra_inconveniente(curp: &mut String) {
        if curp.len() < 4 {
            return;
        }
        let palabra: String = curp.chars().take(4).collect();
        if PALABRAS_INCONVENIENTES.contains(&palabra.as_str()) {
            let mut curp_parts: Vec<char> = curp.chars().collect();
            if curp_parts.len() > 1 {
                curp_parts[1] = 'X';
                *curp = curp_parts.into_iter().collect();
            }
        }
    }

    fn extraer_consonante_interna(cadena: &str) -> char {
        cadena.chars()
            .skip(1)
            .find(|c| !"AEIOUaeiou".contains(*c) && c.is_alphabetic())
            .unwrap_or('X')
    }
}
