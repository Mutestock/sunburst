pub trait ExtractionHelper {
    fn before(self, search_term: &str) -> String;
    fn after(self, search_term: &str) -> String;
    fn at(self, search_term: &str, index: usize) -> String;
}

impl ExtractionHelper for String {
    fn before(self, search_term: &str) -> String {
        self.split(search_term).next().unwrap().to_owned()
    }

    fn after(self, search_term: &str) -> String {
        self.split(search_term).last().unwrap().to_owned()
    }

    fn at(self, search_term: &str, index: usize) -> String {
        self.split(search_term).nth(index).unwrap().to_owned()
    }
}

