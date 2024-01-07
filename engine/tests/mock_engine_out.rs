use engine::EngineOut;
use movegen::r#move::Move;
use search::search::SearchResult;
use std::error::Error;

pub struct MockEngineOut {
    search_info_callback: Box<dyn Fn(Option<SearchResult>)>,
    best_move_callback: Box<dyn Fn(Option<Move>)>,
}

unsafe impl Send for MockEngineOut {}
unsafe impl Sync for MockEngineOut {}

impl EngineOut for MockEngineOut {
    fn info_depth_finished(
        &self,
        search_result: Option<SearchResult>,
    ) -> Result<(), Box<dyn Error>> {
        (self.search_info_callback)(search_result);
        Ok(())
    }

    fn info_string(&self, _s: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn best_move(&self, search_result: Option<Move>) -> Result<(), Box<dyn Error>> {
        (self.best_move_callback)(search_result);
        Ok(())
    }
}

impl MockEngineOut {
    pub fn new(
        search_info_callback: Box<dyn Fn(Option<SearchResult>)>,
        best_move_callback: Box<dyn Fn(Option<Move>)>,
    ) -> Self {
        Self {
            search_info_callback,
            best_move_callback,
        }
    }
}
