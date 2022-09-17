// Pass all tests!
#[cfg(test)]
mod tests {
    use std::fs;
    use sha2::{ Sha256, Digest };

    fn result(filename: String) -> String {
        let file_data = fs::read_to_string(filename).expect("Cant read the file.");

        // Pushing string lines to vector
        let mut file_vec: Vec<String> = Vec::new();
        for line in file_data.lines() {
            file_vec.push(line.to_string());
        }

        // extracting first value (n) from vec we dont need it
        file_vec = file_vec[1..file_vec.len()].to_vec();

        // Hashing merkle tree until root
        while file_vec.len() != 1 {
            let hashed_vec = file_vec
                .iter()
                .map(|value| hash_input(value))
                .collect();
            file_vec = create_next_level(hashed_vec);
        }

        // hashing the root and returning it
        let final_str = hash_input(file_vec.get(0).expect("Couldnt get first index"));
        final_str
    }

    // You can use templates below or just remove
    // Helper function to create a next leaves level may help you :)
    fn create_next_level(current_level: Vec<String>) -> Vec<String> {
        let mut next_level: Vec<String> = Vec::new();
        let mut temp = String::from("");
        for (i, val) in current_level.iter().enumerate() {
            if i % 2 == 0 {
                temp.push_str(val);
            } else {
                temp.push_str(val);
                next_level.push(temp);
                temp = "".to_string();
            }
        }

        next_level
    }

    // Helper function may help you to hash an input or You can write macro rules
    fn hash_input(a: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(a);
        let result = hasher.finalize();
        hex::encode(&result)
    }

    #[test]
    fn input_0() {
        let result = result("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = result("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = result("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = result("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = result("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}

// Import Crates

fn main() {
    // Read Input Data from txt file
    //todo!()

    // Create vector of strings for leaves

    // Hash inputs and append to vector

    // Then Write an algorithm that calculates the ROOT

    // Return the root hash as a String

    //let merkle_tree_str = result("input0.txt".to_string());

    //println!("{}", merkle_tree_str);
}