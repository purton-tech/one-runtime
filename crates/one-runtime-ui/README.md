# UI Guidelines

Agent-first rules for building UI in Dioxus projects that use Daisy UI, `daisy_rsx`, and Tailwind.

## Priority Order

Use this order of preference:

1. `daisy_rsx` components
2. Semantic HTML in `rsx!`
3. Small amounts of Tailwind for layout and spacing
4. Custom utility-heavy styling only when the first three are not enough

## Core Rules

- Prefer `daisy_rsx` when it fits.
- If a fitting Daisy RSX component exists, do not rebuild it from raw HTML.
- Use raw HTML only when the structure is semantic and Daisy RSX is not a good fit.
- Use Tailwind mainly for layout, spacing, sizing, truncation, and light interaction polish.
- Do not hardcode colors. Use Daisy/theme tokens.
- Avoid long Tailwind class strings. If a block starts turning into class soup, extract a component or simplify it.
- Prefer readable composition over large inline `rsx!` blocks.

## File Organization

- `page.rs` owns page-level wiring and composition.
- If `page.rs` grows to multiple substantial sections, break pieces into sibling files.
- Use a local `components/` folder for pieces shared within a route area.
- Use crate-level shared components only for UI reused across multiple pages or route areas.

Default route layout:

```text
example/
  page.rs
  card.rs
  modal.rs
  components/
    filters.rs
    empty_state.rs
```

## Preferred Daisy RSX Components

Use a short preferred set by use case:

- Actions: `Button`
- Surfaces: `Card`, `CardBody`
- Dialogs: `Modal`, `ModalBody`, `ModalAction`
- Navigation: `Breadcrumb`, `NavGroup`, `NavItem`
- Forms: `Fieldset`, `Select`, `SelectOption`
- Feedback: `Alert`, `Badge` when they add real meaning

## Tailwind Rules

- Good uses: layout, spacing, width/height, typography adjustments, truncation, hover/focus polish.
- Bad uses: reimplementing Daisy components, hardcoded colors, long one-off utility soups.
- If one visual block needs many utilities, it probably needs its own component.

## Dioxus Composition

- Keep components small and named for what they render.
- Use inline `rsx!` for small one-off pieces only.
- Extract a component when a block is visually significant, interactive, reused, or making `page.rs` hard to scan.
- Pass structured props when a component becomes important.

## Component Rules

### Modals

- Use `Modal`, `ModalBody`, and `ModalAction`.
- Keep modal structure simple: header, body sections, optional action row.
- Omit the action row when there are no actions.

### Cards

- Use `Card` and `CardBody` for surfaced content.
- Keep each card focused on one job.
- Do not overload cards with too many badges, actions, and secondary metadata.

### Forms

- Use `Fieldset` for labeled sections.
- Prefer simple vertical rhythm and consistent widths.
- Let Daisy components do most of the visual work.

## Anti-Patterns

- Raw modal markup when Daisy RSX already fits
- Hardcoded colors
- Growing `page.rs` without splitting it
- Raw HTML controls when an equivalent Daisy RSX component fits
- Large Tailwind-heavy blocks used to avoid extracting a component
- Too many interaction patterns inside one card or section

## Short Examples

### Page Structure

```rust
pub fn page(...) -> String {
    let page = rsx! {
        Layout {
            title: "Example".to_string(),
            ExampleFilters {}
            ExampleGrid {}
        }
    };

    render(page)
}
```

### Prefer Daisy RSX

```rust
Button {
    button_scheme: ButtonScheme::Primary,
    "Save"
}

Card {
    CardBody {
        h2 { class: "card-title", "Title" }
        p { "Description" }
    }
}
```

## One Runtime Notes

- Shared page shell lives in `src/layout.rs`.
- Use the shared `Layout` component for page shell and width before reaching for ad hoc shell classes.
- Shared widgets belong under `src/components/`.
- Route-specific pieces should stay near the route until they are truly cross-page reusable.

## Review Standard

A UI change should usually:

- use Daisy RSX first
- keep Tailwind minimal
- avoid hardcoded colors
- keep `page.rs` readable
- extract substantial or repeated UI into components

If a change breaks one of these defaults, explain why.
