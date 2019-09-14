main = do
  let a = 3
  let b = 4.0
  let c = "HackerRank "  
  int <- getLine
  print $ a + (read int :: Int)
  double <- getLine
  print $ b + (read double :: Double)
  string <-getLine
  putStrLn(c ++ string)
