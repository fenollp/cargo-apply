.PHONY: test

all:

test: \
	test.rename_method \
#	test.replace_generic_bound_with_impl_trait

test.%:
	@printf '\n%s\n' $*
	! git status --porcelain -- test/patch/$*__*.rs | grep '^.M'
	cargo +nightly fmt --all -- test/patch/$*__*.rs
	! git status --porcelain -- test/patch/$*__*.rs | grep '^.M'
	cfr --apply --suppress-diff --verbose --coccifile test/patch/$*.cocci test/patch/$*__before.rs
	diff --width=150 -y test/patch/$*__after.rs test/patch/$*__before.rs
	git checkout -- test/patch/$*__before.rs
