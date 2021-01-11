use reqwest;
use structopt::{clap::arg_enum, StructOpt};
use url::{ParseError, Url};

arg_enum! {
    #[derive(Debug)]
    enum GraphQLType {
        Query,
        Mutation,
    }
}

fn url_parse_default_base(url: &str) -> Result<Url, ParseError> {
    match Url::parse(url) {
        Err(ParseError::RelativeUrlWithoutBase) => Url::parse(&format!("https://{}", url)),
        result => result,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "gql", about = "A graphql client.")]
struct Opt {
    /// The graphql server address
    #[structopt(parse(try_from_str = url_parse_default_base))]
    url: Url,
    /// The graphql query type
    #[structopt(short, long, default_value = "Query", possible_values = &GraphQLType::variants(), case_insensitive = true)]
    graphql_type: GraphQLType,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    println!("{:?}", opt);

    let client = reqwest::Client::new();
    let result = client.post(opt.url).send().await?;

    println!("{:?}", result);

    Ok(())
}
