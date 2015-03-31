for example in */example.rs; do
    dir=$(dirname $example)
    mv $example "${dir}/${dir}.rs"
done
