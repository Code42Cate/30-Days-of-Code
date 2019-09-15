main = do
  n <- readLn :: IO Int
  if n `mod` 2 > 0
      then putStrLn "Weird"
      else if n > 1 && n < 6
          then putStrLn "Not Weird"
          else if n > 5 && n < 21
              then putStrLn "Weird"
              else putStrLn "Not Weird"
