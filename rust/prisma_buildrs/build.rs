// Determine if the client needs to be regenerated.
let regenerate = if prisma_client_path.exists() {
  // Check to see if the schema has been modified since the last time the client was generated
  let prisma_client_metadata = std::fs::metadata(&prisma_client_path).unwrap();
  let schema_metadata = fs::metadata(&schema_path).unwrap();
  schema_metadata.modified().unwrap() > prisma_client_metadata.modified().unwrap()
} else {
  // Generate if doesn't exist
  true
};

if regenerate {
  // Set the Cargo target directory to a different location from the main project to avoid a deadlock
  let prisma_gen_target_dir = git_root.join("build/prisma_gen_target");
  temp_env::with_var("CARGO_TARGET_DIR", Some(prisma_gen_target_dir), || {
    std::process::Command::new("cargo")
      .current_dir(git_root)
      .args([
        "prisma",
        "generate",
        "--schema",
        schema_path.to_str().unwrap(),
      ])
      .output()
      .expect("failed to regenerate Prisma client.");
  });
}
