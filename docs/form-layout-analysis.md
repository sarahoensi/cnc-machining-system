# Analyse av form-layout

## Kort konklusjon

Formsystemet har allerede flere gode byggeklosser, men ansvaret er fordelt på en
måte som gjør den faktiske layouten vanskelig å lese fra én fil:

- Side-layout bestemmer normalt formområdets bredde.
- Input-primitivene bruker normalt `width: 100%` og arver derfor bredden.
- Feature-CSS overstyrer globale CSS-variabler for spesielle skjermer.
- Formskjermene blander rendering, navigation/fokus, state-overganger og
  business-orientert orkestrering.
- Flere komponenter og CSS-klasser uttrykker nesten samme konsept, mens enkelte
  skjermer bruker globale klassenavn direkte i stedet for komponentene.

Den viktigste implisitte regelen er:

> Et felt fyller bredden til nærmeste layoutkolonne. Kolonnebredden bestemmes av
> side-layout, globale tokens og eventuelle feature-overstyringer.

## Relevant filkart

### App-ramme og overordnet størrelse

- `src/app/shell/AppShell.tsx`
- `src/app/shell/AppShell.css`
- `src/styles/tokens/primitive.css`
- `src/styles/tokens/semantic.v2.css`

### Side-layouts

- `src/shared/ui/layout/page/FormFigureLayout/FormFigureLayout.tsx`
- `src/shared/ui/layout/page/FormFigureLayout/FormFigureLayout.css`
- `src/shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout.tsx`
- `src/shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout.css`
- `src/shared/ui/layout/page/StackedLayout/StackedLayout.tsx`
- `src/shared/ui/layout/page/StackedLayout/StackedLayout.css`

### Form-layouts og wrappers

- `src/shared/ui/layout/container/FormLayout/FormLayout.tsx`
- `src/shared/ui/layout/container/FormLayout/FormLayout.css`
- `src/shared/ui/layout/container/SplitFormLayout/SplitFormLayout.tsx`
- `src/shared/ui/layout/container/SplitFormLayout/SplitFormLayout.css`
- `src/shared/ui/layout/container/FormSection/FormSection.tsx`
- `src/shared/ui/layout/container/FormSection/FormSection.css`
- `src/shared/ui/layout/container/FormStack/FormStack.tsx`
- `src/shared/ui/layout/container/FormStack/FormStack.css`

### Felt og input

- `src/shared/ui/components/form/Field/Field.tsx`
- `src/shared/ui/components/form/Field/Field.css`
- `src/shared/ui/components/form/fields/FormNumberField.tsx`
- `src/shared/ui/components/form/fields/FormTextField.tsx`
- `src/shared/ui/components/form/fields/FormSelectMenuField.tsx`
- `src/shared/ui/components/form/fields/FormModeField.tsx`
- `src/shared/ui/primitives/input/InputControl/InputControl.css`
- `src/shared/ui/primitives/input/NumberInput/NumberInput.base.css`
- `src/shared/ui/primitives/input/TextInput/TextInput.css`
- `src/shared/ui/primitives/Select/SelectMenu.css`

### Representative skjermer

- `src/features/right_triangle/TrianglePage.tsx`
- `src/features/helix/ui/HelixPage.tsx`
- `src/features/cuttingData/ui/cuttingDataPage.tsx`
- `src/features/cuttingData/ui/cuttingDataPage.css`
- `src/features/tolerances/ui/TolerancesPage.tsx`
- `src/features/tolerances/ui/TolerancesPage.css`
- `src/features/cylinder_weight/ui/CylinderWeightPage.tsx`
- `src/features/cylinder_weight/ui/CylinderWeightPage.css`
- `src/features/finishing/page/FinishingPage.tsx`
- `src/features/finishing/plan/ui/PlanForm.tsx`

### State, form-logikk og navigation

- `src/app/providers/FormStateProvider.tsx`
- `src/shared/form/types/forms.ts`
- `src/shared/form/types/fields.ts`
- `src/shared/form/engine/formEngine.ts`
- `src/shared/ui/hooks/form/useFormNavigation.ts`
- feature-spesifikke `use*PageController.ts`
- feature-spesifikke `*FieldConfig.ts`

## 1. Hvor størrelsen på formområdet bestemmes

### App-rammen

`AppShell` fyller hele viewporten. Sidebarbredden kommer fra
`--sidebar-width`, mens hovedområdet tar resten av gridet. `shell-main` scroller,
og `shell-content` tilfører `--layout-padding`.

Dette betyr at tilgjengelig bredde først reduseres av:

1. sidebar
2. side-padding
3. side-layoutens `max-width`
4. formkolonnens bredde

### Standard formbredde

Standardbredden uttrykkes nå eksplisitt med `formWidth="sm"`:

```css
--page-form-grid-width: 200px;
--layout-content-max-width: 1100px;
```

`FormFigureLayout` bruker:

```css
grid-template-columns: var(--page-form-grid-width) 1fr;
max-width: var(--layout-content-max-width);
```

`FormSidebarLayout` bruker:

```css
grid-template-columns: var(--page-form-grid-width) minmax(20rem, 1fr);
max-width: var(--layout-content-max-width);
```

Dermed er standardformene i Triangle, Helix og Finishing i praksis 200 px
brede. Cutting Data bruker også standardbredden, men med saved-results-panel som
andre kolonne.

### Feature-spesifikke bredder

- Tolerances bruker `formWidth="lg"`.
- Cylinder Weight bruker `formWidth="md"`.
- Tolerances setter i tillegg egne bredder for kolonnene inni
  `SplitFormLayout`: 8 rem input og 7.5 rem output.

Breddene på page-layout-nivå er dermed synlige i komponent-props. De interne
kolonnene i `SplitFormLayout` styres fortsatt av feature-CSS.

### Høyde

`FormSidebarLayout` og enkelte feature-wrappers bruker `height: 100%` og
`min-height: 0`. Cutting Data bruker denne kjeden for å skyve actions ned med
`margin-top: auto`.

Dette er indirekte og skjørt fordi `shell-content` ikke selv etablerer en
eksplisitt høyde/flex-fill-kontrakt. Resultatet av `height: 100%` avhenger derfor
av ancestor-kjeden og innholdet.

### Responsive regler

- `FormFigureLayout`: én kolonne under `50rem`.
- `FormSidebarLayout`: én kolonne under `50rem`.
- `SplitFormLayout`: én kolonne under `42rem`.
- `FormSidebarLayout` compact-variant: én kolonne under `760px`.

Breakpointene er hardkodet i tre ulike layoutfiler. Tolerances kan derfor være
én outer-kolonne, men fortsatt to inner-kolonner mellom 42 og 50 rem.

## 2. Hvor inputbredden bestemmes

Inputbredden bestemmes nesten aldri av feltkomponenten selv.

Flyten er:

1. Side-layout bestemmer formkolonnens bredde.
2. `FormLayout`, `FormSection`, `Field` og `.field-control` lar barnet fylle
   tilgjengelig bredde.
3. `.input-control`, `.number-input`, `.ni-input-wrapper` og text-input-wrapper
   bruker `width: 100%`.
4. Inputen ender derfor med samme bredde som nærmeste layoutkolonne.

`min-width: 0` brukes flere steder for å tillate at grid/flex-barn krymper uten
overflow. Dette er riktig, men bidrar til at bredden må forstås gjennom hele
ancestor-kjeden.

### Unntak

- `TextInput` har en `size`-prop, men denne endrer kontrollhøyde/font, ikke
  bredde.
- `NumberInput` setter en inline CSS-variabel basert på enhetens tekstlengde.
  Dette endrer intern padding/enhetsplass, ikke total inputbredde.
- Søkefelt i Cylinder Weight-modaler begrenses av feature-CSS til
  `max-width: 320px`.
- Tolerances gjør inputfeltene smale gjennom nested grid-kolonner, ikke gjennom
  field- eller input-props.

## 3. Hvordan layouten bestemmes

### Dagens layoutnivåer

| Nivå | Ansvar | Teknikk |
| --- | --- | --- |
| App shell | viewport, sidebar, scrolling, padding | CSS grid + flex |
| Page layout | form + figur/sidebar eller stacked view | CSS grid/flex |
| Form layout | fields, error, actions | flex column |
| Form section | grupper og resultatseparator | flex column |
| Field | label, control, error | flex column |
| Input | fyller parent | width 100% |

### Skjermvarianter

- Triangle og Helix: `FormFigureLayout` med form + figur.
- Finishing plan: `FormFigureLayout` med `figure={null}`.
- Cylinder Weight: `FormFigureLayout` med `figure={null}` og feature-overstyrt
  formbredde.
- Cutting Data: `FormSidebarLayout` med form + saved results.
- Tolerances: `FormSidebarLayout`, med `SplitFormLayout` inni formkolonnen.
- Finishing execution: `StackedLayout`, ikke vanlig form-layout.

`figure={null}` reserverer fortsatt den andre gridkolonnen. Det brukes dermed
som en indirekte "single form"-template, men uttrykker ikke intensjonen tydelig.

### Én og flere kolonner

- Vanlige fields er alltid én kolonne.
- `FormStack` er også eksplisitt én kolonne og brukes hovedsakelig i modal.
- Tolerances er den eneste reelle tokolonne-formen, gjennom `SplitFormLayout`.
- Det finnes ingen generell field-grid med `columns`/`span`-API.

### Spacing, labels og actions

- Spacing styres hovedsakelig av globale tokens som `--form-gap`,
  `--form-fields-gap`, `--form-section-gap` og `--form-actions-gap`.
- `Field` eier label/control/error-layout.
- `FormActions` legger Calculate i full bredde og Reset/ekstra actions til
  høyre under.
- Cutting Data overstyrer action-posisjon indirekte med en descendant-selector
  og `margin-top: auto`.

### Saved results

- Cutting Data bruker den generelle `SavedResultsPanel`.
- Tolerances har et eget saved-results-panel med egen grid/tabell-CSS.
- Sidepanelets bredde kommer fra `FormSidebarLayout`, ikke panelkomponenten.
- Begge paneltypene har egen `max-height: 28rem`, selv om parent-layout prøver å
  etablere full høyde.

## 4. State- og prop-flyt

### Eier av form-state

`FormStateProvider` eier en global, in-memory map av forms keyed med strings.
`useFeatureForm` gir hver feature et state-par og gjør at formverdier beholdes
ved navigering mellom routes.

Denne state-eieren kjenner ikke layout, men den gjør forms globale og
string-keyed. Det er derfor vanskeligere å se levetid og reset-regler lokalt.

### Delt ansvar per skjerm

- Triangle og Helix eier orkestrering direkte i page-komponenten:
  form-state, calculate, reset, navigation og aktivt figur-felt.
- Cutting Data deler ansvar mellom page og controller. Controller eier saved
  results, mens page fortsatt eier calculate/edit/navigation.
- Tolerances legger mest business/UI-orkestrering i controller, men page eier
  navigation og felt-rendering.
- Cylinder Weight-controlleren eier form-state, API-data, modal-state,
  create/edit/import/export-state og actions. Page mapper hele controllerens
  store API til felter og modaler.
- Finishing deler state mellom controller og `PlanForm`, og bruker `any` for
  setter-kontrakten.

### Props gjennom field-laget

Feature page/controller lager props fra:

- field config: label, unit, tooltip, readOnly, autoFocus
- form state: value/source/locked/error
- navigation: ref og keydown-handler
- page/controller: onChange

`FormNumberField` kobler deretter `FieldState`, display settings, lock/readOnly,
formattering og primitive input sammen. Komponenten er gjenbrukbar, men er
bundet til den spesifikke `FieldState`-modellen.

### Layout-relaterte avhengigheter i state/rendering

- Read-only/result-felter filtreres og plasseres av pages, ikke av en generell
  renderer.
- Mode bestemmer hvilke felt som rendres i Tolerances.
- Aktivt felt styrer figurhighlight i Triangle/Helix.
- Fokusrekkefølge dupliserer ofte field config eller defineres separat.
- Navigation krever refs og handlers på hvert enkelt felt, noe som gjør
  rendering verbose og binder feltlisten til sidekomponenten.

## 5. Mønstre og problemer

### Dagens implisitte regler

1. Felt fyller alltid parentbredden.
2. Formbredde uttrykkes med `formWidth` på page-layouten.
3. Feature-wrapper kan overstyre delte layout-komponenter via arvede
   CSS-variabler.
4. Fields listes sekvensielt og blir én kolonne med mindre de pakkes i en
   særskilt nested layout.
5. Resultatfelt skilles gjennom config-filtering og manuell plassering.
6. Action-layout kan endres gjennom descendant-selectors fra feature-CSS.

### Duplisering

- Triangle, Helix, Cutting Data og PlanForm gjentar:
  field mapping, calculate, error-fokus, reset-fokus, error og actions.
- Field config-typene er nesten identiske, men definert per feature.
- Helix og PlanForm bruker `<div className="form-section">` direkte, mens
  Tolerances og Cylinder Weight bruker `FormSection`.
- `FormLayout` og `SplitFormLayout` dupliserer struktur for error/actions og
  tilhørende spacing.
- Saved-results-panelene har overlappende struktur og høydebegrensning.

### Tett kobling og skjulte kontrakter

- Globale, generiske klassenavn som `.form-actions`, `.form-section`,
  `.form-panel` og `.figure-panel` kan kollidere eller påvirkes utenfor eier.
- `FormLayout` og `SplitFormLayout` har nå eksplisitte actions-slot-klasser,
  mens `FormActions` beholder sin egen interne `.form-actions`-klasse.
- Helix og PlanForm bruker nå `FormSection` direkte.
- Den tidligere ubrukte `--split-form-output-min-width`-overstyringen i
  Tolerances er fjernet.
- `formClassName`, `sidebarClassName` og compact-variant finnes i
  `FormSidebarLayout`, men er ikke del av et tydelig template-system.
- Single-column-form bruker nå `SingleFormLayout` uten tom figure-kolonne.
- Full-height-atferd styres gjennom flere feature-selectors og en uklar
  ancestor-kontrakt.

### Hvorfor gjenbrukbare form-maler er vanskelige nå

- Page-layout og field-layout har ingen eksplisitt, typed variantmodell.
- Layoutvalg uttrykkes dels med komponentvalg, dels med CSS-variabler, dels med
  wrapper-klasser og descendant-selectors.
- Field config beskriver metadata, men ikke gruppering, kolonne/span eller
  result-seksjon.
- En generell renderer mangler, så hver page må koble state, navigation og
  felter manuelt.
- Controllers har ulikt ansvar og ulik returform.

## 6. Foreslått enklere arkitektur

Målet bør være fire tydelige lag:

```text
Page template
  -> Form composition
    -> Field renderer
      -> Field/control primitives

Feature controller
  -> form state + actions + business orchestration
```

### A. Eksplisitte page templates

Lag et lite, lukket API for de faktiske variantene:

```tsx
<FormPage width="sm">{form}</FormPage>

<FormPageWithAside
  formWidth="sm"
  form={form}
  aside={savedResults}
/>

<FormPageWithFigure
  formWidth="sm"
  form={form}
  figure={figure}
/>
```

`width` bør mappe til dokumenterte tokens, for eksempel `sm`, `md`, `lg`, i
stedet for at features setter interne layoutvariabler. Templates bør eie
responsive regler og høydekontrakt.

### B. Én form-composition

La én komponent eie slots for fields/error/actions:

```tsx
<FormShell
  fields={<FormGrid columns={1}>{fields}</FormGrid>}
  error={error}
  actions={actions}
/>
```

`FormGrid` kan støtte `columns={1 | 2}` og responsiv fallback. `FormSection`
brukes for semantiske grupper og resultater. Da kan `SplitFormLayout` enten
fjernes senere eller bli en ren `FormGrid`-variant.

### C. Felles field-definisjon og renderer

Utvid en felles field-definisjon med kun presentasjonsmetadata:

```ts
type FormFieldDefinition<K extends string> = {
  key: K;
  label: string;
  unit?: string;
  tooltip?: string;
  readOnly?: boolean;
  section?: "input" | "result";
  span?: 1 | 2;
};
```

En `NumberFieldList`/`FormFieldRenderer` kan koble config, field state,
navigation og `onChange`. Feature-spesielle felter som Material og tolerance
selects forblir eksplisitte children.

### D. Konsistent controller-kontrakt

Controllers bør eie:

- form state
- field/mode changes
- calculate/generate/reset
- feature-data og sideeffekter

Pages bør eie:

- valg av page template
- komposisjon av form, figur og aside
- eventuelt rent visuelt aktivt felt

Navigation kan samles i en gjenbrukbar `useCalculatorFormNavigation` som tar
field definitions og resultatet fra calculate. Det reduserer duplisert
fokuslogikk uten å blande den inn i business-controlleren.

## 7. Små refaktoreringssteg i trygg rekkefølge

### Steg 1: Dokumenter og navngi eksisterende bredder

Introduser semantiske tokens som:

```css
--form-page-width-sm: 12.5rem;
--form-page-width-md: 20rem;
--page-content-max-width: 68.75rem;
```

Map dagens verdier til disse uten visuell endring. Dette gjør det mulig å finne
alle størrelsesvalg.

### Steg 2: Fjern tvetydige globale klassenavn

Gi slot-wrapperne egne navn, for eksempel:

- `.form-layout-actions-slot`
- `.form-layout-fields-slot`
- `.form-section`

Start med `.form-actions`, fordi den i dag brukes både av `FormLayout` og
`FormActions`. Dette er en liten CSS/markup-endring med lav funksjonell risiko.

### Steg 3: Bruk `FormSection` konsekvent

Erstatt direkte `<div className="form-section">` i Helix og PlanForm med
`FormSection`. Dette fjerner en skjult global CSS-avhengighet uten å endre
layout.

### Steg 4: Lag en ekte single-form page-layout

Erstatt bruk av `FormFigureLayout` med `figure={null}` i Finishing og Cylinder
Weight med en `FormPage`/`SingleFormLayout`. Dette fjerner tom gridkolonne og
gjør intensjonen eksplisitt.

### Steg 5: Samle page-width-varianter i props

La page-layouts ta `formWidth="sm" | "md"` og mappe dette til modifier-klasser.
Flytt deretter Tolerances- og Cylinder Weight-overstyringene fra feature-CSS til
det eksplisitte API-et.

### Steg 6: Etabler én høyde- og scroll-kontrakt

Bestem om siden eller sidepanelet skal scrolle. Gjør `shell-content` og page
template ansvarlige for dette. Fjern deretter feature-spesifikke kjeder av
`height: 100%` og descendant-selectors gradvis.

### Steg 7: Ekstraher felles field renderer

Start med Triangle og Helix, siden de har enkel og lik mapping. Behold
calculate/state-logikk uendret. Renderer bør først kun redusere JSX-duplisering.

### Steg 8: Ekstraher felles calculator-navigation

Samle gjentatt fokus etter calculate/reset. Gjør dette etter field renderer, så
navigation kan baseres på samme field definitions.

### Steg 9: Standardiser controllers

Flytt calculate/edit/reset fra de enkle pages til små controllers én feature om
gangen. Ikke flytt layout inn i controllerne.

### Steg 10: Generaliser to-kolonne-form

Når én-kolonne-rendering og templates er stabile, erstatt Tolerances sin
spesialtilpassede `SplitFormLayout` med et generelt responsivt `FormGrid`.

## Anbefalt første implementeringspakke

Den første pakken bør kun:

1. bytte Helix og PlanForm til `FormSection`
2. gi `FormLayout` sin actions-slot et unikt klassenavn
3. legge til en eksplisitt single-form page-layout og bruke den i Finishing og
   Cylinder Weight
4. legge til enkle layout-tester eller Storybook-lignende render-tester for
   standard, sidebar og responsiv stacking

Dette angriper de mest skjulte kontraktene uten å endre form-state,
beregningslogikk eller field-komponentenes offentlige API.
