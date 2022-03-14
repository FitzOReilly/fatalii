use search::search::SearchResult;
use std::error::Error;

pub trait EngineOut {
    fn info_depth_finished(
        &self,
        search_result: Option<SearchResult>,
    ) -> Result<(), Box<dyn Error>>;

    fn info_string(&self, s: &str) -> Result<(), Box<dyn Error>>;

    fn best_move(&self, search_result: Option<SearchResult>) -> Result<(), Box<dyn Error>>;
}
