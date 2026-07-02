use stash_core::category::CategoryName;
use stash_core::ids::{CategoryId, ItemId};
use stash_core::money::Money;
use stash_core::sku::Sku;
use stash_storage::category_repository::{CategoryRepository, CreateCategoryInput};
use stash_storage::item_repository::{CreateItemInput, ItemRepository};

#[derive(Debug, serde::Deserialize)]
struct ImportRow {
    sku: String,
    name: String,
    description: Option<String>,
    category: String,
    unit_cost_kobo: i64,
    reorder_threshold: u32,
}

#[derive(Debug, Default)]
pub struct ImportSummary {
    pub imported: usize,

    /// (Row number in file, reason)
    pub skipped: Vec<(usize, String)>,
}

/// Bulk-loads items from `path` through the *same* validation manual entry
/// uses (`Sku::parse`, `CategoryName::parse`) — there is no special-cased
/// import path that bypasses domain invariants (Epic 2.3 / Ticket 7.1).
///
/// # Errors
/// Returns an error if the file can't be opened at all. Per-row problems
/// (bad SKU, duplicate SKU, etc.) are collected in `ImportSummary::skipped`
/// rather than aborting the whole import.
pub async fn import_csv(
    path: &std::path::Path,
    item_repo: &dyn ItemRepository,
    category_repo: &dyn CategoryRepository,
) -> anyhow::Result<ImportSummary> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut categories = category_repo.list().await?;
    let mut summary = ImportSummary::default();

    for (idx, result) in reader.deserialize::<ImportRow>().enumerate() {
        let row_num = idx + 2; // +1 for 0-index, +1 for the header row
        let row = match result {
            Ok(r) => r,
            Err(e) => {
                summary.skipped.push((row_num, format!("couldn't parse row: {e}")));
                continue;
            }
        };

        let sku = match Sku::parse(&row.sku) {
            Ok(s) => s,
            Err(e) => {
                summary.skipped.push((row_num, e.message()));
                continue;
            }
        };

        let category_id = match categories
            .iter()
            .find(|c| c.name.0.eq_ignore_ascii_case(&row.category))
        {
            Some(c) => c.id,
            None => {
                // Unknown categories are created on the fly rather than
                // failing the row — CSV imports commonly introduce new ones.
                let name = match CategoryName::parse(&row.category) {
                    Ok(n) => n,
                    Err(e) => {
                        summary.skipped.push((row_num, e.message()));
                        continue;
                    }
                };
                match category_repo
                    .create(&CreateCategoryInput { id: CategoryId::new(), name, description: None })
                    .await
                {
                    Ok(created) => {
                        let id = created.id;
                        categories.push(created);
                        id
                    }
                    Err(e) => {
                        summary.skipped.push((row_num, e.message()));
                        continue;
                    }
                }
            }
        };

        let input = CreateItemInput {
            id: ItemId::new(),
            sku,
            name: row.name,
            description: row.description,
            category_id,
            unit_cost: Money(row.unit_cost_kobo),
            reorder_threshold: row.reorder_threshold,
        };

        match item_repo.create(&input).await {
            Ok(_) => summary.imported += 1,
            Err(e) => summary.skipped.push((row_num, e.message())),
        }
    }

    Ok(summary)
}
