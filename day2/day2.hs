percentageCalculator :: (Double, Double) -> Double
percentageCalculator (baseprice, percentage) = baseprice * (percentage / 100)

main = do
  mealCost <- readLn :: IO Double 
  tipPercent <- readLn :: IO Double
  taxPercent <- readLn :: IO Double
  let tip = percentageCalculator (mealCost, tipPercent)
  let tax = percentageCalculator (mealCost, taxPercent)
  print (tip + tax + mealCost)
  -- You could also do it with anonymous lambdas!:D
  -- let tip = (\x y -> x * y / 100) (mealCost) (tipPercent)
  -- let tax = (\x y -> x * y / 100) (mealCost) (taxPercent)
  