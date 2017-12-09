# This script takes care of testing your crate

set -ex

main() {
    cross build --features nightly --target $TARGET

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --features nightly --target $TARGET
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
