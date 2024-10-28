use data_analysis::{read_dataset, generate_summary_statistics};


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_read_dataset() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = "https://web.stanford.edu/class/archive/cs/cs109/cs109.1166/stuff/titanic.csv";
        
        // Call the read_dataset function and assert the result
        let data = read_dataset(file_path).await?;
        
        // Assert that the data is not empty and has expected dimensions
        assert!(data.shape()[0] > 0); // There should be rows
        assert!(data.shape()[1] > 0); // There should be columns

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_statistics()-> Result<(), Box<dyn std::error::Error>>{
        let file_path = "https://web.stanford.edu/class/archive/cs/cs109/cs109.1166/stuff/titanic.csv";
        
        // Call the read_dataset function and assert the result
        let data = read_dataset(file_path).await?;
        
        let (mean,median,std_dev) = generate_summary_statistics(&data);

        assert!(mean.shape()[0] > 0);
        assert!(median.shape()[0] > 0);
        assert!(std_dev.shape()[0] > 0);
    }

}

