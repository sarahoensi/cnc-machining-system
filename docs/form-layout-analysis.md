# Oppdatert analyse av form-layout

## Kort konklusjon

Form-layouten er tydeligere enn ved forrige analyse:

- Page-layouts har et felles, typed `formWidth`-API.
- `SingleFormLayout` uttrykker nå single-form-sider uten å reservere en tom
  figure- eller aside-kolonne.
- `FormLayout` og `SplitFormLayout` har eksplisitte actions-slots som er
  kolonneorienterte og fullbredde.
- Triangle og Helix deler nå rendering av number fields gjennom
  `CalculatorNumberFields`.
- Triangle og Helix deler calculate/reset-fokus og aktivt figur-felt gjennom
  `useCalculatorFormNavigation`.
- Helix og PlanForm bruker `FormSection` i stedet for å være avhengige av
  klassenavnet direkte.
- Cutting Data bruker eksplisitte `fillHeight`- og `actionsPlacement`-props i
  stedet for feature-selectors mot layout-internals.
- Tolerances angir full høyde og split-kolonner gjennom eksplisitte props.

Den sentrale breddekontrakten er nå:

> Page-layout og `formWidth` bestemmer formkolonnens bredde. Field- og
> input-laget fyller deretter tilgjengelig bredde med `width: 100%`.

De viktigste gjenværende uklare eller tverrgående kontraktene er:

- full-height som fortsatt krever en høyde fra ancestor-kjeden
- hardkodede breakpoints i flere layout-lag
- ulik fordeling av state, calculate/reset, navigation og field-rendering
  mellom features

## Relevant filkart

### App-ramme og tokens

- `src/app/shell/AppShell.tsx`
- `src/app/shell/AppShell.css`
- `src/styles/tokens/semantic.v2.css`

### Page-layouts og bredde

- `src/shared/ui/layout/page/formWidth.ts`
- `src/shared/ui/layout/page/formWidth.css`
- `src/shared/ui/layout/page/SingleFormLayout/SingleFormLayout.tsx`
- `src/shared/ui/layout/page/SingleFormLayout/SingleFormLayout.css`
- `src/shared/ui/layout/page/FormFigureLayout/FormFigureLayout.tsx`
- `src/shared/ui/layout/page/FormFigureLayout/FormFigureLayout.css`
- `src/shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout.tsx`
- `src/shared/ui/layout/page/FormSidebarLayout/FormSidebarLayout.css`
- `src/shared/ui/layout/page/StackedLayout/StackedLayout.tsx`
- `src/shared/ui/layout/page/StackedLayout/StackedLayout.css`
- `src/shared/ui/layout/page/layouts.test.tsx`

### Form-layouts

- `src/shared/ui/layout/container/FormLayout/FormLayout.tsx`
- `src/shared/ui/layout/container/FormLayout/FormLayout.css`
- `src/shared/ui/layout/container/SplitFormLayout/SplitFormLayout.tsx`
- `src/shared/ui/layout/container/SplitFormLayout/SplitFormLayout.css`
- `src/shared/ui/layout/container/FormSection/FormSection.tsx`
- `src/shared/ui/layout/container/FormSection/FormSection.css`
- `src/shared/ui/layout/container/FormStack/FormStack.tsx`
- `src/shared/ui/layout/container/FormStack/FormStack.css`

### Field- og input-lag

- `src/shared/ui/components/form/Field/Field.tsx`
- `src/shared/ui/components/form/Field/Field.css`
- `src/shared/ui/components/form/fields/FormNumberField.tsx`
- `src/shared/ui/components/form/fields/FormTextField.tsx`
- `src/shared/ui/components/form/fields/FormSelectMenuField.tsx`
- `src/shared/ui/components/form/fields/FormModeField.tsx`
- `src/shared/ui/components/form/fields/CalculatorNumberFields.tsx`
- `src/shared/ui/primitives/input/InputBase/InputBase.tsx`
- `src/shared/ui/primitives/input/InputControl/InputControl.css`
- `src/shared/ui/primitives/input/NumberInput/NumberInput.tsx`
- `src/shared/ui/primitives/input/NumberInput/NumberInput.base.css`
- `src/shared/ui/primitives/input/TextInput/TextInput.css`
- `src/shared/ui/primitives/Select/SelectMenu.tsx`
- `src/shared/ui/primitives/Select/SelectMenu.css`

### State og navigation

- `src/app/providers/FormStateProvider.tsx`
- `src/shared/form/engine/formEngine.ts`
- `src/shared/ui/hooks/form/useFormNavigation.ts`
- `src/shared/ui/hooks/form/useCalculatorFormNavigation.ts`

### Representative features

- `src/features/right_triangle/TrianglePage.tsx`
- `src/features/right_triangle/ui/triangleFieldConfig.ts`
- `src/features/helix/ui/HelixPage.tsx`
- `src/features/helix/ui/helixFieldConfig.ts`
- `src/features/cuttingData/ui/cuttingDataPage.tsx`
- `src/features/cuttingData/ui/cuttingDataPage.css`
- `src/features/cuttingData/ui/useCuttingPageController.ts`
- `src/features/tolerances/ui/TolerancesPage.tsx`
- `src/features/tolerances/ui/TolerancesPage.css`
- `src/features/tolerances/ui/useTolerancePageController.ts`
- `src/features/cylinder_weight/ui/CylinderWeightPage.tsx`
- `src/features/cylinder_weight/ui/CylinderWeightPage.css`
- `src/features/cylinder_weight/ui/useCylinderWeightPageController.ts`
- `src/features/finishing/page/FinishingPage.tsx`
- `src/features/finishing/page/useFinishingPageController.ts`
- `src/features/finishing/plan/ui/PlanForm.tsx`

## 1. Page-layout-laget

### Felles `formWidth`

Alle tre formorienterte page-layouts tar `formWidth?: FormWidth` og bruker
`sm` som default:

```ts
type FormWidth = "sm" | "md" | "lg" | "fluid";
```

Verdiene mappes til modifier-klasser og CSS-variablene `--page-form-width` og
`--page-form-grid-width`.

| Variant | Verdi | Hensikt | Bruk i features |
| --- | --- | --- | --- |
| `sm` | `200px` | Standard calculator-form | Triangle, Helix, Cutting Data, Finishing |
| `md` | `min(20rem, 100%)` | Bredere, avgrenset single form | Cylinder Weight |
| `lg` | `clamp(17.5rem, 34vw, 19rem)` | Responsiv bred form for tett innhold | Tolerances |
| `fluid` | `100%` / `minmax(0, 1fr)` | Fyll tilgjengelig layoutplass | Kun layout-test |

Det finnes ikke lenger feature-overstyringer av `--layout-form-width`.
Page-breddene er synlige i JSX.

### `SingleFormLayout`

`SingleFormLayout` rendrer:

```text
.single-form-layout
  -> .single-form-panel
```

Den har bare én form-slot og reserverer ingen figure- eller aside-kolonne.
Wrapperen er fullbredde, sentrert og begrenset av
`--layout-content-max-width`. Panelet bruker `--page-form-width`.

Under `50rem` settes panelet til `width: 100%`. Dette betyr at både standard-
og `md`-former fyller tilgjengelig mobilbredde.

Brukes av:

- Finishing plan med default `sm`
- Cylinder Weight med `formWidth="md"`

### `FormFigureLayout`

`FormFigureLayout` rendrer alltid to slots:

```text
.form-figure-layout
  -> .form-panel
  -> .figure-panel
```

Desktop-grid:

```css
grid-template-columns: var(--page-form-grid-width) 1fr;
```

Under `50rem` blir layouten én kolonne. Triangle og Helix bruker default
`formWidth="sm"`. Ingen features bruker lenger `figure={null}`.

De generiske klassenavnene `.form-panel` og `.figure-panel` er fortsatt
globale, men ingen feature-CSS peker på dem nå.

### `FormSidebarLayout`

Default-varianten bruker:

```css
grid-template-columns: var(--page-form-grid-width) minmax(20rem, 1fr);
```

Den har typed formbredde, form-slot og semantisk `<aside>`. Den støtter også
`formClassName`, `sidebarClassName`, `className` og en `compact`-variant.

Default-varianten stacker under `50rem`. Da får sidebaren også
`min-height: 20rem`.

Compact-varianten ignorerer `--page-form-grid-width` og bruker to eksplisitte
kolonner mellom `18rem` og `22rem`. Den stacker under `760px`. Ingen av de
analyserte feature-sidene bruker compact-varianten.

Brukes av:

- Cutting Data med default `sm`
- Tolerances med `formWidth="lg"`

### App-ramme og høyde

`AppShell` bruker CSS grid for topbar/sidebar/main. `shell-main` scroller og
`shell-content` tilfører `--layout-padding`, men `shell-content` etablerer ikke
en eksplisitt full-height-kontrakt.

Cutting Data og Tolerances bruker nå eksplisitt `fillHeight` på
`FormSidebarLayout`. Propen gir root-, form- og sidebar-slot full høyde, men den
faktiske høyden avhenger fortsatt av ancestor-kjeden og innholdet.

## 2. Form-layout-laget

### `FormLayout`

`FormLayout` eier tre tydelige slots:

```text
.form-layout
  -> .form-fields
  -> .form-error-block, hvis error finnes
  -> .form-layout-actions-slot
```

- Root er en flex-kolonne med `--form-gap`.
- Fields er en flex-kolonne med `--form-fields-gap`.
- Error reserverer `--form-error-min-height` bare når error-slot rendres.
- Actions-slot er eksplisitt `display: flex`, `flex-direction: column`,
  `gap: var(--form-actions-gap)` og `width: 100%`.

Actions-slot-kontrakten er derfor nå tydelig og fullbredde. `FormActions` har
fortsatt sin egen interne `.form-actions`-klasse, men den kolliderer ikke lenger
med slot-navnet.

`actionsPlacement="bottom"` gjør `FormLayout` full-height/flex-fill og flytter
actions-slot til bunnen med `margin-top: auto`. Cutting Data bruker denne
eksplisitte varianten.

### `SplitFormLayout`

`SplitFormLayout` er en spesialisert to-kolonne-form:

```text
.split-form-layout
  -> .split-form-input-panel
     -> input
     -> error
     -> actions
  -> output
```

Den eier input/output/error/actions, og actions-slot er også eksplisitt
kolonne/fullbredde. Default er to like kolonner. Under `42rem` blir den én
kolonne.

`SplitFormLayout` har tre valgfrie props:

```tsx
<SplitFormLayout
  inputWidth="8rem"
  outputWidth="7.5rem"
  gap="var(--space-3)"
  {...slots}
/>
```

Propsene mappes internt til layoutens CSS-variabler. Når props ikke oppgis,
brukes dagens defaults med to like kolonner og `--layout-form-figure-gap`.
Tolerances bruker propsene og setter ikke lenger interne split-variabler i
feature-CSS.

### `FormSection`

`FormSection` er en flex-kolonne med `--form-section-gap`.
`variant="result"` legger til toppmargin, padding og separator. Helix,
Tolerances, Cylinder Weight og PlanForm bruker komponenten direkte.

Triangle og Cutting Data har flate field-lister og trenger ingen eksplisitt
seksjon.

### `FormStack`

`FormStack` er en enkel én-kolonne-grid med kompakt gap. Den brukes i
Cylinder Weight sin `NewMaterialModal`, ikke som page-form-layout.

### Spacing og actions

Global spacing kommer hovedsakelig fra:

- `--form-gap`
- `--form-fields-gap`
- `--form-actions-gap`
- `--form-section-gap`
- `--form-label-control-gap`
- `--layout-form-figure-gap`

`FormActions` eier intern action-layout:

- primary action fyller bredden
- Calculate-knappen fyller primary-slot
- secondary actions ligger høyrejustert under

Cutting Data flytter actions-slot til bunnen gjennom
`actionsPlacement="bottom"`, ikke gjennom feature-CSS.

## 3. Felt- og input-laget

### Hvor inputbredden bestemmes

Inputbredden bestemmes fortsatt hovedsakelig av parent-layout:

1. Page-layout velger formkolonnens bredde.
2. `FormLayout`, `FormSection` eller `SplitFormLayout` arrangerer fields.
3. `Field` og `.field-control` tillater krymping med `min-width: 0`.
4. Input-primitivene fyller tilgjengelig bredde.

Eksplisitt `width: 100%` finnes blant annet i:

- `.input-control`
- `.number-input`
- `.ni-input-wrapper`
- `.app-text-input-wrapper`
- select-options/labels der full bredde er nødvendig
- `FormActions` sin primary-slot og Calculate-knapp

`TextInput.size` endrer kontrollstørrelse, ikke total bredde.
`NumberInput` setter en inline CSS-variabel basert på enhetens tekstlengde,
men dette påvirker intern padding/enhetsplass, ikke total bredde.

### Komponentkjeden

Number field-kjeden er:

```text
Feature/page eller CalculatorNumberFields
  -> FormNumberField
    -> Field
      -> NumberInput
        -> InputBase
          -> input.input-control
```

`Field` eier label, control-wrapper og error. `FormNumberField` kobler
`FieldState`, display-formattering, lock/disabled/readonly og `NumberInput`.

`CalculatorNumberFields` er en liten renderer som:

- mapper config-rekkefølge til eksisterende `FormNumberField`
- kobler `fields`, `onChange`, refs, keydown, focus og blur
- sender field-key tilbake til callbacks
- behandler config-`readOnly` som disabled/result-field

Den henter ikke global state og bestemmer ikke layout, calculate eller
navigation.

## 4. State, rendering og navigation

### Global form-state

`FormStateProvider` eier en in-memory `Record<string, unknown>`.
`useFeatureForm(key, createInitial)` gir features et state-par og beholder
formverdier ved route-navigering.

Provideren kjenner ikke layout, men string-keyed global levetid gjør fortsatt
eierskap og reset-regler mindre synlige lokalt.

### Delt formmotor

`formEngine.ts` eier delte state-overganger som:

- user edit og constraint-anvendelse
- clearing/unlocking av fields
- calculate/generate
- field- og form-errors

Feature-pages eller controllers bestemmer fortsatt når disse funksjonene
kalles.

### Navigation

`useFormNavigation` er den generelle primitive hooken. Den eier refs,
Enter/piltast-navigation, submit-action-ref, autofocus og hjelpefunksjoner for
fokus etter render.

`useCalculatorFormNavigation` bygger på denne og brukes kun av Triangle og
Helix. Den legger til:

- fokus på første inline-error etter calculate
- fokus på første tomme input ved form-error
- fokus på første input etter reset
- valgfri active-field-sporing

Triangle og Helix setter `trackActiveField: true` for figure-highlight.
Hooken eier ikke figur-rendering eller business-logikk.

### Hva som fortsatt er lokalt

- Triangle og Helix eier calculate/reset/state-overganger i page-komponenten.
- Cutting Data, Cylinder Weight og PlanForm har fortsatt egen, nesten lik
  calculate/reset-fokuslogikk rundt `useFormNavigation`.
- Tolerances har mode- og select-spesifikk navigation.
- Controllers har forskjellige ansvar og returformer.
- Field-seksjoner, input/result-plassering og spesialfelter komponeres fortsatt
  eksplisitt per feature.

## 5. Feature-status

| Feature | Page-layout / bredde | Field-rendering | Actions | Indirekte feature-layout |
| --- | --- | --- | --- | --- |
| Triangle | `FormFigureLayout`, default `sm` | `CalculatorNumberFields`, flat liste | `FormLayout` + `FormActions` | Ingen feature-CSS mot layout-internals |
| Helix | `FormFigureLayout`, default `sm` | Mode-field + `CalculatorNumberFields` i `FormSection` | `FormLayout` + `FormActions` | Ingen feature-CSS mot layout-internals |
| Cutting Data | `FormSidebarLayout`, default `sm`, `fillHeight` | Manuell config-map til `FormNumberField` | `FormLayout actionsPlacement="bottom"` + `FormActions` | Ingen feature-CSS mot layout-internals |
| Tolerances | `FormSidebarLayout`, `lg`, `fillHeight`, med `SplitFormLayout` | Manuell input/output-rendering, mode og selects | `SplitFormLayout` + `FormActions` | Kun feature-wrapper og wrapping av secondary actions |
| Cylinder Weight | `SingleFormLayout`, `md` | Manuell input/result-filtering i `FormSection` | `FormLayout` + `FormActions` | Page-CSS styrer ikke page-layout-bredde; modal/table-regler finnes |
| Finishing plan | `SingleFormLayout`, default `sm` | Manuell mode + config-map i `FormSection` | `FormLayout` + `FormActions` | Ingen feature-CSS mot page/form-layout-internals |
| Finishing execution | `StackedLayout` | Execution-komponenter, ikke vanlig form | Footer-slot | Egen execution-layout |

### Triangle

- `useFeatureForm` eier state via global provider.
- Page eier edit/calculate/reset.
- `CalculatorNumberFields` kobler config, state og navigation.
- `useCalculatorFormNavigation` kobler error/reset-fokus og active field.
- Figuren får kun `activeField`.

### Helix

- Samme hovedmønster som Triangle.
- Mode-field rendres eksplisitt i egen `FormSection`.
- Number fields bruker delt renderer/navigation.
- Page eier fortsatt mode-change, calculate og reset.

### Cutting Data

- Controller eier form-state og saved-results-operasjoner.
- Page eier edit, calculate, reset-fokus, field-map og layout.
- Field config-rekkefølge og egen `focusOrder` er separate kontrakter.
- `fillHeight` og `actionsPlacement="bottom"` uttrykker full-height og
  bunnstilte actions eksplisitt.
- Cutting Data har ikke lenger egen layout-CSS.

### Tolerances

- Controller eier mest business- og dataorkestrering, inkludert API-options og
  saved results.
- Page eier navigation, mode-avhengig rendering og input/output-komposisjon.
- `SplitFormLayout` gir to interne kolonner inne i `FormSidebarLayout` sin
  formkolonne.
- `fillHeight` uttrykker page-layoutens høydekontrakt.
- `inputWidth`, `outputWidth` og `gap` uttrykker split-kolonnene eksplisitt.
- Feature-CSS eier kun den eksplisitte `.tolerances-form-container`-wrapperen
  og wrapping av secondary actions.

### Cylinder Weight

- Controlleren eier form-state, materialdata, API-kall og flere modal-states.
- Page rendrer materialfelt, input/result-seksjoner, navigation og modaler.
- `SingleFormLayout formWidth="md"` gjør bredden eksplisitt.
- Page-CSS inneholder fortsatt feature-layout for modal/table/search, men peker
  ikke på page-layout-internals.

### Finishing / PlanForm

- `FinishingPage` velger mellom single-form-plan og execution-view.
- Planen bruker `SingleFormLayout` med default `sm`.
- `PlanForm` eier field-rendering og navigation; controller eier form,
  execution og generate/reset.
- Setter-kontrakten bruker fortsatt `any`.
- Execution bruker `StackedLayout`, ikke vanlig form-layout.

## 6. Gjenværende skjulte koblinger

### Feature-CSS mot layout-internals

Cutting Data har ingen gjenværende feature-CSS mot layout-internals.

Tolerances peker ikke lenger på `.fsl-form`, `.fsl-sidebar` eller intern
`SplitFormLayout`-markup. Den gjenværende regelen:

```css
.tolerances-page-layout .form-actions-secondary
```

wrapper secondary actions for den smale Tolerances-formen. Dette er en reell
feature-style, men den er fortsatt koblet til `FormActions` sitt interne
klassenavn.

### CSS-variabler fra features

Det finnes ingen gjenværende feature-overstyring av page-formbredde.

Det finnes heller ingen feature-CSS som setter `SplitFormLayout` sine interne
variabler. Tolerances angir kolonnebredde og gap gjennom props.

### Hardkodede responsive grenser

- AppShell åpner/lukker sidebar ved `768px` i JavaScript.
- `SingleFormLayout`, `FormFigureLayout` og default `FormSidebarLayout`
  reagerer ved `50rem`.
- `SplitFormLayout` stacker ved `42rem`.
- Compact `FormSidebarLayout` stacker ved `760px`.

Breakpointene er ikke samlet i én kontrakt. Tolerances kan derfor få outer- og
inner-stacking ved forskjellige bredder.

### Gjenværende duplisering

- Cutting Data, Cylinder Weight og PlanForm dupliserer manuell number-field-map.
- Cutting Data, Cylinder Weight og PlanForm dupliserer inline-error,
  form-error og reset-fokus.
- Tolerances har tilsvarende, men mer feature-spesifikk fokuslogikk.
- Cylinder Weight og Tolerances filtrerer input/result-fields manuelt.
- Field config-typer er fortsatt feature-spesifikke.
- Controllers har ulikt ansvar.

Dette er ikke nødvendigvis feil. Tolerances og Cylinder Weight har nok
spesiallogikk til at de ikke bør presses inn i Triangle/Helix-abstraksjonene
uten en egen vurdering.

## 7. Oppdatert vurdering

### Hva som nå er ryddigere

1. Page-bredde er eksplisitt, typed og testet.
2. Single-form-sider uttrykkes uten tom gridkolonne.
3. Actions-slot og `FormActions` har separate navn og tydelige breddeansvar.
4. `FormSection` brukes konsistent i de analyserte seksjonerte formene.
5. Triangle og Helix deler smale abstraheringer for number-field-rendering og
   calculator-navigation.
6. De tidligere døde/misvisende page-width-kontraktene er borte.
7. Cutting Data og Tolerances bruker eksplisitte full-height-kontrakter.
8. Tolerances sine split-kolonner er synlige i JSX i stedet for feature-CSS.

### Hva som fortsatt er uklart eller skjørt

1. `fillHeight` krever fortsatt at ancestor-kjeden tilbyr en bestemt høyde.
2. Scroll-eierskap er ikke uttrykt eksplisitt.
3. Tolerances sin action-wrapping peker fortsatt på `FormActions` sitt interne
   klassenavn.
4. Navigation og controller-ansvar varierer betydelig mellom features.
5. Responsive breakpoints er hardkodet og ukoordinert.

## Anbefalt neste lille refaktoreringspakke

Neste pakke bør avgrenses til `CalculatorNumberFields` for Cutting Data:

1. Erstatt kun den manuelle config-mappingen med den eksisterende rendereren.
2. Behold Cutting Data sin egen `focusOrder` og `useFormNavigation`.
3. Behold calculate/reset, saved results, controller og layout uendret.
4. Verifiser eksplisitt at `readOnly` fortsatt behandles som disabled/result.

Dette er mindre risikabelt enn breakpoint-opprydding, som berører flere
page-layouts og responsive kontrakter samtidig. En ny samlet layout-analyse er
heller ikke nødvendig nå; rapporten og API-ene beskriver dagens status.

## Hva som bør vente

- Generell `FormGrid`/kolonne- og span-arkitektur
- Erstatning av `SplitFormLayout`
- Standardisering av alle controllers
- Migrering av Tolerances til calculator-renderer/navigation
- Sammenslåing av saved-results-panelene
- Samlet breakpoint-system
- Full opprydding av global form-state

Cutting Data sin navigation bør fortsatt vurderes separat etter en eventuell
field-renderer-migrering, fordi `focusOrder` ikke er identisk med
config-rekkefølgen.
