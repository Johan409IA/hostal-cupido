---
name: Cupido Hotel Management System
colors:
  surface: "#f7f9fb"
  surface-dim: "#d8dadc"
  surface-bright: "#f7f9fb"
  surface-container-lowest: "#ffffff"
  surface-container-low: "#f2f4f6"
  surface-container: "#eceef0"
  surface-container-high: "#e6e8ea"
  surface-container-highest: "#e0e3e5"
  on-surface: "#191c1e"
  on-surface-variant: "#50453b"
  inverse-surface: "#2d3133"
  inverse-on-surface: "#eff1f3"
  outline: "#827569"
  outline-variant: "#d4c4b7"
  surface-tint: "#7c572d"
  primary: "#7c572d"
  on-primary: "#ffffff"
  primary-container: "#d4a574"
  on-primary-container: "#5b3a13"
  inverse-primary: "#efbd8a"
  secondary: "#006a63"
  on-secondary: "#ffffff"
  secondary-container: "#99efe5"
  on-secondary-container: "#006f67"
  tertiary: "#565e74"
  on-tertiary: "#ffffff"
  tertiary-container: "#a6adc6"
  on-tertiary-container: "#394156"
  error: "#ba1a1a"
  on-error: "#ffffff"
  error-container: "#ffdad6"
  on-error-container: "#93000a"
  primary-fixed: "#ffdcbc"
  primary-fixed-dim: "#efbd8a"
  on-primary-fixed: "#2c1700"
  on-primary-fixed-variant: "#614018"
  secondary-fixed: "#9cf2e8"
  secondary-fixed-dim: "#80d5cb"
  on-secondary-fixed: "#00201d"
  on-secondary-fixed-variant: "#00504a"
  tertiary-fixed: "#dae2fd"
  tertiary-fixed-dim: "#bec6e0"
  on-tertiary-fixed: "#131b2e"
  on-tertiary-fixed-variant: "#3f465c"
  background: "#f7f9fb"
  on-background: "#191c1e"
  surface-variant: "#e0e3e5"
typography:
  display-lg:
    fontFamily: hankenGrotesk
    fontSize: 36px
    fontWeight: "700"
    lineHeight: "1.2"
    letterSpacing: -0.02em
  headline-md:
    fontFamily: hankenGrotesk
    fontSize: 24px
    fontWeight: "600"
    lineHeight: "1.3"
  headline-sm:
    fontFamily: hankenGrotesk
    fontSize: 18px
    fontWeight: "600"
    lineHeight: "1.4"
  body-lg:
    fontFamily: plusJakartaSans
    fontSize: 16px
    fontWeight: "400"
    lineHeight: "1.6"
  body-md:
    fontFamily: plusJakartaSans
    fontSize: 14px
    fontWeight: "400"
    lineHeight: "1.5"
  body-sm:
    fontFamily: plusJakartaSans
    fontSize: 13px
    fontWeight: "400"
    lineHeight: "1.5"
  label-md:
    fontFamily: plusJakartaSans
    fontSize: 12px
    fontWeight: "600"
    lineHeight: "1.2"
    letterSpacing: 0.05em
  data-mono:
    fontFamily: plusJakartaSans
    fontSize: 14px
    fontWeight: "500"
    lineHeight: "1"
    letterSpacing: -0.01em
rounded:
  sm: 0.5rem
  DEFAULT: 1rem
  md: 1.5rem
  lg: 2rem
  xl: 3rem
  full: 9999px
spacing:
  sidebar_width: 240px
  container_max_width: 1600px
  gutter: 1.5rem
  margin_page: 2rem
  stack_xs: 0.25rem
  stack_sm: 0.5rem
  stack_md: 1rem
  stack_lg: 1.5rem
---

## Brand & Style

The design system is engineered for **Premium Hospitality Operations**, merging the high-touch aesthetic of luxury hospitality with the rigorous functional requirements of a desktop SaaS. The brand personality is poised, efficient, and sophisticated. It seeks to evoke a sense of calm control for hotel staff, turning complex logistical data into a legible, high-end experience.

The visual style is a refined **Modern Minimalist** approach. It prioritizes information density without sacrificing white space, ensuring that operational dashboards remain breathable. The aesthetic leverages subtle champagne accents to nod to luxury service, while the core interface relies on deep architectural navies and professional teals to signal reliability and precision.

## Colors

The color palette is structured to facilitate long-duration desktop usage while maintaining a premium feel.

- **Architectural Foundation:** Navy (`#0F172A`) serves as the primary anchor for navigation and high-level headings, providing a stable frame for the application.
- **Operational Action:** Primary Teal (`#0F766E`) is reserved for primary calls to action, while Bright Teal (`#14B8A6`) highlights active states and selections.
- **Champagne Accent:** The seed color (`#D4A574`) is used sparingly as a "hallmark" of quality—applied to subtle dividers, premium status indicators, or small branding elements to elevate the UI from a standard tool to a luxury management suite.
- **Surface & Interface:** A crisp background (`#F8FAFC`) and pure white cards (`#FFFFFF`) create a tiered hierarchy, separated by thin, low-contrast borders (`#E2E8F0`).

## Typography

This design system utilizes a high-contrast typographic pairing to balance modern efficiency with approachable warmth.

- **Hanken Grotesk (Headlines):** Used for all structural headings and navigational titles. Its sharp, contemporary geometry provides a professional and tech-forward feel.
- **Plus Jakarta Sans (Body & UI):** Used for all functional text, data entries, and labels. Its softer terminals and open counters ensure high legibility in data-dense tables and property management views.
- **Data Styling:** For room numbers and prices, use `data-mono` (Medium weight) to ensure numerical alignment and quick scanning.
- **Hierarchy:** Use `label-md` for table headers and section overlines, always in uppercase with slight tracking to differentiate from interactive body text.

## Layout & Spacing

The layout is optimized for a **Fixed-Fluid Hybrid** desktop environment.

1. **Sidebar:** A fixed 240px navigation bar anchors the left side, utilizing the Navy palette to recede visually while providing constant access to core modules.
2. **Main Canvas:** A fluid area that stretches to a maximum of 1600px to prevent excessive line lengths on ultra-wide monitors.
3. **Grid:** A 12-column system is used for dashboard widgets. Content usually spans 3, 4, 6, or 12 columns.
4. **Information Density:** Vertical spacing follows a strict 4px/8px rhythm. Tables use a compact height (40px-48px per row) to maximize the "at-a-glance" utility for front-desk staff.

## Elevation & Depth

This design system uses a **Tonal Layering** approach with soft, ambient shadows to define depth without visual clutter.

- **Level 0 (Background):** `#F8FAFC` - The base canvas.
- **Level 1 (Cards/Tables):** White surface with a 1px border of `#E2E8F0`. No shadow. This is the primary container for data.
- **Level 2 (Hover/Active):** A soft, diffused shadow (`0 4px 12px rgba(15, 23, 42, 0.05)`) is applied when a card is hovered or an element is active.
- **Level 3 (Modals/Popovers):** Elevated surfaces use a more pronounced shadow (`0 12px 24px rgba(15, 23, 42, 0.1)`) and a subtle 1px border to separate the dialog from the dimmed background overlay.

## Shapes

The design system employs a **Pill-shaped (3)** roundedness philosophy to soften the industrial nature of a management tool.

- **Primary Components:** Buttons, input fields, and chips use a 1rem (16px) radius, creating a friendly, "premium hospitality" feel.
- **Large Containers:** Data cards and main dashboard sections use `rounded-lg` (2rem/32px) to create distinct "zones" of information.
- **Small Elements:** Checkboxes and secondary indicators use a smaller 4px radius to maintain precision at small scales.

## Components

- **Action Buttons:** Primary buttons use a solid Teal (`#0F766E`) background with White text. Secondary buttons use a Champagne (`#D4A574`) outline with Navy text for a high-end feel.
- **Status Chips:** Used for "Room Status" or "Payment Status." They feature a light tinted background of the semantic color (e.g., light green for Success) with a high-contrast text and a small dot indicator.
- **Compact Tables:** Headers are `#F8FAFC` with `#1E293B` uppercase text. Rows have a subtle hover state using a 2% opacity overlay of Primary Teal.
- **Sidebar Navigation:** Items use Navy background. The active state is indicated by a vertical Champagne (`#D4A574`) bar on the left and a text color shift to White.
- **Input Fields:** Minimalist design with a 1px border. On focus, the border shifts to Primary Teal with a subtle 2px glow. Labels sit above the field in `label-md` style.
- **Data Cards:** Summarize key metrics (e.g., Occupancy Rate). They feature a large `headline-md` value and a small sparkline or trend indicator in the corner.
