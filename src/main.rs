use data_analysis::{read_dataset, generate_summary_statistics};
use std::error::Error;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example file path (URL of the Titanic dataset)
    let file_path = "https://web.stanford.edu/class/archive/cs/cs109/cs109.1166/stuff/titanic.csv";

    // Read the dataset asynchronously
    let data = read_dataset(file_path).await?;

    // Track memory usage and start time
    let initial_memory = sys_info::mem_info()?;
    let start_time = Instant::now();
    
    // Generate summary statistics
    generate_summary_statistics(&data);
    
    // Measure elapsed time
    let duration = start_time.elapsed();
    
    // Final memory usage
    let final_memory = sys_info::mem_info()?;
    let used_memory = initial_memory.total - final_memory.total;

    // Print statistics
    println!("Elapsed time: {:.2?}", duration);
    println!("Memory used: {:.2} KB", used_memory );
    println!("Initial memory: {} KB", initial_memory.total); 
    println!("Final memory: {} KB", final_memory.total); 
    
    Ok(())
}
