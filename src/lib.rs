/*!
Ledger parser implemented with Pest.

 */

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "tests/ledger.pest"]
pub struct LedgerParser;
