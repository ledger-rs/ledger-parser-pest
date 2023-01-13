use ledger_pest_parser::{LedgerParser, Rule};
use pest::{Parser, iterators::Pairs};

#[test]
fn test_running() {
    // todo: read ledger journal file
    let data = "2023-01-01";

    let result = LedgerParser::parse(Rule::file, data);
    let pairs: Pairs<Rule>;
    match result {
        Ok(pairs_int) => pairs = pairs_int,
        Err(e) => {
            panic!("Error parsing: {:?}", e);
        },
    }

    for pair in pairs {
        //pair.
        println!("{:?}", pair);
    }
}
