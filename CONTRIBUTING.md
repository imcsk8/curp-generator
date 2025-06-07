# Guía de Contribución

¡Gracias por tu interés en contribuir a este proyecto!

## Licencia

Al enviar código a este repositorio, aceptas que tu contribución será licenciada bajo los términos de la [Licencia Apache 2.0](LICENSE).

Esto incluye:

- Otorgar al propietario del repositorio y a los usuarios una licencia para usar, modificar y distribuir tu contribución.
- Conceder una licencia de cualquier patente que se aplique a tu contribución, como se describe en la cláusula de la Licencia Apache 2.0.

## Cómo contribuir

1. **Clona el repositorio**:
   ```bash
   git clone https://github.com/rafex/curp-generator.git
   cd curp-generator
   ```
2. **Sincroniza ramas principales**:
   ```bash
   git fetch origin
   git checkout develop
   git pull origin develop
   ```
3. **Crea una rama de feature o hotfix**:
   - Para nuevas funcionalidades:
     ```bash
     git checkout -b feature/mi-nueva-funcionalidad develop
     ```
   - Para correcciones urgentes:
     ```bash
     git checkout -b hotfix/descripcion-correccion main
     ```
4. **Realiza cambios** en tu rama. Asegúrate de:
   - Cumplir el estilo de código del proyecto.
   - Agregar pruebas o ejemplos si corresponde.
5. **Confirma y sube tus commits**:
   ```bash
   git add .
   git commit -m "feature: descripción breve"
   git push origin nombre-de-tu-rama
   ```
6. **Abre un Pull Request**:
   - Dirige el PR hacia `develop` (o `main` si es un hotfix).
   - Describe los cambios y referencia issues relacionados.
7. **Espera revisiones**:
   - Solicita al menos 1–2 aprobaciones.
   - Resuelve todas las conversaciones.
   - Asegúrate de que los checks de CI pasen.
8. **Merge y etiquetas**:
   - Una vez aprobado, integra tu rama usando el merge method permitido (squash o rebase).
   - Para releases, crea un tag en `main`:
     ```bash
     git checkout main
     git pull origin main
     git tag vX.Y.Z
     git push origin main --tags
     ```

## Buenas prácticas

- Sigue el estilo de código ya presente en el proyecto.
- Incluye comentarios explicativos si el cambio es complejo.
- Si agregas una nueva funcionalidad, considera incluir ejemplos o pruebas.

## Código de Conducta

Este proyecto sigue un entorno de colaboración respetuoso. No se tolerará contenido ofensivo, discriminación o comportamiento abusivo.

---

## Flujo de trabajo

Este proyecto utiliza **Gitflow** como modelo de gestión de ramas. Las ramas principales son:

- `main`: código de producción.
- `develop`: integración de nuevas funcionalidades.
- `feature/...`: desarrollo de nuevas características.
- `release/...`: preparación de versiones.
- `hotfix/...`: corrección de errores en producción.

Además, se utiliza un sistema de etiquetas (*tags*) para versionado, siguiendo [Semantic Versioning (SemVer)](https://semver.org/). Cada versión se etiqueta como `vX.Y.Z` (por ejemplo, `v1.2.0`).

```
               Gitflow
╔══════════╗     ╔══════════╗
║  Feature ║──-─▶║  Develop ║
╚══════════╝    ╱╚══════════╝
               ╱     │
╔══════════╗   ╱     ▼
║  Hotfix  ║◀─┘   ╔══════╗    ╔══════╗
╚══════════╝      ║ Main ║-──▶║ Tags ║
                  ╚══════╝    ╚══════╝
```

Si tienes dudas, abre un Issue o escribe al mantenedor del proyecto.
