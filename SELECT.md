Basierend auf der Research erstelle ich jetzt einen umfassenden Implementierungsplan für die Select-Komponente:

  Select Component Implementation Plan

  Architektur-Übersicht

  Die Select-Komponente wird nach dem etablierten Muster im Projekt implementiert und orientiert sich an Radix UI's compositional Ansatz:

  Kern-Komponenten:
  1. SelectComponent - Haupt-Container (analog zu RadioGroupComponent)
  2. SelectTrigger - Klickbarer Button zum Öffnen/Schließen
  3. SelectContent - Portal-basierter Dropdown-Container
  4. SelectOptionComponent - Einzelne Auswahl-Optionen
  5. SelectIndicator - Visuelle Anzeige für aktuell gewählte Option

  Phase 1: Grundstruktur (Tag 1)

  1.1 Component Definitionen
  #[derive(Component)]
  pub struct SelectComponent {
      pub open: bool,
      pub selected_value: Option<String>,
      pub placeholder: String,
      pub disabled: bool,
      pub size: SelectSize,
      pub variant: SelectVariant,
      pub color: UiColorPalette,
  }

  #[derive(Component)]
  pub struct SelectOptionComponent {
      pub value: String,
      pub label: String,
      pub disabled: bool,
      pub selected: bool,
      pub select_entity: Entity,
  }

  1.2 Builder Pattern
  - SelectBuilder mit fluent API
  - Methoden: .placeholder(), .size(), .variant(), .color(), .disabled()
  - Option-Builder: SelectOptionBuilder mit .value(), .label(), .disabled()

  1.3 Basis-Styling
  - Button-ähnlicher Trigger mit Down-Arrow Icon
  - Size-Varianten (Size1, Size2, Size3)
  - Variant-Styling (Surface, Classic, Soft, Ghost, Outline)

  Phase 2: Dropdown Mechanik (Tag 2)

  2.1 Portal-Integration
  #[derive(Component)]
  pub struct SelectDropdownComponent {
      pub select_entity: Entity,
      pub is_open: bool,
      pub positioning: DropdownPositioning,
  }

  fn spawn_select_dropdown(
      commands: &mut Commands,
      select_entity: Entity,
      select: &SelectComponent,
  ) {
      // Portal für korrektes Z-Index Rendering
      commands.spawn((
          Portal::new().container("overlay_layer").build(),
          SelectDropdownComponent { select_entity, is_open: true },
      )).with_children(|dropdown| {
          // Options hier spawnen
      });
  }

  2.2 Positionierung
  - Absolute Positionierung unter dem Trigger
  - Smart Positioning (oben/unten basierend auf Viewport)
  - Breiten-Matching mit Trigger-Element
  - ZIndex-Management für korrekte Layering

  2.3 Click Outside Detection
  - System zur Erkennung von Klicks außerhalb der Dropdown
  - Automatisches Schließen bei Click Outside

  Phase 3: Interaktion & Events (Tag 3)

  3.1 Event System
  #[derive(Event)]
  pub struct SelectOpenEvent {
      pub select_entity: Entity,
      pub open: bool,
  }

  #[derive(Event)]
  pub struct SelectChangeEvent {
      pub select_entity: Entity,
      pub selected_value: String,
      pub previous_value: Option<String>,
      pub selected_label: String,
  }

  3.2 Observer Pattern
  - Trigger-Button Observer für Open/Close
  - Option-Click Observer für Selection
  - Keyboard Navigation Observer (Arrow Keys, Enter, Escape)

  3.3 State Management
  - SelectState enum (Closed, Opening, Open, Closing)
  - Synchronisation zwischen Trigger und Dropdown
  - Selected Option Visual Feedback

  Phase 4: Keyboard Navigation (Tag 4)

  4.1 Focus Management
  #[derive(Component)]
  pub struct SelectFocus {
      pub focused_option_index: Option<usize>,
      pub total_options: usize,
  }

  4.2 Keyboard Shortcuts
  - Space/Enter: Toggle dropdown oder Select Option
  - Arrow Up/Down: Navigation zwischen Optionen
  - Escape: Dropdown schließen
  - A-Z: Typeahead-Suche zu passenden Optionen

  4.3 Accessibility
  - Focus Trapping im geöffneten Dropdown
  - ARIA-ähnliche Attribute als Component Flags
  - Keyboard-only Navigation Support

  Implementation Details

  Theme Integration:
  impl SelectComponent {
      fn calculate_trigger_colors(&self, state: InteractionState) -> (BackgroundColor, BorderColor) {
          match (self.variant, state) {
              (SelectVariant::Surface, InteractionState::Normal) => (
                  self.color.surface,
                  self.color.border
              ),
              (SelectVariant::Surface, InteractionState::Hover) => (
                  self.color.surface_hover,
                  self.color.border_hover
              ),
              // ... weitere State-Kombinationen
          }
      }
  }

  Portal-basierte Dropdown:
  fn toggle_select_dropdown(
      mut commands: Commands,
      mut select_query: Query<(Entity, &mut SelectComponent)>,
      dropdown_query: Query<Entity, With<SelectDropdownComponent>>,
      trigger: Trigger<Pointer<Click>>,
  ) {
      if let Ok((entity, mut select)) = select_query.get_mut(trigger.entity()) {
          select.open = !select.open;

          if select.open {
              spawn_select_dropdown(&mut commands, entity, &select);
          } else {
              // Dropdown despawnen
              for dropdown_entity in &dropdown_query {
                  commands.entity(dropdown_entity).despawn_recursive();
              }
          }
      }
  }

  Smart Positioning:
  fn position_select_dropdown(
      select_query: Query<(&SelectComponent, &GlobalTransform, &Node)>,
      mut dropdown_query: Query<(&mut Transform, &mut Node, &SelectDropdownComponent)>,
      window_query: Query<&Window, With<PrimaryWindow>>,
  ) {
      // Berechne Position basierend auf verfügbarem Viewport-Platz
      // Positioniere dropdown_top oder dropdown_bottom
  }

  Zeitschätzung und Priorisierung

  Tag 1: Grundstruktur + Builder Pattern (6-8h)
  Tag 2: Portal Integration + Positionierung (6-8h)Tag 3: Event System + Basic Interaction (6-8h)
  Tag 4: Keyboard Navigation + Accessibility (6-8h)

  Total: 4 Tage (entspricht der Roadmap-Schätzung)

  Nächste Schritte

  Soll ich mit der Implementierung beginnen?

  1. Phase 1 starten: Component-Definitionen und Builder Pattern
  2. Basic Example erstellen: Einfache Select mit 3-4 Optionen
  3. Schrittweise ausbauen: Portal → Events → Keyboard Navigation
