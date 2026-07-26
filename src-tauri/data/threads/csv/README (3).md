# Thread module dataset

Data version: 2026-07-26-v1

## Filer
- threads_metric.csv
- threads_unc.csv
- threads_unf.csv
- threads_bsp_g.csv
- manifest.json

## Viktige avgrensninger
Dette er et utviklingsdatasett for en CNC-kalkulator. Det er ikke en sertifisert
erstatning for ISO 68-1, ISO 228, ISO 261/262 eller ASME B1.1.

`tap_drill_mm` gjelder praktisk startverdi for skjærende tapp. Materiale,
toleranseklasse, gjengeprosent, verktøyleverandør og formtapping kan kreve en
annen diameter.

`radial_thread_depth_mm` er geometrisk grunnverdi:
- Metric/UNC/UNF: 0.541266 × pitch
- G/BSPP: publisert Whitworth-høyde, omtrent 0.640327 × pitch

G/BSP-filen inneholder bare parallell G/BSPP. R, Rp og Rc/BSPT er ikke med.

## Kilder
Metric pitch combinations:
https://fullerfasteners.com/tech/basic-metric-thread-chart-m1-m100-2/

UNC/UNF sizes and practical tap drills:
https://www.an-engineering.co.uk/unc-and-unf-unified-inch-screw-threads/

G/BSPP geometry and practical tap drills:
https://www.ring-plug-thread-gages.com/PDChart/G-series-Fine-thread-data.html

Whitworth profile formula:
https://www.engineeringtoolbox.com/G-R-Rp-Whitworth-thread-BSPP-BSPT-d_2035.html

## Forslag til backendbruk
Importer én rad per gyldig kombinasjon. Bruk `id` som stabil importnøkkel.
Frontend bør hente family -> designation -> pitch/TPI og aldri konstruere
kombinasjoner selv.
