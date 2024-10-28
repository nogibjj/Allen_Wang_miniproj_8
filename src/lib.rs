use csv::ReaderBuilder;
use ndarray::{Array2, Axis,Array1};
use std::error::Error;
use std::io::Cursor;
use ndarray::s;
pub async fn read_dataset(file_path: &str) -> Result<Array2<f64>, Box<dyn Error>> {
    // Check if the path is a URL
    let mut records = Vec::new();
    if file_path.starts_with("http://") || file_path.starts_with("https://") {
        // Download the file from the URL
        let response = reqwest::get(file_path).await?;
        let content = response.bytes().await?;

        // Read the CSV content using csv crate
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(content));

        for result in rdr.records() {
            let record = result?;
            // Filter out only numeric values
            let row: Vec<f64> = record.iter()
                .filter_map(|field| field.parse::<f64>().ok())
                .collect();
            if !row.is_empty() {
                records.push(row);
            }
        }
    } else {
        // Handle local file reading (only CSV for simplicity)
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;

        for result in rdr.records() {
            let record = result?;
            // Filter out only numeric values
            let row: Vec<f64> = record.iter()
                .filter_map(|field| field.parse::<f64>().ok())
                .collect();
            if !row.is_empty() {
                records.push(row);
            }
        }
    }

    // Convert Vec<Vec<f64>> to ndarray::Array2
    if records.is_empty() {
        return Err("No numeric data found".into());
    }

    let num_rows = records.len();
    let num_cols = records[0].len();
    let flat_data: Vec<f64> = records.into_iter().flatten().collect();
    let array = Array2::from_shape_vec((num_rows, num_cols), flat_data)?;

    Ok(array)
}

pub fn generate_summary_statistics(data: &Array2<f64>)-> (Array1<f64>, Array1<f64>, Array1<f64>) {
    //println!("Original Data:\n{:?}", data);
    // Mean for each column
    let mean = data.mean_axis(Axis(0)).unwrap();
    println!("Mean Values:\n{:?}", mean);

    // Median for each column (sorted data required)
    let mut sorted_data = data.to_owned();
    for mut column in sorted_data.columns_mut() {
        //println!("Original column:\n{:?}", column);
        let mut sorted_column: Vec<f64> = column.to_vec(); // Create a new Vec from the column
        sorted_column.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        column.assign(&Array1::from(sorted_column)); 
    }

    let nrows = data.nrows();
    let medians: Array1<f64> = (0..data.ncols())
        .map(|i| {
            let col = sorted_data.slice(s![.., i]);
            if col.is_empty() {
                return 0.0; // or handle the case as appropriate
            }
            if nrows % 2 == 0 {
                // Even number of elements
                let mid1 = col[nrows / 2 - 1];
                let mid2 = col[nrows / 2];
                (mid1 + mid2) / 2.0
            } else {
                // Odd number of elements
                col[nrows / 2]
            }
        })
        .collect();

    println!("Median Values:\n{:?}", medians);

    // Standard deviation for each column
    let std_dev = data.std_axis(Axis(0), 0.0);
    println!("Standard Deviation:\n{:?}", std_dev);
    (mean, medians, std_dev)
}