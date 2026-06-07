use std::io::{self, IsTerminal, Write};

use clap::{Parser, Subcommand, ValueEnum};
use codesift_core::{Error, Result, Workspace};
use codesift_index::{IndexOptions, Indexer};
use codesift_query::{
    CodeIntel, QueryExecutor, RenderOptions, invalid_query, parse_query_with_hints,
    render_query_hits, render_refs_tree,
};
use codesift_store::IndexStore;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Json,
    Table,
    Plain,
    Ascii,
}

#[derive(Parser)]
#[command(
    name = "codesift",
    about = "IntelliJ-grade code intelligence for agents"
)]
struct Cli {
    #[arg(long, default_value = ".")]
    workspace: camino::Utf8PathBuf,

    #[arg(long)]
    index_path: Option<camino::Utf8PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(short, long)]
    quiet: bool,

    #[arg(long, value_enum)]
    format: Option<OutputFormat>,

    #[arg(long)]
    no_color: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Index {
        path: Option<camino::Utf8PathBuf>,
        #[arg(long)]
        force: bool,
        #[arg(short = 'j', long)]
        jobs: Option<usize>,
    },
    Query {
        query: String,
    },
    Symbol {
        id: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        path: Option<String>,
    },
    Refs {
        id: Option<String>,
        #[arg(long)]
        name: Option<String>,
    },
    Status,
    Export {
        #[arg(long, default_value = "jsonl")]
        format: String,
        #[arg(long)]
        output: Option<camino::Utf8PathBuf>,
        #[arg(long)]
        r#type: Option<String>,
    },
    Mcp,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    init_logging(cli.verbose, cli.quiet);

    let mut workspace = Workspace::discover(&cli.workspace)?;
    if let Some(index_path) = cli.index_path.or_else(index_path_from_env) {
        workspace = workspace.with_index_dir(index_path);
    }

    let format = cli.format.unwrap_or_else(default_format);
    let render_opts = RenderOptions {
        color: !cli.no_color && io::stdout().is_terminal(),
    };

    match cli.command {
        Commands::Index { path, force, jobs } => {
            if let Some(path) = path {
                workspace = Workspace::discover(path)?;
            }
            let report = Indexer::index(
                &workspace,
                &IndexOptions {
                    force,
                    jobs: jobs.unwrap_or_else(|| IndexOptions::default().jobs),
                },
            )?;
            print_json(&report, format)?;
        }
        Commands::Query { query } => {
            let store = open_store(&workspace)?;
            let response = match parse_query_with_hints(&query) {
                Ok(parsed) => QueryExecutor::new(&store).execute(&parsed)?,
                Err(err) => invalid_query(&query, err.to_string()),
            };
            print_query(&response, format, &render_opts)?;
        }
        Commands::Symbol { id, name, path } => {
            let intel = CodeIntel::open(&workspace)?;
            let symbols = intel.resolve_symbol(
                name.as_deref(),
                id.as_deref(),
                None,
                path.as_deref(),
            )?;
            print_symbols(&symbols, format)?;
        }
        Commands::Refs { id, name } => {
            let store = open_store(&workspace)?;
            let executor = QueryExecutor::new(&store);
            if let Some(name) = name {
                let hits = executor.refs_by_name(&name)?;
                let response = codesift_query::QueryResponse {
                    query: format!("refs --name {name}"),
                    workspace_rev: store.meta.workspace_rev,
                    took_ms: 0,
                    hits: hits.clone(),
                    total: hits.len(),
                    error: None,
                };
                if format == OutputFormat::Ascii {
                    print!(
                        "{}",
                        render_refs_tree(&name, &hits, &render_opts)
                    );
                } else {
                    print_query(&response, format, &render_opts)?;
                }
            } else if let Some(id) = id {
                let query = format!("refs:to={id}");
                let parsed = parse_query_with_hints(&query)?;
                let response = executor.execute(&parsed)?;
                print_query(&response, format, &render_opts)?;
            } else {
                return Err(Error::message("provide symbol id or --name"));
            }
        }
        Commands::Status => {
            let intel = CodeIntel::open(&workspace)?;
            let status = intel.index_status();
            if format == OutputFormat::Ascii {
                println!("{}", codesift_query::render_status(&status));
            } else {
                print_json(&status, format)?;
            }
        }
        Commands::Export { format, output, r#type } => {
            if format != "jsonl" {
                return Err(Error::message("only --format jsonl is supported in MVP"));
            }
            let store = open_store(&workspace)?;
            export_jsonl(&store, output.as_deref(), r#type.as_deref())?;
        }
        Commands::Mcp => {
            let intel = CodeIntel::open(&workspace)?;
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| Error::message(e.to_string()))?;
            rt.block_on(codesift_mcp::run_stdio(intel))
                .map_err(|e| Error::message(e.to_string()))?;
        }
    }

    Ok(())
}

fn index_path_from_env() -> Option<camino::Utf8PathBuf> {
    std::env::var("CODESIFT_INDEX_PATH")
        .ok()
        .map(camino::Utf8PathBuf::from)
}

fn open_store(workspace: &Workspace) -> Result<IndexStore> {
    if !workspace.index_dir.exists() {
        return Err(Error::message(
            "index not found; run `codesift index` first",
        ));
    }
    IndexStore::open(&workspace.index_dir)
}

fn default_format() -> OutputFormat {
    if io::stdout().is_terminal() {
        OutputFormat::Table
    } else {
        OutputFormat::Json
    }
}

fn init_logging(verbose: u8, quiet: bool) {
    if quiet {
        return;
    }
    let level = match verbose {
        0 => "warn",
        1 => "info",
        _ => "debug",
    };
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level)),
        )
        .try_init();
}

fn print_json<T: serde::Serialize + ?Sized>(value: &T, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json | OutputFormat::Plain | OutputFormat::Ascii => {
            println!(
                "{}",
                serde_json::to_string_pretty(value).map_err(|e| Error::message(e.to_string()))?
            );
        }
        OutputFormat::Table => {
            println!(
                "{}",
                serde_json::to_string_pretty(value).map_err(|e| Error::message(e.to_string()))?
            );
        }
    }
    Ok(())
}

fn print_query(
    response: &codesift_query::QueryResponse,
    format: OutputFormat,
    render_opts: &RenderOptions,
) -> Result<()> {
    match format {
        OutputFormat::Json | OutputFormat::Plain => print_json(response, format)?,
        OutputFormat::Ascii => {
            if let Some(err) = &response.error {
                println!("error {}: {}", err.code, err.message);
            } else {
                print!("{}", render_query_hits(response, render_opts));
            }
        }
        OutputFormat::Table => {
            if let Some(err) = &response.error {
                println!("error {}: {}", err.code, err.message);
            } else if response.hits.is_empty() {
                println!("no results");
            } else {
                for hit in &response.hits {
                    if let Some(site) = &hit.site {
                        let depth = hit
                            .depth
                            .map(|d| format!(" depth={d}"))
                            .unwrap_or_default();
                        println!(
                            "{} {} {}:{}:{}-{}{}",
                            hit.symbol.kind.as_str(),
                            hit.symbol.name,
                            site.path,
                            site.start_line,
                            site.start_column,
                            hit.symbol.location.end_line,
                            depth
                        );
                    } else {
                        println!(
                            "{} {} {}:{}-{}",
                            hit.symbol.kind.as_str(),
                            hit.symbol.name,
                            hit.symbol.path,
                            hit.symbol.location.start_line,
                            hit.symbol.location.end_line
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn print_symbols(symbols: &[codesift_store::SymbolRecord], format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json | OutputFormat::Plain | OutputFormat::Ascii => {
            print_json(symbols, format)?;
        }
        OutputFormat::Table => {
            for sym in symbols {
                println!(
                    "{} {} {}:{}-{}",
                    sym.kind.as_str(),
                    sym.name,
                    sym.path,
                    sym.location.start_line,
                    sym.location.end_line
                );
            }
        }
    }
    Ok(())
}

fn export_jsonl(
    store: &IndexStore,
    output: Option<&camino::Utf8Path>,
    record_type: Option<&str>,
) -> Result<()> {
    let mut writer: Box<dyn Write> = if let Some(path) = output {
        Box::new(std::fs::File::create(path).map_err(|source| Error::Io {
            path: path.to_string(),
            source,
        })?)
    } else {
        Box::new(io::stdout())
    };

    let export_symbols = record_type.is_none() || record_type == Some("symbol");
    let export_refs = record_type.is_none() || record_type == Some("ref");
    let export_edges = record_type.is_none() || record_type == Some("edge");

    if export_symbols {
        for symbol in store.all_symbols()? {
            let line = serde_json::json!({"type":"symbol","data":symbol});
            writeln!(writer, "{}", line).map_err(|source| Error::Io {
                path: "stdout".to_string(),
                source,
            })?;
        }
    }

    if export_refs {
        for reference in store.all_refs()? {
            let line = serde_json::json!({"type":"ref","data":reference});
            writeln!(writer, "{}", line).map_err(|source| Error::Io {
                path: "stdout".to_string(),
                source,
            })?;
        }
    }

    if export_edges {
        for edge in store.all_edges()? {
            let line = serde_json::json!({"type":"edge","data":edge});
            writeln!(writer, "{}", line).map_err(|source| Error::Io {
                path: "stdout".to_string(),
                source,
            })?;
        }
    }

    Ok(())
}
