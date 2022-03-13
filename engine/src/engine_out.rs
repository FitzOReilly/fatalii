use search::search::SearchResult;
use std::error::Error;

pub trait EngineOut {
    fn info(&self, search_result: Option<SearchResult>) -> Result<(), Box<dyn Error>>;

    fn best_move(&self, search_result: Option<SearchResult>) -> Result<(), Box<dyn Error>>;
}
