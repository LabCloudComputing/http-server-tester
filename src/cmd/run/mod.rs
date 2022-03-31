/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 23:43:14
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-03-31 21:53:42
 * @FilePath: /http-server-tester/src/cmd/run/mod.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

pub mod http;
pub mod perf;
pub mod pipe;
pub mod proxy;

pub use http::http;
pub use perf::performance;
pub use pipe::pipelining;
pub use proxy::proxy;

pub fn compare_result(
    test: &str,
    path: &str,
    file: &str,
    content: &str,
    output: &str,
) -> Result<(), ()> {
    if content != output {
        error!(
            "The content of path {} is different from source {}\n",
            path, file
        );
        debug!("You should recv:\n{}", content);
        debug!("You actually recv:\n{}\n", output);
        Err(())
    } else {
        info!("{}: Path {} pass.", test, path);
        Ok(())
    }
}
