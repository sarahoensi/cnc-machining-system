# Domain Principles

Dette dokumentet beskriver de grunnleggende prinsippene for domenemodellering
i cnc-machining-system.

Prinsippene i dette dokumentet gjelder for **alle domain features**,
inkludert (men ikke begrenset til):
- cutting data
- hole machining
- spiral machining

Dette dokumentet er **normativt**.  
Hvis implementasjonen avviker fra disse prinsippene, er det implementasjonen som er feil.

---

## 1. Overordnet domenemodell

Domenet skiller tydelig mellom tre konseptuelle nivåer:

- **Raw Input** – data slik den kommer fra brukergrensesnittet
- **Valid Domain Input** – data som tilfredsstiller alle domeneregler
- **Solution** – et fullstendig og konsistent resultat

Disse nivåene representerer forskjellige konsepter og skal ikke blandes.

Raw Input → Valid Domain Input → Solution


Solver- og beregningslogikk opererer **kun** på valid domain input.

---

## 2. Raw Input

### Definisjon

Raw input representerer data slik den samles inn fra UI eller andre klienter.

Raw input:
- kan være ufullstendig
- kan inneholde ugyldige kombinasjoner
- kan mangle nødvendige verdier
- kan inneholde gjensidig utelukkende felt samtidig

### Bruksområde

Raw input brukes **kun** til:
- validering
- feilmeldinger
- tolkning til valid domain input

Raw input skal **aldri** brukes direkte i domeneberegninger.

---

## 3. Valid Domain Input

### Definisjon

Valid domain input representerer en domenetilstand der **alle invariants er oppfylt**.

Når valid domain input eksisterer, garanterer domenet at:
- alle nødvendige verdier finnes
- ingen ugyldige kombinasjoner er mulig
- domenelogikk kan kjøres uten defensive sjekker

### Prinsipp

Det skal være **umulig å konstruere** valid domain input som bryter domeneregler.

Domenet skal foretrekke:
- eksplisitte typer
- enums fremfor boolske flagg
- strukturer som gjør ugyldige tilstander umulige

---

## 4. Canonical Units

Alle domenemodeller opererer på **canonical units**, definert i `units.md`.

Følgende regler gjelder:
- Domenekode mottar aldri strings
- Domenekode formatterer aldri tall
- Domenekode utfører ikke implisitte enhetskonverteringer

Enhetskonvertering er kun tillatt i systemgrenser
(DTO-mapping, adaptere, API-lag).

Hvis kode må “gjette” en enhet, er modellen feil.

---

## 5. Input-moduser og eksklusivitet

Gjensidig utelukkende input skal **ikke** modelleres med:
- boolske flagg
- kombinasjoner av `optional`-felt

Slike regler skal modelleres eksplisitt som:
- input-moduser
- enums
- egne typer

Dette gjør:
- XOR-regler eksplisitte
- ugyldige kombinasjoner umulige
- domenelogikk enklere og tryggere

---

## 6. Avledede verdier

Domenet kan inneholde verdier som:
- er direkte oppgitt av bruker
- eller avledet fra andre verdier

Avledede verdier:
- beregnes eksplisitt i domenelogikken
- er deterministiske
- er aldri UI-ansvar

Frontend skal aldri:
- gjette avledede verdier
- utføre deler av domeneberegninger

---

## 7. Solution

En solution representerer et **fullstendig og konsistent** domeneresultat.

Når en solution eksisterer:
- er alle nødvendige verdier tilstede
- er ingen felt valgfrie
- er resultatet klart for presentasjon eller videre bruk

Hvis domenet ikke kan produsere en fullstendig solution,
skal det returneres en **domenefeil**, ikke en delvis løsning.

---

## 8. Validering og feil

Validering skjer i overgangen:

Raw Input → Valid Domain Input


Valideringsregler:
- alle feil rapporteres samlet
- feil knyttes til konkrete felt der det er mulig
- feil er strukturerte, ikke uformaterte strings

Domenelogikk:
- returnerer enten valid domain input / solution
- eller en eksplisitt domenefeil

---

## 9. Ansvarsfordeling

### Frontend

Frontend har ansvar for:
- innsamling av raw input
- presentasjon av feil
- presentasjon av resultater
- format, UX og interaksjon

Frontend skal aldri:
- utføre domeneberegninger
- implementere domeneregler
- håndtere XOR-logikk
- utføre enhetskonvertering

---

### Backend Domain

Domenet har ansvar for:
- definisjon av gyldige tilstander
- håndheving av invariants
- beregning av avledede verdier
- produksjon av konsistente solutions

---

## 10. Designmål

Disse prinsippene er valgt for å:
- gjøre ugyldige tilstander umulige
- redusere behov for defensiv kode
- forhindre at gamle antakelser “henger igjen”
- gjøre domenelogikken enkel å teste
- støtte videre utvidelse med nye features og klienter

---

## 11. Endringsregel

Hvis en ny feature, ny input eller ny beregning
bryter et prinsipp i dette dokumentet,
skal dokumentet oppdateres **før** koden endres.
