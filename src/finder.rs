use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub fn filter(needle: &str, haystack: &[String]) -> Option<Vec<String>> {
    let matcher = SkimMatcherV2::default();
    let mut res = haystack
        .iter()
        .filter_map(|candidate| {
            matcher
                .fuzzy_match(candidate.as_str(), needle)
                .map(|score| (candidate, score))
        })
        .collect::<Vec<_>>();
    res.sort_by(|a, b| b.1.cmp(&a.1));
    Some(res.into_iter().map(|(s, _)| s.clone()).collect())
}
