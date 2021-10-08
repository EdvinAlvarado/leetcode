import Data.List
import Data.Maybe

two_sum :: Integer -> [Integer] -> [Int]
two_sum target nums = [fromJust r | r <- filter (/=Nothing) [elemIndex (target-n) nums | (i,n) <- zip [0..] nums, Just i /= elemIndex (target-n) nums]]
