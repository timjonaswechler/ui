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

**⚠️ UPDATED AFTER BEVY ANALYSIS**: Many core features exist in Bevy - focus on theme integration

### Typography System
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Text | 📋 | Critical | 1 | **REVISED**: Theme wrapper around Bevy's Text2d component |
| Heading | 📋 | Critical | 0.5 | **REVISED**: Text variant with semantic heading levels |
| Code | 📋 | High | 0.5 | **REVISED**: Text variant with monospace font |

### Layout Fundamentals
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Box | 📋 | Critical | 1 | **REVISED**: Theme wrapper around Bevy's Node component |
| Flex | 📋 | Critical | 1 | **REVISED**: Builder pattern for Bevy's flexbox system |
| Grid | 📋 | High | 1 | **REVISED**: Builder pattern for Bevy's grid system |
| Container | 📋 | High | 0.5 | **REVISED**: Box variant with max-width constraints |

**Phase 1 Total**: 5.5 days (**REDUCED from 13 days** - Bevy provides core functionality)

---

## Phase 2: Core UI Elements (Weeks 3-4) - HIGH PRIORITY

**Goal**: Build essential interactive components and visual elements

### Card System
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Card | 📋 | High | 2 | Content containers with variants (surface, outline, ghost) |
| Section | 📋 | Medium | 1 | Semantic content sections |

### Basic Form Controls
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Checkbox | 📋 | High | 2 | Boolean input with custom styling, accessibility |
| Switch | 📋 | High | 2 | Toggle component for settings |
| Radio Group | 📋 | High | 3 | Single-choice selection with keyboard navigation |

### Visual Indicators
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Badge | 📋 | Medium | 1 | Status and category indicators |
| Separator | 📋 | Low | 1 | Visual content dividers |
| Avatar | 📋 | Medium | 2 | User/entity representation with fallbacks |

**Phase 2 Total**: 14 days

---

## Phase 3: Advanced Interactions (Weeks 5-8) - MEDIUM PRIORITY

**Goal**: Implement complex form controls and navigation

### Form Components
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Text Field | 📋 | High | 3 | Text input with validation states, error handling |
| Text Area | 📋 | High | 2 | Multi-line text input with auto-resize |
| Select | 📋 | High | 4 | Dropdown selection with search, keyboard navigation |
| Slider | 📋 | Medium | 3 | Range input component with thumb styling |

### Navigation
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Tabs | 📋 | High | 3 | Tab navigation with state management, keyboard support |
| Tab Nav | 📋 | Medium | 2 | Navigation tabs variant |

### Feedback Systems
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Progress | 📋 | Medium | 2 | Progress indicators and loading states |
| Spinner | 📋 | Low | 1 | Loading animations |
| Toast | 📋 | Medium | 4 | Temporary notifications with positioning |
| Skeleton | 📋 | Low | 2 | Loading placeholders |

**Phase 3 Total**: 26 days

---

## Phase 4: Complex Components (Weeks 9-12) - MEDIUM PRIORITY

**Goal**: Advanced data display and form components

### Data Display
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Table | 📋 | Medium | 5 | Structured data display with sorting, selection |
| Data List | 📋 | Low | 2 | Key-value pair display |

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
| Dialog | 📋 | Medium | 4 | Modal dialogs with focus trapping, ESC handling |
| Alert Dialog | 📋 | Medium | 2 | Confirmation dialogs |
| Popover | 📋 | Medium | 4 | Positioned overlays with arrow positioning |

### Menu Systems
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Dropdown Menu | 📋 | Medium | 5 | Action menus with submenus, keyboard navigation |
| Context Menu | 📋 | Low | 3 | Right-click menus |
| Hover Card | 📋 | Low | 3 | Hover-triggered content |

**Phase 5 Total**: 21 days

---

## Phase 6: Specialized Components (Weeks 17-20) - LOWER PRIORITY

**Goal**: Advanced interaction patterns

### Advanced Interactions
| Component | Status | Priority | Est. Days | Notes |
|-----------|--------|----------|-----------|-------|
| Tooltip | 📋 | Medium | 3 | Hover information with positioning |
| Collapsible | 📋 | Medium | 3 | Expandable content sections |
| Accordion | 📋 | Medium | 4 | Grouped collapsible sections with single/multiple expand |

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
| Scroll Area | 📋 | Medium | 4 | Custom scrollbars with styling |
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
| Button | ✅ | 2025-06-20 | Base button implementation with theme integration |
| UIRoot | ✅ | 2025-06-20 | Foundation UI container (document.body equivalent) |
| Portal | ✅ | 2025-06-20 | Renders UI outside normal hierarchy |

---

## Project Statistics
- **Start Date**: 2025-06-20
- **Total Estimated Days**: 128.5 days (~5.1 months) (**REDUCED** after Bevy analysis)
- **Components Completed**: 3/79 (3.8%)
- **Phase 1 Ready**: Typography and Layout foundations
- **Current Focus**: Text component theme wrapper implementation

## Key Revision Notes
- **Major Discovery**: Bevy 0.16 provides comprehensive UI system with text, layout, and styling
- **Strategy Shift**: Focus on theme integration rather than rebuilding core functionality
- **Time Savings**: ~7.5 days saved in Phase 1 alone due to Bevy's built-in capabilities
- **Implementation Approach**: Wrapper pattern around Bevy's existing components

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

1. **Immediate (Next 3 days)**:
   - Implement Text component with full typography variants
   - Set up component testing framework
   - Create typography theme tokens

2. **This Week**:
   - Complete Heading component
   - Start Box component implementation
   - Establish component documentation pattern

3. **Next Week**:
   - Complete Phase 1 (Typography + Layout)
   - Begin Phase 2 planning
   - Set up accessibility testing tools

---

## Notes

- This roadmap is based on Radix UI component analysis
- Time estimates include implementation, testing, and documentation
- Priority levels may shift based on project needs
- Each component should follow the established builder pattern
- Theme integration is mandatory for all visual components

**Last Updated**: 2025-06-20
**Next Review**: 2025-06-27