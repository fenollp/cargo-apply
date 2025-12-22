fn touched() {
    fn contains<P: Pattern>(pat: P, subj: &str) -> bool {
        subj.contains(pat)
    }

    fn contains<P>(pat: P, subj: &str) -> bool
    where
        P: Pattern,
    {
        subj.contains(pat)
    }
}

// --

fn untouched() {
    fn contains(pat: impl Pattern, subj: &str) -> bool {
        subj.contains(pat)
    }
}
