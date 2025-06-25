# Select Component Implementation Plan

## Überblick

Die **Select-Komponente** ist eine komplexe UI-Komponente, die eine dropdown-basierte Auswahlmöglichkeit für Benutzer bietet. Sie folgt dem Radix UI Design-System und wird mit Bevy's nativen UI-Komponenten implementiert.

## Design-Prinzipien

### Radix UI Inspiration
- **Composable Architecture**: Hierarchische Struktur aus Root → Trigger → Content → Options
- **Controlled/Uncontrolled Modes**: Unterstützung für externe State-Verwaltung und interne Defaults
- **Accessibility First**: Vollständige Keyboard-Navigation (Arrow Keys, Enter, Escape, Typeahead)
- **Smart Positioning**: Automatische Dropdown-Positionierung basierend auf verfügbarem Viewport-Platz

### Bevy Integration
- **Observer Pattern**: Verwendet Bevy's neue Observer-basierte Event-Architektur
- **Portal System**: Leveraged vorhandenes Portal-System für korrekte Z-Index Überlagerung
- **Theme Integration**: Vollständige Integration in das bestehende UiColorPalette-System
- **Builder Pattern**: Konsistente fluent API wie alle anderen Komponenten

## Architektur-Übersicht

```
SelectComponent (Container)
├── SelectTrigger (Clickable Button)
│   ├── Text (Selected Value / Placeholder)
│   └── SelectIndicator (Down Arrow Icon)
└── SelectDropdownComponent (Portal-rendered)
    ├── SelectContent (Scrollable Container)
    │   ├── SelectOptionComponent (Option 1)
    │   ├── SelectOptionComponent (Option 2)
    │   └── SelectOptionComponent (Option N)
    └── Portal → UIRoot("overlay_layer")
```

## Implementation Phasen

### ✅ Phase 1: Grundstruktur (ABGESCHLOSSEN)
**Zeitaufwand**: 1 Tag  
**Status**: Vollständig implementiert ✅
**Abgeschlossen**: 2025-06-25

#### Erreichte Ziele:
- **Component Definitions**: Alle Kern-Komponenten definiert
  - `SelectComponent`: Haupt-Container mit state management
  - `SelectOptionComponent`: Einzelne Optionen mit value/label
  - `SelectTrigger`, `SelectDropdownComponent`, `SelectIndicator`: UI-Hierarchie Komponenten
  
- **Builder Pattern**: Vollständige fluent API
  ```rust
  Select::new()
      .placeholder("Choose option...")
      .size_2()
      .classic()
      .blue()
      .width(Val::Px(200.0))
      .build()
  ```

- **Theme Integration**: 
  - Size-Varianten: Size1 (32px), Size2 (40px), Size3 (48px)
  - Style-Varianten: Surface, Classic, Soft, Ghost, Outline
  - Color-Palette: Gray, Blue, Green, Red, Indigo, etc.
  - State-basierte Farben: Normal, Hover, Disabled

- **Event Definitions**: Event-Architektur vorbereitet
  - `SelectOpenEvent`: Dropdown öffnen/schließen
  - `SelectChangeEvent`: Wert-Änderung mit previous/new values

#### Technische Details:
- **Bundle-Typen**: Konkrete Tuple-Typen statt `impl Bundle` für Stabilität
- **Color System**: Integration mit `UiColorPalettesName` und Theme-Functions
- **Working Example**: Funktionierendes Demo mit allen Varianten

### ✅ Phase 2: Dropdown Mechanik (ABGESCHLOSSEN)
**Zeitaufwand**: 1 Tag  
**Status**: Vollständig implementiert ✅
**Abgeschlossen**: 2025-06-25

#### Erreichte Features:
- **Portal-basierte Dropdown-Erstellung** ✅
  - Dropdowns spawnen erfolgreich mit Portal-System
  - Korrekte Integration mit "overlay_layer" UIRoot
  - Automatische Hierarchie-Trennung für Z-Index Management
  
- **Smart Positioning System** ✅  
  - Dropdown positioniert sich automatisch unter Trigger-Element
  - GlobalTransform-basierte Koordinaten-Berechnung funktioniert
  - Dynamische Positionierung für verschiedene Select-Komponenten
  - ZIndex(1000) für Overlay-Rendering über anderen UI-Elementen

- **Click Outside Detection** ✅
  - Automatisches Schließen bei Klicks außerhalb funktioniert
  - SelectOpenEvent wird korrekt ausgelöst (open: false)
  - State-Reset funktioniert korrekt
  
- **Trigger-Interaktionen** ✅
  - Observer-Pattern für Click/Hover Events implementiert
  - Hover-Effekte (Farb-Änderungen) funktionieren
  - Toggle-Funktionalität (öffnen/schließen) arbeitet korrekt

#### Technische Implementation:
- **Portal System**: `Portal::new().container("overlay_layer").build()`
- **Observer Events**: `trigger.target()` für Entity-Referenzen  
- **Position Calculation**: `GlobalTransform.translation()` für Koordinaten
- **State Management**: `SelectState::Open/Closed` mit Event-Synchronisation
- **Sample Content**: Placeholder-Optionen ("Option 1", "Option 2", "Option 3")

### ✅ Phase 3: Radix-Style Positionierung & Z-Index (ABGESCHLOSSEN)
**Zeitaufwand**: 0.5 Tage  
**Status**: Vollständig implementiert ✅
**Abgeschlossen**: 2025-06-25

#### Erreichte Features:
- **Radix-style Dropdown Positioning** ✅
  - Berechnung des selected option index funktioniert
  - Y-Offset Berechnung: -(index * option_height) für korrekte Ausrichtung
  - Selected option bleibt auf gleicher Höhe wie Trigger (wie in Radix UI)
  - Dynamic option heights basierend auf SelectSize (Size1: 32px, Size2: 36px, Size3: 40px)

- **Selected Option Visual Feedback** ✅
  - Highlighting mit `bg_subtle` background für aktuell ausgewählte Option
  - Korrekte Option-Matching logic (value-based und label-based)
  - Option-Builder mit selected state support

- **Improved Dropdown Structure** ✅
  - Fixed option heights für konsistente Positionierung
  - Proper option entity structure mit Text children
  - Sample data mit realistischen Optionen (Apple, Orange, Grape, Carrot, Potato)

#### Aktuelle Implementierung:
```rust
// Radix-style Y-Offset Berechnung
let y_offset = if let Some(index) = selected_index {
    -(index as f32 * option_height)  // Negative offset für Aufwärts-Bewegung
} else {
    0.0  // Keine Selection = erste Option aligned
};

// Dropdown-Positionierung
top: Val::Px(y_offset),  // Relativ zum Trigger
```

- **Z-Index Layering** ✅
  - Dropdown rendert korrekt über allen UI-Elementen
  - Proper layering mit GlobalZIndex implementation
  - Keine Überlappung mit nachfolgenden UI-Komponenten

#### Technische Implementation:
```rust
// Radix-style positioning mit korrekter Z-Index Layering
GlobalZIndex(1), // Funktioniert perfekt für overlay behavior
```

### 🚧 Phase 4: Interaktion & Events (NÄCHSTE PHASE)
**Zeitaufwand**: 1 Tag  
**Status**: Bereit zur Implementierung 🚀

#### Geplante Features:
- **Option-Click Interaktionen** (Höchste Priorität)
  - Observer für SelectOptionComponent Clicks implementieren
  - SelectChangeEvent bei Option-Auswahl auslösen
  - Selected Value im SelectComponent aktualisieren
  - Dropdown automatisch schließen nach Auswahl

- **Trigger Text Updates** (Hoch)
  - Text-Child des Triggers aktualisieren wenn Wert gewählt wird
  - Placeholder → Selected Option Label Übergang
  - Theme-konforme Text-Styling beibehalten

- **Visual Feedback System** (Mittel)
  - Selected Option Highlighting (Background/Checkmark)
  - Hover-Effekte für Optionen
  - Disabled Option Support

- **Dynamic Option Loading** (Mittel)
  - Entfernung der hardcoded "Option 1/2/3" Placeholder
  - System für dynamisches Hinzufügen von Optionen zur Laufzeit
  - Option-Builder Integration mit Select

- **Improved Click Outside** (Niedrig)
  - Präzise Bounds-Detection statt globaler Click-Handler
  - Bessere Performance durch targeted Event-Handling

**Reduzierte Scope**: Keyboard Navigation wurde aus Phase 4 entfernt um den Fokus auf Kern-Funktionalität zu setzen

## Technische Entscheidungen

### Portal vs. Child-Hierarchie
**Entscheidung**: Portal-basierte Dropdown
**Begründung**: 
- Korrekte Z-Index Layering über andere UI-Elemente
- Vermeidung von Clipping durch Parent-Container
- Bessere Performance bei komplexen UI-Layouts

### Event-System: Observer vs. Legacy Events
**Entscheidung**: Observer Pattern (Bevy 0.16+)
**Begründung**:
- Konsistenz mit anderen Komponenten (Button, Radio, etc.)
- Bessere Performance und Entity-spezifische Event-Handling
- Zukunftssichere Architektur

### State Management: Component vs. Resource
**Entscheidung**: Component-based State
**Begründung**:
- Mehrere Select-Komponenten können unabhängig existieren
- Einfachere Serialisierung und Debugging
- Konsistenz mit anderen UI-Komponenten

### Color Integration: Custom vs. Theme System
**Entscheidung**: UiColorPalettesName Integration
**Begründung**:
- Konsistenz mit bestehenden Komponenten
- Automatische Light/Dark Theme-Unterstützung
- Zukunftssichere Erweiterbarkeit

## Code-Organisation

### Datei-Struktur
```
src/components/select.rs
├── Enums (SelectSize, SelectVariant, SelectState)
├── Components (SelectComponent, SelectOptionComponent, etc.)
├── Events (SelectOpenEvent, SelectChangeEvent)
├── Implementation (Color calculations, theme integration)
├── Builders (SelectBuilder, SelectOptionBuilder)
└── Convenience Structs (Select)

examples/select.rs
├── Basic Select Examples
├── Size/Variant/Color Demonstrations
├── Disabled State Examples
└── (Future: Interactive Examples)
```

### System-Integration
```rust
// In components/mod.rs Plugin
app.add_event::<select::SelectOpenEvent>()
   .add_event::<select::SelectChangeEvent>()
   .add_systems(Update, (
       // Phase 2 Systems:
       select::handle_select_trigger_interactions,
       select::spawn_select_dropdowns,
       select::position_select_dropdowns,
       select::handle_click_outside_detection,
       
       // Phase 3 Systems:
       select::handle_select_option_interactions,
       select::update_select_state,
       select::sync_trigger_with_dropdown,
   ));
```

## API Design

### Builder API
```rust
// Grundlegende Verwendung
Select::new()
    .placeholder("Select option...")
    .build()

// Mit Styling
Select::new()
    .placeholder("Choose color...")
    .size_3()           // Large
    .classic()          // Variant
    .blue()             // Color
    .width(Val::Px(200.0))
    .disabled(false)
    .build()

// Mit vorgewähltem Wert
Select::new()
    .placeholder("Choose...")
    .selected_value("option-1")
    .build()
```

### Option API (Zukunft)
```rust
// Einzelne Option
Select::option("value-1", "Display Label")
    .disabled(false)
    .build()

// Mit Select kombiniert (Phase 3+)
Select::new()
    .placeholder("Choose...")
    .build()
    .with_children(|select| {
        select.spawn(Select::option("red", "Red Color"));
        select.spawn(Select::option("green", "Green Color"));
        select.spawn(Select::option("blue", "Blue Color"));
    })
```

### Event Handling (Zukunft)
```rust
fn handle_select_changes(
    mut select_events: EventReader<SelectChangeEvent>,
) {
    for event in select_events.read() {
        println!(
            "Select {} changed from {:?} to {}",
            event.select_entity,
            event.previous_value,
            event.selected_value
        );
    }
}
```

## Nächste Schritte

### Sofortige Prioritäten (Phase 2)
1. **Portal-System Integration**
   - SelectDropdownComponent spawning
   - Portal container targeting ("overlay_layer")
   - Dropdown content generation

2. **Positioning Implementation**
   - GlobalTransform-basierte Trigger-Position-Berechnung
   - Smart above/below placement basierend auf Viewport
   - ZIndex management für proper layering

3. **Click Outside Detection**
   - System für outside-click detection
   - Automatic dropdown closing
   - State cleanup

### Mittelfristige Ziele (Phase 3)
1. **Observer-basierte Interaktionen**
2. **State Management System**
3. **Event-System Integration**
4. **Basic Keyboard Support**

### Langfristige Vision (Post-Phase 3)
1. **Advanced Keyboard Navigation**
   - Arrow key navigation
   - Typeahead search
   - Home/End key support

2. **Advanced Features**
   - Multi-select support
   - Option grouping
   - Custom option rendering
   - Search/filter functionality

3. **Performance Optimizations**
   - Virtual scrolling für lange Listen
   - Lazy option rendering
   - Optimized event handling

## Erfolgs-Kriterien

### Phase 2 Erfolgskriterien: ✅ ABGESCHLOSSEN
- [x] Dropdown öffnet sich bei Trigger-Klick
- [x] Dropdown positioniert sich korrekt unter/über Trigger  
- [x] Dropdown schließt sich bei Click Outside
- [x] Dropdown rendert über anderen UI-Elementen

### Phase 3 Erfolgskriterien: ✅ ABGESCHLOSSEN
- [x] Radix-style Positionierung implementiert (selected option aligned mit trigger)
- [x] Y-Offset Berechnung basierend auf option index funktioniert
- [x] Dynamic option heights für verschiedene SelectSize
- [x] Selected option highlighting mit bg_subtle
- [x] Z-Index Problem gelöst (dropdown über allen UI-Elementen)

### Phase 4 Erfolgskriterien:
- [ ] Optionen sind klickbar und ändern Selected Value
- [ ] SelectChangeEvent wird korrekt ausgelöst  
- [ ] Trigger zeigt gewählten Wert an
- [ ] Dropdown schließt sich automatisch nach Option-Auswahl
- [ ] Dynamic Option Loading funktioniert

### Gesamt-Erfolgskriterien:
- [ ] Select-Komponente ist vollständig funktional
- [ ] Konsistente API mit anderen Komponenten
- [ ] Performance ist akzeptabel für typische Use Cases
- [ ] Code ist gut dokumentiert und erweiterbar

## Aktueller Status (2025-06-25)

### ✅ Abgeschlossene Phasen
- **Phase 1**: Grundstruktur ✅ (Component Definitions, Builder Pattern, Theme Integration)
- **Phase 2**: Dropdown Mechanik ✅ (Portal System, Positioning, Click Outside Detection)
- **Phase 3**: Radix-Style Positionierung ✅ (Perfect positioning mit Z-Index layering)

### 🚧 Aktuelle Phase
- **Phase 4**: Interaktion & Events 🚀 (Option-Clicks, State Updates, Dynamic Content)

### 📊 Fortschritt
- **3/4 Phasen abgeschlossen** (75%)
- **Vollständige Radix-style UI** funktioniert perfekt ✅
- **Dropdown layering** perfekt gelöst ✅
- **Bereit für finale Interaktions-Phase** 🚀

---

**Erstellt**: 2025-06-25  
**Letzte Aktualisierung**: 2025-06-25 (Phase 3 abgeschlossen!)  
**Status**: Phase 3 Vollständig ✅ - Perfekte Radix-Style Positionierung mit Z-Index Layering  
**Nächster Milestone**: Phase 4 - Option-Click Interaktionen und SelectChangeEvent