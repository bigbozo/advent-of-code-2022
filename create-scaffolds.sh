for x (14 15 16 17 18 19 20 21 22 23 24 25); do
  touch input/day${x}.txt
  touch input/day${x}-test.txt
  mkdir src/day${x}
  touch src/day${x}/mod.rs
done
