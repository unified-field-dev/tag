//! Tag catalog table rows.

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use leptos_router::components::A;
use tag::types::TagRowDto;
use uf_product::components::{
    Button, ButtonAppearance, Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
};

fn format_updated(at: DateTime<Utc>) -> String {
    at.format("%Y-%m-%d").to_string()
}

/// Tag catalog table: name, taxonomy, owner, updated date, and an edit link per row.
#[component]
pub(crate) fn TagListTable(items: Vec<TagRowDto>) -> impl IntoView {
    view! {
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHeaderCell>"Name"</TableHeaderCell>
                    <TableHeaderCell>"Taxonomy"</TableHeaderCell>
                    <TableHeaderCell>"Owner"</TableHeaderCell>
                    <TableHeaderCell>"Updated"</TableHeaderCell>
                    <TableHeaderCell>"Actions"</TableHeaderCell>
                </TableRow>
            </TableHeader>
            <TableBody>
                <For
                    each=move || items.clone()
                    key=|row: &TagRowDto| row.id.clone()
                    let:row
                >
                    <TableRow>
                        <TableCell>{row.name.clone()}</TableCell>
                        <TableCell>
                            {row.taxonomy.clone().unwrap_or_else(|| "—".to_string())}
                        </TableCell>
                        <TableCell>{row.owner_display.clone()}</TableCell>
                        <TableCell>{format_updated(row.updated_at)}</TableCell>
                        <TableCell>
                            <span attr:data-testid=format!("tag-row-edit-{}", row.id)>
                                <A href=format!("/tag/{}", row.id)>
                                    <Button appearance=ButtonAppearance::Subtle>
                                        "Edit"
                                    </Button>
                                </A>
                            </span>
                        </TableCell>
                    </TableRow>
                </For>
            </TableBody>
        </Table>
    }
}
