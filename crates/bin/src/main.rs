use std::time::Instant;

use binstalk::{helpers::jobserver_client::LazyJobserverClient, TARGET};
use log::LevelFilter;
use tracing::debug;

use cargo_binstall::{
    args,
    bin_util::{run_tokio_main, MainExit},
    entry,
    logging::logging,
};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> MainExit {
    // This must be the very first thing to happen
    let jobserver_client = LazyJobserverClient::new();

    let args = args::parse();

    if args.version {
        if args.verbose {
            let build_date = env!("VERGEN_BUILD_DATE");

            let features = env!("VERGEN_CARGO_FEATURES");

            let git_sha = option_env!("VERGEN_GIT_SHA").unwrap_or("UNKNOWN");
            let git_commit_date = option_env!("VERGEN_GIT_COMMIT_DATE").unwrap_or("UNKNOWN");

            let rustc_semver = env!("VERGEN_RUSTC_SEMVER");
            let rustc_commit_hash = env!("VERGEN_RUSTC_COMMIT_HASH");
            let rustc_llvm_version = env!("VERGEN_RUSTC_LLVM_VERSION");

            println!(
                r#"build-date: {build_date}
build-target: {TARGET}
build-features: {features}
build-commit-hash: {git_sha}
build-commit-date: {git_commit_date}
rustc-version: {rustc_semver}
rustc-commit-hash: {rustc_commit_hash}
rustc-llvm-version: {rustc_llvm_version}"#
            );
        } else {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        MainExit::Success(None)
    } else {
        logging(
            args.log_level.unwrap_or(LevelFilter::Info),
            args.json_output,
        );

        let start = Instant::now();

        let result = run_tokio_main(|| entry::install_crates(args, jobserver_client));

        let done = start.elapsed();
        debug!("run time: {done:?}");

        MainExit::new(result, done)
    }
}
