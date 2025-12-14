use polars::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Polars Exploration Examples ===\n");

    // Load data
    let file = std::fs::File::open("data/10000 Sales Records.csv")?;
    let df = CsvReader::new(file).finish()?;

    // ==========================================
    // 1. BASIC OPERATIONS
    // ==========================================
    println!("--- 1. Basic Operations ---");

    // Check null counts
    let null_count = df.null_count();
    println!("Null counts:\n{}\n", null_count);

    // Select single column
    let country_col = df.select(["Country"])?;
    println!("Country column (first 5):\n{}\n", country_col.head(Some(5)));

    // ==========================================
    // 2. FILTERING
    // ==========================================
    println!("--- 2. Filtering ---");

    let latvia_stats = df
        .clone()
        .lazy()
        .filter(col("Country").eq(lit("Latvia")))
        .collect()?;
    println!("Latvia records: {} rows\n", latvia_stats.height());

    // ==========================================
    // 3. EXPRESSIONS & CALCULATIONS
    // ==========================================
    println!("--- 3. Expressions & Calculations ---");

    let calculated = df
        .clone()
        .lazy()
        .filter(col("Country").eq(lit("Latvia")))
        .select([
            col("Units Sold"),
            col("Unit Price"),
            (col("Units Sold") / col("Unit Price").pow(2)).alias("custom_metric"),
        ])
        .collect()?;
    println!("Custom calculation:\n{}\n", calculated.head(Some(5)));

    // ==========================================
    // 4. WORKING WITH NULLS
    // ==========================================
    println!("--- 4. Working with Nulls ---");

    let df_with_nulls = df!(
        "some_numbers" => &[Some(1), Some(2), Some(3), None],
        "strict_numbers" => &[1, 2, 3, 4],
    )?;
    println!("DataFrame with nulls:\n{}\n", df_with_nulls);

    // Access values
    let col_i32 = df_with_nulls.column("some_numbers")?.i32()?;
    let value = col_i32.get(1);
    println!("Value at index 1: {:?}\n", value);

    // ==========================================
    // 5. GROUP BY AGGREGATIONS
    // ==========================================
    println!("--- 5. GroupBy Aggregations ---");

    let agg_by_country = df
        .clone()
        .lazy()
        .group_by([col("Country")])
        .agg([
            len().alias("items_count"),
            col("Item Type").first().alias("first_item_type"),
            col("Item Type").last().alias("last_item_type"),
            col("Total Revenue").sum().alias("total_revenue"),
            col("Total Profit").mean().alias("avg_profit"),
        ])
        .sort(
            ["items_count"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .collect()?;
    println!(
        "Top countries by item count:\n{}\n",
        agg_by_country.head(Some(10))
    );

    // ==========================================
    // 6. WINDOW FUNCTIONS (.over)
    // ==========================================
    println!("--- 6. Window Functions ---");

    let features_mean = df
        .clone()
        .lazy()
        .with_columns([
            // Average units sold per country
            col("Units Sold")
                .mean()
                .over([col("Country")])
                .alias("avg_units_sold_by_country"),
            // Total profit per country + item type
            col("Total Profit")
                .sum()
                .over([col("Country"), col("Item Type")])
                .alias("total_profit_per_country_per_type"),
        ])
        .select([
            col("Country"),
            col("Item Type"),
            col("Units Sold"),
            col("avg_units_sold_by_country"),
            col("Total Profit"),
            col("total_profit_per_country_per_type"),
        ])
        .sort(
            ["Country"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .collect()?;
    println!(
        "Window functions result:\n{}\n",
        features_mean.head(Some(10))
    );

    // ==========================================
    // 7. STRING OPERATIONS
    // ==========================================
    println!("--- 7. String Operations ---");

    let string_ops = df
        .clone()
        .lazy()
        .select([
            col("Country"),
            col("Country").str().to_uppercase().alias("country_upper"),
            col("Country").str().to_lowercase().alias("country_lower"),
            col("Country").str().len_chars().alias("country_length"),
        ])
        .collect()?;
    println!("String operations:\n{}\n", string_ops.head(Some(10)));

    // Filter by exact match
    let fruit_items = df
        .clone()
        .lazy()
        .filter(col("Item Type").eq(lit("Fruits")))
        .select([col("Item Type"), col("Country"), col("Total Revenue")])
        .collect()?;
    println!("Fruit items: {} rows\n", fruit_items.height());

    // ==========================================
    // 8. DATETIME OPERATIONS
    // ==========================================
    println!("--- 8. DateTime Operations ---");

    // Note: DateTime operations require the data to be in proper datetime format
    // For learning purposes, we'll just show the concept
    println!("DateTime operations work when you have proper datetime columns");
    println!("Common operations:");
    println!("  - col(\"date\").dt().year()  // Extract year");
    println!("  - col(\"date\").dt().month() // Extract month");
    println!("  - col(\"date\").dt().day()   // Extract day");
    println!("  - col(\"date\").dt().weekday() // Day of week");
    println!("Skipping actual execution (requires datetime column)\n");

    // ==========================================
    // 9. JOINS
    // ==========================================
    println!("--- 9. Joins ---");

    // Create a lookup table
    let country_regions = df!(
        "Country" => &["Latvia", "Germany", "United States", "France"],
        "Region" => &["Europe", "Europe", "North America", "Europe"],
        "Population" => &[1_900_000, 83_000_000, 331_000_000, 67_000_000],
    )?;

    let joined = df
        .clone()
        .lazy()
        .join(
            country_regions.lazy(),
            [col("Country")],
            [col("Country")],
            JoinArgs::new(JoinType::Left),
        )
        .select([
            col("Country"),
            col("Item Type"),
            col("Total Revenue"),
            col("Region"),
            col("Population"),
        ])
        .collect()?;
    println!("Joined data:\n{}\n", joined.head(Some(10)));

    // ==========================================
    // 10. SAVING DATA
    // ==========================================
    println!("--- 10. Saving Data ---");

    // Save to CSV
    let mut csv_file = std::fs::File::create("data/output_example.csv")?;
    CsvWriter::new(&mut csv_file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut features_mean.clone())?;
    println!("Saved to data/output_example.csv");

    // Save to Parquet (more efficient for large data)
    let parquet_file = std::fs::File::create("data/output_example.parquet")?;
    ParquetWriter::new(parquet_file).finish(&mut features_mean.clone())?;
    println!("Saved to data/output_example.parquet\n");

    // ==========================================
    // 11. CHAINING MULTIPLE OPERATIONS
    // ==========================================
    println!("--- 11. Complex Pipeline Example ---");

    let complex_pipeline = df
        .lazy()
        .filter(col("Total Revenue").gt(lit(10000)))
        .with_columns([
            col("Total Profit")
                .mean()
                .over([col("Country")])
                .alias("avg_country_profit"),
            (col("Total Profit") / col("Total Revenue")).alias("profit_margin"),
        ])
        .filter(col("profit_margin").gt(lit(0.3)))
        .group_by([col("Country")])
        .agg([
            len().alias("high_margin_count"),
            col("profit_margin").mean().alias("avg_margin"),
            col("Total Revenue").sum().alias("total_revenue"),
        ])
        .sort(
            ["total_revenue"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .collect()?;

    println!("Complex pipeline result:\n{}\n", complex_pipeline);

    println!("=== All examples completed! ===");
    Ok(())
}
