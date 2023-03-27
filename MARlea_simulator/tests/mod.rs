#[cfg(test)]
mod tests {
    

    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use std::str::FromStr;

    #[test]
    fn test_full_simulation() {
        // Generate sample CSV file
        let csv ="6 zooble + 4 crand => 1 gubble, 14\n1 gobble + 1 gubble => 1 crangle, 6\n1 gubble => 1 gobble + 10 zooble + 5 crand, 100";
        let input_file_name = "input.csv";
        let input_path = PathBuf::from(input_file_name);
        fs::write(&input_path, csv).expect("Failed to create input file for test");

        // Run command line arguments as MarleaOpts struct
        let output_file_name = "output.txt";
        let output_path = PathBuf::from(output_file_name);
        let args = vec![String::from("simulate"), input_file_name.to_string(), String::from("-o"), output_file_name.to_string()];
        let status = Command::new(env!("CARGO_BIN_EXE_Marlea"))
            .args(&args)
            .status()
            .expect("Failed to execute command");
        assert!(status.success());

        // Verify results are as expected (number of lines in output file should match number of trials + 1 (header))
        let num_trials = 10;
        let result = fs::read_to_string(&output_path).expect("Error reading output file for test");
        assert_eq!(result.lines().count(), num_trials + 1);
    }
}
