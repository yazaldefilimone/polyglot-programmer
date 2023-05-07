(defn divisivel-por? [dividendo divisor]
  (zero? (mod dividendo divisor)))


(defn fizzBuzz [numero]
  (cond (and (divisivel-por? numero 3)
        (divisivel-por? numero 5)) "fizzbuzz"
        (divisivel-por? numero 3) "fizz"
        (divisivel-por? numero 5) "buzz"
        :else numero))
