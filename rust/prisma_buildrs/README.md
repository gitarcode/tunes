This code snippet is adapted from the actual build script used in our codebase to automatically regenerate the Prisma client whenever the Prisma schema file has been modified.

For a detailed description, please visit our blog at [gitar.co/blog](https://gitar.co/blog).

The script first checks if the generated Prisma client file already exists. If it does, it compares the last modified timestamp of the schema file with that of the generated client. If the schema has been modified more recently, or if the generated client doesn't exist yet, the `regenerate` flag is set to `true`.

If `regenerate` is `true`, the script invokes the `prisma generate` command to regenerate the Prisma client. To avoid potential deadlock issues with Cargo, the script temporarily sets the `CARGO_TARGET_DIR` environment variable to a different directory using the `temp_env` crate. This ensures that the `prisma generate` command, which itself invokes Cargo to build the Prisma client library, uses a separate directory for its build artifacts and doesn't conflict with the main build process.

By integrating this script into our build process, we ensure that the Prisma client is always up to date with the latest schema changes, without requiring manual intervention from developers. This helps maintain the reliability and consistency of our codebase as it scales.