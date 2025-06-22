# Bevy UI Component Roadmap

Based on Radix UI primitives and themes, this roadmap outlines the implementation plan for a comprehensive Bevy UI library with 79 total components.

## Project Overview

- **Total Components**: 79 (30 Primitives + 49 Themes)
- **Started**: 2025-06-20
- **Estimated Completion**: 2025-12-20 (6 months)
- **Focus**: Accessibility-first, builder pattern, theme integration

## Implementation Status Legend

- ✅ **Completed** - Fully implemented and tested
- 🚧 **In Progress** - Currently being worked on
- 📋 **Planned** - Scheduled for implementation
- ⏸️ **On Hold** - Temporarily paused
- ❌ **Blocked** - Cannot proceed (dependencies/issues)

---

## Phase 1: Foundation (Weeks 1-2) - HIGHEST PRIORITY

**Goal**: Establish the core building blocks for all UI components

**⚠️ UPDATED AFTER BEVY DOCS ANALYSIS**: Bevy provides comprehensive UI primitives - leverage native components

### Typography System
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Text | ✅ | Critical | 1 | **COMPLETED**: Theme wrapper around Bevy's UI Text component with full font integration |
| Heading | ✅ | Critical | 0.5 | **COMPLETED**: Text extension with semantic heading levels (H1-H6) |

### Layout Fundamentals
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Box |  ✅ | Critical | 1 | **BEVY NATIVE**: `Node` + `BackgroundColor` + `BorderRadius` + `Outline` ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/struct.Node.html)) |
| Flex | ✅ | Critical | 1 | **COMPLETED**: Flexbox layout component with builder pattern, theme integration, and comprehensive flex properties |
| Grid | 📋 | High | 1 | **BEVY NATIVE**: `Node` + `Display::Grid` - Grid layout available! ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/enum.Display.html)) |
| Container | 📋 | High | 0.5 | **BEVY NATIVE**: `Node` + `UiRect` for max-width constraints ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/struct.UiRect.html)) |

**Phase 1 Total**: 5 days (**REDUCED from 13 days** - Bevy provides core functionality, Code component removed)

---

## Phase 2: Core UI Elements (Weeks 3-4) - HIGH PRIORITY

**Goal**: Build essential interactive components and visual elements

### Card System
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Card | 📋 | High | 2 | **BEVY NATIVE**: `Node` + `BackgroundColor` + `BorderRadius` + `BoxShadow` ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/struct.BoxShadow.html)) |
| Section | 📋 | Medium | 1 | **BEVY NATIVE**: `Node` wrapper for semantic sections |

### Basic Form Controls
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Checkbox | 📋 | High | 2 | **BEVY NATIVE**: `Button` + `Interaction` + `ButtonInput<MouseButton>` + checkmark graphics ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/widget/struct.Button.html)) |
| Switch | 📋 | High | 2 | **BEVY NATIVE**: `Button` + `Interaction` + Toggle-State + slide animation |
| Radio Group | 📋 | High | 3 | **BEVY NATIVE**: Multiple `Button` + `Focusable` + `FocusPolicy` + keyboard nav ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/struct.Focusable.html)) |

### Visual Indicators
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Badge | 📋 | Medium | 1 | **BEVY NATIVE**: `Node` + `Text` + `BorderRadius` for pill shapes |
| Separator | 📋 | Low | 1 | **BEVY NATIVE**: `Node` + `BackgroundColor` for divider lines |
| Avatar | 📋 | Medium | 2 | **BEVY NATIVE**: `ImageNode` + `ImageMeasure` + fallback Text ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/widget/struct.ImageNode.html)) |

**Phase 2 Total**: 14 days

---

## Phase 3: Advanced Interactions (Weeks 5-8) - MEDIUM PRIORITY

**Goal**: Implement complex form controls and navigation

### Form Components
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Text Field | 📋 | High | 3 | **BEVY NATIVE**: `TextMeasure` + `Button` (focus) + `ButtonInput<KeyCode>` for input ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/widget/struct.TextMeasure.html)) |
| Text Area | 📋 | High | 2 | **BEVY NATIVE**: `Text` + `ScrollPosition` + multi-line handling ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/struct.ScrollPosition.html)) |
| Select | 📋 | High | 4 | **BEVY NATIVE**: `Button` + `Node` dropdown + `Interaction` + `ui_focus_system` |
| Slider | 📋 | Medium | 3 | **BEVY NATIVE**: `Button` (thumb) + `Node` (track) + `Axis` values + drag ([docs.rs](https://docs.rs/bevy/latest/bevy/input/struct.Axis.html)) |

### Navigation
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Tabs | 📋 | High | 3 | **BEVY NATIVE**: Multiple `Button` + `Focusable` + state management + keyboard support |
| Tab Nav | 📋 | Medium | 2 | **BEVY NATIVE**: Tabs variant with navigation styling |

### Feedback Systems
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Progress | 📋 | Medium | 2 | **BEVY NATIVE**: `Node` + `BackgroundColor` + width animation for bars |
| Spinner | 📋 | Low | 1 | **BEVY NATIVE**: `ImageNode` + rotation animation (or animated sprite) |
| Toast | 📋 | Medium | 4 | **BEVY NATIVE**: `Node` + `PositionType::Absolute` + Portal system for positioning |
| Skeleton | 📋 | Low | 2 | **BEVY NATIVE**: `Node` + `BackgroundColor` + shimmer animation |

**Phase 3 Total**: 26 days

---

## Phase 4: Complex Components (Weeks 9-12) - MEDIUM PRIORITY

**Goal**: Advanced data display and form components

### Data Display
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Table | 📋 | Medium | 5 | **BEVY NATIVE**: `Node` + `Display::Grid` + `ScrollPosition` for large tables ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/enum.Display.html)) |
| Data List | 📋 | Low | 2 | **BEVY NATIVE**: `Node` + `FlexDirection::Column` for key-value pairs |

### Advanced Form Controls
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Checkbox Cards | 📋 | Low | 3 | Card-based selection interface |
| Radio Cards | 📋 | Low | 3 | Card-based single selection |
| Segmented Control | 📋 | Medium | 3 | Segmented button groups |

**Phase 4 Total**: 16 days

---

## Phase 5: Overlay Components (Weeks 13-16) - LOWER PRIORITY

**Goal**: Modal systems and contextual overlays

### Modal Systems
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Dialog | 📋 | Medium | 4 | **BEVY NATIVE**: `Node` + `PositionType::Absolute` + `ui_focus_system` for focus trapping ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/enum.PositionType.html)) |
| Alert Dialog | 📋 | Medium | 2 | **BEVY NATIVE**: Dialog variant with confirmation buttons |
| Popover | 📋 | Medium | 4 | **BEVY NATIVE**: `Node` + `PositionType::Absolute` + positioning algorithms |

### Menu Systems
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Dropdown Menu | 📋 | Medium | 5 | **BEVY NATIVE**: `Node` + `PositionType::Absolute` + `Interaction` + keyboard nav |
| Context Menu | 📋 | Low | 3 | **BEVY NATIVE**: Dropdown Menu + right-click detection |
| Hover Card | 📋 | Low | 3 | **BEVY NATIVE**: `Node` + `PositionType::Absolute` + hover detection |

**Phase 5 Total**: 21 days

---

## Phase 6: Specialized Components (Weeks 17-20) - LOWER PRIORITY

**Goal**: Advanced interaction patterns

### Advanced Interactions
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Tooltip | 📋 | Medium | 3 | **BEVY NATIVE**: `Node` + `PositionType::Absolute` + hover detection + positioning |
| Collapsible | 📋 | Medium | 3 | **BEVY NATIVE**: `Button` + `Node` content + expand/collapse animation |
| Accordion | 📋 | Medium | 4 | **BEVY NATIVE**: Multiple Collapsible + state management |

### Complex Forms
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Form | 📋 | High | 5 | Form validation and submission handling |
| Toggle Group | 📋 | Low | 3 | Multiple toggle selections |
| Toolbar | 📋 | Low | 3 | Action toolbars with grouping |

**Phase 6 Total**: 21 days

---

## Phase 7: Utilities & Accessibility (Weeks 21-24) - SPECIALIZED

**Goal**: Accessibility utilities and specialized components

### Accessibility Utilities
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Accessible Icon | 📋 | Medium | 2 | Screen reader friendly icons |
| Visually Hidden | 📋 | Medium | 1 | Screen reader only content |
| Portal | ✅ | High | 0 | Already implemented - renders outside component tree |

### Specialized Components
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Aspect Ratio | 📋 | Low | 2 | Maintaining aspect ratios for media |
| Scroll Area | 📋 | Medium | 4 | **BEVY NATIVE**: `ScrollPosition` + `Node` + custom scrollbar styling ([docs.rs](https://docs.rs/bevy/latest/bevy/ui/struct.ScrollPosition.html)) |
| One-Time Password Field | 📋 | Low | 3 | OTP input with auto-focus |
| Password Toggle Field | 📋 | Low | 2 | Password visibility toggle |

**Phase 7 Total**: 14 days

---

## Additional Theme Components

### Typography Extensions
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Blockquote | 📋 | Low | 1 | Styled quote blocks |
| Em | 📋 | Low | 0.5 | Emphasis text styling |
| Strong | 📋 | Low | 0.5 | Strong text styling |
| Quote | 📋 | Low | 1 | Inline quotations |
| Kbd | 📋 | Low | 1 | Keyboard key styling |

### Layout Extensions
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Inset | 📋 | Low | 1 | Content inset spacing |
| Slot | 📋 | Low | 2 | Component composition utility |

### Utility Components
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Reset | 📋 | High | 1 | CSS reset for consistent styling |
| Theme | 📋 | Medium | 2 | Theme provider and switching |

**Additional Components Total**: 11 days

---

## Completed Components

### Utilities
| Component | Status | Completion Date | Notes |
|-----------|--------|-----------------|-------|
| UIRoot | ✅ | 2025-06-20 | Foundation UI container (document.body equivalent) |
| Portal | ✅ | 2025-06-20 | Renders UI outside normal hierarchy |

### Components
| Component | Status | Completion Date | Notes |
|-----------|--------|-----------------|-------|
| Button | ✅ | 2025-06-20 | **UPDATED**: Now uses new Text component system with typography integration |
| Text | ✅ | 2025-06-20 | Theme wrapper around Bevy's UI Text with full font support (Sans/Serif/Mono, weights, italic) |
| Heading | ✅ | 2025-06-20 | Semantic heading levels (H1-H6) with appropriate sizing and weights |
| Box | ✅ | 2025-06-20 | Layout foundation with theme integration and comprehensive styling options |
| Flex | ✅ | 2025-06-22 | **NEW**: Flexbox layout component with builder pattern, gap control, and flex properties |

---

## Project Statistics
- **Start Date**: 2025-06-20
- **Current Date**: 2025-06-22 (Day 3)
- **Total Estimated Days**: 127.5 days (~5.1 months) (**REDUCED** after Bevy analysis & optimizations)
- **Components Completed**: 7/79 (8.9%) - **AHEAD OF SCHEDULE**
- **Phase 1 Progress**: 4/4 Critical components complete (Box ✅, Flex ✅, Text ✅, Heading ✅)
- **Current Focus**: Remaining Phase 1 (Grid, Container) → Phase 2 (Card, Form Controls)
- **Days Saved**: 1.0 day (Flex component implemented faster than estimated)
- **Velocity**: 2.33 components/day (projected: 1.58 components/day)

## Key Revision Notes
- **Major Discovery**: Bevy 0.16 provides comprehensive UI system with text, layout, and styling
- **Strategy Shift**: Focus on theme integration rather than rebuilding core functionality
- **Time Savings**: ~7.5 days saved in Phase 1 alone due to Bevy's built-in capabilities
- **Implementation Approach**: Wrapper pattern around Bevy's existing components

## Bevy UI Documentation Analysis (2025-06-20)
**Sources Analyzed**: 
- [Bevy UI Module](https://docs.rs/bevy/latest/bevy/ui/index.html)
- [Bevy UI Widgets](https://docs.rs/bevy/latest/bevy/ui/widget/index.html)  
- [Bevy Input System](https://docs.rs/bevy/latest/bevy/input/index.html)

**Key Bevy UI Components Available**:
- **Layout**: `Node`, `Display::Grid`, `FlexDirection`, `PositionType::Absolute`
- **Styling**: `BackgroundColor`, `BorderRadius`, `BoxShadow`, `Outline`, `BorderColor`
- **Interaction**: `Button`, `Interaction`, `Focusable`, `FocusPolicy`, `ui_focus_system`
- **Input**: `ButtonInput<MouseButton>`, `ButtonInput<KeyCode>`, `Axis` for values
- **Text**: `Text`, `TextMeasure`, `Label` for text handling  
- **Images**: `ImageNode`, `ImageMeasure` for image display
- **Scrolling**: `ScrollPosition` for scrollable content
- **Positioning**: `UiRect`, `Val`, `ContentSize` for layout properties

**Architecture Impact**: Almost all roadmap components can leverage Bevy's native UI primitives instead of custom implementations, significantly reducing development time and improving performance.

## Recent Achievements (2025-06-20 to 2025-06-22)
✅ **Text Component Completed**: Full typography system with theme integration
- Semantic variants (Display, Title, Body, Label, Caption)
- Complete font family support (Sans, Serif, Mono)
- All font weights (Light, Regular, Medium, Bold) + Italic support
- Responsive sizing system (Xs to X9l)
- Theme-integrated colors
- Builder pattern with fluent API

✅ **Button Component Enhanced**: Now uses new Text component system
- Automatic text sizing based on button size (Small→Sm, Default→Base, Large→Lg)
- Semantic text weights based on button variant
- Full typography integration with theme colors
- Maintains all existing button functionality while gaining typography benefits

✅ **Heading Component Completed**: Semantic heading system
- Six heading levels (H1-H6) with appropriate sizing hierarchy
- H1: X5l/Bold, H2: X3l/Bold, H3: X2l/Medium, H4: Xl/Medium, H5: Lg/Medium, H6: Base/Medium
- Convenience methods (Heading::h1(), Heading::h2(), etc.)
- Extension trait for converting Text to headings
- Full theme integration through Text component system

✅ **Box Component Completed**: Layout foundation component
- Four visual variants (Surface, Panel, Card, Outline)
- Complete spacing system with theme-integrated levels
- Comprehensive sizing, positioning, and styling methods
- Border radius and color palette integration
- Flex child properties for layout composition

✅ **Flex Component Completed (2025-06-22)**: Flexbox layout system
- Full flexbox API (direction, wrap, justify, align)
- Gap control with uniform and axis-specific options
- Convenience methods (row(), column(), center())
- Theme-integrated spacing and color management
- Working example with multiple layout patterns
- **Implemented 1 day faster than estimated**

## Dependencies & Blockers

### Prerequisites
- ✅ Theme system (colors, typography, spacing)
- ✅ Builder pattern infrastructure
- ✅ Basic component structure

### Key Dependencies
- Typography system → All text-based components
- Layout system → All container components
- Theme integration → All styled components
- Accessibility framework → All interactive components

## Accessibility Goals

Each component must include:
- **WAI-ARIA compliance** - Proper aria labels and roles
- **Keyboard navigation** - Full keyboard accessibility
- **Focus management** - Logical focus flow
- **Screen reader support** - Meaningful announcements
- **High contrast** support - Theme-aware contrast ratios

## Testing Strategy

- **Unit tests** for component logic
- **Integration tests** for theme integration
- **Accessibility tests** for ARIA compliance
- **Visual regression tests** for theme consistency
- **Performance tests** for render efficiency

## Next Actions

1. **Immediate (Next 2 days - 2025-06-23/24)**:
   - Implement Grid component (CSS Grid layout)
   - Implement Container component (max-width constraints)
   - **Complete Phase 1** - All layout foundations ready

2. **This Week (2025-06-25-27)**:
   - Begin Phase 2: Card component implementation
   - Start form control prototypes (Checkbox, Switch)
   - Set up component testing framework

3. **Next Week (2025-06-30-07-04)**:
   - Complete core form controls (Checkbox, Switch, Radio Group)
   - Implement visual indicators (Badge, Separator, Avatar)
   - Establish accessibility testing patterns

---

## Notes

- This roadmap is based on Radix UI component analysis
- Time estimates include implementation, testing, and documentation
- Priority levels may shift based on project needs
- Each component should follow the established builder pattern
- Theme integration is mandatory for all visual components

**Last Updated**: 2025-06-22 (Flex component completion + progress update)
**Next Review**: 2025-06-29 (Phase 1 completion milestone)