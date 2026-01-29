# Domain Model – Cutting Data

Dette dokumentet beskriver domenemodellen for **skjæredata (cutting data)** i cnc-machining-system.

Modellen i dette dokumentet følger prinsippene definert i
`domain-principles.md` og bruker canonical units definert i `units.md`.

Dette dokumentet er **normativt**.  
Hvis implementasjonen av cutting data avviker fra dette dokumentet, er implementasjonen feil.

---

## 1. Feature scope

Cutting data-featuret har ansvar for:
- beregning av skjæredata basert på gyldig input
- håndheving av cutting data-spesifikke regler
- produksjon av et fullstendig og konsistent sett med skjæredata

Featuret har **ingen kunnskap** om:
- UI
- presentasjon
- input-format (strings, desimaler, osv.)
- enhetskonvertering utenfor canonical units

---

## 2. Canonical units (cutting data)

Alle verdier i cutting data-domenet bruker følgende canonical units:

| Konsept             | Enhet                          |
|------               |------                          |
| Tool diameter (D)   | millimeter (mm)                |
| Cutting speed (Vc)  | meter per minutt (m/min)       |
| Spindle speed (n)   | revolutions per minute (RPM)   |
| Feed rate (F)       | millimeter per minutt (mm/min) |
| Feed per tooth (fz) | millimeter per tann (mm/tooth) |
| Number of teeth (z) | heltall                        |

Alle verdier må være større enn 0.

---

## 3. Raw Cutting Input

### Definisjon

Raw cutting input representerer data slik den samles inn fra frontend.

Raw cutting input:
- kan være ufullstendig
- kan inneholde ugyldige kombinasjoner
- kan mangle nødvendige verdier
- kan inneholde gjensidig utelukkende felt samtidig

### Bruksområde

Raw cutting input brukes **kun** til:
- validering
- generering av feilmeldinger
- konstruksjon av valid cutting input

Raw cutting input skal aldri brukes direkte i solver.

---

## 4. Valid Cutting Input

### Definisjon

Valid cutting input representerer en domene-tilstand der **alle cutting data-regler er oppfylt**.

Når valid cutting input eksisterer, garanterer domenet at:
- alle nødvendige verdier finnes
- alle verdier er gyldige (> 0)
- ingen ugyldige kombinasjoner er mulig
- solver kan kjøres uten defensive sjekker

---

### 4.1 Invariants

En gyldig cutting input garanterer:

- Tool diameter (D) er kjent og > 0
- Number of teeth (z) er kjent og > 0
- Akkurat én speed input mode er valgt
- Akkurat én feed input mode er valgt

---

## 5. Speed Input Mode

Cutting data støtter **akkurat én** av følgende hastighetsmoduser:

- **Cutting speed (Vc)**  
  Skjærehastighet oppgitt direkte av bruker

- **Spindle speed (n)**  
  Turtall oppgitt direkte av bruker

Disse to modusene er:
- gjensidig utelukkende
- obligatoriske (én må velges)
- eksplisitt modellert i domenet

Konseptuelt:

SpeedInput =
| CuttingSpeed(Vc)
| SpindleSpeed(n)


---

## 6. Feed Input Mode

Cutting data støtter **akkurat én** av følgende matemoduser:

- **Feed rate (F)**  
  Total mating per minutt

- **Feed per tooth (fz)**  
  Mating per tann

Disse to modusene er:
- gjensidig utelukkende
- obligatoriske (én må velges)
- eksplisitt modellert i domenet

Konseptuelt:

FeedInput =
| FeedRate(F)
| FeedPerTooth(fz)

---

## 7. Avledede verdier

Basert på gyldig cutting input kan domenet avlede følgende verdier:

- Spindle speed (n) avledes fra:
  - Cutting speed (Vc)
  - Tool diameter (D)

- Cutting speed (Vc) avledes fra:
  - Spindle speed (n)
  - Tool diameter (D)

- Feed rate (F) avledes fra:
  - Feed per tooth (fz)
  - Number of teeth (z)
  - Spindle speed (n)

- Feed per tooth (fz) avledes fra:
  - Feed rate (F)
  - Number of teeth (z)
  - Spindle speed (n)

Avledning:
- er deterministisk
- utføres eksplisitt i solver
- er aldri frontend-ansvar

---

## 8. Cutting Data Solution

### Definisjon

En cutting data solution representerer et **fullstendig og konsistent** sett med skjæredata.

Når solver lykkes, inneholder solution alltid:

- Tool diameter (D)
- Number of teeth (z)
- Cutting speed (Vc)
- Spindle speed (n)
- Feed rate (F)
- Feed per tooth (fz)

Ingen felt i solution er valgfrie.

Hvis domenet ikke kan produsere en fullstendig solution, returneres en domenefeil.

---

## 9. Validering og feil

Validering skjer i overgangen:

Raw Cutting Input → Valid Cutting Input


Cutting data-validering:
- håndhever invariants
- håndhever input-modusregler
- rapporterer alle relevante feil samlet
- knytter feil til konkrete felt der det er mulig

Solveren:
- returnerer enten en gyldig cutting data solution
- eller en strukturert domenefeil

---

## 10. Forhold til frontend

Frontend:
- samler raw cutting input
- presenterer valideringsfeil
- presenterer ferdig løsning

Frontend skal aldri:
- beregne skjæredata
- implementere cutting data-regler
- avgjøre hvilke verdier som kan avledes
- utføre enhetskonvertering

---

## 11. Utvidelse

Denne modellen er designet for å kunne utvides med:
- flere parametere
- flere beregningsregler
- mer avanserte machining-strategier

Utvidelser skal:
- respektere input-moduser
- bevare eksplisitte invariants
- ikke introdusere implisitte regler

---

## 12. Endringsregel

Hvis en endring i cutting data-featuret
bryter en antakelse i dette dokumentet,
skal dokumentet oppdateres **før** koden endres.
