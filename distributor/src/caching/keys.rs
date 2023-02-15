pub const READ_LIST_ARTICLE_CACHE_KEY: &'static str = "article_list_all";
pub const READ_LIST_ARTICLE_SEARCH_TERM_CACHE_KEY_PREFIX: &'static str = "article_search_term => ";
pub const READ_LIST_ARTICLE_SITE_CACHE_KEY_PREFIX: &'static str = "article_site => ";
pub const READ_ARTICLE_COUNT_SEARCH_SITE: &'static str = "article_cnt_search_site => ";

pub fn formatted_search_term_cache_key(search_term: &str) -> String {
    format!(
        "{}{}",
        READ_LIST_ARTICLE_SEARCH_TERM_CACHE_KEY_PREFIX, search_term
    )
}

pub fn formatted_site_cache_key(site: &str) -> String {
    format!("{}{}", READ_LIST_ARTICLE_SITE_CACHE_KEY_PREFIX, site)
}

pub fn formatted_cnt_search_site_cache_key(search_term: &str, site: &str) -> String {
    format!(
        "{}{} - {}",
        READ_ARTICLE_COUNT_SEARCH_SITE, search_term, site
    )
}
