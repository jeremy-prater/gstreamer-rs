// Generated by gir (https://github.com/gtk-rs/gir @ b5068ede6c51)
// from gir-files (https://github.com/gtk-rs/gir-files @ 01c4ec663b3f)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 40cea11af0f3)
// DO NOT EDIT

use gstreamer_validate_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gstreamer-validate-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GstValidateAction",
        Layout {
            size: size_of::<GstValidateAction>(),
            alignment: align_of::<GstValidateAction>(),
        },
    ),
    (
        "GstValidateActionParameter",
        Layout {
            size: size_of::<GstValidateActionParameter>(),
            alignment: align_of::<GstValidateActionParameter>(),
        },
    ),
    (
        "GstValidateActionReturn",
        Layout {
            size: size_of::<GstValidateActionReturn>(),
            alignment: align_of::<GstValidateActionReturn>(),
        },
    ),
    (
        "GstValidateActionType",
        Layout {
            size: size_of::<GstValidateActionType>(),
            alignment: align_of::<GstValidateActionType>(),
        },
    ),
    (
        "GstValidateActionTypeFlags",
        Layout {
            size: size_of::<GstValidateActionTypeFlags>(),
            alignment: align_of::<GstValidateActionTypeFlags>(),
        },
    ),
    (
        "GstValidateBinMonitor",
        Layout {
            size: size_of::<GstValidateBinMonitor>(),
            alignment: align_of::<GstValidateBinMonitor>(),
        },
    ),
    (
        "GstValidateBinMonitorClass",
        Layout {
            size: size_of::<GstValidateBinMonitorClass>(),
            alignment: align_of::<GstValidateBinMonitorClass>(),
        },
    ),
    (
        "GstValidateDebugFlags",
        Layout {
            size: size_of::<GstValidateDebugFlags>(),
            alignment: align_of::<GstValidateDebugFlags>(),
        },
    ),
    (
        "GstValidateElementMonitor",
        Layout {
            size: size_of::<GstValidateElementMonitor>(),
            alignment: align_of::<GstValidateElementMonitor>(),
        },
    ),
    (
        "GstValidateElementMonitorClass",
        Layout {
            size: size_of::<GstValidateElementMonitorClass>(),
            alignment: align_of::<GstValidateElementMonitorClass>(),
        },
    ),
    (
        "GstValidateInterceptionReturn",
        Layout {
            size: size_of::<GstValidateInterceptionReturn>(),
            alignment: align_of::<GstValidateInterceptionReturn>(),
        },
    ),
    (
        "GstValidateIssue",
        Layout {
            size: size_of::<GstValidateIssue>(),
            alignment: align_of::<GstValidateIssue>(),
        },
    ),
    (
        "GstValidateIssueFlags",
        Layout {
            size: size_of::<GstValidateIssueFlags>(),
            alignment: align_of::<GstValidateIssueFlags>(),
        },
    ),
    (
        "GstValidateIssueId",
        Layout {
            size: size_of::<GstValidateIssueId>(),
            alignment: align_of::<GstValidateIssueId>(),
        },
    ),
    (
        "GstValidateMediaDescriptor",
        Layout {
            size: size_of::<GstValidateMediaDescriptor>(),
            alignment: align_of::<GstValidateMediaDescriptor>(),
        },
    ),
    (
        "GstValidateMediaDescriptorClass",
        Layout {
            size: size_of::<GstValidateMediaDescriptorClass>(),
            alignment: align_of::<GstValidateMediaDescriptorClass>(),
        },
    ),
    (
        "GstValidateMediaDescriptorParser",
        Layout {
            size: size_of::<GstValidateMediaDescriptorParser>(),
            alignment: align_of::<GstValidateMediaDescriptorParser>(),
        },
    ),
    (
        "GstValidateMediaDescriptorParserClass",
        Layout {
            size: size_of::<GstValidateMediaDescriptorParserClass>(),
            alignment: align_of::<GstValidateMediaDescriptorParserClass>(),
        },
    ),
    (
        "GstValidateMediaDescriptorWriter",
        Layout {
            size: size_of::<GstValidateMediaDescriptorWriter>(),
            alignment: align_of::<GstValidateMediaDescriptorWriter>(),
        },
    ),
    (
        "GstValidateMediaDescriptorWriterClass",
        Layout {
            size: size_of::<GstValidateMediaDescriptorWriterClass>(),
            alignment: align_of::<GstValidateMediaDescriptorWriterClass>(),
        },
    ),
    (
        "GstValidateMediaDescriptorWriterFlags",
        Layout {
            size: size_of::<GstValidateMediaDescriptorWriterFlags>(),
            alignment: align_of::<GstValidateMediaDescriptorWriterFlags>(),
        },
    ),
    (
        "GstValidateMediaInfo",
        Layout {
            size: size_of::<GstValidateMediaInfo>(),
            alignment: align_of::<GstValidateMediaInfo>(),
        },
    ),
    (
        "GstValidateMonitor",
        Layout {
            size: size_of::<GstValidateMonitor>(),
            alignment: align_of::<GstValidateMonitor>(),
        },
    ),
    (
        "GstValidateMonitorClass",
        Layout {
            size: size_of::<GstValidateMonitorClass>(),
            alignment: align_of::<GstValidateMonitorClass>(),
        },
    ),
    (
        "GstValidateOverride",
        Layout {
            size: size_of::<GstValidateOverride>(),
            alignment: align_of::<GstValidateOverride>(),
        },
    ),
    (
        "GstValidateOverrideClass",
        Layout {
            size: size_of::<GstValidateOverrideClass>(),
            alignment: align_of::<GstValidateOverrideClass>(),
        },
    ),
    (
        "GstValidateOverrideRegistry",
        Layout {
            size: size_of::<GstValidateOverrideRegistry>(),
            alignment: align_of::<GstValidateOverrideRegistry>(),
        },
    ),
    (
        "GstValidatePadMonitor",
        Layout {
            size: size_of::<GstValidatePadMonitor>(),
            alignment: align_of::<GstValidatePadMonitor>(),
        },
    ),
    (
        "GstValidatePadMonitorClass",
        Layout {
            size: size_of::<GstValidatePadMonitorClass>(),
            alignment: align_of::<GstValidatePadMonitorClass>(),
        },
    ),
    (
        "GstValidatePipelineMonitor",
        Layout {
            size: size_of::<GstValidatePipelineMonitor>(),
            alignment: align_of::<GstValidatePipelineMonitor>(),
        },
    ),
    (
        "GstValidatePipelineMonitorClass",
        Layout {
            size: size_of::<GstValidatePipelineMonitorClass>(),
            alignment: align_of::<GstValidatePipelineMonitorClass>(),
        },
    ),
    (
        "GstValidateReport",
        Layout {
            size: size_of::<GstValidateReport>(),
            alignment: align_of::<GstValidateReport>(),
        },
    ),
    (
        "GstValidateReportLevel",
        Layout {
            size: size_of::<GstValidateReportLevel>(),
            alignment: align_of::<GstValidateReportLevel>(),
        },
    ),
    (
        "GstValidateReporterInterface",
        Layout {
            size: size_of::<GstValidateReporterInterface>(),
            alignment: align_of::<GstValidateReporterInterface>(),
        },
    ),
    (
        "GstValidateReportingDetails",
        Layout {
            size: size_of::<GstValidateReportingDetails>(),
            alignment: align_of::<GstValidateReportingDetails>(),
        },
    ),
    (
        "GstValidateRunner",
        Layout {
            size: size_of::<GstValidateRunner>(),
            alignment: align_of::<GstValidateRunner>(),
        },
    ),
    (
        "GstValidateRunnerClass",
        Layout {
            size: size_of::<GstValidateRunnerClass>(),
            alignment: align_of::<GstValidateRunnerClass>(),
        },
    ),
    (
        "GstValidateScenario",
        Layout {
            size: size_of::<GstValidateScenario>(),
            alignment: align_of::<GstValidateScenario>(),
        },
    ),
    (
        "GstValidateScenarioClass",
        Layout {
            size: size_of::<GstValidateScenarioClass>(),
            alignment: align_of::<GstValidateScenarioClass>(),
        },
    ),
    (
        "GstValidateStructureResolveVariablesFlags",
        Layout {
            size: size_of::<GstValidateStructureResolveVariablesFlags>(),
            alignment: align_of::<GstValidateStructureResolveVariablesFlags>(),
        },
    ),
    (
        "GstValidateVerbosityFlags",
        Layout {
            size: size_of::<GstValidateVerbosityFlags>(),
            alignment: align_of::<GstValidateVerbosityFlags>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(guint) GST_VALIDATE_ACTION_TYPE_ASYNC", "4"),
    ("(guint) GST_VALIDATE_ACTION_TYPE_CAN_BE_OPTIONAL", "128"),
    (
        "(guint) GST_VALIDATE_ACTION_TYPE_CAN_EXECUTE_ON_ADDITION",
        "16",
    ),
    ("(guint) GST_VALIDATE_ACTION_TYPE_CHECK", "1024"),
    ("(guint) GST_VALIDATE_ACTION_TYPE_CONFIG", "2"),
    (
        "(guint) GST_VALIDATE_ACTION_TYPE_DOESNT_NEED_PIPELINE",
        "256",
    ),
    ("(guint) GST_VALIDATE_ACTION_TYPE_HANDLED_IN_CONFIG", "512"),
    ("(guint) GST_VALIDATE_ACTION_TYPE_INTERLACED", "8"),
    ("(guint) GST_VALIDATE_ACTION_TYPE_NEEDS_CLOCK", "32"),
    ("(guint) GST_VALIDATE_ACTION_TYPE_NONE", "0"),
    ("(guint) GST_VALIDATE_ACTION_TYPE_NON_BLOCKING", "8"),
    (
        "(guint) GST_VALIDATE_ACTION_TYPE_NO_EXECUTION_NOT_FATAL",
        "64",
    ),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_ASYNC", "2"),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_DONE", "7"),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_ERROR", "0"),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_ERROR_REPORTED", "4"),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_INTERLACED", "3"),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_IN_PROGRESS", "5"),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_NONE", "6"),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_NON_BLOCKING", "3"),
    ("(gint) GST_VALIDATE_EXECUTE_ACTION_OK", "1"),
    ("(guint) GST_VALIDATE_FATAL_CRITICALS", "4"),
    ("(guint) GST_VALIDATE_FATAL_DEFAULT", "0"),
    ("(guint) GST_VALIDATE_FATAL_ISSUES", "1"),
    ("(guint) GST_VALIDATE_FATAL_WARNINGS", "2"),
    ("(guint) GST_VALIDATE_ISSUE_FLAGS_FORCE_BACKTRACE", "4"),
    ("(guint) GST_VALIDATE_ISSUE_FLAGS_FULL_DETAILS", "1"),
    ("(guint) GST_VALIDATE_ISSUE_FLAGS_NONE", "0"),
    ("(guint) GST_VALIDATE_ISSUE_FLAGS_NO_BACKTRACE", "2"),
    (
        "(guint) GST_VALIDATE_MEDIA_DESCRIPTOR_WRITER_FLAGS_FULL",
        "4",
    ),
    (
        "(guint) GST_VALIDATE_MEDIA_DESCRIPTOR_WRITER_FLAGS_HANDLE_GLOGS",
        "8",
    ),
    (
        "(guint) GST_VALIDATE_MEDIA_DESCRIPTOR_WRITER_FLAGS_NONE",
        "1",
    ),
    (
        "(guint) GST_VALIDATE_MEDIA_DESCRIPTOR_WRITER_FLAGS_NO_PARSER",
        "2",
    ),
    ("(guint) GST_VALIDATE_PRINT_CRITICALS", "32"),
    ("(guint) GST_VALIDATE_PRINT_ISSUES", "8"),
    ("(guint) GST_VALIDATE_PRINT_WARNINGS", "16"),
    ("(gint) GST_VALIDATE_REPORTER_DROP", "0"),
    ("(gint) GST_VALIDATE_REPORTER_KEEP", "1"),
    ("(gint) GST_VALIDATE_REPORTER_REPORT", "2"),
    ("(gint) GST_VALIDATE_REPORT_LEVEL_CRITICAL", "0"),
    ("(gint) GST_VALIDATE_REPORT_LEVEL_EXPECTED", "5"),
    ("(gint) GST_VALIDATE_REPORT_LEVEL_IGNORE", "3"),
    ("(gint) GST_VALIDATE_REPORT_LEVEL_ISSUE", "2"),
    ("(gint) GST_VALIDATE_REPORT_LEVEL_NUM_ENTRIES", "6"),
    ("(gint) GST_VALIDATE_REPORT_LEVEL_UNKNOWN", "4"),
    ("(gint) GST_VALIDATE_REPORT_LEVEL_WARNING", "1"),
    ("(gint) GST_VALIDATE_SHOW_ALL", "5"),
    ("(gint) GST_VALIDATE_SHOW_COUNT", "7"),
    ("(gint) GST_VALIDATE_SHOW_MONITOR", "4"),
    ("(gint) GST_VALIDATE_SHOW_NONE", "1"),
    ("(gint) GST_VALIDATE_SHOW_SMART", "6"),
    ("(gint) GST_VALIDATE_SHOW_SUBCHAIN", "3"),
    ("(gint) GST_VALIDATE_SHOW_SYNTHETIC", "2"),
    ("(gint) GST_VALIDATE_SHOW_UNKNOWN", "0"),
    ("(guint) GST_VALIDATE_STRUCTURE_RESOLVE_VARIABLES_ALL", "0"),
    (
        "(guint) GST_VALIDATE_STRUCTURE_RESOLVE_VARIABLES_LOCAL_ONLY",
        "1",
    ),
    (
        "(guint) GST_VALIDATE_STRUCTURE_RESOLVE_VARIABLES_NO_EXPRESSION",
        "2",
    ),
    (
        "(guint) GST_VALIDATE_STRUCTURE_RESOLVE_VARIABLES_NO_FAILURE",
        "2",
    ),
    ("GST_VALIDATE_UNKNOWN_BOOL", "-1"),
    ("GST_VALIDATE_UNKNOWN_UINT64", "-1"),
    ("(guint) GST_VALIDATE_VERBOSITY_ALL", "30"),
    ("(guint) GST_VALIDATE_VERBOSITY_MESSAGES", "4"),
    ("(guint) GST_VALIDATE_VERBOSITY_NEW_ELEMENTS", "16"),
    ("(guint) GST_VALIDATE_VERBOSITY_NONE", "0"),
    ("(guint) GST_VALIDATE_VERBOSITY_POSITION", "2"),
    ("(guint) GST_VALIDATE_VERBOSITY_PROPS_CHANGES", "8"),
];
