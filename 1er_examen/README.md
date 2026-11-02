### Primer examen: triángulo de área máxima

Implementación en Rust de dos soluciones para encontrar los tres postes que forman el triángulo de mayor área: una por fuerza bruta y otra optimizada mediante envolvente convexa.

### Archivos

- `fuerza_bruta.rs`: evalúa todas las combinaciones de tres postes.
- `optimizado.rs`: construye la envolvente convexa y busca el triángulo entre sus vértices.
- `campo.in`: coordenadas de entrada.
- `campo.out`: coordenadas de los tres postes seleccionados.

### Requisitos

Tener Rust instalado. Puedes comprobarlo desde la terminal:

```bash
rustc --version
```

### Datos de entrada

El archivo `campo.in` debe contener una pareja de coordenadas enteras por línea y terminar con `-1 -1`. Se admiten hasta 500 postes, con coordenadas entre 0 y 999.

### Ejecución en Linux Fedora

Abre una terminal dentro de la carpeta `1er_examen`, donde se encuentran los programas y `campo.in`.

#### Fuerza bruta

```bash
rustc -O fuerza_bruta.rs -o fuerza_bruta
./fuerza_bruta
```

#### Solución optimizada

```bash
rustc -O optimizado.rs -o optimizado
./optimizado
```

La opción `-O` activa las optimizaciones del compilador.

### Resultados

Ambos programas muestran en la terminal las coordenadas del triángulo encontrado, su área, el tiempo del algoritmo y el tiempo total de ejecución. La versión optimizada también muestra cuántos vértices tiene la envolvente convexa.

El archivo `campo.out` contiene únicamente las tres parejas de coordenadas. Para consultarlo:

```bash
cat campo.out
```

**Cada ejecución reemplaza el contenido de `campo.out`.** Los tiempos pueden variar entre ejecuciones y equipos.
