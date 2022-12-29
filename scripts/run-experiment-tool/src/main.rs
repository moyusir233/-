use std::{path::PathBuf, process::Child};

fn main() {
    let bin_dir = PathBuf::from("/home/ubuntu/PPF_Paper_Source_Code-master/bin");
    let trace_dir = PathBuf::from("/home/ubuntu/PPF_Paper_Source_Code-master/traces");
    let log_dir = PathBuf::from("/home/ubuntu/PPF_Paper_Source_Code-master/my_result");
    let log_suffix = "2";

    let traces: Vec<_> = [
        "603.bwaves_s-1080B.champsimtrace.xz",
        "605.mcf_s-1152B.champsimtrace.xz",
        "623.xalancbmk_s-10B.champsimtrace.xz",
        "649.fotonik3d_s-10881B.champsimtrace.xz",
    ]
    .into_iter()
    .map(|trace| trace_dir.join(trace))
    .collect();

    let binaries = vec![
        // "perceptron-no-no-lru-4core",
        // "perceptron-no-bop-lru-4core",
        // "perceptron-no-daampm-lru-4core",
        // "perceptron-no-spp_dev-lru-4core",
        // "perceptron-no-spp_dev_orig-lru-4core",
        // "perceptron-no-spp_dev_zhu-lru-4core",
        "perceptron-no-no-lru-1core",
        "perceptron-no-bop-lru-1core",
        "perceptron-no-daampm-lru-1core",
        "perceptron-no-spp_dev-lru-1core",
        "perceptron-no-spp_dev_orig-lru-1core",
    ];

    let children: Vec<_> = binaries
        .into_iter()
        .flat_map(|binary| {
            let core_num = binary
                .trim_end_matches("core")
                .as_bytes()
                .last()
                .unwrap_or(&1)
                - b'0';

            traces.chunks_exact(core_num as usize).map(|traces| {
                let binary = binary.to_owned();
                run_exper_process(
                    bin_dir.clone().join(&binary),
                    traces,
                    log_dir
                        .join(format!(
                            "{}{}{}",
                            binary,
                            traces
                                .iter()
                                .map(|trace| trace.file_name().unwrap().to_str().unwrap())
                                .collect::<Vec<_>>()
                                .join("_"),
                            log_suffix
                        ))
                        .with_extension("log"),
                )
                .unwrap()
            })
        })
        .collect();

    std::thread::scope(|s| {
        children.into_iter().for_each(|child| {
            s.spawn(|| match child.wait_with_output() {
                Ok(output) => println!("child exit status:{}", output.status),
                Err(err) => eprintln!("process error:{}", err),
            });
        });
    });
}

fn run_exper_process<Path: AsRef<std::path::Path>>(
    binary_path: Path,
    traces: &[Path],
    log_path: Path,
) -> Result<Child, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::process::Command;
    let mut command = Command::new(binary_path.as_ref());

    command
        .args(["-warmup_instructions", "20000000"])
        .args(["-simulation_instructions", "10000000"])
        .arg("-traces")
        .args(traces.iter().map(|trace| trace.as_ref()))
        .stdout(File::create(log_path)?);
    println!(
        "begin to experiment:{:?} {:?}",
        command.get_program(),
        command.get_args().collect::<Vec<_>>()
    );

    command.spawn().map_err(|err| err.into())
}
