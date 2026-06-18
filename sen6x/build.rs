fn main() {
    {
        let features = [
            cfg!(feature = "sen62"),
            cfg!(feature = "sen63c"),
            cfg!(feature = "sen65"),
            cfg!(feature = "sen66"),
            cfg!(feature = "sen68"),
            cfg!(feature = "sen69c"),
        ];

        let enabled_count = features.iter().filter(|&&f| f).count();

        if enabled_count != 1 {
            panic!(
                "Exactly one sensor of [sen62, sen63c, sen65, sen66, sen68, sen69c] must be enabled"
            );
        }
    }
}
